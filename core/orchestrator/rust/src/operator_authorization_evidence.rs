use async_trait::async_trait;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    OperatorAuthorizationDecisionEvidence, OperatorAuthorizationOutcome, OperatorPermission,
    OperatorRole, OrchestratorError, OrchestratorResult, PostgresExecutionStore,
};

#[async_trait]
pub trait OperatorAuthorizationEvidenceStore: Send + Sync {
    async fn append_authorization_decision(
        &self,
        evidence: &OperatorAuthorizationDecisionEvidence,
    ) -> OrchestratorResult<OperatorAuthorizationDecisionEvidence>;

    async fn list_authorization_decisions(
        &self,
        principal_id: Uuid,
        limit: u32,
    ) -> OrchestratorResult<Vec<OperatorAuthorizationDecisionEvidence>>;
}

impl PostgresExecutionStore {
    async fn append_operator_authorization_decision(
        &self,
        evidence: &OperatorAuthorizationDecisionEvidence,
    ) -> OrchestratorResult<OperatorAuthorizationDecisionEvidence> {
        validate_evidence(evidence)?;

        let mut tx = self.pool().begin().await.map_err(db_error)?;
        let inserted = sqlx::query(
            "INSERT INTO cat_operator_authorization_decisions
             (decision_id, principal_id, session_id, role, permission, outcome, policy_version,
              tenant_id, project_id, authentication_method, reasons, recorded_at)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,TO_TIMESTAMP($12 / 1000.0))
             ON CONFLICT (decision_id) DO NOTHING
             RETURNING decision_sequence",
        )
        .bind(evidence.decision_id)
        .bind(evidence.principal_id)
        .bind(evidence.session_id)
        .bind(evidence.role_as_str())
        .bind(evidence.permission_as_str())
        .bind(evidence.outcome_as_str())
        .bind(&evidence.policy_version)
        .bind(evidence.tenant_id)
        .bind(evidence.project_id)
        .bind(&evidence.authentication_method)
        .bind(serde_json::to_value(&evidence.reasons).map_err(json_error)?)
        .bind(evidence.recorded_at_ms as f64)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_error)?;

        if inserted.is_none() {
            let row = sqlx::query(
                "SELECT decision_sequence, decision_id, principal_id, session_id, role,
                        permission, outcome, policy_version, tenant_id, project_id,
                        authentication_method, reasons,
                        EXTRACT(EPOCH FROM recorded_at) * 1000 AS recorded_at_ms
                 FROM cat_operator_authorization_decisions
                 WHERE decision_id = $1",
            )
            .bind(evidence.decision_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(db_error)?;
            let existing = decode_evidence(row)?;
            if existing != *evidence {
                return Err(OrchestratorError::Serialization(
                    "operator authorization decision identity conflict".into(),
                ));
            }
            tx.commit().await.map_err(db_error)?;
            return Ok(existing);
        }

        tx.commit().await.map_err(db_error)?;
        Ok(evidence.clone())
    }

    async fn list_operator_authorization_decisions(
        &self,
        principal_id: Uuid,
        limit: u32,
    ) -> OrchestratorResult<Vec<OperatorAuthorizationDecisionEvidence>> {
        if principal_id.is_nil() {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "operator principal id must not be nil".into(),
            ));
        }
        let limit = limit.clamp(1, 500) as i64;
        let rows = sqlx::query(
            "SELECT decision_sequence, decision_id, principal_id, session_id, role,
                    permission, outcome, policy_version, tenant_id, project_id,
                    authentication_method, reasons,
                    EXTRACT(EPOCH FROM recorded_at) * 1000 AS recorded_at_ms
             FROM cat_operator_authorization_decisions
             WHERE principal_id = $1
             ORDER BY decision_sequence DESC
             LIMIT $2",
        )
        .bind(principal_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(db_error)?;
        rows.into_iter().map(decode_evidence).collect()
    }
}

#[async_trait]
impl OperatorAuthorizationEvidenceStore for PostgresExecutionStore {
    async fn append_authorization_decision(
        &self,
        evidence: &OperatorAuthorizationDecisionEvidence,
    ) -> OrchestratorResult<OperatorAuthorizationDecisionEvidence> {
        self.append_operator_authorization_decision(evidence).await
    }

    async fn list_authorization_decisions(
        &self,
        principal_id: Uuid,
        limit: u32,
    ) -> OrchestratorResult<Vec<OperatorAuthorizationDecisionEvidence>> {
        self.list_operator_authorization_decisions(principal_id, limit).await
    }
}

fn validate_evidence(evidence: &OperatorAuthorizationDecisionEvidence) -> OrchestratorResult<()> {
    if evidence.decision_id.is_nil()
        || evidence.principal_id.is_nil()
        || evidence.session_id.is_nil()
        || evidence.policy_version.trim().is_empty()
        || evidence.authentication_method.trim().is_empty()
        || evidence.recorded_at_ms == 0
    {
        return Err(OrchestratorError::InvalidAuthorizationInput(
            "operator authorization decision evidence is invalid".into(),
        ));
    }
    if evidence.tenant_id.is_some_and(Uuid::is_nil)
        || evidence.project_id.is_some_and(Uuid::is_nil)
    {
        return Err(OrchestratorError::InvalidAuthorizationInput(
            "operator authorization decision scope is invalid".into(),
        ));
    }
    Ok(())
}

fn decode_evidence(row: sqlx::postgres::PgRow) -> OrchestratorResult<OperatorAuthorizationDecisionEvidence> {
    let role = parse_role(&row.try_get::<String, _>("role").map_err(row_error)?)?;
    let permission = parse_permission(&row.try_get::<String, _>("permission").map_err(row_error)?)?;
    let outcome = parse_outcome(&row.try_get::<String, _>("outcome").map_err(row_error)?)?;
    let reasons: Vec<String> = serde_json::from_value(row.try_get("reasons").map_err(row_error)?)
        .map_err(json_error)?;
    let recorded_at_ms: f64 = row.try_get("recorded_at_ms").map_err(row_error)?;

    Ok(OperatorAuthorizationDecisionEvidence {
        decision_id: row.try_get("decision_id").map_err(row_error)?,
        principal_id: row.try_get("principal_id").map_err(row_error)?,
        session_id: row.try_get("session_id").map_err(row_error)?,
        role,
        permission,
        outcome,
        policy_version: row.try_get("policy_version").map_err(row_error)?,
        tenant_id: row.try_get("tenant_id").map_err(row_error)?,
        project_id: row.try_get("project_id").map_err(row_error)?,
        authentication_method: row.try_get("authentication_method").map_err(row_error)?,
        reasons,
        recorded_at_ms: recorded_at_ms.max(0.0) as u64,
    })
}

fn parse_role(value: &str) -> OrchestratorResult<OperatorRole> {
    match value {
        "owner" => Ok(OperatorRole::Owner),
        "operator" => Ok(OperatorRole::Operator),
        "auditor" => Ok(OperatorRole::Auditor),
        other => Err(OrchestratorError::Serialization(format!("unknown operator role: {other}"))),
    }
}

fn parse_permission(value: &str) -> OrchestratorResult<OperatorPermission> {
    match value {
        "read_audit" => Ok(OperatorPermission::ReadAudit),
        "read_audit_evidence" => Ok(OperatorPermission::ReadAuditEvidence),
        "rebuild_audit_read_model" => Ok(OperatorPermission::RebuildAuditReadModel),
        other => Err(OrchestratorError::Serialization(format!("unknown operator permission: {other}"))),
    }
}

fn parse_outcome(value: &str) -> OrchestratorResult<OperatorAuthorizationOutcome> {
    match value {
        "allowed" => Ok(OperatorAuthorizationOutcome::Allowed),
        "denied" => Ok(OperatorAuthorizationOutcome::Denied),
        other => Err(OrchestratorError::Serialization(format!("unknown operator authorization outcome: {other}"))),
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql operator authorization evidence error: {error}"))
}

fn row_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql operator authorization evidence row error: {error}"))
}

fn json_error(error: serde_json::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("operator authorization evidence JSON error: {error}"))
}

trait OperatorDecisionEvidenceFields {
    fn role_as_str(&self) -> &'static str;
    fn permission_as_str(&self) -> &'static str;
    fn outcome_as_str(&self) -> &'static str;
}

impl OperatorDecisionEvidenceFields for OperatorAuthorizationDecisionEvidence {
    fn role_as_str(&self) -> &'static str {
        match self.role {
            OperatorRole::Owner => "owner",
            OperatorRole::Operator => "operator",
            OperatorRole::Auditor => "auditor",
        }
    }
    fn permission_as_str(&self) -> &'static str { self.permission.as_str() }
    fn outcome_as_str(&self) -> &'static str { self.outcome.as_str() }
}