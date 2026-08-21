use crate::{EventBusError, EventBusResult, EventEnvelope, InboxStore};
use async_nats::jetstream::{consumer, AckKind, Context};
use async_trait::async_trait;
use futures_util::StreamExt;
use std::time::Duration;

/// Application handler invoked only after inbox admission succeeds.
#[async_trait]
pub trait AsyncEventHandler: Send + Sync {
    async fn handle(&self, event: EventEnvelope) -> EventBusResult<()>;
}

/// Durable pull-consumer configuration for CAT event processing.
#[derive(Clone, Debug)]
pub struct NatsConsumerConfig {
    pub durable_name: String,
    pub filter_subject: String,
    pub ack_wait: Duration,
    pub max_deliver: i64,
    pub max_ack_pending: i64,
    pub batch_size: usize,
}

impl Default for NatsConsumerConfig {
    fn default() -> Self {
        Self {
            durable_name: "cat-eventbus-worker".to_owned(),
            filter_subject: "cat.events.>".to_owned(),
            ack_wait: Duration::from_secs(30),
            max_deliver: 5,
            max_ack_pending: 1024,
            batch_size: 64,
        }
    }
}

/// Pull-based JetStream consumer that couples broker delivery to CAT inbox semantics.
///
/// Processing is at-least-once at the broker boundary. The InboxStore provides the
/// application-level idempotency boundary required for safe redelivery.
pub struct NatsJetStreamConsumer<I> {
    context: Context,
    stream_name: String,
    config: NatsConsumerConfig,
    inbox: I,
}

impl<I> NatsJetStreamConsumer<I>
where
    I: InboxStore,
{
    pub fn new(
        context: Context,
        stream_name: impl Into<String>,
        config: NatsConsumerConfig,
        inbox: I,
    ) -> Self {
        Self {
            context,
            stream_name: stream_name.into(),
            config,
            inbox,
        }
    }

    pub fn inbox(&self) -> &I {
        &self.inbox
    }

    pub fn inbox_mut(&mut self) -> &mut I {
        &mut self.inbox
    }

    pub async fn ensure_consumer(&self) -> EventBusResult<consumer::Consumer<consumer::pull::Config>> {
        let stream = self
            .context
            .get_stream(&self.stream_name)
            .await
            .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;

        let config = consumer::pull::Config {
            durable_name: Some(self.config.durable_name.clone()),
            filter_subject: self.config.filter_subject.clone(),
            ack_wait: self.config.ack_wait,
            max_deliver: self.config.max_deliver,
            max_ack_pending: self.config.max_ack_pending,
            ..Default::default()
        };

        stream
            .get_or_create_consumer(&self.config.durable_name, config)
            .await
            .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))
    }

    /// Process up to `batch_size` messages and return the number consumed.
    pub async fn process_batch<H>(&mut self, handler: &H) -> EventBusResult<usize>
    where
        H: AsyncEventHandler,
    {
        let consumer = self.ensure_consumer().await?;
        let mut messages = consumer
            .fetch()
            .max_messages(self.config.batch_size)
            .messages()
            .await
            .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;

        let mut processed = 0usize;

        while let Some(message) = messages.next().await {
            let message = message
                .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;

            let event: EventEnvelope = match serde_json::from_slice(&message.payload) {
                Ok(event) => event,
                Err(error) => {
                    message
                        .ack_with(AckKind::Term)
                        .await
                        .map_err(|ack| EventBusError::TransportUnavailable(ack.to_string()))?;
                    return Err(EventBusError::Serialization(error.to_string()));
                }
            };

            // A duplicate event is already being processed or has completed.
            // It is safe to acknowledge the broker delivery without invoking the handler.
            if !self.inbox.accept(event.event_id)? {
                message
                    .ack()
                    .await
                    .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;
                processed += 1;
                continue;
            }

            match handler.handle(event.clone()).await {
                Ok(()) => {
                    self.inbox.mark_succeeded(event.event_id)?;
                    // Broker ACK is deliberately last: application success is recorded
                    // before the durable delivery is acknowledged.
                    message
                        .double_ack()
                        .await
                        .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;
                    processed += 1;
                }
                Err(error) => {
                    self.inbox.mark_failed(event.event_id)?;
                    message
                        .ack_with(AckKind::Nak(None))
                        .await
                        .map_err(|ack| EventBusError::TransportUnavailable(ack.to_string()))?;
                    return Err(error);
                }
            }
        }

        Ok(processed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_consumer_policy_is_bounded() {
        let config = NatsConsumerConfig::default();
        assert_eq!(config.max_deliver, 5);
        assert_eq!(config.max_ack_pending, 1024);
        assert_eq!(config.batch_size, 64);
        assert_eq!(config.durable_name, "cat-eventbus-worker");
    }
}
