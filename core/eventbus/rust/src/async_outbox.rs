use crate::{DeliveryState, EventEnvelope, EventBusResult, RetryPolicy};
use async_trait::async_trait;

/// Asynchronous outbox boundary for durable transactional publication.
#[async_trait]
pub trait AsyncOutboxStore: Send + Sync {
    async fn enqueue(&self, event: EventEnvelope) -> EventBusResult<()>;
    async fn claim_next(&self) -> EventBusResult<Option<EventEnvelope>>;
    async fn acknowledge(&self, event_id: uuid::Uuid) -> EventBusResult<()>;
    async fn fail(&self, event_id: uuid::Uuid, attempt: u32, policy: &RetryPolicy, error: &str) -> EventBusResult<DeliveryState>;
}
