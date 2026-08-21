use crate::{EventBusError, EventBusResult};
use uuid::Uuid;

/// Broker-neutral acknowledgement result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AckDecision {
    Ack,
    Retry,
    DeadLetter,
}

/// Acknowledgement boundary kept separate from publication so a broker adapter
/// can map native ACK/NACK semantics without leaking them into the EventBus.
pub trait TransportAcker: Send + Sync {
    fn ack(&mut self, event_id: Uuid) -> EventBusResult<()>;
    fn retry(&mut self, event_id: Uuid) -> EventBusResult<()>;
    fn dead_letter(&mut self, event_id: Uuid) -> EventBusResult<()>;
}

#[derive(Debug, Default)]
pub struct RecordingAcker {
    pub acked: Vec<Uuid>,
    pub retried: Vec<Uuid>,
    pub dead_lettered: Vec<Uuid>,
}

impl TransportAcker for RecordingAcker {
    fn ack(&mut self, event_id: Uuid) -> EventBusResult<()> {
        if self.acked.contains(&event_id) {
            return Err(EventBusError::DuplicateAcknowledgement(event_id));
        }
        self.acked.push(event_id);
        Ok(())
    }

    fn retry(&mut self, event_id: Uuid) -> EventBusResult<()> {
        self.retried.push(event_id);
        Ok(())
    }

    fn dead_letter(&mut self, event_id: Uuid) -> EventBusResult<()> {
        self.dead_lettered.push(event_id);
        Ok(())
    }
}
