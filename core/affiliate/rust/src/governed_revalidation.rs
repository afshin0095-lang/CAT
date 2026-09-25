use async_trait::async_trait;
use cat_kernel::{
    AgentContract, AgentId, CapabilityContract, CapabilityId, CapabilityLifecycle, CorrelationId,
    EntityId, ExecutionContext, IdempotencyKey, InvocationRequest, SideEffectClass, TenantId,
    TimestampMs,
};
use cat_orchestrator::{
    AsyncRevalidationRequestStore, AsyncWorkerExecutor, ExecutionAuthorization, ExecutionIntent,
    WorkerExecutionInput, WorkerExecutionResult, WorkflowDefinition, WorkflowInstance,
    WorkflowState, WorkflowStep, StepState, RevalidationExecutionResult, OrchestratorError,
    OrchestratorResult, WorkflowRegistrationStore, DurableExecutionCoordinator, CapabilityAdmission,
    ApprovalContext, DispatchResult, ExecutionAttemptStore, AsyncPostgresExecutionStore,
};
use serde_json::json;
use uuid::Uuid;

pub const REVALIDATION_CAPABILITY_ID: &str = "cat.capability.affiliate.revalidate.v1";
pub const REVALIDATION_WORKFLOW_TYPE: &str = "affiliate.revalidation";

pub fn revalidation_capability_id() -> CapabilityId {
    CapabilityId::new(REVALIDATION_CAPABILITY_ID).expect("canonical CAT capability id")
}

pub fn revalidation_capability_contract() -> CapabilityContract {
    let mut contract = CapabilityContract::new(
        revalidation_capability_id(),
        "affiliate",
        "Revalidate one affiliate opportunity against a selected source",
    )
    .expect("valid revalidation capability contract");
    contract.inputs = vec![
        "request_id".into(),
        "opportunity_id".into(),
        "identity".into(),
        "source".into(),
        "reason".into(),
        "attempt".into(),
    ];
    contract.outputs = vec!["observed_at_ms".into(), "revision".into()];
    contract.preconditions = vec![
        "revalidation request is durably claimed".into(),
        "target source is represented by an approved affiliate adapter".into(),
    ];
    contract.postconditions = vec![
        "result is represented by durable revalidation bookkeeping".into(),
        "execution identity remains traceable to the request".into(),
    ];
    contract.failure_model = vec![
        "typed source failure".into(),
        "provider unavailable".into(),
        "invalid source response".into(),
        "durable persistence conflict".into(),
    ];
    contract.observability = vec![
        "affiliate.revalidation.started".into(),
        "affiliate.revalidation.completed".into(),
        "affiliate.revalidation.failed".into(),
    ];
    contract.evaluation = vec![
        "observation freshness".into(),
        "source consistency".into(),
        "execution trace completeness".into(),
    ];
    contract.idempotency = cat_kernel::IdempotencyPolicy::NotApplicable;
    contract.lifecycle = CapabilityLifecycle::Validating;
    contract
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernedRevalidationPlan {
    pub capability: CapabilityId,
    pub workflow: WorkflowInstance,
    pub agent: AgentContract,
    pub invocation: InvocationRequest,
}

impl GovernedRevalidationPlan {
    pub fn build(
        request: &crate::RevalidationRequestRecord,
        tenant_id: TenantId,
        project_id: Option<EntityId>,
        agent_id: AgentId,
    ) -> OrchestratorResult<Self> {
        if tenant_id.as_uuid().is_nil() || agent_id.as_entity_id().as_uuid().is_nil() {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "affiliate revalidation plan requires non-nil tenant and agent identities".into(),
            ));
        }

        let capability = revalidation_capability_id();
        let workflow_id = request.request.request_id;
        let workflow = WorkflowInstance {
            id: workflow_id,
            state: WorkflowState::Ready,
            revision: 0,
            definition: WorkflowDefinition {
                workflow_type: REVALIDATION_WORKFLOW_TYPE.into(),
                version: 1,
                steps: vec![WorkflowStep {
                    id: "revalidate".into(),
                    capability_id: capability.clone(),
                    dependencies: Vec::new(),
                    state: StepState::Ready,
                    attempt: request.attempt.max(1).saturating_sub(1),
                    max_attempts: 3,
                    compensation_step: None,
                }],
            },
        };

        let invocation = InvocationRequest::new(
            agent_id,
            capability.as_str(),
            ExecutionContext::new(
                tenant_id,
                CorrelationId::new(),
                EntityId::from_uuid(request.request.opportunity_id),
                TimestampMs::new(request.request.requested_at_ms),
            )
            .with_project(project_id.unwrap_or_else(EntityId::new)),
            IdempotencyKey::new(format!(
                "affiliate-revalidation:{}",
                request.request.request_id
            ))
            .map_err(|error| OrchestratorError::InvalidAuthorizationInput(error.to_string()))?,
            json!({
                "request_id": request.request.request_id,
                "opportunity_id": request.request.opportunity_id,
                "identity": request.request.target.identity,
                "source": request.request.target.source,
                "reason": request.request.reason.as_str(),
                "attempt": request.attempt,
            }),
            SideEffectClass::S0,
            TimestampMs::new(request.request.requested_at_ms),
        )
        .map_err(|error| OrchestratorError::InvalidAuthorizationInput(error.to_string()))?;

        let agent = AgentContract::new(
            agent_id,
            "affiliate.revalidation.agent",
            "Execute governed affiliate opportunity revalidation",
        )
        .map_err(|error| OrchestratorError::InvalidAuthorizationInput(error.to_string()))?
        .with_capability(capability.as_str())
        .map_err(|error| OrchestratorError::InvalidAuthorizationInput(error.to_string()))?
        .enable();

        Ok(Self {
            capability,
            workflow,
            agent,
            invocation,
        })
    }
}

#[async_trait]
pub trait GovernedRevalidationExecutor: Send + Sync {
    async fn revalidate(
        &self,
        request: &crate::RevalidationRequestRecord,
        authorization: &ExecutionAuthorization,
    ) -> Result<RevalidationExecutionResult, String>;
}

pub struct AffiliateRevalidationWorker<R, E> {
    pub request_store: R,
    pub executor: E,
}

impl<R, E> AffiliateRevalidationWorker<R, E> {
    pub fn new(request_store: R, executor: E) -> Self {
        Self { request_store, executor }
    }
}

#[async_trait]
impl<R, E> AsyncWorkerExecutor for AffiliateRevalidationWorker<R, E>
where
    R: AsyncRevalidationRequestStore + Send + Sync,
    E: GovernedRevalidationExecutor + Send + Sync,
{
    async fn execute(&self, input: WorkerExecutionInput) -> WorkerExecutionResult {
        if input.authorization().capability_id().as_str() != REVALIDATION_CAPABILITY_ID {
            return WorkerExecutionResult::failure(
                "affiliate_revalidation_capability_mismatch",
                json!({"expected": REVALIDATION_CAPABILITY_ID}),
            );
        }

        let request = match self.request_store.get(input.workflow_id()).await {
            Ok(request) => request,
            Err(error) => return WorkerExecutionResult::failure(
                "affiliate_revalidation_request_load_failed",
                json!({"error": error.to_string()}),
            ),
        };

        match self.executor.revalidate(&request, input.authorization()).await {
            Ok(result) => WorkerExecutionResult::success(json!({
                "observed_at_ms": result.observed_at_ms,
                "revision": result.revision,
                "request_id": request.request.request_id,
            })),
            Err(error) => WorkerExecutionResult::failure(
                "affiliate_revalidation_execution_failed",
                json!({"error": error, "request_id": request.request.request_id}),
            ),
        }
    }
}

/// Register a newly-built affiliate revalidation workflow without exposing
/// direct SQL to the affiliate domain.
pub async fn register_revalidation_workflow<S: WorkflowRegistrationStore>(
    store: &S,
    plan: &GovernedRevalidationPlan,
) -> OrchestratorResult<()> {
    store.register_workflow(&plan.workflow).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RevalidationPriority, RevalidationReason, RevalidationRequest, RevalidationTarget};

    fn request() -> crate::RevalidationRequestRecord {
        crate::RevalidationRequestRecord {
            request: RevalidationRequest::new(
                Uuid::new_v4(),
                RevalidationTarget::new("acme:widget", "network-a"),
                RevalidationReason::Stale,
                RevalidationPriority::High,
                1_000,
                2_000,
            ),
            status: crate::RevalidationStatus::Claimed,
            attempt: 1,
            created_at_ms: 1_000,
            started_at_ms: Some(2_000),
            completed_at_ms: None,
            last_error: None,
        }
    }

    #[test]
    fn capability_contract_is_pure_and_runtime_activatable() {
        let mut contract = revalidation_capability_contract();
        assert_eq!(contract.side_effects, SideEffectClass::S0);
        assert!(contract.validate().is_ok());
        contract.lifecycle = CapabilityLifecycle::Active;
        assert!(contract.lifecycle.is_runtime_usable());
    }

    #[test]
    fn plan_binds_workflow_to_request_and_tenant() {
        let request = request();
        let plan = GovernedRevalidationPlan::build(
            &request,
            TenantId::new(),
            Some(EntityId::new()),
            AgentId::new(),
        )
        .unwrap();
        assert_eq!(plan.workflow.id, request.request.request_id);
        assert_eq!(plan.workflow.definition.steps[0].capability_id, plan.capability);
        assert_eq!(plan.invocation.capability, REVALIDATION_CAPABILITY_ID);
    }
}