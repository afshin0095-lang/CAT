use crate::{DeliveryState, EventBusResult, EventEnvelope, RetryPolicy};
use async_trait::async_trait;

/// Asynchronous outbox boundary for durable transactional publication.
///
/// The business transaction commits the canonical event and the outbox record first;
/// a dispatcher later claims, publishes, acknowledges, or schedules retry. The trait
/// deliberately contains no broker-specific types so PostgreSQL, another database,
/// or a test implementation can share the same delivery contract.
#[async_trait]
pub trait AsyncOutboxStore: Send + Sync {
    async fn enqueue(&self, event: EventEnvelope) -> EventBusResult<()>;
    async fn claim_next(&self) -> EventBusResult<Option<EventEnvelope>>;
    async fn acknowledge(&self, event_id: uuid::Uuid) -> EventBusResult<()>;
    async fn fail(
        &self,
        event_id: uuid::Uuid,
        attempt: u32,
        policy: &RetryPolicy,
        error: &str,
    ) -> EventBusResult<DeliveryState>;

    /// Returns the current producer-side delivery state when exposed by the store.
    ///
    /// The default keeps existing third-party implementations source-compatible while
    /// allowing observability decorators to report lifecycle state without reaching
    /// into a concrete persistence implementation.
    async fn state(&self, _event_id: uuid::Uuid) -> EventBusResult<Option<DeliveryState>> {
        Ok(None)
    }
}
