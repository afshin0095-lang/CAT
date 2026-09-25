use crate::{
    AuthorizedAuditService, ExecutionAuditEvent, ExecutionAuditQuery, ExecutionAuditStore,
    OperatorAuthorizationEvidenceStore, OperatorPermission, OperatorPrincipal, OrchestratorResult,
};
use uuid::Uuid;

/// Operator audit service that records the authorization decision before the
/// protected audit operation executes.
pub struct DurableOperatorAuditService<S, D> {
    audit: AuthorizedAuditService<S>,
    decisions: D,
}

impl<S, D> DurableOperatorAuditService<S, D>
where
    S: ExecutionAuditStore,
    D: OperatorAuthorizationEvidenceStore,
{
    pub fn new(audit_store: S, decisions: D) -> Self {
        Self {
            audit: AuthorizedAuditService::new(audit_store, Default::default()),
            decisions,
        }
    }

    pub fn with_policy(
        audit_store: S,
        policy: crate::OperatorAccessPolicy,
        decisions: D,
    ) -> Self {
        Self {
            audit: AuthorizedAuditService::new(audit_store, policy),
            decisions,
        }
    }

    pub async fn query_for_principal(
        &self,
        principal: &OperatorPrincipal,
        query: ExecutionAuditQuery,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<Vec<ExecutionAuditEvent>> {
        self.authorize_and_record(principal, OperatorPermission::ReadAuditEvidence, recorded_at_ms)
            .await?;
        self.audit.query(principal, query).await
    }

    pub async fn query_for_session<I: crate::OperatorIdentityStore>(
        &self,
        identity_store: &I,
        session_id: Uuid,
        now_ms: u64,
        query: ExecutionAuditQuery,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<Vec<ExecutionAuditEvent>> {
        let principal = identity_store.load_principal_for_session(session_id, now_ms).await?;
        self.query_for_principal(&principal, query, recorded_at_ms).await
    }

    pub async fn load_latest_for_session<I: crate::OperatorIdentityStore>(
        &self,
        identity_store: &I,
        session_id: Uuid,
        now_ms: u64,
        execution_id: Uuid,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<ExecutionAuditEvent> {
        let principal = identity_store.load_principal_for_session(session_id, now_ms).await?;
        self.authorize_and_record(
            &principal,
            OperatorPermission::ReadAuditEvidence,
            recorded_at_ms,
        )
        .await?;
        self.audit.load_latest(&principal, execution_id).await
    }

    pub async fn rebuild_for_principal(
        &self,
        principal: &OperatorPrincipal,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<u64> {
        self.authorize_and_record(
            principal,
            OperatorPermission::RebuildAuditReadModel,
            recorded_at_ms,
        )
        .await?;
        self.audit.rebuild(principal).await
    }

    pub fn audit_service(&self) -> &AuthorizedAuditService<S> {
        &self.audit
    }

    async fn authorize_and_record(
        &self,
        principal: &OperatorPrincipal,
        permission: OperatorPermission,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<()> {
        let decision = self.audit.policy().authorize(principal, permission);
        let evidence = decision.evidence(principal, recorded_at_ms)?;
        self.decisions.append_authorization_decision(&evidence).await?;
        if decision.is_allowed() {
            Ok(())
        } else {
            Err(crate::OrchestratorError::InvalidAuthorizationInput(format!(
                "operator authorization denied: {:?}",
                decision.reasons
            )))
        }
    }
}