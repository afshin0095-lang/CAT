use crate::{AsyncInboxStore, DeliveryState, EventBusResult, InMemoryInbox};

/// Async facade over the synchronous in-memory inbox used by local tests and development.
///
/// The lock-free application path is intentionally small; this type exists to let the
/// same async consumer contract be exercised without a database or NATS server.
#[derive(Clone, Debug, Default)]
pub struct AsyncInMemoryInbox {
    inner: std::sync::Arc<std::sync::Mutex<InMemoryInbox>>,
}

impl AsyncInMemoryInbox {
    pub fn new() -> Self { Self::default() }
}

#[async_trait::async_trait]
impl AsyncInboxStore for AsyncInMemoryInbox {
    async fn accept(&self, event_id: uuid::Uuid) -> EventBusResult<bool> {
        let mut inner = self.inner.lock().expect("in-memory inbox mutex poisoned");
        inner.accept(event_id)
    }

    async fn mark_succeeded(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        let mut inner = self.inner.lock().expect("in-memory inbox mutex poisoned");
        inner.mark_succeeded(event_id)
    }

    async fn mark_failed(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        let mut inner = self.inner.lock().expect("in-memory inbox mutex poisoned");
        inner.mark_failed(event_id)
    }

    async fn state(&self, event_id: uuid::Uuid) -> EventBusResult<Option<DeliveryState>> {
        let inner = self.inner.lock().expect("in-memory inbox mutex poisoned");
        Ok(inner.state(event_id))
    }
}
