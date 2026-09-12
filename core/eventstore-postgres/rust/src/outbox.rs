use cat_eventbus::{EventBusError, EventBusResult, EventEnvelope, RetryPolicy};
use sqlx::{PgPool, Row};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct PostgresOutbox {
    pool: PgPool,
}

#[derive(Clone, Debug)]
pub struct OutboxRecord {
    pub event: EventEnvelope,
    pub stream_id: uuid::Uuid,
    pub sequence: u64,
    pub attempts: u32,
}

impl PostgresOutbox {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn claim_next(&self) -> EventBusResult<Option<OutboxRecord>> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| EventBusError::Storage(e.to_string()))?;
        let row = sqlx::query(
            "SELECT event_id, stream_id, sequence, envelope, attempts
             FROM cat_event_outbox
             WHERE state IN ('pending', 'retry_scheduled') AND available_at <= NOW()
             ORDER BY created_at, event_id
             FOR UPDATE SKIP LOCKED LIMIT 1",
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| EventBusError::Storage(e.to_string()))?;

        let Some(row) = row else {
            tx.commit()
                .await
                .map_err(|e| EventBusError::Storage(e.to_string()))?;
            return Ok(None);
        };

        let event_id: uuid::Uuid = row
            .try_get("event_id")
            .map_err(|e| EventBusError::Storage(e.to_string()))?;
        let stream_id: uuid::Uuid = row
            .try_get("stream_id")
            .map_err(|e| EventBusError::Storage(e.to_string()))?;
        let sequence: i64 = row
            .try_get("sequence")
            .map_err(|e| EventBusError::Storage(e.to_string()))?;
        let attempts: i32 = row
            .try_get("attempts")
            .map_err(|e| EventBusError::Storage(e.to_string()))?;
        let envelope: serde_json::Value = row
            .try_get("envelope")
            .map_err(|e| EventBusError::Storage(e.to_string()))?;

        sqlx::query(
            "UPDATE cat_event_outbox SET state = 'in_flight', attempts = attempts + 1, claimed_at = NOW(), updated_at = NOW() WHERE event_id = $1",
        ).bind(event_id).execute(&mut *tx).await.map_err(|e| EventBusError::Storage(e.to_string()))?;
        tx.commit()
            .await
            .map_err(|e| EventBusError::Storage(e.to_string()))?;

        let event: EventEnvelope = serde_json::from_value(envelope)
            .map_err(|e| EventBusError::Serialization(e.to_string()))?;
        Ok(Some(OutboxRecord {
            event,
            stream_id,
            sequence: sequence as u64,
            attempts: attempts as u32 + 1,
        }))
    }

    pub async fn requeue_stale(&self, stale_after: Duration) -> EventBusResult<u64> {
        let changed = sqlx::query(
            "UPDATE cat_event_outbox
             SET state = 'retry_scheduled', available_at = NOW(), claimed_at = NULL,
                 last_error = COALESCE(last_error, 'stale outbox claim recovered'), updated_at = NOW()
             WHERE state = 'in_flight' AND claimed_at IS NOT NULL
               AND claimed_at < NOW() - ($1 * INTERVAL '1 millisecond')",
        ).bind(stale_after.as_millis() as i64).execute(&self.pool).await
            .map_err(|e| EventBusError::Storage(e.to_string()))?;
        Ok(changed.rows_affected())
    }

    pub async fn acknowledge(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        sqlx::query("UPDATE cat_event_outbox SET state = 'succeeded', claimed_at = NULL, updated_at = NOW() WHERE event_id = $1")
            .bind(event_id).execute(&self.pool).await.map_err(|e| EventBusError::Storage(e.to_string()))?;
        Ok(())
    }

    pub async fn fail(
        &self,
        event_id: uuid::Uuid,
        attempt: u32,
        policy: &RetryPolicy,
        error: &str,
    ) -> EventBusResult<cat_eventbus::DeliveryState> {
        if policy.exhausted(attempt) {
            sqlx::query("UPDATE cat_event_outbox SET state = 'dead_lettered', claimed_at = NULL, last_error = $2, updated_at = NOW() WHERE event_id = $1")
                .bind(event_id).bind(error).execute(&self.pool).await.map_err(|e| EventBusError::Storage(e.to_string()))?;
            return Ok(cat_eventbus::DeliveryState::DeadLettered);
        }

        let delay = policy.delay_for(attempt);
        sqlx::query(
            "UPDATE cat_event_outbox SET state = 'retry_scheduled', available_at = NOW() + ($2 * INTERVAL '1 millisecond'),
             claimed_at = NULL, last_error = $3, updated_at = NOW() WHERE event_id = $1",
        ).bind(event_id).bind(delay.as_millis() as i64).bind(error).execute(&self.pool).await
            .map_err(|e| EventBusError::Storage(e.to_string()))?;
        Ok(cat_eventbus::DeliveryState::RetryScheduled)
    }
}
