use cat_eventbus::{DeliveryState, EventBusError, EventBusResult};
use sqlx::{PgPool, Row};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct PostgresInbox {
    pool: PgPool,
    consumer_name: String,
}

impl PostgresInbox {
    pub fn new(pool: PgPool, consumer_name: impl Into<String>) -> Self {
        Self { pool, consumer_name: consumer_name.into() }
    }

    /// Claims an event for this consumer. Failed claims are re-entered as a new
    /// attempt; succeeded claims remain permanently suppressed.
    pub async fn accept(&self, event_id: uuid::Uuid) -> EventBusResult<bool> {
        let result = sqlx::query(
            "INSERT INTO cat_event_inbox (event_id, consumer_name, state, attempts, claimed_at, updated_at)
             VALUES ($1, $2, 'in_flight', 1, NOW(), NOW())
             ON CONFLICT (event_id, consumer_name) DO UPDATE
             SET state = 'in_flight',
                 attempts = cat_event_inbox.attempts + 1,
                 claimed_at = NOW(),
                 updated_at = NOW(),
                 last_error = NULL
             WHERE cat_event_inbox.state = 'failed'",
        )
        .bind(event_id)
        .bind(&self.consumer_name)
        .execute(&self.pool)
        .await
        .map_err(|e| EventBusError::Storage(e.to_string()))?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn succeed(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        sqlx::query(
            "UPDATE cat_event_inbox
             SET state = 'succeeded', completed_at = NOW(), claimed_at = NULL,
                 updated_at = NOW(), last_error = NULL
             WHERE event_id = $1 AND consumer_name = $2",
        )
        .bind(event_id)
        .bind(&self.consumer_name)
        .execute(&self.pool)
        .await
        .map_err(|e| EventBusError::Storage(e.to_string()))?;
        Ok(())
    }

    pub async fn fail(&self, event_id: uuid::Uuid, error: &str) -> EventBusResult<()> {
        sqlx::query(
            "UPDATE cat_event_inbox
             SET state = 'failed', claimed_at = NULL, updated_at = NOW(), last_error = $3
             WHERE event_id = $1 AND consumer_name = $2",
        )
        .bind(event_id)
        .bind(&self.consumer_name)
        .bind(error)
        .execute(&self.pool)
        .await
        .map_err(|e| EventBusError::Storage(e.to_string()))?;
        Ok(())
    }

    /// Returns abandoned in-flight claims to the failed/reclaimable state.
    ///
    /// Recovery is explicitly time-bounded and consumer-scoped. A successful
    /// claim is never reopened by this method.
    pub async fn requeue_stale(&self, stale_after: Duration) -> EventBusResult<u64> {
        let changed = sqlx::query(
            "UPDATE cat_event_inbox
             SET state = 'failed',
                 claimed_at = NULL,
                 updated_at = NOW(),
                 last_error = COALESCE(last_error, 'stale inbox claim recovered')
             WHERE consumer_name = $1
               AND state = 'in_flight'
               AND claimed_at IS NOT NULL
               AND claimed_at < NOW() - ($2 * INTERVAL '1 millisecond')",
        )
        .bind(&self.consumer_name)
        .bind(stale_after.as_millis() as i64)
        .execute(&self.pool)
        .await
        .map_err(|e| EventBusError::Storage(e.to_string()))?;
        Ok(changed.rows_affected())
    }

    pub async fn state(&self, event_id: uuid::Uuid) -> EventBusResult<Option<DeliveryState>> {
        let row = sqlx::query(
            "SELECT state FROM cat_event_inbox WHERE event_id = $1 AND consumer_name = $2",
        )
        .bind(event_id)
        .bind(&self.consumer_name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| EventBusError::Storage(e.to_string()))?;

        Ok(row.map(|row| match row.try_get::<String, _>("state").unwrap_or_default().as_str() {
            "succeeded" => DeliveryState::Succeeded,
            "failed" => DeliveryState::RetryScheduled,
            _ => DeliveryState::InFlight,
        }))
    }
}
