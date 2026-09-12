use cat_kernel::{
    Clock, CorrelationId, EntityId, EventEnvelope, EventStore, ExpectedVersion, IdempotencyKey,
    KernelResult, SequenceNumber, TenantId,
};

use crate::{DecisionError, DecisionOutcome, DecisionReplay, DecisionTrace, DecisionTraceStore};

/// Event-store backed persistence boundary for immutable decision traces.
///
/// This adapter deliberately persists traces as domain events while keeping
/// replay advisory-only. It does not execute the selected alternative.
#[derive(Clone, Debug)]
pub struct DecisionTraceEventStore {
    stream_id: EntityId,
    events: EventStore<DecisionTrace>,
}

impl Default for DecisionTraceEventStore {
    fn default() -> Self {
        Self::new(EntityId::new())
    }
}

impl DecisionTraceEventStore {
    pub fn new(stream_id: EntityId) -> Self {
        Self {
            stream_id,
            events: EventStore::new(),
        }
    }

    pub fn stream_id(&self) -> EntityId {
        self.stream_id
    }

    pub fn current_version(&self) -> SequenceNumber {
        self.events.current_version(self.stream_id)
    }

    pub fn event_count(&self) -> usize {
        self.events.event_count()
    }

    pub fn persist(
        &mut self,
        trace: DecisionTrace,
        tenant_id: TenantId,
        correlation_id: CorrelationId,
        expected: ExpectedVersion,
        idempotency_key: IdempotencyKey,
        clock: &dyn Clock,
    ) -> KernelResult<()> {
        let sequence = self.current_version().next()?;
        let envelope = EventEnvelope::new(
            "cat.decision.trace.recorded",
            1,
            tenant_id,
            correlation_id,
            None,
            self.stream_id,
            clock.now()?,
            sequence,
            trace,
        )?;

        self.events
            .append(self.stream_id, expected, idempotency_key, envelope)?;

        Ok(())
    }

    pub fn hydrate(&self) -> Result<DecisionTraceStore, DecisionError> {
        let mut traces = DecisionTraceStore::default();

        for stored in self.events.read_stream(self.stream_id) {
            traces.record(stored.envelope.payload.clone())?;
        }

        Ok(traces)
    }

    pub fn replay(
        &self,
        trace_id: cat_kernel::Uuid,
        outcome: &DecisionOutcome,
    ) -> Result<DecisionReplay, DecisionError> {
        self.hydrate()?.replay(trace_id, outcome)
    }
}
