use crate::{EventEnvelope, EventBusError, EventBusResult};

/// Serialization boundary. Transport adapters never need to know CAT's concrete event types.
pub trait EventCodec: Send + Sync {
    fn encode(&self, event: &EventEnvelope) -> EventBusResult<Vec<u8>>;
    fn decode(&self, bytes: &[u8]) -> EventBusResult<EventEnvelope>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct JsonEventCodec;

impl EventCodec for JsonEventCodec {
    fn encode(&self, event: &EventEnvelope) -> EventBusResult<Vec<u8>> {
        serde_json::to_vec(event).map_err(|error| EventBusError::Serialization(error.to_string()))
    }

    fn decode(&self, bytes: &[u8]) -> EventBusResult<EventEnvelope> {
        serde_json::from_slice(bytes).map_err(|error| EventBusError::Serialization(error.to_string()))
    }
}

/// Quarantine boundary for poison events. DLQ is never a silent discard path.
pub trait DeadLetterStore: Send + Sync {
    fn park(&mut self, event: EventEnvelope, reason: impl Into<String>) -> EventBusResult<()>;
    fn len(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadLetter {
    pub event: EventEnvelope,
    pub reason: String,
}

#[derive(Debug, Default)]
pub struct InMemoryDeadLetterStore {
    entries: Vec<DeadLetter>,
}

impl InMemoryDeadLetterStore {
    pub fn entries(&self) -> &[DeadLetter] {
        &self.entries
    }
}

impl DeadLetterStore for InMemoryDeadLetterStore {
    fn park(&mut self, event: EventEnvelope, reason: impl Into<String>) -> EventBusResult<()> {
        self.entries.push(DeadLetter { event, reason: reason.into() });
        Ok(())
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}
