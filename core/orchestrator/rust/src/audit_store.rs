use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::{
    ExecutionAuditEvidence, OrchestratorError, OrchestratorResult, PostgresExecutionStore,
    ReconciliationAction,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionAuditEvent {
    pub audit_sequence: i64,
    pub audit_id: Uuid,
    pub event_key: String,
    pub execution_id: Uuid,
    pub workflow_id: Uuid,
    pub step_id: String,
    pub attempt: u32,
    pub action: ReconciliationAction,
    pub agent_id: Uuid,
    pub capability_id: String,
    pub requested_side_effect: String,
    pub approval_reference: Option<String>,
    pub correlation_id: Uuid,
    pub recorded_at_ms: u64,
    pub evidence: ExecutionAuditEvidence,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExecutionAuditQuery {
    pub agent_id: Option<Uuid>,
    pub capability_id: Option<String>,
    pub action: Option<ReconciliationAction>,
    pub limit: u32,
}

#[async_trait]
pub trait ExecutionAuditStore: Send + Sync {
    async fn append_audit_event(
        &self,
        event_key: &str,
        evidence: &ExecutionAuditEvidence,
        action: ReconciliationAction,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<ExecutionAuditEvent>;

    async fn load_latest_audit(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<Option<ExecutionAuditEvent>>;

    async fn query_audit(
        &self,
        query: ExecutionAuditQuery,
    ) -> OrchestratorResult<Vec<ExecutionAuditEvent>>;
}

impl PostgresExecutionStore {
    async fn append_audit_event_internal(
        &self,
        event_key: &str,
        evidence: &ExecutionAuditEvidence,
        action: ReconciliationAction,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<ExecutionAuditEvent> {
        if event_key.trim().is_empty()
            || event_key.len() > 512
            || evidence.execution_id.is_nil()
            || evidence.workflow_id.is_nil()
            || evidence.step_id.trim().is_empty()
            || evidence.attempt == 0
            || !evidence.authorization.validate()
        {
            return Err(OrchestratorError::Serialization(
                "invalid execution audit event".into(),
            ));
        }

        let payload = serde_json::to_value(evidence).map_err(json_error)?;
        let side_effect = match evidence.authorization.requested_side_effect {
            cat_kernel::SideEffectClass::S0 => "S0",
            cat_kernel::SideEffectClass::S1 => "S1",
            cat_kernel::SideEffectClass::S2 => "S2",
            cat_kernel::SideEffectClass::S3 => "S3",
        };

        let mut tx = self.pool().begin().await.map_err(db_error)?;
        let audit_id = Uuid::now_v7();

        let inserted = sqlx::query(
            "INSERT INTO cat_execution_audit_events
             (audit_id, event_key, execution_id, workflow_id, step_id, attempt, action,
              agent_id, capability_id, requested_side_effect, approval_reference,
              correlation_id, recorded_at, evidence)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,TO_TIMESTAMP($13 / 1000.0),$14)
             ON CONFLICT (event_key) DO NOTHING
             RETURNING audit_sequence",
        )
        .bind(audit_id)
        .bind(event_key)
        .bind(evidence.execution_id)
        .bind(evidence.workflow_id)
        .bind(&evidence.step_id)
        .bind(evidence.attempt as i32)
        .bind(action.as_str())
        .bind(evidence.authorization.agent_id.as_entity_id().as_uuid())
        .bind(evidence.authorization.capability_id.as_str())
        .bind(side_effect)
        .bind(&evidence.authorization.approval_reference)
        .bind(evidence.authorization.correlation_id.as_uuid())
        .bind(recorded_at_ms as f64)
        .bind(&payload)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_error)?;

        let (audit_id, audit_sequence) = if let Some(row) = inserted {
            (
                audit_id,
                row.try_get::<i64, _>("audit_sequence").map_err(row_error)?,
            )
        } else {
            let row = sqlx::query(
                "SELECT audit_sequence, audit_id, event_key, execution_id, workflow_id, step_id,
                        attempt, action, agent_id, capability_id, requested_side_effect,
                        approval_reference, correlation_id,
                        EXTRACT(EPOCH FROM recorded_at) * 1000 AS recorded_at_ms, evidence
                 FROM cat_execution_audit_events
                 WHERE event_key = $1",
            )
            .bind(event_key)
            .fetch_one(&mut *tx)
            .await
            .map_err(db_error)?;

            let existing_execution: Uuid =
                row.try_get("execution_id").map_err(row_error)?;
            let existing_evidence: serde_json::Value =
                row.try_get("evidence").map_err(row_error)?;
            if existing_execution != evidence.execution_id || existing_evidence != payload {
                return Err(OrchestratorError::Serialization(
                    "execution audit event identity conflict".into(),
                ));
            }
            (
                row.try_get("audit_id").map_err(row_error)?,
                row.try_get("audit_sequence").map_err(row_error)?,
            )
        };

        sqlx::query(
            "INSERT INTO cat_execution_audit_read_model
             (execution_id, event_key, workflow_id, step_id, attempt, status, action, agent_id,
              capability_id, requested_side_effect, approval_reference, correlation_id,
              recorded_at, source_audit_sequence, evidence)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,TO_TIMESTAMP($13 / 1000.0),$14,$15)
             ON CONFLICT (execution_id) DO UPDATE
             SET event_key = EXCLUDED.event_key,
                 workflow_id = EXCLUDED.workflow_id,
                 step_id = EXCLUDED.step_id,
                 attempt = EXCLUDED.attempt,
                 status = EXCLUDED.status,
                 action = EXCLUDED.action,
                 agent_id = EXCLUDED.agent_id,
                 capability_id = EXCLUDED.capability_id,
                 requested_side_effect = EXCLUDED.requested_side_effect,
                 approval_reference = EXCLUDED.approval_reference,
                 correlation_id = EXCLUDED.correlation_id,
                 recorded_at = EXCLUDED.recorded_at,
                 source_audit_sequence = EXCLUDED.source_audit_sequence,
                 evidence = EXCLUDED.evidence
             WHERE EXCLUDED.source_audit_sequence > cat_execution_audit_read_model.source_audit_sequence",
        )
        .bind(evidence.execution_id)
        .bind(event_key)
        .bind(evidence.workflow_id)
        .bind(&evidence.step_id)
        .bind(evidence.attempt as i32)
        .bind(status_text(evidence.status))
        .bind(action.as_str())
        .bind(evidence.authorization.agent_id.as_entity_id().as_uuid())
        .bind(evidence.authorization.capability_id.as_str())
        .bind(side_effect)
        .bind(&evidence.authorization.approval_reference)
        .bind(evidence.authorization.correlation_id.as_uuid())
        .bind(recorded_at_ms as f64)
        .bind(audit_sequence)
        .bind(&payload)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        tx.commit().await.map_err(db_error)?;

        Ok(ExecutionAuditEvent {
            audit_sequence,
            audit_id,
            event_key: event_key.to_owned(),
            execution_id: evidence.execution_id,
            workflow_id: evidence.workflow_id,
            step_id: evidence.step_id.clone(),
            attempt: evidence.attempt,
            action,
            agent_id: evidence.authorization.agent_id.as_entity_id().as_uuid(),
            capability_id: evidence.authorization.capability_id.to_string(),
            requested_side_effect: side_effect.to_owned(),
            approval_reference: evidence.authorization.approval_reference.clone(),
            correlation_id: evidence.authorization.correlation_id.as_uuid(),
            recorded_at_ms,
            evidence: evidence.clone(),
        })
    }
}

#[async_trait]
impl ExecutionAuditStore for PostgresExecutionStore {
    async fn append_audit_event(
        &self,
        event_key: &str,
        evidence: &ExecutionAuditEvidence,
        action: ReconciliationAction,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<ExecutionAuditEvent> {
        self.append_audit_event_internal(event_key, evidence, action, recorded_at_ms)
            .await
    }

    async fn load_latest_audit(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<Option<ExecutionAuditEvent>> {
        let row = sqlx::query(
            "SELECT audit_sequence, audit_id, event_key, execution_id, workflow_id, step_id,
                    attempt, action, agent_id, capability_id, requested_side_effect,
                    approval_reference, correlation_id,
                    EXTRACT(EPOCH FROM recorded_at) * 1000 AS recorded_at_ms, evidence
             FROM cat_execution_audit_read_model
             WHERE execution_id = $1",
        )
        .bind(execution_id)
        .fetch_optional(self.pool())
        .await
        .map_err(db_error)?;

        row.map(decode_audit_read_model).transpose()
    }

    async fn query_audit(
        &self,
        query: ExecutionAuditQuery,
    ) -> OrchestratorResult<Vec<ExecutionAuditEvent>> {
        let limit = query.limit.clamp(1, 500) as i64;
        let action = query.action.map(|value| value.as_str().to_owned());

        let rows = sqlx::query(
            "SELECT audit_sequence, audit_id, event_key, execution_id, workflow_id, step_id, attempt,
                    action, agent_id, capability_id, requested_side_effect, approval_reference,
                    correlation_id, EXTRACT(EPOCH FROM recorded_at) * 1000 AS recorded_at_ms,
                    evidence, execution_id::text || ':latest' AS event_key
             FROM cat_execution_audit_read_model
             WHERE ($1::uuid IS NULL OR agent_id = $1)
               AND ($2::text IS NULL OR capability_id = $2)
               AND ($3::text IS NULL OR action = $3)
             ORDER BY source_audit_sequence DESC
             LIMIT $4",
        )
        .bind(query.agent_id)
        .bind(query.capability_id)
        .bind(action)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(db_error)?;

        rows.into_iter()
            .map(decode_audit_read_model)
            .collect::<OrchestratorResult<Vec<_>>>()
    }
}

fn decode_audit_read_model(
    row: sqlx::postgres::PgRow,
) -> OrchestratorResult<ExecutionAuditEvent> {
    let action_text: String = row.try_get("action").map_err(row_error)?;
    let action = ReconciliationAction::from_str(&action_text).ok_or_else(|| {
        OrchestratorError::Serialization(format!("unknown reconciliation action: {action_text}"))
    })?;
    let evidence_value: serde_json::Value = row.try_get("evidence").map_err(row_error)?;
    let evidence: ExecutionAuditEvidence =
        serde_json::from_value(evidence_value).map_err(json_error)?;

    Ok(ExecutionAuditEvent {
        audit_sequence: row.try_get("audit_sequence").map_err(row_error)?,
        audit_id: row.try_get("audit_id").map_err(row_error)?,
        event_key: row.try_get("event_key").map_err(row_error)?,
        execution_id: row.try_get("execution_id").map_err(row_error)?,
        workflow_id: row.try_get("workflow_id").map_err(row_error)?,
        step_id: row.try_get("step_id").map_err(row_error)?,
        attempt: row
            .try_get::<i32, _>("attempt")
            .map_err(row_error)?
            .max(0) as u32,
        action,
        agent_id: row.try_get("agent_id").map_err(row_error)?,
        capability_id: row.try_get("capability_id").map_err(row_error)?,
        requested_side_effect: row.try_get("requested_side_effect").map_err(row_error)?,
        approval_reference: row.try_get("approval_reference").map_err(row_error)?,
        correlation_id: row.try_get("correlation_id").map_err(row_error)?,
        recorded_at_ms: row
            .try_get::<f64, _>("recorded_at_ms")
            .map_err(row_error)?
            .max(0.0) as u64,
        evidence,
    })
}

fn status_text(status: crate::ExecutionAttemptStatus) -> &'static str {
    match status {
        crate::ExecutionAttemptStatus::Running => "running",
        crate::ExecutionAttemptStatus::Succeeded => "succeeded",
        crate::ExecutionAttemptStatus::Failed => "failed",
        crate::ExecutionAttemptStatus::Cancelled => "cancelled",
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql audit error: {error}"))
}

fn row_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql audit row error: {error}"))
}

fn json_error(error: serde_json::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("audit JSON serialization error: {error}"))
}
