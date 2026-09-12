use uuid::Uuid;

use crate::{
    DurableWorkflowStore, ExecutionEngine, ExecutionEventSink, LeaseProvider, OrchestratorResult,
    RetryPolicy, StepState, WorkerExecutionInput, WorkerExecutionOutcome, WorkerExecutor,
    WorkflowEventFactory, decide_retry,
};

/// Coordinates one worker attempt across lease, state transition, durable commit, and event delivery.
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
        let expected_revision = workflow.revision;
        let step = workflow
            .definition
            .steps
            .iter()
            .find(|step| step.id == step_id)
            .ok_or_else(|| crate::OrchestratorError::UnknownStep {
                step_id: step_id.to_owned(),
            })?;
        if step.state != StepState::Ready {
            return Err(crate::OrchestratorError::StepNotReady {
                step: step_id.to_owned(),
            });
        }

        let resource = format!("workflow/{workflow_id}/step/{step_id}");
        self.leases
            .acquire(&resource, &self.owner, now_ms, self.lease_ttl_ms)?;

        let request = crate::claim_step(&self.engine, &mut workflow, step_id)?;
        let execution_id = Uuid::now_v7();
        let result = self
            .worker
            .execute(WorkerExecutionInput::from_request(execution_id, &request));
        let retry = matches!(result.outcome, WorkerExecutionOutcome::Failed)
            .then(|| decide_retry(self.retry_policy, request.attempt));
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
            request.attempt,
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
    use crate::{
        InMemoryDurableWorkflowStore, InMemoryLeaseProvider, RecordingExecutionEventSink,
        WorkerExecutionResult, WorkflowDefinition, WorkflowInstance, WorkflowState, WorkflowStep,
    };

    struct SuccessWorker;
    impl WorkerExecutor for SuccessWorker {
        fn execute(&mut self, _input: WorkerExecutionInput) -> WorkerExecutionResult {
            WorkerExecutionResult::success(serde_json::json!({"ok": true}))
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
                    dependencies: vec![],
                    state: StepState::Ready,
                    attempt: 0,
                    max_attempts: 3,
                    compensation_step: None,
                }],
            },
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
            store: &mut store,
            leases: &mut leases,
            worker: &mut worker,
            events: &mut events,
            engine: ExecutionEngine::default(),
            retry_policy: RetryPolicy::default(),
            owner: "worker-1".into(),
            lease_ttl_ms: 10_000,
        };
        let result = coordinator.execute_step(id, "publish", 100).unwrap();
        assert_eq!(result.action, crate::DispatchAction::Complete);
        assert_eq!(
            store.load(id).unwrap().definition.steps[0].state,
            StepState::Succeeded
        );
        assert_eq!(store.outbox().len(), 1);
        assert_eq!(events.events().len(), 1);
    }
}
