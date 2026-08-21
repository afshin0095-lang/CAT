use cat_eventbus::{DeliveryState, EventBusError, EventBusResult};
use sqlx::{PgPool, Row};

#[derive(Clone, Debug)]
pub struct PostgresInbox {
    pool: PgPool,
    consumer_name: String,
}

impl PostgresInbox {
    pub fn new(pool: PgPool, consumer_name: impl Into<String>) -> Self {
        Self { pool, consumer_name: consumer_name.into() }
    }

    /// Claims an event for this consumer. The primary key makes the claim
    /// idempotent across retries and process restarts.
    pub async fn accept(&self, event_id: uuid::Uuid) -> EventBusResult<bool> {
        let result = sqlx::query(
            "INSERT INTO cat_event_inbox (event_id, consumer_name) VALUES ($1, $2)
             ON CONFLICT (event_id) DO NOTHING",
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
            "UPDATE cat_event_inbox SET state = 'succeeded', completed_at = NOW(), last_error = NULL WHERE event_id = $1 AND consumer_name = $2",
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
            "UPDATE cat_event_inbox SET state = 'failed', attempts = attempts + 1, last_error = $3 WHERE event_id = $1 AND consumer_name = $2",
        )
        .bind(event_id)
        .bind(&self.consumer_name)
        .bind(error)
        .execute(&self.pool)
        .await
        .map_err(|e| EventBusError::Storage(e.to_string()))?;
        Ok(())
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
