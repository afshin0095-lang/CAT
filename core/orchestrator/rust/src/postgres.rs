use async_trait::async_trait;
use cat_eventbus::EventEnvelope;
use sqlx::{postgres::PgPool, Row};
use uuid::Uuid;

use crate::{FencingToken, OrchestratorError, OrchestratorResult, WorkflowInstance};

/// PostgreSQL-backed asynchronous durable execution adapter.
///
/// It keeps SQL client details behind an async boundary while preserving CAT's
/// workflow revision, transactional outbox, and lease-fencing invariants.
#[derive(Clone, Debug)]
pub struct PostgresExecutionStore {
    pool: PgPool,
}

impl PostgresExecutionStore {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &PgPool { &self.pool }
}

#[async_trait]
pub trait AsyncPostgresExecutionStore: Send + Sync {
    async fn load_workflow(&self, workflow_id: Uuid) -> OrchestratorResult<WorkflowInstance>;

    async fn commit_workflow_and_outbox(
        &self,
        workflow: &WorkflowInstance,
        expected_revision: u64,
        events: &[EventEnvelope],
    ) -> OrchestratorResult<()>;

    async fn acquire_fenced_lease(
        &self,
        resource: &str,
        owner: &str,
        now_ms: u64,
        ttl_ms: u64,
    ) -> OrchestratorResult<FencingToken>;
}

#[async_trait]
impl AsyncPostgresExecutionStore for PostgresExecutionStore {
    async fn load_workflow(&self, workflow_id: Uuid) -> OrchestratorResult<WorkflowInstance> {
        let row = sqlx::query("SELECT state, revision FROM cat_workflows WHERE id = $1")
            .bind(workflow_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(db_error)?
            .ok_or_else(|| OrchestratorError::WorkflowNotFound(workflow_id.to_string()))?;

        let state: serde_json::Value = row.try_get("state").map_err(row_error)?;
        let revision: i64 = row.try_get("revision").map_err(row_error)?;
        let mut workflow: WorkflowInstance = serde_json::from_value(state).map_err(json_error)?;
        workflow.revision = revision.max(0) as u64;
        Ok(workflow)
    }

    async fn commit_workflow_and_outbox(
        &self,
        workflow: &WorkflowInstance,
        expected_revision: u64,
        events: &[EventEnvelope],
    ) -> OrchestratorResult<()> {
        if workflow.revision <= expected_revision {
            return Err(OrchestratorError::InvalidStateTransition {
                from: format!("revision {expected_revision}"),
                to: format!("revision {}", workflow.revision),
            });
        }

        let mut tx = self.pool.begin().await.map_err(db_error)?;
        let state = serde_json::to_value(workflow).map_err(json_error)?;
        let update = sqlx::query(
            "UPDATE cat_workflows SET state = $1, revision = $2, updated_at = NOW() WHERE id = $3 AND revision = $4",
        )
        .bind(state)
        .bind(workflow.revision as i64)
        .bind(workflow.id)
        .bind(expected_revision as i64)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        if update.rows_affected() != 1 {
            return Err(OrchestratorError::RevisionConflict {
                workflow_id: workflow.id.to_string(),
                expected: expected_revision,
                actual: expected_revision,
            });
        }

        for event in events {
            sqlx::query(
                "INSERT INTO cat_workflow_outbox (event_id, workflow_id, event_type, version, occurred_at_ms, payload) VALUES ($1,$2,$3,$4,$5,$6) ON CONFLICT (event_id) DO NOTHING",
            )
            .bind(event.event_id)
            .bind(workflow.id)
            .bind(&event.event_type)
            .bind(event.version as i32)
            .bind(event.occurred_at_ms as i64)
            .bind(&event.payload)
            .execute(&mut *tx)
            .await
            .map_err(db_error)?;
        }

        tx.commit().await.map_err(db_error)?;
        Ok(())
    }

    async fn acquire_fenced_lease(
        &self,
        resource: &str,
        owner: &str,
        now_ms: u64,
        ttl_ms: u64,
    ) -> OrchestratorResult<FencingToken> {
        let mut tx = self.pool.begin().await.map_err(db_error)?;
        let row = sqlx::query(
            "SELECT owner, fencing_token, EXTRACT(EPOCH FROM expires_at) * 1000 AS expires_at_ms FROM cat_execution_leases WHERE resource = $1 FOR UPDATE",
        )
        .bind(resource)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_error)?;

        let next_token = match row {
            None => 1_u64,
            Some(row) => {
                let current_owner: String = row.try_get("owner").map_err(row_error)?;
                let expires_at_ms: f64 = row.try_get("expires_at_ms").map_err(row_error)?;
                if expires_at_ms > now_ms as f64 && current_owner != owner {
                    return Err(OrchestratorError::LeaseUnavailable { resource: resource.to_owned() });
                }
                let token: i64 = row.try_get("fencing_token").map_err(row_error)?;
                (token.max(0) as u64).saturating_add(1)
            }
        };

        let expiry_ms = now_ms.saturating_add(ttl_ms) as f64;
        sqlx::query(
            "INSERT INTO cat_execution_leases (resource, owner, fencing_token, expires_at, updated_at) VALUES ($1,$2,$3,TO_TIMESTAMP($4 / 1000.0),NOW()) ON CONFLICT (resource) DO UPDATE SET owner = EXCLUDED.owner, fencing_token = EXCLUDED.fencing_token, expires_at = EXCLUDED.expires_at, updated_at = NOW()",
        )
        .bind(resource)
        .bind(owner)
        .bind(next_token as i64)
        .bind(expiry_ms)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        tx.commit().await.map_err(db_error)?;
        Ok(FencingToken::from_value(next_token))
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql error: {error}"))
}

fn row_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql row error: {error}"))
}

fn json_error(error: serde_json::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("workflow serialization error: {error}"))
}
