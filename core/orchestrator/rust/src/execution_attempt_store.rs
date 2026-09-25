use async_trait::async_trait;
use cat_kernel::{
    AgentId, CorrelationId, EntityId, InvocationId, IdempotencyKey, SideEffectClass, TenantId,
};
use sqlx::Row;
use uuid::Uuid;

use crate::{
    ExecutionAttempt, ExecutionAttemptStatus, ExecutionAuthorization,
    ExecutionAuthorizationRecord, FencingToken, OrchestratorError, OrchestratorResult,
    PostgresExecutionStore,
};

#[async_trait]
pub trait ExecutionAttemptStore: Send + Sync {
    async fn record_execution_start(
        &self,
        attempt: &ExecutionAttempt,
        authorization: &ExecutionAuthorization,
    ) -> OrchestratorResult<()>;

    async fn heartbeat_execution(
        &self,
        execution_id: Uuid,
        owner: &str,
        token: FencingToken,
        now_ms: u64,
    ) -> OrchestratorResult<()>;

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

    async fn load_execution_authorization(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<Option<ExecutionAuthorizationRecord>>;
}

#[async_trait]
impl ExecutionAttemptStore for PostgresExecutionStore {
    async fn record_execution_start(
        &self,
        attempt: &ExecutionAttempt,
        authorization: &ExecutionAuthorization,
    ) -> OrchestratorResult<()> {
        if !authorization.matches_execution(
            attempt.workflow_id,
            &attempt.step_id,
            attempt.attempt,
        ) {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "execution authorization does not match durable attempt".to_owned(),
            ));
        }

        let record = ExecutionAuthorizationRecord::from_receipt(authorization);
        if !record.validate() {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "execution authorization record is invalid".to_owned(),
            ));
        }

        let required_policies =
            serde_json::to_value(&record.required_policies).map_err(json_error)?;

        let mut tx = self.pool().begin().await.map_err(db_error)?;

        sqlx::query(
            "INSERT INTO cat_execution_attempts             (execution_id, workflow_id, step_id, attempt, status, owner, fencing_token,              started_at, heartbeat_at, finished_at, result, error)             VALUES ($1,$2,$3,$4,'running',$5,$6,TO_TIMESTAMP($7 / 1000.0),                     TO_TIMESTAMP($8 / 1000.0),NULL,NULL,NULL)",
        )
        .bind(attempt.execution_id)
        .bind(attempt.workflow_id)
        .bind(&attempt.step_id)
        .bind(attempt.attempt as i32)
        .bind(&attempt.owner)
        .bind(attempt.fencing_token as i64)
        .bind(attempt.started_at_ms as f64)
        .bind(attempt.heartbeat_at_ms as f64)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        sqlx::query(
            "INSERT INTO cat_execution_authorizations             (execution_id, invocation_id, agent_id, capability_id, requested_side_effect,              required_policies, approval_reference, idempotency_key, correlation_id, tenant_id, project_id, admitted_at)             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,TO_TIMESTAMP($12 / 1000.0))",
        )
        .bind(attempt.execution_id)
        .bind(record.invocation_id.as_entity_id().as_uuid())
        .bind(record.agent_id.as_entity_id().as_uuid())
        .bind(record.capability_id.as_str())
        .bind(side_effect_text(record.requested_side_effect))
        .bind(required_policies)
        .bind(&record.approval_reference)
        .bind(record.idempotency_key.as_str())
        .bind(record.correlation_id.as_uuid())
        .bind(record.tenant_id.as_uuid())
        .bind(record.project_id.map(|value| value.as_uuid()))
        .bind(record.admitted_at_ms as f64)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        tx.commit().await.map_err(db_error)?;
        Ok(())
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

    async fn load_execution_authorization(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<Option<ExecutionAuthorizationRecord>> {
        let row = sqlx::query(
            "SELECT invocation_id, agent_id, capability_id, requested_side_effect,             required_policies, approval_reference, idempotency_key, correlation_id, tenant_id, project_id,             EXTRACT(EPOCH FROM admitted_at) * 1000 AS admitted_at_ms             FROM cat_execution_authorizations WHERE execution_id = $1",
        )
        .bind(execution_id)
        .fetch_optional(self.pool())
        .await
        .map_err(db_error)?;

        row.map(decode_authorization).transpose()
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

fn decode_authorization(
    row: sqlx::postgres::PgRow,
) -> OrchestratorResult<ExecutionAuthorizationRecord> {
    let invocation_uuid: Uuid = row.try_get("invocation_id").map_err(row_error)?;
    let agent_uuid: Uuid = row.try_get("agent_id").map_err(row_error)?;
    let correlation_uuid: Uuid = row.try_get("correlation_id").map_err(row_error)?;
    let tenant_uuid: Uuid = row.try_get("tenant_id").map_err(row_error)?;
    let project_uuid: Option<Uuid> = row.try_get("project_id").map_err(row_error)?;
    let invocation_id = InvocationId::from_entity_id(EntityId::from_uuid(invocation_uuid));
    let agent_id = AgentId::from_entity_id(EntityId::from_uuid(agent_uuid));
    let correlation_id = CorrelationId::from_uuid(correlation_uuid);

    let capability_value: String = row.try_get("capability_id").map_err(row_error)?;
    let capability_id =
        cat_kernel::CapabilityId::new(capability_value).map_err(kernel_error)?;

    let requested_side_effect: String = row
        .try_get("requested_side_effect")
        .map_err(row_error)?;
    let requested_side_effect = parse_side_effect(&requested_side_effect)?;

    let required_policies_value: serde_json::Value =
        row.try_get("required_policies").map_err(row_error)?;
    let required_policies: Vec<String> =
        serde_json::from_value(required_policies_value).map_err(json_error)?;

    let approval_reference: Option<String> =
        row.try_get("approval_reference").map_err(row_error)?;
    let idempotency_value: String = row.try_get("idempotency_key").map_err(row_error)?;
    let idempotency_key = IdempotencyKey::new(idempotency_value).map_err(kernel_error)?;
    let admitted_at: f64 = row.try_get("admitted_at_ms").map_err(row_error)?;

    let record = ExecutionAuthorizationRecord {
        invocation_id,
        agent_id,
        capability_id,
        requested_side_effect,
        required_policies,
        approval_reference,
        idempotency_key,
        correlation_id,
        admitted_at_ms: admitted_at.max(0.0) as u64,
    };

    if !record.validate() {
        return Err(OrchestratorError::InvalidAuthorizationInput(
            "persisted execution authorization record is invalid".to_owned(),
        ));
    }

    Ok(record)
}

fn side_effect_text(side_effect: SideEffectClass) -> &'static str {
    match side_effect {
        SideEffectClass::S0 => "S0",
        SideEffectClass::S1 => "S1",
        SideEffectClass::S2 => "S2",
        SideEffectClass::S3 => "S3",
    }
}

fn parse_side_effect(value: &str) -> OrchestratorResult<SideEffectClass> {
    match value {
        "S0" => Ok(SideEffectClass::S0),
        "S1" => Ok(SideEffectClass::S1),
        "S2" => Ok(SideEffectClass::S2),
        "S3" => Ok(SideEffectClass::S3),
        other => Err(OrchestratorError::Serialization(format!(
            "unknown execution side-effect class: {other}"
        ))),
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql error: {error}"))
}

fn row_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql row error: {error}"))
}

fn json_error(error: serde_json::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("authorization JSON serialization error: {error}"))
}

fn kernel_error(error: cat_kernel::KernelError) -> OrchestratorError {
    OrchestratorError::InvalidAuthorizationInput(error.to_string())
}
