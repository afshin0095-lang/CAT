use async_trait::async_trait;
use cat_eventbus::EventEnvelope;
use sqlx::{migrate::Migrator, postgres::PgPool, Row};
use uuid::Uuid;

use crate::{FencingToken, OrchestratorError, OrchestratorResult, WorkflowInstance};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[derive(Clone, Debug)]
pub struct PostgresExecutionStore { pool: PgPool }
impl PostgresExecutionStore {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &PgPool { &self.pool }
    pub async fn ensure_schema(&self) -> OrchestratorResult<()> { MIGRATOR.run(&self.pool).await.map_err(|error| OrchestratorError::Serialization(format!("postgresql migration error: {error}"))) }
}

#[async_trait]
pub trait AsyncPostgresExecutionStore: Send + Sync {
    async fn load_workflow(&self, workflow_id: Uuid) -> OrchestratorResult<WorkflowInstance>;
    async fn commit_workflow_and_outbox(&self, workflow: &WorkflowInstance, expected_revision: u64, events: &[EventEnvelope]) -> OrchestratorResult<()>;
    async fn acquire_fenced_lease(&self, resource: &str, owner: &str, now_ms: u64, ttl_ms: u64) -> OrchestratorResult<FencingToken>;
    async fn renew_fenced_lease(&self, resource: &str, owner: &str, token: FencingToken, now_ms: u64, ttl_ms: u64) -> OrchestratorResult<()>;
    async fn validate_fencing_token(&self, resource: &str, token: FencingToken, now_ms: u64) -> OrchestratorResult<()>;
}

#[async_trait]
impl AsyncPostgresExecutionStore for PostgresExecutionStore {
    async fn load_workflow(&self, workflow_id: Uuid) -> OrchestratorResult<WorkflowInstance> {
        let row = sqlx::query("SELECT state, revision FROM cat_workflows WHERE id = $1").bind(workflow_id).fetch_optional(&self.pool).await.map_err(db_error)?.ok_or_else(|| OrchestratorError::WorkflowNotFound(workflow_id.to_string()))?;
        let state: serde_json::Value = row.try_get("state").map_err(row_error)?; let revision: i64 = row.try_get("revision").map_err(row_error)?;
        let mut workflow: WorkflowInstance = serde_json::from_value(state).map_err(json_error)?; workflow.revision = revision.max(0) as u64; Ok(workflow)
    }

    async fn commit_workflow_and_outbox(&self, workflow: &WorkflowInstance, expected_revision: u64, events: &[EventEnvelope]) -> OrchestratorResult<()> {
        if workflow.revision <= expected_revision { return Err(OrchestratorError::InvalidStateTransition { from: format!("revision {expected_revision}"), to: format!("revision {}", workflow.revision) }); }
        let mut tx = self.pool.begin().await.map_err(db_error)?; let state = serde_json::to_value(workflow).map_err(json_error)?;
        let update = sqlx::query("UPDATE cat_workflows SET state = $1, revision = $2, updated_at = NOW() WHERE id = $3 AND revision = $4").bind(state).bind(workflow.revision as i64).bind(workflow.id).bind(expected_revision as i64).execute(&mut *tx).await.map_err(db_error)?;
        if update.rows_affected() != 1 { return Err(OrchestratorError::RevisionConflict { workflow_id: workflow.id.to_string(), expected: expected_revision, actual: expected_revision }); }
        for event in events {
            let event_kind = serde_json::to_value(event.kind).map_err(json_error)?.as_str().map(str::to_owned).ok_or_else(|| OrchestratorError::Serialization("event kind is not a string".into()))?;
            sqlx::query("INSERT INTO cat_workflow_outbox (event_id, workflow_id, event_type, version, event_kind, occurred_at_ms, producer, correlation_id, causation_id, subject_id, payload) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) ON CONFLICT (event_id) DO NOTHING")
                .bind(event.event_id).bind(workflow.id).bind(&event.event_type).bind(event.version as i32).bind(event_kind).bind(event.occurred_at_ms as i64).bind(&event.producer).bind(event.correlation_id).bind(event.causation_id).bind(event.subject_id.map(|id| id.as_uuid())).bind(&event.payload).execute(&mut *tx).await.map_err(db_error)?;
        }
        tx.commit().await.map_err(db_error)?; Ok(())
    }

    async fn acquire_fenced_lease(&self, resource: &str, owner: &str, now_ms: u64, ttl_ms: u64) -> OrchestratorResult<FencingToken> {
        let mut tx = self.pool.begin().await.map_err(db_error)?;
        let row = sqlx::query("SELECT owner, fencing_token, EXTRACT(EPOCH FROM expires_at) * 1000 AS expires_at_ms FROM cat_execution_leases WHERE resource = $1 FOR UPDATE").bind(resource).fetch_optional(&mut *tx).await.map_err(db_error)?;
        let next_token = match row { None => 1_u64, Some(row) => { let current_owner: String = row.try_get("owner").map_err(row_error)?; let expires_at_ms: f64 = row.try_get("expires_at_ms").map_err(row_error)?; if expires_at_ms > now_ms as f64 && current_owner != owner { return Err(OrchestratorError::LeaseUnavailable { resource: resource.to_owned() }); } let token: i64 = row.try_get("fencing_token").map_err(row_error)?; (token.max(0) as u64).saturating_add(1) } };
        let expiry_ms = now_ms.saturating_add(ttl_ms) as f64;
        sqlx::query("INSERT INTO cat_execution_leases (resource, owner, fencing_token, expires_at, updated_at) VALUES ($1,$2,$3,TO_TIMESTAMP($4 / 1000.0),NOW()) ON CONFLICT (resource) DO UPDATE SET owner = EXCLUDED.owner, fencing_token = EXCLUDED.fencing_token, expires_at = EXCLUDED.expires_at, updated_at = NOW()").bind(resource).bind(owner).bind(next_token as i64).bind(expiry_ms).execute(&mut *tx).await.map_err(db_error)?;
        tx.commit().await.map_err(db_error)?; Ok(FencingToken::from_value(next_token))
    }

    async fn renew_fenced_lease(&self, resource: &str, owner: &str, token: FencingToken, now_ms: u64, ttl_ms: u64) -> OrchestratorResult<()> {
        let expiry_ms = now_ms.saturating_add(ttl_ms) as f64;
        let result = sqlx::query("UPDATE cat_execution_leases SET expires_at = TO_TIMESTAMP($4 / 1000.0), updated_at = NOW() WHERE resource = $1 AND owner = $2 AND fencing_token = $3 AND expires_at > TO_TIMESTAMP($5 / 1000.0)").bind(resource).bind(owner).bind(token.value() as i64).bind(expiry_ms).bind(now_ms as f64).execute(&self.pool).await.map_err(db_error)?;
        if result.rows_affected() != 1 { return Err(OrchestratorError::LeaseExpired { lease_id: resource.to_owned() }); } Ok(())
    }

    async fn validate_fencing_token(&self, resource: &str, token: FencingToken, now_ms: u64) -> OrchestratorResult<()> {
        let row = sqlx::query("SELECT fencing_token, EXTRACT(EPOCH FROM expires_at) * 1000 AS expires_at_ms FROM cat_execution_leases WHERE resource = $1").bind(resource).fetch_optional(&self.pool).await.map_err(db_error)?.ok_or_else(|| OrchestratorError::LeaseUnavailable { resource: resource.to_owned() })?;
        let current_token: i64 = row.try_get("fencing_token").map_err(row_error)?; let expires_at_ms: f64 = row.try_get("expires_at_ms").map_err(row_error)?;
        if expires_at_ms <= now_ms as f64 { return Err(OrchestratorError::LeaseExpired { lease_id: resource.to_owned() }); }
        if current_token != token.value() as i64 { return Err(OrchestratorError::FencingTokenMismatch { resource: resource.to_owned(), expected: token.value(), actual: current_token.max(0) as u64 }); }
        Ok(())
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError { OrchestratorError::Serialization(format!("postgresql error: {error}")) }
fn row_error(error: sqlx::Error) -> OrchestratorError { OrchestratorError::Serialization(format!("postgresql row error: {error}")) }
fn json_error(error: serde_json::Error) -> OrchestratorError { OrchestratorError::Serialization(format!("workflow serialization error: {error}")) }
