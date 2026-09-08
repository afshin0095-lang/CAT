use crate::{EntityId, EventId, KernelError, KernelResult, SequenceNumber};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionPhase { Catchup, Live }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionCheckpoint {
    pub projection_id: String,
    pub stream_id: EntityId,
    pub sequence: SequenceNumber,
    pub event_id: EventId,
    pub phase: ProjectionPhase,
}

impl ProjectionCheckpoint {
    pub fn new(projection_id: impl Into<String>, stream_id: EntityId, sequence: SequenceNumber, event_id: EventId, phase: ProjectionPhase) -> KernelResult<Self> {
        let projection_id = projection_id.into();
        if projection_id.trim().is_empty() {
            return Err(KernelError::InvalidInput("projection_id must not be empty".into()));
        }
        Ok(Self { projection_id, stream_id, sequence, event_id, phase })
    }

    pub fn advance(&mut self, next: &Self) -> KernelResult<()> {
        if self.projection_id != next.projection_id || self.stream_id != next.stream_id {
            return Err(KernelError::InvalidInput("projection checkpoint identity mismatch".into()));
        }
        if next.sequence < self.sequence {
            return Err(KernelError::SequenceConflict { expected: self.sequence, actual: next.sequence });
        }
        if next.sequence == self.sequence && next.event_id != self.event_id {
            return Err(KernelError::InvalidInput("projection checkpoint cannot replace an event at the same sequence".into()));
        }
        self.sequence = next.sequence;
        self.event_id = next.event_id;
        self.phase = next.phase;
        Ok(())
    }

    pub fn is_caught_up(&self, target: SequenceNumber) -> bool { self.sequence >= target }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advances_monotonically_and_switches_to_live() {
        let stream = EntityId::new();
        let mut current = ProjectionCheckpoint::new("affiliate-summary-v1", stream, SequenceNumber::new(1), EventId::new(), ProjectionPhase::Catchup).unwrap();
        let next = ProjectionCheckpoint::new("affiliate-summary-v1", stream, SequenceNumber::new(2), EventId::new(), ProjectionPhase::Live).unwrap();
        current.advance(&next).unwrap();
        assert_eq!(current.sequence, SequenceNumber::new(2));
        assert_eq!(current.phase, ProjectionPhase::Live);
        assert!(current.is_caught_up(SequenceNumber::new(2)));
    }

    #[test]
    fn rejects_stale_checkpoint() {
        let stream = EntityId::new();
        let mut current = ProjectionCheckpoint::new("p", stream, SequenceNumber::new(2), EventId::new(), ProjectionPhase::Live).unwrap();
        let stale = ProjectionCheckpoint::new("p", stream, SequenceNumber::new(1), EventId::new(), ProjectionPhase::Catchup).unwrap();
        assert!(current.advance(&stale).is_err());
    }

    #[test]
    fn rejects_projection_identity_mismatch() {
        let stream = EntityId::new();
        let mut current = ProjectionCheckpoint::new("p1", stream, SequenceNumber::new(1), EventId::new(), ProjectionPhase::Catchup).unwrap();
        let other = ProjectionCheckpoint::new("p2", stream, SequenceNumber::new(2), EventId::new(), ProjectionPhase::Live).unwrap();
        assert!(current.advance(&other).is_err());
    }
}
