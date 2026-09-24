use cat_kernel::{
    AgentContract, ApprovalContext, CapabilityId, InvocationRequest,
};
use uuid::Uuid;

use crate::{
    CapabilityAdmission, CapabilityAdmissionResult, DurableWorkflowStore, ExecutionEngine,
    ExecutionEventSink, ExecutionRequest, ExecutionIntent, LeaseProvider, OrchestratorResult,
    RetryPolicy, StepState, WorkerExecutionInput, WorkerExecutionOutcome, WorkerExecutor,
    WorkflowEventFactory, decide_retry,
};

/// Coordinates one governed worker attempt across authorization, lease, state transition,
/// durable commit, and event delivery.
pub struct ExecutionCoordinator<'a, S, L, W, E> {
    pub store: &'a mut S,
    pub leases: &'a mut L,
    pub worker: &'a mut W,
    pub events: &'a mut E,
    pub engine: ExecutionEngine,
    pub retry_policy: RetryPolicy,
    pub owner: String,
    pub lease_ttl_ms: u64,
}

impl<'a, S, L, W, E> ExecutionCoordinator<'a, S, L, W, E>
where
    S: DurableWorkflowStore,
    L: FencedLeaseProvider,
    W: WorkerExecutor,
    E: ExecutionEventSink,
{
    /// Executes exactly one already-authorized workflow step.
    ///
    /// Authorization is evaluated before lease acquisition. A denied or approval-required
    /// request never claims the step and never reaches the worker.
    pub fn execute_step(
        &mut self,
        workflow_id: Uuid,
        step_id: &str,
        now_ms: u64,
        admission: &CapabilityAdmission<'_>,
        agent: &AgentContract,
        invocation: &InvocationRequest,
        approval: ApprovalContext,
    ) -> OrchestratorResult<crate::DispatchResult> {
        let mut workflow = self.store.load(workflow_id)?;
        let expected_revision = workflow.revision;
        let expected_attempt = workflow
            .definition
            .steps
            .iter()
            .find(|step| step.id == step_id)
            .ok_or_else(|| crate::OrchestratorError::UnknownStep {
                step_id: step_id.to_owned(),
            })?
            .attempt
            .saturating_add(1);

        let step_capability = workflow
            .definition
            .steps
            .iter()
            .find(|step| step.id == step_id)
            .map(|step| step.capability_id.clone())
            .ok_or_else(|| crate::OrchestratorError::UnknownStep {
                step_id: step_id.to_owned(),
            })?;

        if invocation.capability != step_capability.as_str() {
            return Err(crate::OrchestratorError::StepCapabilityMismatch {
                step_id: step_id.to_owned(),
                expected: step_capability.to_string(),
                requested: invocation.capability.clone(),
            });
        }

        let step_is_ready = workflow
            .definition
            .steps
            .iter()
            .find(|step| step.id == step_id)
            .is_some_and(|step| step.state == StepState::Ready);
        if !step_is_ready {
            return Err(crate::OrchestratorError::StepNotReady {
                step: step_id.to_owned(),
            });
        }

        let admission = admission.admit(
            workflow_id,
            step_id,
            expected_attempt,
            agent,
            invocation,
            approval,
            now_ms,
        )?;

        let authorization = match admission {
            CapabilityAdmissionResult::Admitted(receipt) => receipt,
            CapabilityAdmissionResult::ApprovalRequired(decision) => {
                return Err(crate::OrchestratorError::CapabilityApprovalRequired {
                    capability: decision.capability_id.to_string(),
                    reasons: decision.reasons,
                });
            }
            CapabilityAdmissionResult::Denied(decision) => {
                return Err(crate::OrchestratorError::CapabilityAuthorizationDenied {
                    capability: decision.capability_id.to_string(),
                    reasons: decision.reasons,
                });
            }
        };

        let resource = format!("workflow/{workflow_id}/step/{step_id}");
        let fenced_lease =
            self.leases
                .acquire(&resource, &self.owner, now_ms, self.lease_ttl_ms)?;

        let intent = crate::claim_step(&self.engine, &mut workflow, step_id)?;
        let request = ExecutionRequest::from_authorized(intent, authorization)?;
        let execution_id = Uuid::now_v7();
        let result = self
            .worker
            .execute(WorkerExecutionInput::from_request(execution_id, &request));
        let retry = matches!(result.outcome, WorkerExecutionOutcome::Failed)
            .then(|| decide_retry(self.retry_policy, request.attempt()));
        let dispatch = crate::DispatchResult::from_worker(result, retry);

        match dispatch.action {
            crate::DispatchAction::Complete => self.engine.succeed_step(&mut workflow, step_id)?,
            crate::DispatchAction::Retry { .. } | crate::DispatchAction::WaitForApproval => {
                self.engine.wait_step(&mut workflow, step_id)?
            }
            crate::DispatchAction::Cancel => self.engine.cancel_step(&mut workflow, step_id)?,
            crate::DispatchAction::Fail => self.engine.fail_step(&mut workflow, step_id)?,
        }

        let final_state = workflow
            .definition
            .steps
            .iter()
            .find(|step| step.id == step_id)
            .map(|step| step.state)
            .unwrap_or(StepState::Failed);
        let event = WorkflowEventFactory::step_state_changed(
            workflow_id,
            step_id,
            final_state,
            request.attempt(),
            workflow.revision,
            "orchestrator.execution_coordinator",
        )?;

        self.store
            .commit(workflow, expected_revision, std::slice::from_ref(&event))?;
        // If delivery fails, the event remains in the durable outbox for replay.
        self.events.publish(event)?;
        Ok(dispatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cat_kernel::{
        AgentId, CapabilityContract, CapabilityLifecycle, CapabilityRegistry, CorrelationId,
        EntityId, ExecutionContext, IdempotencyKey, IdempotencyPolicy, SideEffectClass, TenantId,
        TimestampMs,
    };
    use crate::{
        InMemoryDurableWorkflowStore, InMemoryFencedLeaseProvider, RecordingExecutionEventSink, WorkflowDefinition,
        WorkflowInstance, WorkflowState, WorkflowStep,
    };

    struct SuccessWorker {
        capability_seen: Option<CapabilityId>,
    }

    impl WorkerExecutor for SuccessWorker {
        fn execute(&mut self, input: WorkerExecutionInput) -> crate::WorkerExecutionResult {
            self.capability_seen = Some(input.authorization().capability_id().clone());
            crate::WorkerExecutionResult::success(serde_json::json!({"ok": true}))
        }
    }

    #[derive(Default)]
    struct CountingLease {
        acquisitions: u32,
    }

    impl LeaseProvider for CountingLease {
        fn acquire(
            &mut self,
            resource: &str,
            owner: &str,
            now_ms: u64,
            ttl_ms: u64,
        ) -> OrchestratorResult<crate::FencedLease> {
            self.acquisitions += 1;
            Ok(Lease::acquire(resource, owner, now_ms, ttl_ms))
        }
    }

    fn ready_workflow() -> WorkflowInstance {
        WorkflowInstance {
            id: Uuid::now_v7(),
            state: WorkflowState::Running,
            revision: 0,
            definition: WorkflowDefinition {
                workflow_type: "affiliate.test".into(),
                version: 1,
                steps: vec![WorkflowStep {
                    id: "publish".into(),
                    capability_id: cat_kernel::CapabilityId::new("cat.capability.test.publish.v1").unwrap(),
                    dependencies: vec![],
                    state: StepState::Ready,
                    attempt: 0,
                    max_attempts: 3,
                    compensation_step: None,
                }],
            },
        }
    }

    fn governed_context() -> (
        CapabilityRegistry,
        CapabilityId,
        AgentContract,
        InvocationRequest,
    ) {
        let capability_id = CapabilityId::new("cat.capability.test.publish.v1").unwrap();
        let mut capability =
            CapabilityContract::new(capability_id.clone(), "test", "Publish test output").unwrap();
        capability.inputs.push("input".into());
        capability.outputs.push("output".into());
        capability.failure_model.push("typed failure".into());
        capability.observability.push("publish.execution".into());
        capability.evaluation.push("deterministic".into());
        capability.lifecycle = CapabilityLifecycle::Validating;

        let mut registry = CapabilityRegistry::new();
        registry.register(capability).unwrap();
        registry
            .transition(&capability_id, CapabilityLifecycle::Active)
            .unwrap();

        let agent_id = AgentId::new();
        let agent = AgentContract::new(agent_id, "test.publisher", "Publish governed output")
            .unwrap()
            .with_capability(capability_id.as_str())
            .unwrap()
            .enable();

        let invocation = InvocationRequest::new(
            agent_id,
            capability_id.as_str(),
            ExecutionContext::new(
                TenantId::new(),
                CorrelationId::new(),
                EntityId::new(),
                TimestampMs::new(1_000),
            ),
            IdempotencyKey::new("coordinator-test-1").unwrap(),
            serde_json::json!({"publish":true}),
            SideEffectClass::S0,
            TimestampMs::new(1_000),
        )
        .unwrap();

        (registry, capability_id, agent, invocation)
    }

    #[test]
    fn coordinator_requires_governed_worker_dispatch() {
        let mut store = InMemoryDurableWorkflowStore::default();
        let workflow = ready_workflow();
        let id = workflow.id;
        store.insert(workflow);
        let mut leases = InMemoryFencedLeaseProvider::default();
        let mut worker = SuccessWorker { capability_seen: None };
        let mut events = RecordingExecutionEventSink::default();

        let (registry, capability_id, agent, invocation) = governed_context();
        let admission = CapabilityAdmission::new(&registry);

        let mut coordinator = ExecutionCoordinator {
            store: &mut store,
            leases: &mut leases,
            worker: &mut worker,
            events: &mut events,
            engine: ExecutionEngine::default(),
            retry_policy: RetryPolicy::default(),
            owner: "worker-1".into(),
            lease_ttl_ms: 10_000,
        };

        let result = coordinator
            .execute_step(
                id,
                "publish",
                100,
                &admission,
                &agent,
                &invocation,
                ApprovalContext::none(),
            )
            .unwrap();

        assert_eq!(result.action, crate::DispatchAction::Complete);
        assert_eq!(worker.capability_seen.as_ref(), Some(&capability_id));
        assert_eq!(
            store.load(id).unwrap().definition.steps[0].state,
            StepState::Succeeded
        );
        assert_eq!(store.outbox().len(), 1);
        assert_eq!(events.events().len(), 1);
    }

    #[test]
    fn mismatched_step_capability_is_rejected_before_lease() {
        let mut store = InMemoryDurableWorkflowStore::default();
        let workflow = ready_workflow();
        let id = workflow.id;
        store.insert(workflow);

        let (registry, _capability_id, agent, mut invocation) = governed_context();
        invocation.capability = "cat.capability.test.other.v1".into();
        let admission = CapabilityAdmission::new(&registry);

        let mut leases = CountingLease::default();
        let mut worker = SuccessWorker { capability_seen: None };
        let mut events = RecordingExecutionEventSink::default();

        let mut coordinator = ExecutionCoordinator {
            store: &mut store,
            leases: &mut leases,
            worker: &mut worker,
            events: &mut events,
            engine: ExecutionEngine::default(),
            retry_policy: RetryPolicy::default(),
            owner: "worker-1".into(),
            lease_ttl_ms: 10_000,
        };

        let error = coordinator
            .execute_step(
                id,
                "publish",
                100,
                &admission,
                &agent,
                &invocation,
                ApprovalContext::none(),
            )
            .unwrap_err();

        assert!(matches!(
            error,
            crate::OrchestratorError::StepCapabilityMismatch { .. }
        ));
        assert_eq!(leases.acquisitions, 0);
        assert!(worker.capability_seen.is_none());
    }

    #[test]
    fn approval_required_never_acquires_worker_lease() {
        let mut store = InMemoryDurableWorkflowStore::default();
        let workflow = ready_workflow();
        let id = workflow.id;
        store.insert(workflow);

        let mut capability_registry = CapabilityRegistry::new();
        let capability_id = CapabilityId::new("cat.capability.test.high_impact.v1").unwrap();
        let mut capability =
            CapabilityContract::new(capability_id.clone(), "test", "High impact test action")
                .unwrap();
        capability.inputs.push("input".into());
        capability.outputs.push("output".into());
        capability.failure_model.push("typed failure".into());
        capability.observability.push("high-impact".into());
        capability.evaluation.push("deterministic".into());
        capability.side_effects = SideEffectClass::S3;
        capability.required_policy.push("test.high_impact".into());
        capability.evidence.push("approval.audit".into());
        capability.idempotency = IdempotencyPolicy::Required;
        capability.lifecycle = CapabilityLifecycle::Validating;
        capability_registry.register(capability).unwrap();
        capability_registry
            .transition(&capability_id, CapabilityLifecycle::Active)
            .unwrap();

        let agent_id = AgentId::new();
        let agent = AgentContract::new(agent_id, "test.publisher", "High impact action")
            .unwrap()
            .with_capability(capability_id.as_str())
            .unwrap()
            .with_policy_scope("test.high_impact")
            .unwrap()
            .with_side_effect_limit(SideEffectClass::S3)
            .enable();

        let invocation = InvocationRequest::new(
            agent_id,
            capability_id.as_str(),
            ExecutionContext::new(
                TenantId::new(),
                CorrelationId::new(),
                EntityId::new(),
                TimestampMs::new(1_000),
            ),
            IdempotencyKey::new("approval-coordinator-test").unwrap(),
            serde_json::json!({"charge":true}),
            SideEffectClass::S3,
            TimestampMs::new(1_000),
        )
        .unwrap();

        let admission = CapabilityAdmission::new(&capability_registry);
        let mut leases = CountingLease::default();
        let mut worker = SuccessWorker { capability_seen: None };
        let mut events = RecordingExecutionEventSink::default();

        let mut coordinator = ExecutionCoordinator {
            store: &mut store,
            leases: &mut leases,
            worker: &mut worker,
            events: &mut events,
            engine: ExecutionEngine::default(),
            retry_policy: RetryPolicy::default(),
            owner: "worker-1".into(),
            lease_ttl_ms: 10_000,
        };

        let error = coordinator
            .execute_step(
                id,
                "publish",
                100,
                &admission,
                &agent,
                &invocation,
                ApprovalContext::none(),
            )
            .unwrap_err();

        assert!(matches!(
            error,
            crate::OrchestratorError::CapabilityApprovalRequired { .. }
        ));
        assert_eq!(leases.acquisitions, 0);
        assert!(worker.capability_seen.is_none());
        assert_eq!(
            store.load(id).unwrap().definition.steps[0].state,
            StepState::Ready
        );
    }
}
