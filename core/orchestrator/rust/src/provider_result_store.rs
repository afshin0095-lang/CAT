use async_trait::async_trait;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    AsyncPostgresExecutionStore, ExecutionAttempt, ExecutionAttemptKey, ExecutionAttemptStatus,
    FencingToken, OrchestratorError, OrchestratorResult, PostgresExecutionStore,
    ProviderExecutionRecord, ProviderOutcomeState, ExecutionReconciliationStore,
};

#[async_trait]
impl ExecutionReconciliationStore for PostgresExecutionStore {
    async fn load_execution_attempt(&self, execution_id: Uuid) -> OrchestratorResult<ExecutionAttempt> {
        let row = sqlx::query(
            "SELECT execution_id, workflow_id, step_id, attempt, status, owner, fencing_token,\
             EXTRACT(EPOCH FROM started_at) * 1000 AS started_at_ms,\
             EXTRACT(EPOCH FROM heartbeat_at) * 1000 AS heartbeat_at_ms,\
             EXTRACT(EPOCH FROM finished_at) * 1000 AS finished_at_ms, result, error\
             FROM cat_execution_attempts WHERE execution_id = $1",
        )
        .bind(execution_id)
        .fetch_optional(self.pool())
        .await
        .map_err(db_error)?
        .ok_or_else(|| OrchestratorError::Serialization(format!("execution attempt not found: {execution_id}")))?;

        let status: String = row.try_get("status").map_err(row_error)?;
        let status = match status.as_str() {
            "running" => ExecutionAttemptStatus::Running,
            "succeeded" => ExecutionAttemptStatus::Succeeded,
            "failed" => ExecutionAttemptStatus::Failed,
            "cancelled" => ExecutionAttemptStatus::Cancelled,
            other => return Err(OrchestratorError::Serialization(format!("unknown execution attempt status: {other}"))),
        };

        let fencing_token: i64 = row.try_get("fencing_token").map_err(row_error)?;
        let started_at_ms: f64 = row.try_get("started_at_ms").map_err(row_error)?;
        let heartbeat_at_ms: f64 = row.try_get("heartbeat_at_ms").map_err(row_error)?;
        let finished_at_ms: Option<f64> = row.try_get("finished_at_ms").map_err(row_error)?;
        let attempt: i32 = row.try_get("attempt").map_err(row_error)?;
        let workflow_id: Uuid = row.try_get("workflow_id").map_err(row_error)?;
        let step_id: String = row.try_get("step_id").map_err(row_error)?;
        let owner: String = row.try_get("owner").map_err(row_error)?;
        let result: Option<serde_json::Value> = row.try_get("result").map_err(row_error)?;
        let error: Option<String> = row.try_get("error").map_err(row_error)?;

        Ok(ExecutionAttempt {
            execution_id,
            key: ExecutionAttemptKey { workflow_id, step_id, attempt: attempt.max(0) as u32 },
            status,
            owner,
            fencing_token: FencingToken::from_value(fencing_token.max(0) as u64),
            started_at_ms: started_at_ms.max(0.0) as u64,
            heartbeat_at_ms: heartbeat_at_ms.max(0.0) as u64,
            finished_at_ms: finished_at_ms.map(|value| value.max(0.0) as u64),
            result,
            error,
        })
    }

    async fn load_provider_result(&self, execution_id: Uuid) -> OrchestratorResult<Option<ProviderExecutionRecord>> {
        let row = sqlx::query(
            "SELECT execution_id, provider, provider_execution_id, request_hash,\
             EXTRACT(EPOCH FROM submitted_at) * 1000 AS submitted_at_ms, outcome_state,\
             EXTRACT(EPOCH FROM observed_at) * 1000 AS observed_at_ms, result, error\
             FROM cat_provider_execution_results WHERE execution_id = $1",
        )
        .bind(execution_id)
        .fetch_optional(self.pool())
        .await
        .map_err(db_error)?;

        let Some(row) = row else { return Ok(None); };
        let outcome_state: Option<String> = row.try_get("outcome_state").map_err(row_error)?;
        let outcome = outcome_state.as_deref().map(parse_outcome).transpose()?;
        let submitted_at_ms: f64 = row.try_get("submitted_at_ms").map_err(row_error)?;
        let observed_at_ms: Option<f64> = row.try_get("observed_at_ms").map_err(row_error)?;

        Ok(Some(ProviderExecutionRecord {
            execution_id: row.try_get("execution_id").map_err(row_error)?,
            provider: row.try_get("provider").map_err(row_error)?,
            provider_execution_id: row.try_get("provider_execution_id").map_err(row_error)?,
            request_hash: row.try_get("request_hash").map_err(row_error)?,
            submitted_at_ms: submitted_at_ms.max(0.0) as u64,
            outcome,
            observed_at_ms: observed_at_ms.map(|value| value.max(0.0) as u64),
            result: row.try_get("result").map_err(row_error)?,
            error: row.try_get("error").map_err(row_error)?,
        }))
    }

    async fn record_provider_submission(
        &self,
        execution_id: Uuid,
        provider: &str,
        provider_execution_id: &str,
        request_hash: &str,
        submitted_at_ms: u64,
    ) -> OrchestratorResult<()> {
        let result = sqlx::query(
            "INSERT INTO cat_provider_execution_results\
             (execution_id, provider, provider_execution_id, request_hash, submitted_at)\
             VALUES ($1, $2, $3, $4, TO_TIMESTAMP($5 / 1000.0))\
             ON CONFLICT (execution_id) DO UPDATE SET provider = EXCLUDED.provider,\
             provider_execution_id = EXCLUDED.provider_execution_id, request_hash = EXCLUDED.request_hash",
        )
        .bind(execution_id)
        .bind(provider)
        .bind(provider_execution_id)
        .bind(request_hash)
        .bind(submitted_at_ms as f64)
        .execute(self.pool())
        .await
        .map_err(db_error)?;
        if result.rows_affected() != 1 {
            return Err(OrchestratorError::Serialization(format!("provider submission was not persisted: {execution_id}")));
        }
        Ok(())
    }

    async fn record_provider_result(
        &self,
        execution_id: Uuid,
        provider_execution_id: &str,
        outcome: ProviderOutcomeState,
        observed_at_ms: u64,
        result: Option<serde_json::Value>,
        error: Option<&str>,
    ) -> OrchestratorResult<()> {
        let outcome_value = match outcome {
            ProviderOutcomeState::Succeeded => "succeeded",
            ProviderOutcomeState::Failed => "failed",
            ProviderOutcomeState::Unknown => "unknown",
        };
        let update = sqlx::query(
            "UPDATE cat_provider_execution_results\
             SET outcome_state = $1, observed_at = TO_TIMESTAMP($2 / 1000.0), result = $3, error = $4\
             WHERE execution_id = $5 AND provider_execution_id = $6\
               AND (outcome_state IS NULL OR outcome_state = $1) RETURNING execution_id",
        )
        .bind(outcome_value)
        .bind(observed_at_ms as f64)
        .bind(result)
        .bind(error)
        .bind(execution_id)
        .bind(provider_execution_id)
        .fetch_optional(self.pool())
        .await
        .map_err(db_error)?;

        if update.is_none() {
            return Err(OrchestratorError::Serialization(format!(
                "provider result rejected or conflicting for execution {execution_id}"
            )));
        }
        Ok(())
    }
}

fn parse_outcome(value: &str) -> OrchestratorResult<ProviderOutcomeState> {
    match value {
        "succeeded" => Ok(ProviderOutcomeState::Succeeded),
        "failed" => Ok(ProviderOutcomeState::Failed),
        "unknown" => Ok(ProviderOutcomeState::Unknown),
        other => Err(OrchestratorError::Serialization(format!("unknown provider outcome state: {other}"))),
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql error: {error}"))
}

fn row_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql row error: {error}"))
}
