use cat_eventstore_postgres::{PostgresEventStore, PostgresEventStoreError};
use cat_kernel::{
    Clock, CorrelationId, EntityId, EventEnvelope, ExpectedVersion, IdempotencyKey, KernelError,
    TenantId,
};

use crate::{DecisionError, DecisionOutcome, DecisionReplay, DecisionTrace, DecisionTraceStore};

/// PostgreSQL-backed persistence adapter for immutable decision traces.
///
/// The durable adapter preserves the same invariants as the in-memory adapter:
/// optimistic concurrency, idempotency, immutable trace payloads, and
/// advisory-only replay. Persisting a trace never executes a decision.
#[derive(Clone, Debug)]
pub struct DurableDecisionTraceEventStore {
    stream_id: EntityId,
    events: PostgresEventStore,
}

impl DurableDecisionTraceEventStore {
    pub fn new(stream_id: EntityId, events: PostgresEventStore) -> Self {
        Self { stream_id, events }
    }

    pub fn stream_id(&self) -> EntityId {
        self.stream_id
    }

    pub fn event_store(&self) -> &PostgresEventStore {
        &self.events
    }

    pub async fn current_version(
        &self,
    ) -> Result<cat_kernel::SequenceNumber, PostgresEventStoreError> {
        self.events.current_version(self.stream_id.as_uuid()).await
    }

    pub async fn persist(
        &self,
        trace: DecisionTrace,
        tenant_id: TenantId,
        correlation_id: CorrelationId,
        expected: ExpectedVersion,
        idempotency_key: &IdempotencyKey,
        clock: &dyn Clock,
    ) -> Result<(), DurableDecisionTracePersistenceError> {
        let sequence = self.current_version().await?.next()?;
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
            .append(
                self.stream_id.as_uuid(),
                expected,
                idempotency_key,
                &envelope,
            )
            .await?;

        Ok(())
    }

    pub async fn hydrate(
        &self,
    ) -> Result<DecisionTraceStore, DurableDecisionTracePersistenceError> {
        let mut traces = DecisionTraceStore::default();
        for stored in self
            .events
            .read_stream::<DecisionTrace>(self.stream_id.as_uuid())
            .await?
        {
            traces.record(stored.payload)?;
        }
        Ok(traces)
    }

    pub async fn replay(
        &self,
        trace_id: cat_kernel::Uuid,
        outcome: &DecisionOutcome,
    ) -> Result<DecisionReplay, DurableDecisionTracePersistenceError> {
        Ok(self.hydrate().await?.replay(trace_id, outcome)?)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DurableDecisionTracePersistenceError {
    #[error(transparent)]
    EventStore(#[from] PostgresEventStoreError),
    #[error(transparent)]
    Kernel(#[from] KernelError),
    #[error(transparent)]
    Decision(#[from] DecisionError),
}
