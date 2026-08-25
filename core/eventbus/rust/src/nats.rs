use crate::{EventBusError, EventBusResult, EventEnvelope};
use async_nats::jetstream::Context;
use async_trait::async_trait;
use std::time::Duration;

/// Async broker boundary for transports whose native API is asynchronous.
#[async_trait]
pub trait AsyncEventTransport: Send + Sync {
    async fn publish(&self, event: &EventEnvelope) -> EventBusResult<()>;
}

/// NATS JetStream transport for durable event publication.
#[derive(Clone)]
pub struct NatsJetStreamTransport {
    context: Context,
    subject_prefix: String,
    publish_timeout: Duration,
}

impl NatsJetStreamTransport {
    pub async fn connect(
        url: impl AsRef<str>,
        subject_prefix: impl Into<String>,
        publish_timeout: Duration,
    ) -> EventBusResult<Self> {
        let client = async_nats::connect(url.as_ref())
            .await
            .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;
        Ok(Self::from_context(
            async_nats::jetstream::new(client),
            subject_prefix,
            publish_timeout,
        ))
    }

    pub fn from_context(
        context: Context,
        subject_prefix: impl Into<String>,
        publish_timeout: Duration,
    ) -> Self {
        Self {
            context,
            subject_prefix: normalize_prefix(subject_prefix.into()),
            publish_timeout,
        }
    }

    pub fn subject_for(&self, event_type: &str) -> EventBusResult<String> {
        subject_for_prefix(&self.subject_prefix, event_type)
    }

    pub fn subject_prefix(&self) -> &str { &self.subject_prefix }
    pub fn publish_timeout(&self) -> Duration { self.publish_timeout }
}

#[async_trait]
impl AsyncEventTransport for NatsJetStreamTransport {
    async fn publish(&self, event: &EventEnvelope) -> EventBusResult<()> {
        let subject = self.subject_for(&event.event_type)?;
        let payload = serde_json::to_vec(event)
            .map_err(|error| EventBusError::Serialization(error.to_string()))?;

        let publish = self
            .context
            .send_publish(
                subject,
                async_nats::jetstream::message::PublishMessage::build()
                    .payload(payload.into())
                    .message_id(event.event_id.to_string()),
            )
            .await
            .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;

        tokio::time::timeout(self.publish_timeout, publish)
            .await
            .map_err(|_| EventBusError::TransportUnavailable("JetStream publish acknowledgement timed out".into()))?
            .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;
        Ok(())
    }
}

pub fn subject_for_prefix(prefix: &str, event_type: &str) -> EventBusResult<String> {
    validate_subject_fragment(event_type)?;
    Ok(format!("{}{}", normalize_prefix(prefix.to_owned()), event_type))
}

fn normalize_prefix(prefix: String) -> String {
    let prefix = prefix.trim().trim_matches('.');
    if prefix.is_empty() { String::new() } else { format!("{}.", prefix) }
}

fn validate_subject_fragment(value: &str) -> EventBusResult<()> {
    if value.is_empty()
        || value.starts_with('.')
        || value.ends_with('.')
        || value.contains(' ')
        || value.contains('*')
        || value.contains('>')
    {
        return Err(EventBusError::InvalidConfiguration(format!("invalid NATS subject fragment: {value}")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_prefix_is_normalized_without_double_dots() {
        assert_eq!(subject_for_prefix("cat.events...", "affiliate.created").unwrap(), "cat.events.affiliate.created");
        assert_eq!(subject_for_prefix("...", "affiliate.created").unwrap(), "affiliate.created");
    }

    #[test]
    fn subject_rejects_wildcards_and_invalid_fragments() {
        for invalid in ["", ".affiliate.created", "affiliate.created.", "affiliate created", "affiliate.*", "affiliate.>"] {
            assert!(matches!(subject_for_prefix("cat", invalid), Err(EventBusError::InvalidConfiguration(_))));
        }
    }

    #[test]
    fn subject_preserves_event_version_in_canonical_event_type() {
        assert_eq!(subject_for_prefix("cat.domain", "affiliate.created.v1").unwrap(), "cat.domain.affiliate.created.v1");
    }
}
