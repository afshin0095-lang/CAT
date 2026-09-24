use cat_kernel::{AgentContract, ApprovalContext, InvocationRequest};
use uuid::Uuid;

use crate::{
    AsyncPostgresExecutionStore, AsyncWorkerExecutor, CapabilityAdmission, CapabilityAdmissionResult,
    DispatchAction, DispatchResult, ExecutionAttempt, ExecutionAttemptStatus,
    ExecutionAttemptStore, ExecutionEngine, ExecutionRequest, OrchestratorError, OrchestratorResult,
    RetryPolicy, StepState, WorkerExecutionInput, WorkerExecutionOutcome, WorkflowEventFactory,
    WorkflowInstance, decide_retry,
};

/// Production async execution coordinator.
///
/// The coordinator is deliberately ordered so that no worker or external side effect can start
/// before capability authorization and durable attempt registration have both succeeded.
pub struct DurableExecutionCoordinator<'a, S, W> {
    pub store: &'a S,
    pub worker: &'a W,
    pub engine: ExecutionEngine,
    pub retry_policy: RetryPolicy,
    pub owner: String,
    pub lease_ttl_ms: u64,
}

impl<'a, S, W> DurableExecutionCoordinator<'a, S, W>
where
    S: AsyncPostgresExecutionStore + ExecutionAttemptStore,
    W: AsyncWorkerExecutor,
{
    pub fn new(
        store: &'a S,
        worker: &'a W,
        owner: impl Into<String>,
        lease_ttl_ms: u64,
        retry_policy: RetryPolicy,
    ) -> Self {
        Self {
            store,
            worker,
            engine: ExecutionEngine::default(),
            retry_policy,
            owner: owner.into(),
            lease_ttl_ms,
        }
    }

    /// Executes one ready workflow step through the fully governed durable path.
    pub async fn execute_step(
        &self,
        workflow_id: Uuid,
        step_id: &str,
        now_ms: u64,
        admission: &CapabilityAdmission<'_>,
        agent: &AgentContract,
        invocation: &InvocationRequest,
        approval: ApprovalContext,
    ) -> OrchestratorResult<DispatchResult> {
        let mut workflow = self.store.load_workflow(workflow_id).await?;
        let expected_revision = workflow.revision;

        let step = workflow
            .definition
            .steps
            .iter()
            .find(|step| step.id == step_id)
            .ok_or_else(|| OrchestratorError::UnknownStep {
                step_id: step_id.to_owned(),
            })?;

        if step.state != StepState::Ready {
            return Err(OrchestratorError::StepNotReady {
                step: step_id.to_owned(),
            });
        }

        if invocation.capability != step.capability_id.as_str() {
            return Err(OrchestratorError::StepCapabilityMismatch {
                step_id: step_id.to_owned(),
                expected: step.capability_id.to_string(),
                requested: invocation.capability.clone(),
            });
        }

        let attempt = step.attempt.saturating_add(1);
        if attempt == 0 {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "execution attempt overflowed to zero".to_owned(),
            ));
        }

        let authorization = match admission
            .admit(
                workflow_id,
                step_id,
                attempt,
                agent,
                invocation,
                approval,
                now_ms,
            )? {
            CapabilityAdmissionResult::Admitted(receipt) => receipt,
            CapabilityAdmissionResult::ApprovalRequired(decision) => {
                return Err(OrchestratorError::CapabilityApprovalRequired {
                    capability: decision.capability_id.to_string(),
                    reasons: decision.reasons,
                });
            }
            CapabilityAdmissionResult::Denied(decision) => {
                return Err(OrchestratorError::CapabilityAuthorizationDenied {
                    capability: decision.capability_id.to_string(),
                    reasons: decision.reasons,
                });
            }
        };

        let resource = format!("workflow/{workflow_id}/step/{step_id}");
        let fenced_lease = self
            .store
            .acquire_fenced_lease(&resource, &self.owner, now_ms, self.lease_ttl_ms)
            .await?;

        let intent = crate::execution_dispatch::claim_step(&self.engine, &mut workflow, step_id)?;
        let request = ExecutionRequest::from_authorized(intent, authorization.clone())?;
        let execution_id = Uuid::now_v7();

        let execution_attempt = ExecutionAttempt {
            execution_id,
            workflow_id,
            step_id: step_id.to_owned(),
            attempt: request.attempt(),
            status: ExecutionAttemptStatus::Running,
            owner: self.owner.clone(),
            fencing_token: fenced_lease.fencing_token.value(),
            started_at_ms: now_ms,
            heartbeat_at_ms: now_ms,
            finished_at_ms: None,
            result: None,
            error: None,
        };

        // This is the durable gate immediately before any worker-side effect.
        self.store
            .record_execution_start(&execution_attempt, &authorization)
            .await?;

        let worker_result = self
            .worker
            .execute(WorkerExecutionInput::from_request(
                execution_id,
                &request,
                fenced_lease.fencing_token,
            ))
            .await;

        // Approval is a pre-dispatch governance boundary. A worker that returns this
        // after admission violates the governed worker contract, so it cannot leave
        // the workflow in a retryable READY state.
        if matches!(worker_result.outcome, WorkerExecutionOutcome::WaitingApproval) {
            let error_code = "post_admission_approval_required";
            self.store
                .complete_execution(
                    execution_id,
                    &self.owner,
                    fenced_lease.fencing_token,
                    ExecutionAttemptStatus::Failed,
                    now_ms,
                    Some(worker_result.output.clone()),
                    Some(error_code),
                )
                .await?;
            return Err(OrchestratorError::InvalidStateTransition {
                from: "worker_waiting_approval".to_owned(),
                to: "worker_dispatch_requires_preapproval".to_owned(),
            });
        }

        let retry = matches!(worker_result.outcome, WorkerExecutionOutcome::Failed)
            .then(|| decide_retry(self.retry_policy, request.attempt()));
        let dispatch = DispatchResult::from_worker(worker_result, retry);

        match dispatch.action {
            DispatchAction::Complete => self.engine.succeed_step(&mut workflow, step_id)?,
            DispatchAction::Retry { .. } | DispatchAction::WaitForApproval => {
                self.engine.wait_step(&mut workflow, step_id)?
            }
            DispatchAction::Cancel => self.engine.cancel_step(&mut workflow, step_id)?,
            DispatchAction::Fail => self.engine.fail_step(&mut workflow, step_id)?,
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
            "orchestrator.durable_execution_coordinator",
        )?;

        // Workflow state and outbox event remain one durable transaction.
        self.store
            .commit_workflow_and_outbox(
                &workflow,
                expected_revision,
                std::slice::from_ref(&event),
            )
            .await?;

        let attempt_status = match dispatch.action {
            DispatchAction::Complete => ExecutionAttemptStatus::Succeeded,
            DispatchAction::Retry { .. } | DispatchAction::Fail => ExecutionAttemptStatus::Failed,
            DispatchAction::Cancel => ExecutionAttemptStatus::Cancelled,
            DispatchAction::WaitForApproval => ExecutionAttemptStatus::Failed,
        };
        let attempt_error = if matches!(attempt_status, ExecutionAttemptStatus::Failed) {
            dispatch.result.error_code.as_deref()
        } else {
            None
        };

        self.store
            .complete_execution(
                execution_id,
                &self.owner,
                fenced_lease.fencing_token,
                attempt_status,
                now_ms,
                Some(dispatch.result.output.clone()),
                attempt_error,
            )
            .await?;

        Ok(dispatch)
    }
}

/// Helper for tests and adapters that need to construct the durable execution coordinator
/// without exposing a second execution path.
pub fn workflow_is_ready(workflow: &WorkflowInstance, step_id: &str) -> OrchestratorResult<()> {
    let step = workflow
        .definition
        .steps
        .iter()
        .find(|step| step.id == step_id)
        .ok_or_else(|| OrchestratorError::UnknownStep {
            step_id: step_id.to_owned(),
        })?;
    if step.state != StepState::Ready {
        return Err(OrchestratorError::StepNotReady {
            step: step_id.to_owned(),
        });
    }
    Ok(())
}
