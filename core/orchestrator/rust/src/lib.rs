#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod compensation;
mod error;
mod events;
mod execution;
mod execution_cursor;
mod execution_event;
mod execution_state;
mod execution_request;
mod lease;
mod idempotency;
mod model;
mod retry;
mod retry_decision;
mod replay;
mod scheduler;
mod validation;

pub use compensation::{begin_compensation, compensation_order};
pub use error::{OrchestratorError, OrchestratorResult};
pub use events::{WorkflowCompleted, WorkflowEventFactory, WorkflowStarted, WorkflowStepStateChanged};
pub use execution::ExecutionEngine;
pub use execution_cursor::{cursor, ExecutionCursor};
pub use execution_event::{ExecutionEvent, ExecutionEventKind};
pub use execution_state::{ExecutionState, ExecutionStepState};
pub use execution_request::ExecutionRequest;
pub use lease::Lease;
pub use idempotency::{workflow_key, IdempotencyRegistry};
pub use model::{StepState, WorkflowDefinition, WorkflowInstance, WorkflowState, WorkflowStep};
pub use retry::RetryPolicy;
pub use retry_decision::{decide_retry, RetryDecision};
pub use replay::{snapshot as replay_snapshot, verify_replay, ReplaySnapshot};
pub use scheduler::{ScheduleRequest, Scheduler};
pub use validation::{new_validated_instance, ready_steps, topological_order, validate_definition, workflow_id};

#[cfg(test)]
mod tests {
    use super::*;

    fn workflow() -> WorkflowInstance {
        WorkflowInstance::new(WorkflowDefinition {
            workflow_type: "affiliate.settlement".into(),
            version: 1,
            steps: vec![
                WorkflowStep { id: "reserve".into(), dependencies: vec![], state: StepState::Pending, attempt: 0, max_attempts: 3, compensation_step: Some("release_reserve".into()) },
                WorkflowStep { id: "settle".into(), dependencies: vec!["reserve".into()], state: StepState::Pending, attempt: 0, max_attempts: 3, compensation_step: Some("reverse_settlement".into()) },
            ],
        })
    }

    #[test]
    fn workflow_starts_pending_and_advances_revision_on_compensation() {
        let workflow = WorkflowInstance::new(WorkflowDefinition {
            workflow_type: "affiliate.settlement".into(),
            version: 1,
            steps: vec![
                WorkflowStep { id: "reserve".into(), dependencies: vec![], state: StepState::Succeeded, attempt: 1, max_attempts: 3, compensation_step: Some("release_reserve".into()) },
            ],
        });
        assert_eq!(workflow.state, WorkflowState::Pending);
        assert_eq!(compensation_order(&workflow), vec!["release_reserve"]);
        let mut running = workflow;
        begin_compensation(&mut running);
        assert_eq!(running.state, WorkflowState::Compensating);
        assert_eq!(running.revision, 1);
    }

    #[test]
    fn execution_engine_unlocks_dependent_steps_deterministically() {
        let engine = ExecutionEngine::default();
        let mut instance = workflow();
        engine.start(&mut instance).unwrap();
        assert_eq!(instance.definition.steps[0].state, StepState::Ready);
        assert_eq!(instance.definition.steps[1].state, StepState::Pending);

        engine.begin_step(&mut instance, "reserve").unwrap();
        engine.succeed_step(&mut instance, "reserve").unwrap();
        assert_eq!(instance.definition.steps[1].state, StepState::Ready);
    }

    #[test]
    fn execution_engine_rejects_invalid_step_transition() {
        let engine = ExecutionEngine::default();
        let mut instance = workflow();
        engine.start(&mut instance).unwrap();
        let error = engine.succeed_step(&mut instance, "reserve").unwrap_err();
        assert!(matches!(error, OrchestratorError::InvalidStepTransition { .. }));
    }

    #[test]
    fn execution_engine_fails_unknown_step_without_mutating_revision() {
        let engine = ExecutionEngine::default();
        let mut instance = workflow();
        engine.start(&mut instance).unwrap();
        let revision = instance.revision;
        let error = engine.begin_step(&mut instance, "missing").unwrap_err();
        assert!(matches!(error, OrchestratorError::UnknownStep { .. }));
        assert_eq!(instance.revision, revision);
    }

    #[test]
    fn execution_engine_cancels_non_terminal_workflow() {
        let engine = ExecutionEngine::default();
        let mut instance = workflow();
        engine.start(&mut instance).unwrap();
        engine.cancel(&mut instance).unwrap();
        assert_eq!(instance.state, WorkflowState::Cancelled);
        assert!(instance.state.terminal());
    }
}
