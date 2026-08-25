use crate::{DeliveryState, EventEnvelope, EventBusResult, RetryPolicy};
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
}
