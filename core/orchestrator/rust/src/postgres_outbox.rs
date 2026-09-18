use async_trait::async_trait;
use cat_eventbus::{EventEnvelope, EventKind};
use cat_kernel::EntityId;
use sqlx::{Row, postgres::PgPool};
use uuid::Uuid;

use crate::{OrchestratorError, OrchestratorResult, RetryPolicy};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresOutboxRecord {
    pub event: EventEnvelope,
    pub attempt: u32,
    pub available_at_ms: u64,
    pub claimed_by: Option<String>,
    pub claimed_until_ms: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PostgresOutboxDisposition {
    RetryScheduled { attempt: u32, available_at_ms: u64 },
    DeadLettered { attempt: u32 },
}

#[async_trait]
pub trait AsyncPostgresOutbox: Send + Sync {
    async fn claim_next(
        &self,
        owner: &str,
        now_ms: u64,
        claim_ttl_ms: u64,
    ) -> OrchestratorResult<Option<PostgresOutboxRecord>>;
    async fn acknowledge(&self, event_id: Uuid, owner: &str) -> OrchestratorResult<()>;
    async fn fail(
        &self,
        event_id: Uuid,
        owner: &str,
        now_ms: u64,
        policy: RetryPolicy,
        error: &str,
    ) -> OrchestratorResult<PostgresOutboxDisposition>;
}

impl super::PostgresExecutionStore {
    pub async fn enqueue_outbox(
        &self,
        event: &EventEnvelope,
        workflow_id: Uuid,
    ) -> OrchestratorResult<()> {
        let event_kind = serde_json::to_string(&event.kind).map_err(|error| {
            OrchestratorError::Serialization(format!("event kind serialization error: {error}"))
        })?;
        let event_kind = event_kind.trim_matches('"');
        sqlx::query("INSERT INTO cat_workflow_outbox (event_id, workflow_id, event_type, version, event_kind, occurred_at_ms, producer, correlation_id, causation_id, subject_id, payload) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) ON CONFLICT (event_id) DO NOTHING")
            .bind(event.event_id)
            .bind(workflow_id)
            .bind(&event.event_type)
            .bind(event.version as i32)
            .bind(event_kind)
            .bind(event.occurred_at_ms as i64)
            .bind(&event.producer)
            .bind(event.correlation_id)
            .bind(event.causation_id)
            .bind(event.subject_id.map(|id| id.as_uuid()))
            .bind(&event.payload)
            .execute(self.pool())
            .await
            .map_err(|e| OrchestratorError::Serialization(format!("postgresql outbox error: {e}")))?;
        Ok(())
    }
}

#[async_trait]
impl AsyncPostgresOutbox for super::PostgresExecutionStore {
    async fn claim_next(
        &self,
        owner: &str,
        now_ms: u64,
        claim_ttl_ms: u64,
    ) -> OrchestratorResult<Option<PostgresOutboxRecord>> {
        let mut tx = self.pool().begin().await.map_err(db_error)?;
        let claim_until = now_ms.saturating_add(claim_ttl_ms) as f64;
        let row = sqlx::query("UPDATE cat_workflow_outbox SET claimed_by = $1, claimed_until = TO_TIMESTAMP($2 / 1000.0) WHERE event_id = (SELECT event_id FROM cat_workflow_outbox WHERE available_at <= NOW() AND (claimed_by IS NULL OR claimed_until <= NOW()) ORDER BY available_at, event_id FOR UPDATE SKIP LOCKED LIMIT 1) RETURNING event_id, workflow_id, event_type, version, event_kind, occurred_at_ms, producer, correlation_id, causation_id, subject_id, payload, attempt, EXTRACT(EPOCH FROM available_at) * 1000 AS available_at_ms, claimed_by, EXTRACT(EPOCH FROM claimed_until) * 1000 AS claimed_until_ms")
            .bind(owner)
            .bind(claim_until)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db_error)?;
        tx.commit().await.map_err(db_error)?;

        let Some(row) = row else {
            return Ok(None);
        };
        let workflow_id: Uuid = row.try_get("workflow_id").map_err(row_error)?;
        let event_kind_raw: String = row.try_get("event_kind").map_err(row_error)?;
        let event_kind: EventKind = serde_json::from_str(&format!("\"{event_kind_raw}\""))
            .map_err(|error| {
                OrchestratorError::Serialization(format!(
                    "event kind deserialization error: {error}"
                ))
            })?;
        let event = EventEnvelope {
            event_id: row.try_get("event_id").map_err(row_error)?,
            event_type: row.try_get("event_type").map_err(row_error)?,
            version: row.try_get::<i32, _>("version").map_err(row_error)?.max(0) as u16,
            kind: event_kind,
            occurred_at_ms: row
                .try_get::<i64, _>("occurred_at_ms")
                .map_err(row_error)?
                .max(0) as u64,
            producer: row.try_get("producer").map_err(row_error)?,
            correlation_id: row.try_get("correlation_id").map_err(row_error)?,
            causation_id: row.try_get("causation_id").map_err(row_error)?,
            subject_id: row
                .try_get::<Option<Uuid>, _>("subject_id")
                .map_err(row_error)?
                .map(EntityId::from_uuid),
            payload: row.try_get("payload").map_err(row_error)?,
        };
        Ok(Some(PostgresOutboxRecord {
            event,
            attempt: row.try_get::<i32, _>("attempt").map_err(row_error)?.max(0) as u32,
            available_at_ms: row
                .try_get::<f64, _>("available_at_ms")
                .map_err(row_error)?
                .max(0.0) as u64,
            claimed_by: row.try_get("claimed_by").map_err(row_error)?,
            claimed_until_ms: row
                .try_get::<Option<f64>, _>("claimed_until_ms")
                .map_err(row_error)?
                .map(|v| v.max(0.0) as u64),
        }))
    }

    async fn acknowledge(&self, event_id: Uuid, owner: &str) -> OrchestratorResult<()> {
        let result =
            sqlx::query("DELETE FROM cat_workflow_outbox WHERE event_id = $1 AND claimed_by = $2")
                .bind(event_id)
                .bind(owner)
                .execute(self.pool())
                .await
                .map_err(db_error)?;
        if result.rows_affected() != 1 {
            return Err(OrchestratorError::LeaseOwnerMismatch {
                lease_id: event_id.to_string(),
                owner: owner.to_owned(),
            });
        }
        Ok(())
    }

    async fn fail(
        &self,
        event_id: Uuid,
        owner: &str,
        now_ms: u64,
        policy: RetryPolicy,
        error: &str,
    ) -> OrchestratorResult<PostgresOutboxDisposition> {
        let mut tx = self.pool().begin().await.map_err(db_error)?;
        let row = sqlx::query("SELECT event_id, workflow_id, event_type, version, event_kind, occurred_at_ms, producer, correlation_id, causation_id, subject_id, payload, attempt FROM cat_workflow_outbox WHERE event_id = $1 AND claimed_by = $2 FOR UPDATE")
            .bind(event_id)
            .bind(owner)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db_error)?
            .ok_or_else(|| OrchestratorError::LeaseOwnerMismatch { lease_id: event_id.to_string(), owner: owner.to_owned() })?;

        let attempt = row.try_get::<i32, _>("attempt").map_err(row_error)?.max(0) as u32 + 1;
        if policy.retryable(attempt) {
            let delay = policy.delay_ms(attempt);
            let available_at = now_ms.saturating_add(delay);
            let updated = sqlx::query("UPDATE cat_workflow_outbox SET attempt = $3, available_at = TO_TIMESTAMP($4 / 1000.0), claimed_by = NULL, claimed_until = NULL, last_error = $5 WHERE event_id = $1 AND claimed_by = $2")
                .bind(event_id).bind(owner).bind(attempt as i32).bind(available_at as f64).bind(error)
                .execute(&mut *tx).await.map_err(db_error)?;
            if updated.rows_affected() != 1 {
                return Err(OrchestratorError::LeaseOwnerMismatch {
                    lease_id: event_id.to_string(),
                    owner: owner.to_owned(),
                });
            }
            tx.commit().await.map_err(db_error)?;
            Ok(PostgresOutboxDisposition::RetryScheduled {
                attempt,
                available_at_ms: available_at,
            })
        } else {
            let dead_letter_insert = sqlx::query("INSERT INTO cat_workflow_dead_letters (event_id, workflow_id, event_type, version, event_kind, occurred_at_ms, producer, correlation_id, causation_id, subject_id, payload, attempt, last_error) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) ON CONFLICT (event_id) DO NOTHING")
                .bind(row.try_get::<Uuid, _>("event_id").map_err(row_error)?)
                .bind(row.try_get::<Uuid, _>("workflow_id").map_err(row_error)?)
                .bind(row.try_get::<String, _>("event_type").map_err(row_error)?)
                .bind(row.try_get::<i32, _>("version").map_err(row_error)?)
                .bind(row.try_get::<String, _>("event_kind").map_err(row_error)?)
                .bind(row.try_get::<i64, _>("occurred_at_ms").map_err(row_error)?)
                .bind(row.try_get::<String, _>("producer").map_err(row_error)?)
                .bind(row.try_get::<Option<Uuid>, _>("correlation_id").map_err(row_error)?)
                .bind(row.try_get::<Option<Uuid>, _>("causation_id").map_err(row_error)?)
                .bind(row.try_get::<Option<Uuid>, _>("subject_id").map_err(row_error)?)
                .bind(row.try_get::<serde_json::Value, _>("payload").map_err(row_error)?)
                .bind(attempt as i32)
                .bind(error)
                .execute(&mut *tx)
                .await
                .map_err(db_error)?;
            if dead_letter_insert.rows_affected() != 1 {
                return Err(OrchestratorError::Serialization(
                    "failed to persist PostgreSQL dead letter".to_owned(),
                ));
            }

            let deleted = sqlx::query(
                "DELETE FROM cat_workflow_outbox WHERE event_id = $1 AND claimed_by = $2",
            )
            .bind(event_id)
            .bind(owner)
            .execute(&mut *tx)
            .await
            .map_err(db_error)?;
            if deleted.rows_affected() != 1 {
                return Err(OrchestratorError::LeaseOwnerMismatch {
                    lease_id: event_id.to_string(),
                    owner: owner.to_owned(),
                });
            }
            tx.commit().await.map_err(db_error)?;
            Ok(PostgresOutboxDisposition::DeadLettered { attempt })
        }
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql outbox error: {error}"))
}
fn row_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql outbox row error: {error}"))
}
