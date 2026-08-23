use crate::{DeliveryState, EventBusResult};
use async_trait::async_trait;

/// Asynchronous consumer-side inbox boundary.
///
/// This boundary exists alongside the synchronous `InboxStore` because durable
/// stores such as PostgreSQL are inherently asynchronous. It preserves the same
/// state machine while allowing the broker consumer to await storage operations
/// without blocking a Tokio worker thread.
#[async_trait]
pub trait AsyncInboxStore: Send + Sync {
    async fn accept(&self, event_id: uuid::Uuid) -> EventBusResult<bool>;
    async fn mark_succeeded(&self, event_id: uuid::Uuid) -> EventBusResult<()>;
    async fn mark_failed(&self, event_id: uuid::Uuid) -> EventBusResult<()>;
    async fn state(&self, event_id: uuid::Uuid) -> EventBusResult<Option<DeliveryState>>;
}
