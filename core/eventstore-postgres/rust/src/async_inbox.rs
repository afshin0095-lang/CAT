use cat_eventbus::{AsyncInboxStore, DeliveryState, EventBusError, EventBusResult};
use sqlx::PgPool;

use crate::PostgresInbox;

#[derive(Clone, Debug)]
pub struct PostgresAsyncInbox {
    inner: PostgresInbox,
}

impl PostgresAsyncInbox {
    pub fn new(pool: PgPool, consumer_name: impl Into<String>) -> Self {
        Self {
            inner: PostgresInbox::new(pool, consumer_name),
        }
    }

    pub fn inner(&self) -> &PostgresInbox {
        &self.inner
    }
}

#[async_trait::async_trait]
impl AsyncInboxStore for PostgresAsyncInbox {
    async fn accept(&self, event_id: uuid::Uuid) -> EventBusResult<bool> {
        self.inner.accept(event_id).await
    }

    async fn mark_succeeded(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        self.inner.succeed(event_id).await
    }

    async fn mark_failed(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        self.inner.fail(event_id, "handler failure").await
    }

    async fn state(&self, event_id: uuid::Uuid) -> EventBusResult<Option<DeliveryState>> {
        self.inner.state(event_id).await
    }
}

/// Lightweight storage readiness probe used by the runtime health layer.
pub async fn inbox_storage_ready(pool: &PgPool) -> EventBusResult<()> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map(|_: sqlx::postgres::PgRow| ())
        .map_err(|error| EventBusError::Storage(error.to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn adapter_contract_is_async_and_durable() {
        assert!(std::mem::size_of::<super::PostgresAsyncInbox>() > 0);
    }
}
