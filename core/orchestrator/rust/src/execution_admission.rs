use cat_kernel::{
    AgentContract, AgentId, ApprovalContext, AuthorizationDecision, AuthorizationOutcome,
    AuthorizationRequest, CapabilityAuthorizationEngine, CapabilityId, CorrelationId,
    IdempotencyKey, InvocationId, InvocationRequest, KernelError, SideEffectClass,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{OrchestratorError, OrchestratorResult};

/// Immutable evidence that a concrete invocation passed the kernel capability boundary.
///
/// Creation is intentionally restricted to this module so callers cannot fabricate an
/// authorization receipt for a worker dispatch path.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExecutionAuthorization {
    invocation_id: InvocationId,
    workflow_id: Uuid,
    step_id: String,
    attempt: u32,
    agent_id: AgentId,
    capability_id: CapabilityId,
    requested_side_effect: SideEffectClass,
    required_policies: Vec<String>,
    approval_reference: Option<String>,
    idempotency_key: IdempotencyKey,
    correlation_id: CorrelationId,
    admitted_at_ms: u64,
}

impl ExecutionAuthorization {
    pub fn invocation_id(&self) -> InvocationId {
        self.invocation_id
    }

    pub fn workflow_id(&self) -> Uuid {
        self.workflow_id
    }

    pub fn step_id(&self) -> &str {
        &self.step_id
    }

    pub fn attempt(&self) -> u32 {
        self.attempt
    }

    pub fn agent_id(&self) -> AgentId {
        self.agent_id
    }

    pub fn capability_id(&self) -> &CapabilityId {
        &self.capability_id
    }

    pub fn requested_side_effect(&self) -> SideEffectClass {
        self.requested_side_effect
    }

    pub fn required_policies(&self) -> &[String] {
        &self.required_policies
    }

    pub fn approval_reference(&self) -> Option<&str> {
        self.approval_reference.as_deref()
    }

    pub fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    pub fn correlation_id(&self) -> CorrelationId {
        self.correlation_id
    }

    pub fn admitted_at_ms(&self) -> u64 {
        self.admitted_at_ms
    }

    pub fn matches_execution(
        &self,
        workflow_id: Uuid,
        step_id: &str,
        attempt: u32,
    ) -> bool {
        self.workflow_id == workflow_id
            && self.step_id == step_id
            && self.attempt == attempt
    }
}

/// Result of admission before any worker lease or external side effect is attempted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CapabilityAdmissionResult {
    Admitted(ExecutionAuthorization),
    ApprovalRequired(AuthorizationDecision),
    Denied(AuthorizationDecision),
}

/// Kernel-backed admission service for durable orchestrator execution.
#[derive(Debug)]
pub struct CapabilityAdmission<'a> {
    engine: CapabilityAuthorizationEngine<'a>,
}

impl<'a> CapabilityAdmission<'a> {
    pub const fn new(registry: &'a cat_kernel::CapabilityRegistry) -> Self {
        Self {
            engine: CapabilityAuthorizationEngine::new(registry),
        }
    }

    pub fn admit(
        &self,
        workflow_id: Uuid,
        step_id: impl Into<String>,
        attempt: u32,
        agent: &AgentContract,
        invocation: &InvocationRequest,
        approval: ApprovalContext,
        admitted_at_ms: u64,
    ) -> OrchestratorResult<CapabilityAdmissionResult> {
        if workflow_id.is_nil() {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "workflow id must not be nil".to_owned(),
            ));
        }

        let step_id = step_id.into();
        if step_id.trim().is_empty() {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "step id must not be empty".to_owned(),
            ));
        }

        if attempt == 0 {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "execution attempt must be greater than zero".to_owned(),
            ));
        }

        invocation
            .validate()
            .map_err(map_kernel_validation_error)?;

        let capability_id = CapabilityId::new(invocation.capability.clone())
            .map_err(map_kernel_validation_error)?;

        let request = AuthorizationRequest::new(
            invocation.agent_id,
            capability_id.clone(),
            invocation.requested_side_effect,
        )
        .with_approval(approval);

        let decision = self.engine.authorize(agent, &request);

        match decision.outcome {
            AuthorizationOutcome::Allowed => Ok(CapabilityAdmissionResult::Admitted(
                ExecutionAuthorization {
                    invocation_id: invocation.invocation_id,
                    workflow_id,
                    step_id,
                    attempt,
                    agent_id: invocation.agent_id,
                    capability_id,
                    requested_side_effect: invocation.requested_side_effect,
                    required_policies: decision.required_policies,
                    approval_reference: request.approval.reference,
                    idempotency_key: invocation.idempotency_key.clone(),
                    correlation_id: invocation.context.correlation_id,
                    admitted_at_ms,
                },
            )),
            AuthorizationOutcome::ApprovalRequired => {
                Ok(CapabilityAdmissionResult::ApprovalRequired(decision))
            }
            AuthorizationOutcome::Denied => Ok(CapabilityAdmissionResult::Denied(decision)),
        }
    }
}

fn map_kernel_validation_error(error: KernelError) -> OrchestratorError {
    OrchestratorError::InvalidAuthorizationInput(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cat_kernel::{
        CapabilityContract, CapabilityLifecycle, CapabilityRegistry, ContractVersion,
        EntityId, ExecutionContext, IdempotencyPolicy, TenantId, TimestampMs,
    };

    fn registry(side_effects: SideEffectClass) -> (CapabilityRegistry, CapabilityId) {
        let id = CapabilityId::new("cat.capability.test.execute.v1").unwrap();
        let mut capability = CapabilityContract::new(
            id.clone(),
            "test",
            "Execute a governed test action",
        )
        .unwrap();
        capability.contract_version = ContractVersion::V1;
        capability.inputs.push("input".into());
        capability.outputs.push("output".into());
        capability.failure_model.push("typed failure".into());
        capability.observability.push("test.execute".into());
        capability.evaluation.push("deterministic".into());
        capability.side_effects = side_effects;
        capability.idempotency = match side_effects {
            SideEffectClass::S0 => IdempotencyPolicy::NotApplicable,
            _ => IdempotencyPolicy::Required,
        };
        if side_effects.requires_authorization() {
            capability.required_policy.push("test.execute".into());
            capability.evidence.push("audit.receipt".into());
        }
        capability.lifecycle = CapabilityLifecycle::Validating;

        let mut registry = CapabilityRegistry::new();
        registry.register(capability).unwrap();
        registry.transition(&id, CapabilityLifecycle::Active).unwrap();
        (registry, id)
    }

    fn agent(
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

    fn invocation(agent_id: AgentId, capability: &CapabilityId, side_effects: SideEffectClass) -> InvocationRequest {
        InvocationRequest::new(
            agent_id,
            capability.as_str(),
            ExecutionContext::new(
                TenantId::new(),
                CorrelationId::new(),
                EntityId::new(),
                TimestampMs::new(1_000),
            ),
            IdempotencyKey::new("exec-1").unwrap(),
            serde_json::json!({"ok":true}),
            side_effects,
            TimestampMs::new(1_000),
        )
        .unwrap()
    }

    #[test]
    fn admitted_execution_binds_workflow_step_attempt_and_identity() {
        let (registry, capability_id) = registry(SideEffectClass::S0);
        let agent_id = AgentId::new();
        let agent = agent(agent_id, &capability_id, SideEffectClass::S0);
        let invocation = invocation(agent_id, &capability_id, SideEffectClass::S0);
        let admission = CapabilityAdmission::new(&registry);

        let result = admission
            .admit(
                Uuid::now_v7(),
                "publish",
                1,
                &agent,
                &invocation,
                ApprovalContext::none(),
                2_000,
            )
            .unwrap();

        let CapabilityAdmissionResult::Admitted(authorization) = result else {
            panic!("expected admission");
        };
        assert_eq!(authorization.agent_id(), agent_id);
        assert_eq!(authorization.capability_id(), &capability_id);
        assert_eq!(authorization.attempt(), 1);
        assert!(authorization.matches_execution(authorization.workflow_id(), "publish", 1));
        assert!(!authorization.idempotency_key().as_str().is_empty());
    }

    #[test]
    fn high_impact_execution_stops_before_worker_admission() {
        let (registry, capability_id) = registry(SideEffectClass::S3);
        let agent_id = AgentId::new();
        let agent = agent(agent_id, &capability_id, SideEffectClass::S3);
        let invocation = invocation(agent_id, &capability_id, SideEffectClass::S3);
        let admission = CapabilityAdmission::new(&registry);

        let result = admission
            .admit(
                Uuid::now_v7(),
                "publish",
                1,
                &agent,
                &invocation,
                ApprovalContext::none(),
                2_000,
            )
            .unwrap();

        assert!(matches!(
            result,
            CapabilityAdmissionResult::ApprovalRequired(_)
        ));
    }

    #[test]
    fn denied_execution_never_produces_an_authorization_receipt() {
        let (registry, capability_id) = registry(SideEffectClass::S1);
        let agent_id = AgentId::new();
        let wrong_agent = AgentId::new();
        let agent = agent(agent_id, &capability_id, SideEffectClass::S1);
        let invocation = invocation(wrong_agent, &capability_id, SideEffectClass::S1);
        let admission = CapabilityAdmission::new(&registry);

        let result = admission
            .admit(
                Uuid::now_v7(),
                "publish",
                1,
                &agent,
                &invocation,
                ApprovalContext::none(),
                2_000,
            )
            .unwrap();

        assert!(matches!(result, CapabilityAdmissionResult::Denied(_)));
    }

    #[test]
    fn zero_attempt_is_rejected_at_orchestrator_boundary() {
        let (registry, capability_id) = registry(SideEffectClass::S0);
        let agent_id = AgentId::new();
        let agent = agent(agent_id, &capability_id, SideEffectClass::S0);
        let invocation = invocation(agent_id, &capability_id, SideEffectClass::S0);
        let admission = CapabilityAdmission::new(&registry);

        let result = admission.admit(
            Uuid::now_v7(),
            "publish",
            0,
            &agent,
            &invocation,
            ApprovalContext::none(),
            2_000,
        );

        assert!(result.is_err());
    }
}
