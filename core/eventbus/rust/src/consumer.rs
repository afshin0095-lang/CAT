use crate::{EventEnvelope, EventBusError, EventBusResult};
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct ConsumerContext {
    pub consumer_id: String,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
}

impl ConsumerContext {
    pub fn from_event(consumer_id: impl Into<String>, event: &EventEnvelope) -> Self {
        Self {
            consumer_id: consumer_id.into(),
            correlation_id: event.correlation_id,
            causation_id: Some(event.event_id),
        }
    }

    pub fn validate(&self) -> EventBusResult<()> {
        if self.consumer_id.trim().is_empty() {
            return Err(EventBusError::InvalidConsumer("consumer_id cannot be empty".into()));
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ConsumerReceipt {
    pub consumer_id: String,
    pub event_id: Uuid,
    pub accepted: bool,
    pub completed: bool,
}

impl ConsumerReceipt {
    pub fn accepted(consumer_id: impl Into<String>, event_id: Uuid) -> Self {
        Self {
            consumer_id: consumer_id.into(),
            event_id,
            accepted: true,
            completed: false,
        }
    }

    pub fn complete(mut self) -> Self {
        self.completed = true;
        self
    }
}
