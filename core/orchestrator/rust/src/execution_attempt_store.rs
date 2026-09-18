use async_trait::async_trait;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    ExecutionAttempt, ExecutionAttemptStatus, FencingToken, OrchestratorError, OrchestratorResult,
    PostgresExecutionStore,
};

#[async_trait]
pub trait ExecutionAttemptStore: Send + Sync {
    async fn record_execution_start(&self, attempt: &ExecutionAttempt) -> OrchestratorResult<()>;
    async fn heartbeat_execution(
        &self,
        execution_id: Uuid,
        owner: &str,
        token: FencingToken,
        now_ms: u64,
    ) -> OrchestratorResult<()>;
    #[allow(clippy::too_many_arguments)]
    async fn complete_execution(
        &self,
        execution_id: Uuid,
        owner: &str,
        token: FencingToken,
        status: ExecutionAttemptStatus,
        finished_at_ms: u64,
        result: Option<serde_json::Value>,
        error: Option<&str>,
    ) -> OrchestratorResult<()>;
    async fn load_execution(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<Option<ExecutionAttempt>>;
}

#[async_trait]
impl ExecutionAttemptStore for PostgresExecutionStore {
    async fn record_execution_start(&self, attempt: &ExecutionAttempt) -> OrchestratorResult<()> {
        sqlx::query(
            "INSERT INTO cat_execution_attempts (execution_id, workflow_id, step_id, attempt, status, owner, fencing_token, started_at, heartbeat_at, finished_at, result, error) VALUES ($1,$2,$3,$4,'running',$5,$6,TO_TIMESTAMP($7 / 1000.0),TO_TIMESTAMP($8 / 1000.0),NULL,NULL,NULL)"
        )
        .bind(attempt.execution_id)
        .bind(attempt.workflow_id)
        .bind(&attempt.step_id)
        .bind(attempt.attempt as i32)
        .bind(&attempt.owner)
        .bind(attempt.fencing_token as i64)
        .bind(attempt.started_at_ms as f64)
        .bind(attempt.heartbeat_at_ms as f64)
        .execute(self.pool())
        .await
        .map(|_| ())
        .map_err(db_error)
    }

    async fn heartbeat_execution(
        &self,
        execution_id: Uuid,
        owner: &str,
        token: FencingToken,
        now_ms: u64,
    ) -> OrchestratorResult<()> {
        let result = sqlx::query("UPDATE cat_execution_attempts SET heartbeat_at = TO_TIMESTAMP($4 / 1000.0) WHERE execution_id = $1 AND owner = $2 AND fencing_token = $3 AND status = 'running'")
            .bind(execution_id)
            .bind(owner)
            .bind(token.value() as i64)
            .bind(now_ms as f64)
            .execute(self.pool())
            .await
            .map_err(db_error)?;
        if result.rows_affected() != 1 {
            return Err(OrchestratorError::LeaseOwnerMismatch {
                lease_id: execution_id.to_string(),
                owner: owner.to_owned(),
            });
        }
        Ok(())
    }

    async fn complete_execution(
        &self,
        execution_id: Uuid,
        owner: &str,
        token: FencingToken,
        status: ExecutionAttemptStatus,
        finished_at_ms: u64,
        result: Option<serde_json::Value>,
        error: Option<&str>,
    ) -> OrchestratorResult<()> {
        let status_text = match status {
            ExecutionAttemptStatus::Running => {
                return Err(OrchestratorError::InvalidStateTransition {
                    from: "running".into(),
                    to: "running".into(),
                });
            }
            ExecutionAttemptStatus::Succeeded => "succeeded",
            ExecutionAttemptStatus::Failed => "failed",
            ExecutionAttemptStatus::Cancelled => "cancelled",
        };
        let updated = sqlx::query("UPDATE cat_execution_attempts SET status = $4, finished_at = TO_TIMESTAMP($5 / 1000.0), heartbeat_at = TO_TIMESTAMP($5 / 1000.0), result = $6, error = $7 WHERE execution_id = $1 AND owner = $2 AND fencing_token = $3 AND status = 'running'")
            .bind(execution_id)
            .bind(owner)
            .bind(token.value() as i64)
            .bind(status_text)
            .bind(finished_at_ms as f64)
            .bind(result)
            .bind(error)
            .execute(self.pool())
            .await
            .map_err(db_error)?;
        if updated.rows_affected() != 1 {
            return Err(OrchestratorError::LeaseOwnerMismatch {
                lease_id: execution_id.to_string(),
                owner: owner.to_owned(),
            });
        }
        Ok(())
    }

    async fn load_execution(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<Option<ExecutionAttempt>> {
        let row = sqlx::query("SELECT execution_id, workflow_id, step_id, attempt, status, owner, fencing_token, EXTRACT(EPOCH FROM started_at) * 1000 AS started_at_ms, EXTRACT(EPOCH FROM heartbeat_at) * 1000 AS heartbeat_at_ms, EXTRACT(EPOCH FROM finished_at) * 1000 AS finished_at_ms, result, error FROM cat_execution_attempts WHERE execution_id = $1")
            .bind(execution_id)
            .fetch_optional(self.pool())
            .await
            .map_err(db_error)?;
        row.map(decode_attempt).transpose()
    }
}

fn decode_attempt(row: sqlx::postgres::PgRow) -> OrchestratorResult<ExecutionAttempt> {
    let status: String = row.try_get("status").map_err(row_error)?;
    let status = match status.as_str() {
        "running" => ExecutionAttemptStatus::Running,
        "succeeded" => ExecutionAttemptStatus::Succeeded,
        "failed" => ExecutionAttemptStatus::Failed,
        "cancelled" => ExecutionAttemptStatus::Cancelled,
        other => {
            return Err(OrchestratorError::Serialization(format!(
                "unknown execution attempt status: {other}"
            )));
        }
    };
    let started: f64 = row.try_get("started_at_ms").map_err(row_error)?;
    let heartbeat: f64 = row.try_get("heartbeat_at_ms").map_err(row_error)?;
    let finished: Option<f64> = row.try_get("finished_at_ms").map_err(row_error)?;
    let fencing_token: i64 = row.try_get("fencing_token").map_err(row_error)?;
    let attempt: i32 = row.try_get("attempt").map_err(row_error)?;
    Ok(ExecutionAttempt {
        execution_id: row.try_get("execution_id").map_err(row_error)?,
        workflow_id: row.try_get("workflow_id").map_err(row_error)?,
        step_id: row.try_get("step_id").map_err(row_error)?,
        attempt: attempt.max(0) as u32,
        status,
        owner: row.try_get("owner").map_err(row_error)?,
        fencing_token: fencing_token.max(0) as u64,
        started_at_ms: started.max(0.0) as u64,
        heartbeat_at_ms: heartbeat.max(0.0) as u64,
        finished_at_ms: finished.map(|value| value.max(0.0) as u64),
        result: row.try_get("result").map_err(row_error)?,
        error: row.try_get("error").map_err(row_error)?,
    })
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql error: {error}"))
}
fn row_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql row error: {error}"))
}
