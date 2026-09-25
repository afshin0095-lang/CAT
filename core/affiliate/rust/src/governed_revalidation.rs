use async_trait::async_trait;
use cat_kernel::{
    AgentContract, AgentId, CapabilityContract, CapabilityId, CapabilityLifecycle, CapabilityRegistry,
    CorrelationId, EntityId, ExecutionContext, IdempotencyKey, InvocationRequest, SideEffectClass,
    TenantId, TimestampMs,
};
use cat_orchestrator::{
    AsyncWorkerExecutor, ApprovalContext, CapabilityAdmission, DispatchAction, DurableExecutionCoordinator,
    ExecutionAuthorization, ExecutionAttemptStore, AsyncPostgresExecutionStore,
    RetryPolicy, StepState, WorkerExecutionInput, WorkerExecutionResult, WorkflowDefinition,
    WorkflowInstance, WorkflowRegistrationStore, WorkflowState, WorkflowStep, OrchestratorError,
    OrchestratorResult,
};
use crate::{
    AsyncRevalidationRequestStore, RevalidationExecutionResult, RevalidationRequestRecord,
};
use serde_json::json;
use uuid::Uuid;

pub const REVALIDATION_CAPABILITY_ID: &str = "cat.capability.affiliate.revalidate.v1";
pub const REVALIDATION_WORKFLOW_TYPE: &str = "affiliate.revalidation";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RevalidationExecutionScope {
    pub tenant_id: TenantId,
    pub project_id: Option<EntityId>,
    pub agent_id: AgentId,
}

#[async_trait]
pub trait RevalidationScopeResolver: Send + Sync {
    async fn resolve_scope(
        &self,
        request: &RevalidationRequestRecord,
    ) -> Result<RevalidationExecutionScope, String>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FixedRevalidationScopeResolver {
    pub tenant_id: TenantId,
    pub project_id: Option<EntityId>,
    pub agent_id: AgentId,
}

#[async_trait]
impl RevalidationScopeResolver for FixedRevalidationScopeResolver {
    async fn resolve_scope(
        &self,
        _request: &RevalidationRequestRecord,
    ) -> Result<RevalidationExecutionScope, String> {
        if self.tenant_id.as_uuid().is_nil() || self.agent_id.as_entity_id().as_uuid().is_nil() {
            return Err("fixed revalidation scope contains a nil identity".into());
        }
        Ok(RevalidationExecutionScope {
            tenant_id: self.tenant_id,
            project_id: self.project_id,
            agent_id: self.agent_id,
        })
    }
}

pub fn register_revalidation_capability(
    registry: &mut CapabilityRegistry,
) -> OrchestratorResult<CapabilityId> {
    let capability = revalidation_capability_id();
    if registry.contains(&capability) {
        return Ok(capability);
    }
    registry
        .register(revalidation_capability_contract())
        .map_err(|error| OrchestratorError::InvalidAuthorizationInput(error.to_string()))?;
    registry
        .transition(&capability, CapabilityLifecycle::Specified)
        .map_err(|error| OrchestratorError::InvalidAuthorizationInput(error.to_string()))?;
    registry
        .transition(&capability, CapabilityLifecycle::Validating)
        .map_err(|error| OrchestratorError::InvalidAuthorizationInput(error.to_string()))?;
    registry
        .transition(&capability, CapabilityLifecycle::Active)
        .map_err(|error| OrchestratorError::InvalidAuthorizationInput(error.to_string()))?;
    Ok(capability)
}

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
        request: &RevalidationRequestRecord,
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

        let mut execution_context = ExecutionContext::new(
            tenant_id,
            CorrelationId::new(),
            EntityId::from_uuid(request.request.opportunity_id),
            TimestampMs::new(request.request.requested_at_ms),
        );
        if let Some(project_id) = project_id {
            execution_context = execution_context.with_project(project_id);
        }

        let invocation = InvocationRequest::new(
            agent_id,
            capability.as_str(),
            execution_context,
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
        request: &RevalidationRequestRecord,
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

/// Executes claimed affiliate revalidation requests through the same governed
/// durable Orchestrator path used by other CAT capabilities.
pub struct GovernedRevalidationCoordinator<'a, R, S, E, X> {
    pub request_store: &'a R,
    pub execution_store: &'a S,
    pub executor: &'a E,
    pub scope_resolver: &'a X,
    pub capability_registry: &'a CapabilityRegistry,
    pub owner: String,
    pub lease_ttl_ms: u64,
    pub retry_policy: RetryPolicy,
}

impl<'a, R, S, E, X> GovernedRevalidationCoordinator<'a, R, S, E, X>
where
    R: AsyncRevalidationRequestStore + Send + Sync,
    S: AsyncPostgresExecutionStore + ExecutionAttemptStore + WorkflowRegistrationStore + Send + Sync,
    E: GovernedRevalidationExecutor + Send + Sync,
    X: RevalidationScopeResolver + Send + Sync,
{
    pub fn new(
        request_store: &'a R,
        execution_store: &'a S,
        executor: &'a E,
        scope_resolver: &'a X,
        capability_registry: &'a CapabilityRegistry,
        owner: impl Into<String>,
        lease_ttl_ms: u64,
        retry_policy: RetryPolicy,
    ) -> Self {
        Self {
            request_store,
            execution_store,
            executor,
            scope_resolver,
            capability_registry,
            owner: owner.into(),
            lease_ttl_ms,
            retry_policy,
        }
    }

    pub async fn run_due(
        &self,
        now_ms: u64,
        limit: u32,
    ) -> Result<Vec<GovernedRevalidationRunReport>, String> {
        let claimed = self.request_store.claim_due(now_ms, limit).await.map_err(|e| e.to_string())?;
        let mut reports = Vec::with_capacity(claimed.len());

        for request in claimed {
            reports.push(self.execute_claimed(request, now_ms).await?);
        }

        Ok(reports)
    }

    async fn execute_claimed(
        &self,
        request: RevalidationRequestRecord,
        now_ms: u64,
    ) -> Result<GovernedRevalidationRunReport, String> {
        let running = match self
            .request_store
            .transition(request.request.request_id, crate::RevalidationStatus::Running, now_ms, None)
            .await
        {
            Ok(value) => value,
            Err(error) => return Err(error.to_string()),
        };

        let scope = match self.scope_resolver.resolve_scope(&running).await {
            Ok(value) => value,
            Err(error) => {
                self.fail_request(&running, now_ms, &error).await?;
                return Ok(GovernedRevalidationRunReport::Failed {
                    request_id: running.request.request_id,
                    reason: error,
                });
            }
        };

        let plan = match GovernedRevalidationPlan::build(
            &running,
            scope.tenant_id,
            scope.project_id,
            scope.agent_id,
        ) {
            Ok(plan) => plan,
            Err(error) => {
                let reason = error.to_string();
                self.fail_request(&running, now_ms, &reason).await?;
                return Ok(GovernedRevalidationRunReport::Failed {
                    request_id: running.request.request_id,
                    reason,
                });
            }
        };

        if let Err(error) = register_revalidation_workflow(self.execution_store, &plan).await {
            let reason = error.to_string();
            self.fail_request(&running, now_ms, &reason).await?;
            return Ok(GovernedRevalidationRunReport::Failed {
                request_id: running.request.request_id,
                reason,
            });
        }

        let admission = CapabilityAdmission::new(self.capability_registry);
        let worker = AffiliateRevalidationWorker::new(self.request_store, self.executor);
        let coordinator = DurableExecutionCoordinator::new(
            self.execution_store,
            &worker,
            &self.owner,
            self.lease_ttl_ms,
            self.retry_policy,
        );

        let dispatch = match coordinator
            .execute_step(
                plan.workflow.id,
                "revalidate",
                now_ms,
                &admission,
                &plan.agent,
                &plan.invocation,
                ApprovalContext::none(),
            )
            .await
        {
            Ok(dispatch) => dispatch,
            Err(error) => {
                let reason = error.to_string();
                self.fail_request(&running, now_ms, &reason).await?;
                return Ok(GovernedRevalidationRunReport::Failed {
                    request_id: running.request.request_id,
                    reason,
                });
            }
        };

        match dispatch.action {
            DispatchAction::Complete => {
                let result = decode_worker_result(&dispatch.result.output)
                    .map_err(|error| error.to_string())?;
                self.request_store
                    .transition(
                        running.request.request_id,
                        crate::RevalidationStatus::Succeeded,
                        result.observed_at_ms,
                        None,
                    )
                    .await
                    .map_err(|error| error.to_string())?;
                Ok(GovernedRevalidationRunReport::Succeeded {
                    request_id: running.request.request_id,
                    attempt: running.attempt,
                    revision: result.revision,
                })
            }
            DispatchAction::Retry { delay_ms } => {
                let reason = format!(
                    "durable retry requested but revalidation attempt rotation is not yet implemented; delay_ms={delay_ms}"
                );
                self.fail_request(&running, now_ms, &reason).await?;
                Ok(GovernedRevalidationRunReport::Failed {
                    request_id: running.request.request_id,
                    reason,
                })
            }
            DispatchAction::Cancel | DispatchAction::Fail | DispatchAction::WaitForApproval => {
                let reason = dispatch
                    .result
                    .error_code
                    .clone()
                    .unwrap_or_else(|| "governed_revalidation_execution_failed".into());
                self.fail_request(&running, now_ms, &reason).await?;
                Ok(GovernedRevalidationRunReport::Failed {
                    request_id: running.request.request_id,
                    reason,
                })
            }
        }
    }

    async fn fail_request(
        &self,
        request: &RevalidationRequestRecord,
        now_ms: u64,
        reason: &str,
    ) -> Result<(), String> {
        self.request_store
            .transition(
                request.request.request_id,
                crate::RevalidationStatus::Failed,
                now_ms,
                Some(reason.to_owned()),
            )
            .await
            .map(|_| ())
            .map_err(|error| error.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GovernedRevalidationRunReport {
    Succeeded {
        request_id: Uuid,
        attempt: u32,
        revision: u64,
    },
    Failed {
        request_id: Uuid,
        reason: String,
    },
}

fn decode_worker_result(value: &serde_json::Value) -> OrchestratorResult<RevalidationExecutionResult> {
    let observed_at_ms = value
        .get("observed_at_ms")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| OrchestratorError::Serialization("revalidation worker result is missing observed_at_ms".into()))?;
    let revision = value
        .get("revision")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| OrchestratorError::Serialization("revalidation worker result is missing revision".into()))?;
    Ok(RevalidationExecutionResult {
        observed_at_ms,
        revision,
    })
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
        let tenant = TenantId::new();
        let project = EntityId::new();
        let plan = GovernedRevalidationPlan::build(
            &request,
            tenant,
            Some(project),
            AgentId::new(),
        )
        .unwrap();
        assert_eq!(plan.workflow.id, request.request.request_id);
        assert_eq!(plan.workflow.definition.steps[0].capability_id, plan.capability);
        assert_eq!(plan.invocation.capability, REVALIDATION_CAPABILITY_ID);
        assert_eq!(plan.invocation.context.tenant_id, tenant);
        assert_eq!(plan.invocation.context.project_id, Some(project));
    }

    #[test]
    fn plan_preserves_absent_project_scope() {
        let request = request();
        let plan = GovernedRevalidationPlan::build(
            &request,
            TenantId::new(),
            None,
            AgentId::new(),
        )
        .unwrap();
        assert_eq!(plan.invocation.context.project_id, None);
    }
}