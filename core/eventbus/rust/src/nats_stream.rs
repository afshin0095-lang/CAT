use crate::{EventBusError, EventBusResult};
use async_nats::jetstream::{self, Context, stream};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct NatsStreamConfig {
    pub name: String,
    pub subjects: Vec<String>,
    pub max_age: Duration,
    pub max_messages: i64,
    pub max_bytes: i64,
    pub storage: stream::StorageType,
}

impl Default for NatsStreamConfig {
    fn default() -> Self {
        Self {
            name: "CAT_EVENTS".to_owned(),
            subjects: vec!["cat.events.>".to_owned()],
            max_age: Duration::from_secs(7 * 24 * 60 * 60),
            max_messages: -1,
            max_bytes: -1,
            storage: stream::StorageType::File,
        }
    }
}

impl NatsStreamConfig {
    pub fn validate(&self) -> EventBusResult<()> {
        if self.name.trim().is_empty() {
            return Err(EventBusError::InvalidConfiguration(
                "NATS stream name cannot be empty".into(),
            ));
        }
        if self.subjects.is_empty() || self.subjects.iter().any(|s| s.trim().is_empty()) {
            return Err(EventBusError::InvalidConfiguration(
                "NATS stream requires at least one subject".into(),
            ));
        }
        if self.max_messages == 0 || self.max_bytes == 0 {
            return Err(EventBusError::InvalidConfiguration(
                "stream limits must be -1 or positive".into(),
            ));
        }
        Ok(())
    }

    pub fn into_stream_config(self) -> EventBusResult<stream::Config> {
        self.validate()?;
        Ok(stream::Config {
            name: self.name,
            subjects: self.subjects,
            max_age: self.max_age,
            max_messages: self.max_messages,
            max_bytes: self.max_bytes,
            storage: self.storage,
            ..Default::default()
        })
    }
}

pub async fn ensure_stream(
    context: &Context,
    config: NatsStreamConfig,
) -> EventBusResult<jetstream::stream::Stream> {
    let config = config.into_stream_config()?;
    context
        .get_or_create_stream(config)
        .await
        .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_define_a_durable_cat_event_stream() {
        let config = NatsStreamConfig::default();
        assert_eq!(config.name, "CAT_EVENTS");
        assert_eq!(config.subjects, vec!["cat.events.>"]);
        assert_eq!(config.storage, stream::StorageType::File);
        assert!(config.max_age > Duration::ZERO);
    }

    #[test]
    fn invalid_limits_are_rejected_before_network_access() {
        let mut config = NatsStreamConfig::default();
        config.max_messages = 0;
        assert!(matches!(
            config.validate(),
            Err(EventBusError::InvalidConfiguration(_))
        ));

        config.max_messages = -1;
        config.max_bytes = 0;
        assert!(matches!(
            config.validate(),
            Err(EventBusError::InvalidConfiguration(_))
        ));
    }
}
