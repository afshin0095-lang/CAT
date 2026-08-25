use crate::{PostgresEventStore, PostgresEventStoreResult};

/// Readiness state for the PostgreSQL event-store adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventStoreHealth {
    pub database_reachable: bool,
    pub schema_ready: bool,
}

impl EventStoreHealth {
    pub const fn not_ready() -> Self {
        Self {
            database_reachable: false,
            schema_ready: false,
        }
    }

    pub const fn ready() -> Self {
        Self {
            database_reachable: true,
            schema_ready: true,
        }
    }

    pub const fn is_ready(&self) -> bool {
        self.database_reachable && self.schema_ready
    }
}

impl PostgresEventStore {
    /// Performs a cheap database readiness probe without mutating state.
    pub async fn health(&self) -> PostgresEventStoreResult<EventStoreHealth> {
        sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(self.pool())
            .await?;

        let schema_ready = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public' AND table_name = 'cat_events'",
        )
        .fetch_one(self.pool())
        .await?
            > 0;

        Ok(EventStoreHealth {
            database_reachable: true,
            schema_ready,
        })
    }

    /// A strict readiness check suitable for startup gates and health endpoints.
    pub async fn readiness(&self) -> PostgresEventStoreResult<()> {
        let health = self.health().await?;
        if health.is_ready() {
            Ok(())
        } else {
            Err(crate::PostgresEventStoreError::Database(
                sqlx::Error::Protocol("event-store schema is not ready".into()),
            ))
        }
    }
}
