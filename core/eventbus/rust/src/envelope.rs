use cat_kernel::EntityId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{CatEvent, EventBusError, EventBusResult};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Domain,
    Integration,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub event_id: Uuid,
    pub event_type: String,
    pub version: u16,
    pub kind: EventKind,
    pub occurred_at_ms: u64,
    pub producer: String,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub subject_id: Option<EntityId>,
    pub payload: Value,
}

impl EventEnvelope {
    pub fn from_typed<T: CatEvent>(event: T, producer: impl Into<String>) -> EventBusResult<Self> {
        Ok(Self {
            event_id: Uuid::now_v7(),
            event_type: T::TYPE.to_owned(),
            version: T::VERSION,
            kind: EventKind::Domain,
            occurred_at_ms: cat_kernel::time::unix_millis()?,
            producer: producer.into(),
            correlation_id: None,
            causation_id: None,
            subject_id: None,
            payload: serde_json::to_value(event)
                .map_err(|error| EventBusError::Serialization(error.to_string()))?,
        })
    }

    pub fn with_kind(mut self, kind: EventKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_causation_id(mut self, causation_id: Uuid) -> Self {
        self.causation_id = Some(causation_id);
        self
    }

    pub fn with_subject_id(mut self, subject_id: EntityId) -> Self {
        self.subject_id = Some(subject_id);
        self
    }
}
