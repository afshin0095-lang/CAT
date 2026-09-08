use crate::{KernelResult, ProjectionCheckpoint, ProjectionPhase, SequenceNumber, StoredEvent};

/// A deterministic projection applies canonical events to derived state.
pub trait ProjectionHandler<T, S>: Send {
    fn projection_id(&self) -> &str;
    fn apply(&mut self, state: &mut S, event: &StoredEvent<T>) -> KernelResult<()>;
}

/// In-process projection runtime with monotonic checkpoint semantics.
/// Persistence is deliberately outside this type. A durable adapter may save
/// the returned checkpoint after the derived-state mutation succeeds.
pub struct ProjectionRuntime<T, S, H> {
    state: S,
    checkpoint: ProjectionCheckpoint,
    handler: H,
    _event: std::marker::PhantomData<T>,
}

impl<T, S, H: ProjectionHandler<T, S>> ProjectionRuntime<T, S, H> {
    pub fn new(
        stream_id: crate::EntityId,
        initial_sequence: SequenceNumber,
        initial_event_id: crate::EventId,
        state: S,
        handler: H,
    ) -> KernelResult<Self> {
        let checkpoint = ProjectionCheckpoint::new(
            handler.projection_id(), stream_id, initial_sequence, initial_event_id, ProjectionPhase::Catchup,
        )?;
        Ok(Self { state, checkpoint, handler, _event: std::marker::PhantomData })
    }

    pub fn state(&self) -> &S { &self.state }
    pub fn state_mut(&mut self) -> &mut S { &mut self.state }
    pub fn checkpoint(&self) -> &ProjectionCheckpoint { &self.checkpoint }

    /// Applies one event only when it is the next sequence for this projection.
    pub fn apply(&mut self, event: &StoredEvent<T>) -> KernelResult<()> {
        if event.stream_id != self.checkpoint.stream_id {
            return Err(crate::KernelError::InvalidInput("projection stream mismatch".into()));
        }
        let sequence = event.envelope.sequence;
        if sequence < self.checkpoint.sequence { return Ok(()); }
        if sequence == self.checkpoint.sequence {
            if event.envelope.event_id == self.checkpoint.event_id { return Ok(()); }
            return Err(crate::KernelError::InvalidInput(
                "projection received a different event at an existing checkpoint sequence".into(),
            ));
        }
        let expected = self.checkpoint.sequence.next()?;
        if sequence != expected {
            return Err(crate::KernelError::SequenceConflict { expected, actual: sequence });
        }
        self.handler.apply(&mut self.state, event)?;
        let next = ProjectionCheckpoint::new(
            self.checkpoint.projection_id.clone(), self.checkpoint.stream_id, sequence,
            event.envelope.event_id, self.checkpoint.phase,
        )?;
        self.checkpoint.advance(&next)?;
        Ok(())
    }

    /// Applies an ordered catch-up batch and then marks the projection live.
    pub fn catch_up(&mut self, events: &[StoredEvent<T>]) -> KernelResult<()> {
        for event in events { self.apply(event)?; }
        self.checkpoint.phase = ProjectionPhase::Live;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CorrelationId, EntityId, EventEnvelope, EventId, TenantId, TimestampMs};

    struct SumProjection;
    impl ProjectionHandler<i64, u64> for SumProjection {
        fn projection_id(&self) -> &str { "sum-v1" }
        fn apply(&mut self, state: &mut u64, event: &StoredEvent<i64>) -> KernelResult<()> {
            let value = u64::try_from(event.envelope.payload)
                .map_err(|_| crate::KernelError::InvalidInput("projection payload must be non-negative".into()))?;
            *state += value;
            Ok(())
        }
    }

    fn event(stream: EntityId, sequence: u64, payload: i64) -> StoredEvent<i64> {
        StoredEvent { stream_id: stream, envelope: EventEnvelope::new(
            "cat.test.projection", 1, TenantId::new(), CorrelationId::new(), None,
            EntityId::new(), TimestampMs::new(sequence), SequenceNumber::new(sequence), payload,
        ).unwrap() }
    }

    #[test]
    fn catchup_applies_ordered_events_and_enters_live_phase() {
        let stream = EntityId::new();
        let mut runtime = ProjectionRuntime::new(stream, SequenceNumber::ZERO, EventId::new(), 0, SumProjection).unwrap();
        let first = event(stream, 1, 3);
        let second = event(stream, 2, 4);
        runtime.catch_up(&[first, second]).unwrap();
        assert_eq!(*runtime.state(), 7);
        assert_eq!(runtime.checkpoint().sequence, SequenceNumber::new(2));
        assert_eq!(runtime.checkpoint().phase, ProjectionPhase::Live);
    }

    #[test]
    fn duplicate_event_is_idempotent() {
        let stream = EntityId::new();
        let mut runtime = ProjectionRuntime::new(stream, SequenceNumber::ZERO, EventId::new(), 0, SumProjection).unwrap();
        let first = event(stream, 1, 5);
        runtime.apply(&first).unwrap();
        runtime.apply(&first).unwrap();
        assert_eq!(*runtime.state(), 5);
    }

    #[test]
    fn gaps_are_rejected() {
        let stream = EntityId::new();
        let mut runtime = ProjectionRuntime::new(stream, SequenceNumber::ZERO, EventId::new(), 0, SumProjection).unwrap();
        let gap = event(stream, 2, 5);
        assert!(matches!(runtime.apply(&gap), Err(crate::KernelError::SequenceConflict { .. })));
    }
}
