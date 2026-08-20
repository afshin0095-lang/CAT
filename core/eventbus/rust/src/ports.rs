use crate::{DeliveryState, EventEnvelope, EventBusResult, RetryPolicy};

/// Idempotency storage boundary. Implementations may be local, Redis-backed, or database-backed.
pub trait IdempotencyStore: Send + Sync {
    fn claim(&mut self, event_id: uuid::Uuid) -> EventBusResult<bool>;
    fn complete(&mut self, event_id: uuid::Uuid) -> EventBusResult<()>;
    fn state(&self, event_id: uuid::Uuid) -> Option<DeliveryState>;
}

/// Outbox persistence boundary. Business transactions own the concrete atomicity mechanism.
pub trait OutboxStore: Send + Sync {
    fn enqueue(&mut self, event: EventEnvelope) -> EventBusResult<()>;
    fn next(&mut self) -> EventBusResult<Option<EventEnvelope>>;
    fn acknowledge(&mut self, event_id: uuid::Uuid) -> EventBusResult<()>;
    fn fail(&mut self, event_id: uuid::Uuid, attempt: u32, policy: &RetryPolicy) -> EventBusResult<DeliveryState>;
}

/// Inbox persistence boundary for consumer-side deduplication and replay safety.
pub trait InboxStore: Send + Sync {
    fn accept(&mut self, event_id: uuid::Uuid) -> EventBusResult<bool>;
    fn mark_succeeded(&mut self, event_id: uuid::Uuid) -> EventBusResult<()>;
    fn mark_failed(&mut self, event_id: uuid::Uuid) -> EventBusResult<()>;
}

/// Broker/transport boundary. The EventBus remains independent of NATS/Kafka/Redis/etc.
pub trait EventTransport: Send + Sync {
    fn publish(&mut self, event: &EventEnvelope) -> EventBusResult<()>;
}
