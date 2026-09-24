use crate::{
    AgentContract, AgentId, CapabilityContract, CapabilityId, CapabilityLifecycle,
    CapabilityRegistry, KernelError, KernelResult, SideEffectClass,
};

/// Outcome of a capability authorization evaluation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AuthorizationOutcome {
    Allowed,
    Denied,
    ApprovalRequired,
}

/// Explicit approval evidence presented to the authorization boundary.
///
/// The kernel stores only the approval reference; Decision Core remains the
/// owner of durable approval records and human authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApprovalContext {
    pub granted: bool,
    pub reference: Option<String>,
}

impl ApprovalContext {
    pub const fn none() -> Self {
        Self { granted: false, reference: None }
    }

    pub fn granted(reference: impl Into<String>) -> KernelResult<Self> {
        let reference = reference.into();
        if reference.trim().is_empty() {
            return Err(KernelError::InvalidInput(
                "approval reference must not be empty".to_owned(),
            ));
        }
        Ok(Self { granted: true, reference: Some(reference) })
    }
}

impl Default for ApprovalContext {
    fn default() -> Self { Self::none() }
}

/// Typed request evaluated by the capability authorization boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizationRequest {
    pub agent_id: AgentId,
    pub capability_id: CapabilityId,
    pub requested_side_effect: SideEffectClass,
    pub approval: ApprovalContext,
}

impl AuthorizationRequest {
    pub const fn new(
        agent_id: AgentId,
        capability_id: CapabilityId,
        requested_side_effect: SideEffectClass,
    ) -> Self {
        Self {
            agent_id,
            capability_id,
            requested_side_effect,
            approval: ApprovalContext::none(),
        }
    }

    pub fn with_approval(mut self, approval: ApprovalContext) -> Self {
        self.approval = approval;
        self
    }
}

/// Machine-readable authorization result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizationDecision {
    pub outcome: AuthorizationOutcome,
    pub agent_id: AgentId,
    pub capability_id: CapabilityId,
    pub reasons: Vec<String>,
    pub required_policies: Vec<String>,
    pub approval_required: bool,
}

impl AuthorizationDecision {
    fn denied(
        request: &AuthorizationRequest,
        reasons: Vec<String>,
        required_policies: Vec<String>,
    ) -> Self {
        Self {
            outcome: AuthorizationOutcome::Denied,
            agent_id: request.agent_id,
            capability_id: request.capability_id.clone(),
            reasons,
            required_policies,
            approval_required: false,
        }
    }

    fn approval_required(
        request: &AuthorizationRequest,
        required_policies: Vec<String>,
    ) -> Self {
        Self {
            outcome: AuthorizationOutcome::ApprovalRequired,
            agent_id: request.agent_id,
            capability_id: request.capability_id.clone(),
            reasons: vec![
                "high-impact capability requires explicit approval before execution".to_owned(),
            ],
            required_policies,
            approval_required: true,
        }
    }

    fn allowed(
        request: &AuthorizationRequest,
        required_policies: Vec<String>,
    ) -> Self {
        Self {
            outcome: AuthorizationOutcome::Allowed,
            agent_id: request.agent_id,
            capability_id: request.capability_id.clone(),
            reasons: Vec::new(),
            required_policies,
            approval_required: false,
        }
    }

    pub const fn is_allowed(&self) -> bool {
        matches!(self.outcome, AuthorizationOutcome::Allowed)
    }
}

/// Deterministic, provider-neutral capability authorization engine.
///
/// The engine is a policy enforcement point, not a provider/tool executor.
/// It evaluates the executable authorization dimensions represented by the
/// current kernel contracts and fails closed for unsupported or invalid state.
#[derive(Debug)]
pub struct CapabilityAuthorizationEngine<'a> {
    registry: &'a CapabilityRegistry,
}

impl<'a> CapabilityAuthorizationEngine<'a> {
    pub const fn new(registry: &'a CapabilityRegistry) -> Self {
        Self { registry }
    }

    pub fn authorize(
        &self,
        agent: &AgentContract,
        request: &AuthorizationRequest,
    ) -> AuthorizationDecision {
        if agent.agent_id != request.agent_id {
            return AuthorizationDecision::denied(
                request,
                vec!["authorization agent identity does not match the request".to_owned()],
                Vec::new(),
            );
        }

        if !agent.enabled {
            return AuthorizationDecision::denied(
                request,
                vec!["agent is disabled".to_owned()],
                Vec::new(),
            );
        }

        if let Err(error) = agent.validate() {
            return AuthorizationDecision::denied(
                request,
                vec![format!("agent contract validation failed: {error}")],
                Vec::new(),
            );
        }

        let Some(capability) = self.registry.get(&request.capability_id) else {
            return AuthorizationDecision::denied(
                request,
                vec!["unknown capability".to_owned()],
                Vec::new(),
            );
        };

        if let Err(error) = capability.validate() {
            return AuthorizationDecision::denied(
                request,
                vec![format!("capability contract validation failed: {error}")],
                capability.required_policy.clone(),
            );
        }

        if capability.lifecycle != CapabilityLifecycle::Active {
            return AuthorizationDecision::denied(
                request,
                vec![format!(
                    "capability lifecycle is not runtime-usable: {:?}",
                    capability.lifecycle
                )],
                capability.required_policy.clone(),
            );
        }

        if !agent.capabilities.iter().any(|declared| declared == request.capability_id.as_str()) {
            return AuthorizationDecision::denied(
                request,
                vec!["agent has not declared the requested capability".to_owned()],
                capability.required_policy.clone(),
            );
        }

        if request.requested_side_effect > capability.side_effects {
            return AuthorizationDecision::denied(
                request,
                vec!["requested side-effect exceeds the capability contract".to_owned()],
                capability.required_policy.clone(),
            );
        }

        if request.requested_side_effect > agent.allowed_side_effect {
            return AuthorizationDecision::denied(
                request,
                vec!["requested side-effect exceeds the agent side-effect ceiling".to_owned()],
                capability.required_policy.clone(),
            );
        }

        let missing_policies: Vec<String> = capability
            .required_policy
            .iter()
            .filter(|required| !agent.policy_scope.iter().any(|granted| granted == *required))
            .cloned()
            .collect();

        if !missing_policies.is_empty() {
            return AuthorizationDecision::denied(
                request,
                missing_policies
                    .iter()
                    .map(|policy| format!("agent lacks required policy scope: {policy}"))
                    .collect(),
                capability.required_policy.clone(),
            );
        }

        if capability.side_effects.is_high_impact() {
            if !request.approval.granted {
                return AuthorizationDecision::approval_required(
                    request,
                    capability.required_policy.clone(),
                );
            }

            let valid_reference = request
                .approval
                .reference
                .as_deref()
                .is_some_and(|reference| !reference.trim().is_empty());
            if !valid_reference {
                return AuthorizationDecision::denied(
                    request,
                    vec![
                        "approved high-impact execution requires an approval reference"
                            .to_owned(),
                    ],
                    capability.required_policy.clone(),
                );
            }
        }

        AuthorizationDecision::allowed(request, capability.required_policy.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CapabilityContract, IdempotencyPolicy};

    fn active_registry(side_effects: SideEffectClass) -> (CapabilityRegistry, CapabilityId) {
        let id = CapabilityId::new("cat.capability.test.execute.v1").unwrap();
        let mut capability = CapabilityContract::new(
            id.clone(),
            "test",
            "Execute a test capability",
        )
        .unwrap();
        capability.inputs.push("input".to_owned());
        capability.outputs.push("output".to_owned());
        capability.failure_model.push("typed failure".to_owned());
        capability.observability.push("test.capability.*".to_owned());
        capability.evaluation.push("determinism".to_owned());
        capability.side_effects = side_effects;
        capability.idempotency = match side_effects {
            SideEffectClass::S0 => IdempotencyPolicy::NotApplicable,
            _ => IdempotencyPolicy::Required,
        };
        if side_effects.requires_authorization() {
            capability.required_policy.push("test.execute".to_owned());
            capability.evidence.push("audit.receipt".to_owned());
        }
        capability.lifecycle = CapabilityLifecycle::Validating;

        let mut registry = CapabilityRegistry::new();
        registry.register(capability).unwrap();
        registry
            .transition(&id, CapabilityLifecycle::Active)
            .unwrap();
        (registry, id)
    }

    fn enabled_agent(
        agent_id: AgentId,
        capability_id: &CapabilityId,
        side_effects: SideEffectClass,
    ) -> AgentContract {
        let mut agent = AgentContract::new(
            agent_id,
            "test.agent",
            "Execute governed test actions",
        )
        .unwrap()
        .with_capability(capability_id.as_str())
        .unwrap()
        .with_side_effect_limit(side_effects)
        .enable();

        if side_effects.requires_authorization() {
            agent = agent.with_policy_scope("test.execute").unwrap();
        }

        agent
    }

    #[test]
    fn unknown_capability_is_denied() {
        let registry = CapabilityRegistry::new();
        let agent_id = AgentId::new();
        let capability_id =
            CapabilityId::new("cat.capability.test.unknown.v1").unwrap();
        let agent = enabled_agent(agent_id, &capability_id, SideEffectClass::S0);
        let engine = CapabilityAuthorizationEngine::new(&registry);
        let request =
            AuthorizationRequest::new(agent_id, capability_id, SideEffectClass::S0);

        assert_eq!(
            engine.authorize(&agent, &request).outcome,
            AuthorizationOutcome::Denied
        );
    }

    #[test]
    fn undeclared_capability_is_denied() {
        let (registry, capability_id) = active_registry(SideEffectClass::S1);
        let agent_id = AgentId::new();
        let agent = AgentContract::new(
            agent_id,
            "test.agent",
            "Execute governed test actions",
        )
        .unwrap()
        .with_side_effect_limit(SideEffectClass::S1)
        .with_policy_scope("test.execute")
        .unwrap()
        .enable();
        let engine = CapabilityAuthorizationEngine::new(&registry);
        let request =
            AuthorizationRequest::new(agent_id, capability_id, SideEffectClass::S1);

        assert_eq!(
            engine.authorize(&agent, &request).outcome,
            AuthorizationOutcome::Denied
        );
    }

    #[test]
    fn missing_policy_scope_is_denied() {
        let (registry, capability_id) = active_registry(SideEffectClass::S1);
        let agent_id = AgentId::new();
        let agent = AgentContract::new(
            agent_id,
            "test.agent",
            "Execute governed test actions",
        )
        .unwrap()
        .with_capability(capability_id.as_str())
        .unwrap()
        .with_side_effect_limit(SideEffectClass::S1)
        .enable();
        let engine = CapabilityAuthorizationEngine::new(&registry);
        let request =
            AuthorizationRequest::new(agent_id, capability_id, SideEffectClass::S1);

        assert_eq!(
            engine.authorize(&agent, &request).outcome,
            AuthorizationOutcome::Denied
        );
    }

    #[test]
    fn high_impact_action_requires_approval_reference() {
        let (registry, capability_id) = active_registry(SideEffectClass::S3);
        let agent_id = AgentId::new();
        let agent = enabled_agent(agent_id, &capability_id, SideEffectClass::S3);
        let engine = CapabilityAuthorizationEngine::new(&registry);

        let pending = engine.authorize(
            &agent,
            &AuthorizationRequest::new(
                agent_id,
                capability_id.clone(),
                SideEffectClass::S3,
            ),
        );
        assert_eq!(
            pending.outcome,
            AuthorizationOutcome::ApprovalRequired
        );
        assert!(pending.approval_required);

        let approved = engine.authorize(
            &agent,
            &AuthorizationRequest::new(
                agent_id,
                capability_id,
                SideEffectClass::S3,
            )
            .with_approval(ApprovalContext::granted("approval-123").unwrap()),
        );
        assert!(approved.is_allowed());
    }

    #[test]
    fn side_effect_escalation_is_denied() {
        let (registry, capability_id) = active_registry(SideEffectClass::S1);
        let agent_id = AgentId::new();
        let agent = enabled_agent(agent_id, &capability_id, SideEffectClass::S1);
        let engine = CapabilityAuthorizationEngine::new(&registry);
        let request =
            AuthorizationRequest::new(agent_id, capability_id, SideEffectClass::S2);

        assert_eq!(
            engine.authorize(&agent, &request).outcome,
            AuthorizationOutcome::Denied
        );
    }

    #[test]
    fn disabled_agent_is_denied() {
        let (registry, capability_id) = active_registry(SideEffectClass::S0);
        let agent_id = AgentId::new();
        let agent = AgentContract::new(
            agent_id,
            "test.agent",
            "Execute governed test actions",
        )
        .unwrap()
        .with_capability(capability_id.as_str())
        .unwrap();
        let engine = CapabilityAuthorizationEngine::new(&registry);
        let request =
            AuthorizationRequest::new(agent_id, capability_id, SideEffectClass::S0);

        assert_eq!(
            engine.authorize(&agent, &request).outcome,
            AuthorizationOutcome::Denied
        );
    }

    #[test]
    fn mismatched_agent_identity_is_denied() {
        let (registry, capability_id) = active_registry(SideEffectClass::S0);
        let agent_id = AgentId::new();
        let request_id = AgentId::new();
        let agent = enabled_agent(agent_id, &capability_id, SideEffectClass::S0);
        let engine = CapabilityAuthorizationEngine::new(&registry);
        let request =
            AuthorizationRequest::new(request_id, capability_id, SideEffectClass::S0);

        assert_eq!(
            engine.authorize(&agent, &request).outcome,
            AuthorizationOutcome::Denied
        );
    }

    #[test]
    fn approval_reference_cannot_be_empty() {
        assert!(ApprovalContext::granted(" ").is_err());
    }
}
