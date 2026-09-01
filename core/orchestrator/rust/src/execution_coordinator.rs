use uuid::Uuid;

use crate::{
    decide_retry, DurableWorkflowStore, ExecutionEngine, ExecutionEventSink, ExecutionRequest,
    LeaseProvider, OrchestratorResult, RetryPolicy, StepState, WorkerExecutionInput,
    WorkerExecutionOutcome, WorkerExecutor, WorkerExecutionResult, WorkflowEventFactory,
};

/// Coordinates one durable worker attempt without owning transport or persistence details.
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
    L: LeaseProvider,
    W: WorkerExecutor,
    E: ExecutionEventSink,
{
    pub fn execute_step(
        &mut self,
        workflow_id: Uuid,
        step_id: &str,
        now_ms: u64,
    ) -> OrchestratorResult<crate::DispatchResult> {
        let mut workflow = self.store.load(workflow_id)?;
        let current_revision = workflow.revision;
        let step = workflow
            .definition
            .steps
            .iter()
            .find(|step| step.id == step_id)
            .ok_or_else(|| crate::OrchestratorError::UnknownStep { step_id: step_id.to_owned() })?;

        if step.state != StepState::Ready {
            return Err(crate::OrchestratorError::StepNotReady { step: step_id.to_owned() });
        }

        let resource = format!("workflow/{workflow_id}/step/{step_id}");
        self.leases.acquire(&resource, &self.owner, now_ms, self.lease_ttl_ms)?;

        let request = crate::claim_step(&self.engine, &mut workflow, step_id)?;
        let execution_id = Uuid::now_v7();
        let input = WorkerExecutionInput::from_request(execution_id, &request);
        let result = self.worker.execute(input);
        let dispatch = crate::DispatchResult::from_worker(result.clone(), self.retry_policy);

        let event = WorkflowEventFactory::step_state_changed(
            workflow_id,
            step_id,
            next_state(&dispatch.action),
            request.attempt,
            workflow.revision.saturating_add(1),
            "orchestrator.execution_coordinator",
        )?;

        match dispatch.action {
            crate::DispatchAction::Complete => {
                self.engine.succeed_step(&mut workflow, step_id)?;
            }
            crate::DispatchAction::Retry { delay_ms } => {
                self.engine.wait_step(&mut workflow, step_id, Some(now_ms.saturating_add(delay_ms)))?;
            }
            crate::DispatchAction::WaitForApproval => {
                self.engine.wait_step(&mut workflow, step_id, None)?;
            }
            crate::DispatchAction::Cancel => {
                self.engine.cancel(&mut workflow)?;
            }
            crate::DispatchAction::Fail => {
                self.engine.fail_step(&mut workflow, step_id)?;
            }
        }

        let event = WorkflowEventFactory::step_state_changed(
            workflow_id,
            step_id,
            workflow.definition.steps.iter().find(|step| step.id == step_id).map(|step| step.state).unwrap_or(StepState::Failed),
            request.attempt,
            workflow.revision,
            "orchestrator.execution_coordinator",
        )?;
        self.store.commit(workflow, current_revision, std::slice::from_ref(&event))?;
        self.events.publish(event)?;

        Ok(dispatch)
    }
}

fn next_state(action: &crate::DispatchAction) -> StepState {
    match action {
        crate::DispatchAction::Complete => StepState::Succeeded,
        crate::DispatchAction::Retry { .. } | crate::DispatchAction::WaitForApproval => StepState::Waiting,
        crate::DispatchAction::Cancel => StepState::Skipped,
        crate::DispatchAction::Fail => StepState::Failed,
    }
}

#[allow(dead_code)]
fn retry_policy_for_result(result: &WorkerExecutionResult, policy: RetryPolicy, attempt: u32) -> crate::RetryDecision {
    match result.outcome {
        WorkerExecutionOutcome::Failed => decide_retry(policy, attempt),
        _ => crate::RetryDecision::Exhausted,
    }
}

#[allow(dead_code)]
fn execution_request(execution_id: Uuid, request: &ExecutionRequest) -> WorkerExecutionInput {
    WorkerExecutionInput::from_request(execution_id, request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{InMemoryDurableWorkflowStore, InMemoryLeaseProvider, RecordingExecutionEventSink, WorkflowDefinition, WorkflowInstance, WorkflowStep, WorkflowState};

    struct SuccessWorker;
    impl WorkerExecutor for SuccessWorker {
        fn execute(&mut self, _input: WorkerExecutionInput) -> WorkerExecutionResult {
            WorkerExecutionResult::success(serde_json::json!({"ok": true}))
        }
    }

    fn ready_workflow() -> WorkflowInstance {
        WorkflowInstance {
            id: Uuid::now_v7(),
            definition: WorkflowDefinition {
                workflow_type: "affiliate.test".into(), version: 1,
                steps: vec![WorkflowStep { id: "publish".into(), dependencies: vec![], state: StepState::Ready, attempt: 0, max_attempts: 3, compensation_step: None }],
            },
            state: WorkflowState::Running, revision: 0,
        }
    }

    #[test]
    fn coordinator_commits_worker_success() {
        let mut store = InMemoryDurableWorkflowStore::default();
        let workflow = ready_workflow();
        let id = workflow.id;
        store.insert(workflow);
        let mut leases = InMemoryLeaseProvider::default();
        let mut worker = SuccessWorker;
        let mut events = RecordingExecutionEventSink::default();
        let mut coordinator = ExecutionCoordinator {
            store: &mut store, leases: &mut leases, worker: &mut worker, events: &mut events,
            engine: ExecutionEngine::default(), retry_policy: RetryPolicy::default(), owner: "worker-1".into(), lease_ttl_ms: 10_000,
        };

        let result = coordinator.execute_step(id, "publish", 100).unwrap();
        assert_eq!(result.action, crate::DispatchAction::Complete);
        assert_eq!(store.load(id).unwrap().definition.steps[0].state, StepState::Succeeded);
        assert_eq!(store.outbox().len(), 1);
        assert_eq!(events.events().len(), 1);
    }
}
