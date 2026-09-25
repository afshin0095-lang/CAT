use async_trait::async_trait;
use cat_kernel::{EntityId, TenantId};
use uuid::Uuid;

use crate::{
    ExecutionAuditEvent, ExecutionAuditQuery, ExecutionAuditStore, OrchestratorError,
    OrchestratorResult,
};

/// Human/service principal roles recognized by the operator control-plane boundary.
///
/// Authentication itself is intentionally outside this crate. This type represents
/// an already-authenticated principal presented by the upstream identity layer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatorRole {
    Owner,
    Operator,
    Auditor,
}

/// Evidence that the identity layer successfully authenticated a principal.
///
/// The access service never receives or stores raw credentials.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthenticationEvidence {
    pub method: String,
    pub session_id: Uuid,
    pub authenticated_at_ms: u64,
}

impl AuthenticationEvidence {
    pub fn new(
        method: impl Into<String>,
        session_id: Uuid,
        authenticated_at_ms: u64,
    ) -> OrchestratorResult<Self> {
        let method = method.into();
        if method.trim().is_empty() || session_id.is_nil() {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "authentication evidence requires a method and non-nil session".into(),
            ));
        }

        Ok(Self {
            method,
            session_id,
            authenticated_at_ms,
        })
    }
}

/// Authenticated principal presented to the operator authorization boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorScope {
    pub tenant_id: TenantId,
    pub project_id: Option<EntityId>,
    pub resources: Vec<String>,
}

impl OperatorScope {
    pub fn new(tenant_id: TenantId, project_id: Option<EntityId>) -> OrchestratorResult<Self> {
        if tenant_id.as_uuid().is_nil() {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "operator tenant scope must not be nil".into(),
            ));
        }
        Ok(Self {
            tenant_id,
            project_id,
            resources: Vec::new(),
        })
    }

    pub fn with_resource(mut self, resource: impl Into<String>) -> OrchestratorResult<Self> {
        let resource = resource.into();
        if resource.trim().is_empty() || resource.len() > 512 {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "operator resource scope is invalid".into(),
            ));
        }
        self.resources.push(resource);
        Ok(self)
    }

    pub fn allows_resource(&self, resource: &str) -> bool {
        self.resources.is_empty() || self.resources.iter().any(|allowed| {
            allowed == "*" || allowed == resource || (allowed.ends_with("/*") && resource.starts_with(&allowed[..allowed.len() - 1]))
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorPrincipal {
    pub principal_id: Uuid,
    pub role: OperatorRole,
    pub enabled: bool,
    pub authentication: AuthenticationEvidence,
    pub scope: Option<OperatorScope>,
}

impl OperatorPrincipal {
    pub fn new(
        principal_id: Uuid,
        role: OperatorRole,
        authentication: AuthenticationEvidence,
    ) -> OrchestratorResult<Self> {
        if principal_id.is_nil() {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "operator principal id must not be nil".into(),
            ));
        }

        Ok(Self {
            principal_id,
            role,
            enabled: true,
            authentication,
            scope: None,
        })
    }

    pub fn with_scope(mut self, scope: OperatorScope) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// Explicit operator permissions. Authentication and authorization remain separate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatorPermission {
    ReadAudit,
    ReadAuditEvidence,
    RebuildAuditReadModel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatorAuthorizationOutcome {
    Allowed,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorAuthorizationDecision {
    pub outcome: OperatorAuthorizationOutcome,
    pub principal_id: Uuid,
    pub permission: OperatorPermission,
    pub reasons: Vec<String>,
    pub policy_version: &'static str,
}

impl OperatorAuthorizationDecision {
    pub fn is_allowed(&self) -> bool {
        matches!(self.outcome, OperatorAuthorizationOutcome::Allowed)
    }
}

/// Deterministic authorization policy for the operator audit surface.
///
/// The policy is intentionally small and explicit in P0:
/// - Owner: read, sensitive evidence, and rebuild.
/// - Operator: read and sensitive evidence.
/// - Auditor: read and sensitive evidence.
/// - Disabled/invalid principals: denied.
/// - Services/agents are not represented by this role contract and cannot be
///   promoted into operator authority by accident.
#[derive(Clone, Copy, Debug, Default)]
pub struct OperatorAccessPolicy;

impl OperatorAccessPolicy {
    pub const VERSION: &'static str = "operator-audit-access.v1";

    pub fn authorize(
        &self,
        principal: &OperatorPrincipal,
        permission: OperatorPermission,
    ) -> OperatorAuthorizationDecision {
        let deny = |reason: String| OperatorAuthorizationDecision {
            outcome: OperatorAuthorizationOutcome::Denied,
            principal_id: principal.principal_id,
            permission,
            reasons: vec![reason],
            policy_version: Self::VERSION,
        };

        if principal.principal_id.is_nil() {
            return deny("operator principal id must not be nil".into());
        }

        if !principal.enabled {
            return deny("operator principal is disabled".into());
        }

        if principal.authentication.session_id.is_nil()
            || principal.authentication.method.trim().is_empty()
        {
            return deny("authentication evidence is incomplete".into());
        }

        let allowed = match (principal.role, permission) {
            (
                OperatorRole::Owner,
                OperatorPermission::ReadAudit
                    | OperatorPermission::ReadAuditEvidence
                    | OperatorPermission::RebuildAuditReadModel,
            ) => true,
            (
                OperatorRole::Operator | OperatorRole::Auditor,
                OperatorPermission::ReadAudit | OperatorPermission::ReadAuditEvidence,
            ) => true,
            _ => false,
        };

        if allowed {
            OperatorAuthorizationDecision {
                outcome: OperatorAuthorizationOutcome::Allowed,
                principal_id: principal.principal_id,
                permission,
                reasons: Vec::new(),
                policy_version: Self::VERSION,
            }
        } else {
            deny(format!(
                "role {:?} does not grant permission {:?}",
                principal.role, permission
            ))
        }
    }
}

/// Application boundary that prevents callers from reaching audit storage without
/// first passing operator authorization.
///
/// Storage implementations remain authorization-agnostic by design.
pub struct AuthorizedAuditService<S> {
    store: S,
    policy: OperatorAccessPolicy,
}

impl<S> AuthorizedAuditService<S>
where
    S: ExecutionAuditStore,
{
    pub fn new(store: S, policy: OperatorAccessPolicy) -> Self {
        Self { store, policy }
    }

    pub fn policy(&self) -> OperatorAccessPolicy {
        self.policy
    }

    pub fn store(&self) -> &S {
        &self.store
    }

    pub async fn load_latest(
        &self,
        principal: &OperatorPrincipal,
        execution_id: Uuid,
    ) -> OrchestratorResult<ExecutionAuditEvent> {
        self.authorize(principal, OperatorPermission::ReadAuditEvidence)?;
        if execution_id.is_nil() {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "execution id must not be nil".into(),
            ));
        }

        let event = self
            .store
            .load_latest_audit(execution_id)
            .await?
            .ok_or_else(|| OrchestratorError::Serialization("audit event not found".into()))?;
        self.assert_event_scope(principal, &event)?;
        Ok(event)
    }

    pub async fn query(
        &self,
        principal: &OperatorPrincipal,
        query: ExecutionAuditQuery,
    ) -> OrchestratorResult<Vec<ExecutionAuditEvent>> {
        self.authorize(principal, OperatorPermission::ReadAuditEvidence)?;
        let query = self.apply_scope(principal, query)?;
        if query.limit == 0 {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "audit query limit must be greater than zero".into(),
            ));
        }

        self.store.query_audit(query).await
    }

    pub async fn query_for_session<I: crate::OperatorIdentityStore>(
        &self,
        identity_store: &I,
        session_id: Uuid,
        now_ms: u64,
        query: ExecutionAuditQuery,
    ) -> OrchestratorResult<Vec<ExecutionAuditEvent>> {
        let principal = identity_store
            .load_principal_for_session(session_id, now_ms)
            .await?;
        self.query(&principal, query).await
    }

    pub async fn load_latest_for_session<I: crate::OperatorIdentityStore>(
        &self,
        identity_store: &I,
        session_id: Uuid,
        now_ms: u64,
        execution_id: Uuid,
    ) -> OrchestratorResult<ExecutionAuditEvent> {
        let principal = identity_store
            .load_principal_for_session(session_id, now_ms)
            .await?;
        self.load_latest(&principal, execution_id).await
    }

    pub async fn rebuild(
        &self,
        principal: &OperatorPrincipal,
    ) -> OrchestratorResult<u64> {
        self.authorize(principal, OperatorPermission::RebuildAuditReadModel)?;
        if principal.scope.is_some() {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "scoped operators cannot rebuild the global audit read model".into(),
            ));
        }
        self.store.rebuild_audit_read_model().await
    }

    fn apply_scope(
        &self,
        principal: &OperatorPrincipal,
        mut query: ExecutionAuditQuery,
    ) -> OrchestratorResult<ExecutionAuditQuery> {
        let Some(scope) = principal.scope.as_ref() else {
            return Ok(query);
        };
        let tenant_id = scope.tenant_id.as_uuid();
        if query.tenant_id.is_some_and(|value| value != tenant_id) {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "audit query crosses operator tenant scope".into(),
            ));
        }
        query.tenant_id = Some(tenant_id);
        if let Some(project_id) = scope.project_id {
            let project_uuid = project_id.as_uuid();
            if query.project_id.is_some_and(|value| value != project_uuid) {
                return Err(OrchestratorError::InvalidAuthorizationInput(
                    "audit query crosses operator project scope".into(),
                ));
            }
            query.project_id = Some(project_uuid);
        }
        Ok(query)
    }

    fn assert_event_scope(
        &self,
        principal: &OperatorPrincipal,
        event: &ExecutionAuditEvent,
    ) -> OrchestratorResult<()> {
        let Some(scope) = principal.scope.as_ref() else {
            return Ok(());
        };
        let evidence_scope = &event.evidence.authorization;
        if evidence_scope.tenant_id != scope.tenant_id {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "audit execution is outside operator tenant scope".into(),
            ));
        }
        if scope.project_id.is_some() && evidence_scope.project_id != scope.project_id {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "audit execution is outside operator project scope".into(),
            ));
        }
        Ok(())
    }

    fn authorize(
        &self,
        principal: &OperatorPrincipal,
        permission: OperatorPermission,
    ) -> OrchestratorResult<()> {
        let decision = self.policy.authorize(principal, permission);
        if decision.is_allowed() {
            Ok(())
        } else {
            Err(OrchestratorError::InvalidAuthorizationInput(format!(
                "operator authorization denied: {:?}",
                decision.reasons
            )))
        }
    }
}

/// Optional adapter used by Control Plane handlers that need an authorization
/// receipt before invoking a storage operation.
#[async_trait]
pub trait AuthorizedAuditReader: Send + Sync {
    async fn load_latest_authorized(
        &self,
        principal: &OperatorPrincipal,
        execution_id: Uuid,
    ) -> OrchestratorResult<ExecutionAuditEvent>;

    async fn query_authorized(
        &self,
        principal: &OperatorPrincipal,
        query: ExecutionAuditQuery,
    ) -> OrchestratorResult<Vec<ExecutionAuditEvent>>;
}

#[async_trait]
impl<S> AuthorizedAuditReader for AuthorizedAuditService<S>
where
    S: ExecutionAuditStore,
{
    async fn load_latest_authorized(
        &self,
        principal: &OperatorPrincipal,
        execution_id: Uuid,
    ) -> OrchestratorResult<ExecutionAuditEvent> {
        self.load_latest(principal, execution_id).await
    }

    async fn query_authorized(
        &self,
        principal: &OperatorPrincipal,
        query: ExecutionAuditQuery,
    ) -> OrchestratorResult<Vec<ExecutionAuditEvent>> {
        self.query(principal, query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn principal(role: OperatorRole) -> OperatorPrincipal {
        OperatorPrincipal::new(
            Uuid::new_v4(),
            role,
            AuthenticationEvidence::new("oidc", Uuid::new_v4(), 1_000).expect("valid auth"),
        )
        .expect("valid principal")
    }

    #[test]
    fn owner_can_rebuild_audit_read_model() {
        let decision =
            OperatorAccessPolicy.authorize(&principal(OperatorRole::Owner), OperatorPermission::RebuildAuditReadModel);

        assert!(decision.is_allowed());
        assert_eq!(decision.policy_version, OperatorAccessPolicy::VERSION);
    }

    #[test]
    fn operator_cannot_rebuild_audit_read_model() {
        let decision = OperatorAccessPolicy.authorize(
            &principal(OperatorRole::Operator),
            OperatorPermission::RebuildAuditReadModel,
        );

        assert!(!decision.is_allowed());
        assert!(decision
            .reasons
            .iter()
            .any(|reason| reason.contains("does not grant")));
    }

    #[test]
    fn disabled_principal_is_denied_even_with_owner_role() {
        let disabled = principal(OperatorRole::Owner).disabled();
        let decision =
            OperatorAccessPolicy.authorize(&disabled, OperatorPermission::ReadAuditEvidence);

        assert!(!decision.is_allowed());
        assert!(decision
            .reasons
            .iter()
            .any(|reason| reason == "operator principal is disabled"));
    }

    #[test]
    fn incomplete_authentication_is_denied() {
        let principal = OperatorPrincipal {
            principal_id: Uuid::new_v4(),
            role: OperatorRole::Auditor,
            enabled: true,
            authentication: AuthenticationEvidence {
                method: String::new(),
                session_id: Uuid::new_v4(),
                authenticated_at_ms: 1,
            },
            scope: None,
        };

        let decision =
            OperatorAccessPolicy.authorize(&principal, OperatorPermission::ReadAudit);

        assert!(!decision.is_allowed());
    }

    #[test]
    #[test]
    fn scoped_operator_cannot_rebuild_global_audit() {
        let tenant = TenantId::new();
        let principal = principal(OperatorRole::Owner)
            .with_scope(OperatorScope::new(tenant, None).expect("scope"));
        let service_policy = OperatorAccessPolicy;
        let decision = service_policy.authorize(&principal, OperatorPermission::RebuildAuditReadModel);
        assert!(decision.is_allowed());
    }

    #[test]
    fn resource_scope_supports_exact_and_prefix_matches() {
        let scope = OperatorScope::new(TenantId::new(), None)
            .unwrap()
            .with_resource("audit/read")
            .unwrap()
            .with_resource("provider/*")
            .unwrap();
        assert!(scope.allows_resource("audit/read"));
        assert!(scope.allows_resource("provider/callback"));
        assert!(!scope.allows_resource("billing/write"));
    }

    #[test]
    fn policy_is_copyable_and_sendable_without_shared_state() {
        let policy = OperatorAccessPolicy;
        let shared = Arc::new(policy);
        assert_eq!(shared.policy, OperatorAccessPolicy);
    }
}
