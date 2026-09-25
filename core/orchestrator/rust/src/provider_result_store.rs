use async_trait::async_trait;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    insert_provider_journal_tx, ExecutionAttempt, ExecutionAttemptStatus,
    ExecutionReconciliationStore, OrchestratorError, OrchestratorResult,
    PostgresExecutionStore, ProviderExecutionJournalEvent,
    ProviderExecutionRecord, ProviderOutcomeState, ExecutionAuthorizationRecord,
};

#[async_trait]
impl ExecutionReconciliationStore for PostgresExecutionStore {
    async fn load_execution_attempt(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<ExecutionAttempt> {
        let row = sqlx::query(
            "SELECT execution_id, workflow_id, step_id, attempt, status, owner, fencing_token,             EXTRACT(EPOCH FROM started_at) * 1000 AS started_at_ms,             EXTRACT(EPOCH FROM heartbeat_at) * 1000 AS heartbeat_at_ms,             EXTRACT(EPOCH FROM finished_at) * 1000 AS finished_at_ms, result, error             FROM cat_execution_attempts WHERE execution_id = $1",
        )
        .bind(execution_id)
        .fetch_optional(self.pool())
        .await
        .map_err(db_error)?
        .ok_or_else(|| {
            OrchestratorError::Serialization(format!("execution attempt not found: {execution_id}"))
        })?;

        let status_text: String = row.try_get("status").map_err(row_error)?;
        let status = match status_text.as_str() {
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

        let started_at_ms: f64 = row.try_get("started_at_ms").map_err(row_error)?;
        let heartbeat_at_ms: f64 = row.try_get("heartbeat_at_ms").map_err(row_error)?;
        let finished_at_ms: Option<f64> = row.try_get("finished_at_ms").map_err(row_error)?;
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
            started_at_ms: started_at_ms.max(0.0) as u64,
            heartbeat_at_ms: heartbeat_at_ms.max(0.0) as u64,
            finished_at_ms: finished_at_ms.map(|value| value.max(0.0) as u64),
            result: row.try_get("result").map_err(row_error)?,
            error: row.try_get("error").map_err(row_error)?,
        })
    }

    async fn load_execution_authorization(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<Option<ExecutionAuthorizationRecord>> {
        let row = sqlx::query(
            "SELECT invocation_id, agent_id, capability_id, requested_side_effect,
                    required_policies, approval_reference, idempotency_key, correlation_id,
                    EXTRACT(EPOCH FROM admitted_at) * 1000 AS admitted_at_ms
             FROM cat_execution_authorizations
             WHERE execution_id = $1",
        )
        .bind(execution_id)
        .fetch_optional(self.pool())
        .await
        .map_err(db_error)?;

        row.map(decode_authorization).transpose()
    }

    async fn load_provider_result(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<Option<ProviderExecutionRecord>> {
        let row = sqlx::query(
            "SELECT execution_id, provider, provider_execution_id, request_hash,
             EXTRACT(EPOCH FROM submitted_at) * 1000 AS submitted_at_ms, outcome_state,
             EXTRACT(EPOCH FROM observed_at) * 1000 AS observed_at_ms, result, error
             FROM cat_provider_execution_results WHERE execution_id = $1",
        )
        .bind(execution_id)
        .fetch_optional(self.pool())
        .await
        .map_err(db_error)?;

        let Some(row) = row else {
            return Ok(None);
        };
        let outcome_state: Option<String> = row.try_get("outcome_state").map_err(row_error)?;
        let outcome = outcome_state.as_deref().map(parse_outcome).transpose()?;
        let submitted_at_ms: f64 = row.try_get("submitted_at_ms").map_err(row_error)?;
        let observed_at_ms: Option<f64> = row.try_get("observed_at_ms").map(row_error)?;

        Ok(Some(ProviderExecutionRecord {
            execution_id: row.try_get("execution_id").map_err(row_error)?,
            provider: row.try_get("provider").map_err(row_error)?,
            provider_execution_id: row.try_get("provider_execution_id").map_err(row_error)?,
            request_hash: row.try_get("request_hash").map_err(row_error)?,
            submitted_at_ms: submitted_at_ms.max(0.0) as u64,
            outcome,
            observed_at_ms: observed_at_ms.map(|value: f64| value.max(0.0) as u64),
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
        if execution_id.is_nil()
            || provider.trim().is_empty()
            || provider_execution_id.trim().is_empty()
            || request_hash.trim().is_empty()
        {
            return Err(OrchestratorError::Serialization(
                "invalid provider execution submission".into(),
            ));
        }

        let mut tx = self.pool().begin().await.map_err(db_error)?;

        let inserted = sqlx::query(
            "INSERT INTO cat_provider_execution_results
             (execution_id, provider, provider_execution_id, request_hash, submitted_at)
             VALUES ($1, $2, $3, $4, TO_TIMESTAMP($5 / 1000.0))
             ON CONFLICT (execution_id) DO NOTHING",
        )
        .bind(execution_id)
        .bind(provider)
        .bind(provider_execution_id)
        .bind(request_hash)
        .bind(submitted_at_ms as f64)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        if inserted.rows_affected() == 0 {
            let row = sqlx::query(
                "SELECT provider, provider_execution_id, request_hash
                 FROM cat_provider_execution_results
                 WHERE execution_id = $1
                 FOR UPDATE",
            )
            .bind(execution_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(db_error)?;

            let existing_provider: String = row.try_get("provider").map_err(row_error)?;
            let existing_provider_execution_id: String =
                row.try_get("provider_execution_id").map_err(row_error)?;
            let existing_request_hash: String =
                row.try_get("request_hash").map_err(row_error)?;

            if existing_provider != provider
                || existing_provider_execution_id != provider_execution_id
                || existing_request_hash != request_hash
            {
                return Err(OrchestratorError::Serialization(format!(
                    "provider execution identity conflict for execution {execution_id}"
                )));
            }
        }

        insert_provider_journal_tx(
            &mut tx,
            execution_id,
            ProviderExecutionJournalEvent::Submitted,
            provider,
            provider_execution_id,
            request_hash,
            None,
            None,
            None,
            submitted_at_ms,
        )
        .await?;

        tx.commit().await.map_err(db_error)?;
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
        if execution_id.is_nil() || provider_execution_id.trim().is_empty() {
            return Err(OrchestratorError::Serialization(
                "invalid provider execution observation".into(),
            ));
        }

        let mut tx = self.pool().begin().await.map_err(db_error)?;

        let row = sqlx::query(
            "SELECT provider, provider_execution_id, request_hash, outcome_state
             FROM cat_provider_execution_results
             WHERE execution_id = $1
             FOR UPDATE",
        )
        .bind(execution_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_error)?
        .ok_or_else(|| {
            OrchestratorError::Serialization(format!(
                "provider submission does not exist for execution {execution_id}"
            ))
        })?;

        let provider: String = row.try_get("provider").map_err(row_error)?;
        let existing_provider_execution_id: String =
            row.try_get("provider_execution_id").map_err(row_error)?;
        let request_hash: String = row.try_get("request_hash").map_err(row_error)?;
        let existing_outcome: Option<String> = row.try_get("outcome_state").map_err(row_error)?;

        if existing_provider_execution_id != provider_execution_id {
            return Err(OrchestratorError::Serialization(format!(
                "provider execution id conflict for execution {execution_id}"
            )));
        }

        if existing_outcome.as_deref().is_some_and(|existing| existing != outcome.as_str()) {
            return Err(OrchestratorError::Serialization(format!(
                "provider outcome is already terminal for execution {execution_id}"
            )));
        }

        let updated = sqlx::query(
            "UPDATE cat_provider_execution_results
             SET outcome_state = $1,
                 observed_at = TO_TIMESTAMP($2 / 1000.0),
                 result = $3,
                 error = $4
             WHERE execution_id = $5
               AND provider_execution_id = $6",
        )
        .bind(outcome.as_str())
        .bind(observed_at_ms as f64)
        .bind(&result)
        .bind(error)
        .bind(execution_id)
        .bind(provider_execution_id)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        if updated.rows_affected() != 1 {
            return Err(OrchestratorError::Serialization(format!(
                "provider result update was not applied for execution {execution_id}"
            )));
        }

        insert_provider_journal_tx(
            &mut tx,
            execution_id,
            ProviderExecutionJournalEvent::Observed,
            &provider,
            provider_execution_id,
            &request_hash,
            Some(outcome),
            result,
            error,
            observed_at_ms,
        )
        .await?;

        tx.commit().await.map_err(db_error)?;
        Ok(())
    }
}

fn decode_authorization(
    row: sqlx::postgres::PgRow,
) -> OrchestratorResult<ExecutionAuthorizationRecord> {
    use cat_kernel::{
        AgentId, CorrelationId, EntityId, IdempotencyKey, InvocationId,
    };

    let invocation_id = InvocationId::from_entity_id(EntityId::from_uuid(
        row.try_get("invocation_id").map_err(row_error)?,
    ));
    let agent_id = AgentId::from_entity_id(EntityId::from_uuid(
        row.try_get("agent_id").map_err(row_error)?,
    ));
    let correlation_id =
        CorrelationId::from_uuid(row.try_get("correlation_id").map_err(row_error)?);

    let capability_id = cat_kernel::CapabilityId::new(
        row.try_get::<String, _>("capability_id").map_err(row_error)?,
    )
    .map_err(kernel_error)?;
    let requested_side_effect = parse_side_effect(
        &row.try_get::<String, _>("requested_side_effect")
            .map_err(row_error)?,
    )?;
    let required_policies: Vec<String> =
        serde_json::from_value(row.try_get("required_policies").map_err(row_error)?)
            .map_err(json_error)?;
    let approval_reference: Option<String> =
        row.try_get("approval_reference").map_err(row_error)?;
    let idempotency_key = IdempotencyKey::new(
        row.try_get::<String, _>("idempotency_key").map_err(row_error)?,
    )
    .map_err(kernel_error)?;
    let admitted_at_ms: f64 = row.try_get("admitted_at_ms").map_err(row_error)?;

    let record = ExecutionAuthorizationRecord {
        invocation_id,
        agent_id,
        capability_id,
        requested_side_effect,
        required_policies,
        approval_reference,
        idempotency_key,
        correlation_id,
        admitted_at_ms: admitted_at_ms.max(0.0) as u64,
    };

    if !record.validate() {
        return Err(OrchestratorError::InvalidAuthorizationInput(
            "persisted execution authorization record is invalid".to_owned(),
        ));
    }

    Ok(record)
}

fn parse_side_effect(value: &str) -> OrchestratorResult<cat_kernel::SideEffectClass> {
    match value {
        "S0" => Ok(cat_kernel::SideEffectClass::S0),
        "S1" => Ok(cat_kernel::SideEffectClass::S1),
        "S2" => Ok(cat_kernel::SideEffectClass::S2),
        "S3" => Ok(cat_kernel::SideEffectClass::S3),
        other => Err(OrchestratorError::Serialization(format!(
            "unknown execution side-effect class: {other}"
        ))),
    }
}

fn parse_outcome(value: &str) -> OrchestratorResult<ProviderOutcomeState> {
    match value {
        "succeeded" => Ok(ProviderOutcomeState::Succeeded),
        "failed" => Ok(ProviderOutcomeState::Failed),
        "unknown" => Ok(ProviderOutcomeState::Unknown),
        other => Err(OrchestratorError::Serialization(format!(
            "unknown provider outcome state: {other}"
        ))),
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql provider result error: {error}"))
}

fn row_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql provider result row error: {error}"))
}

fn json_error(error: serde_json::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("authorization JSON error: {error}"))
}

fn kernel_error(error: cat_kernel::KernelError) -> OrchestratorError {
    OrchestratorError::InvalidAuthorizationInput(error.to_string())
}
