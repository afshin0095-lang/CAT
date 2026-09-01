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
mod execution_dispatch;
mod dispatch_result;
mod worker;
mod lease;
mod idempotency;
mod model;
mod retry;
mod retry_decision;
mod replay;
mod scheduler;
mod validation;
mod durable;
mod execution_coordinator;
mod fencing;
mod outbox;
mod outbox_dispatcher;
mod postgres_contract;

pub use compensation::{begin_compensation, compensation_order};
pub use error::{OrchestratorError, OrchestratorResult};
pub use events::{WorkflowCompleted, WorkflowEventFactory, WorkflowStarted, WorkflowStepStateChanged};
pub use execution::ExecutionEngine;
pub use execution_cursor::{cursor, ExecutionCursor};
pub use execution_event::{ExecutionEvent, ExecutionEventKind};
pub use execution_state::{ExecutionState, ExecutionStepState};
pub use execution_request::ExecutionRequest;
pub use execution_dispatch::{claim_step, ready_requests};
pub use dispatch_result::{DispatchAction, DispatchResult};
pub use worker::{WorkerExecutionInput, WorkerExecutionOutcome, WorkerExecutionResult, WorkerExecutor};
pub use lease::Lease;
pub use idempotency::{workflow_key, IdempotencyRegistry};
pub use model::{StepState, WorkflowDefinition, WorkflowInstance, WorkflowState, WorkflowStep};
pub use retry::RetryPolicy;
pub use retry_decision::{decide_retry, RetryDecision};
pub use replay::{snapshot as replay_snapshot, verify_replay, ReplaySnapshot};
pub use scheduler::{ScheduleRequest, Scheduler};
pub use validation::{new_validated_instance, ready_steps, topological_order, validate_definition, workflow_id};
pub use durable::{DurableWorkflowStore, EventBusExecutionEventSink, ExecutionEventSink, InMemoryDurableWorkflowStore, InMemoryLeaseProvider, LeaseProvider, RecordingExecutionEventSink};
pub use execution_coordinator::ExecutionCoordinator;
pub use fencing::{FencedLease, FencedLeaseProvider, FencingToken, InMemoryFencedLeaseProvider};
pub use outbox::{DurableOutboxStore, InMemoryDurableOutbox, OutboxDisposition, OutboxRecord};
pub use outbox_dispatcher::{OutboxDispatchOutcome, OutboxDispatcher};
pub use postgres_contract::{PostgresDurableExecutor, PostgresSchemaV1};

#[cfg(test)]
mod tests {
    use super::*;

    fn workflow() -> WorkflowInstance {
        WorkflowInstance::new(WorkflowDefinition {
            workflow_type: "affiliate.settlement".into(), version: 1,
            steps: vec![
                WorkflowStep { id: "reserve".into(), dependencies: vec![], state: StepState::Pending, attempt: 0, max_attempts: 3, compensation_step: Some("release_reserve".into()) },
                WorkflowStep { id: "settle".into(), dependencies: vec!["reserve".into()], state: StepState::Pending, attempt: 0, max_attempts: 3, compensation_step: Some("reverse_settlement".into()) },
            ],
        })
    }

    #[test]
    fn workflow_starts_pending_and_advances_revision_on_compensation() {
        let workflow = WorkflowInstance::new(WorkflowDefinition {
            workflow_type: "affiliate.settlement".into(), version: 1,
            steps: vec![WorkflowStep { id: "reserve".into(), dependencies: vec![], state: StepState::Succeeded, attempt: 1, max_attempts: 3, compensation_step: Some("release_reserve".into()) }],
        });
        assert_eq!(workflow.state, WorkflowState::Pending); assert_eq!(compensation_order(&workflow), vec!["release_reserve"]);
        let mut running = workflow; begin_compensation(&mut running); assert_eq!(running.state, WorkflowState::Compensating); assert_eq!(running.revision, 1);
    }

    #[test]
    fn execution_engine_unlocks_dependent_steps_deterministically() {
        let engine = ExecutionEngine::default(); let mut instance = workflow(); engine.start(&mut instance).unwrap();
        assert_eq!(instance.definition.steps[0].state, StepState::Ready); assert_eq!(instance.definition.steps[1].state, StepState::Pending);
        engine.begin_step(&mut instance, "reserve").unwrap(); engine.succeed_step(&mut instance, "reserve").unwrap(); assert_eq!(instance.definition.steps[1].state, StepState::Ready);
    }

    #[test]
    fn execution_engine_rejects_invalid_step_transition() {
        let engine = ExecutionEngine::default(); let mut instance = workflow(); engine.start(&mut instance).unwrap();
        assert!(matches!(engine.succeed_step(&mut instance, "reserve"), Err(OrchestratorError::InvalidStepTransition { .. })));
    }

    #[test]
    fn execution_engine_fails_unknown_step_without_mutating_revision() {
        let engine = ExecutionEngine::default(); let mut instance = workflow(); engine.start(&mut instance).unwrap(); let revision = instance.revision;
        assert!(matches!(engine.begin_step(&mut instance, "missing"), Err(OrchestratorError::UnknownStep { .. }))); assert_eq!(instance.revision, revision);
    }

    #[test]
    fn execution_engine_cancels_non_terminal_workflow() {
        let engine = ExecutionEngine::default(); let mut instance = workflow(); engine.start(&mut instance).unwrap(); engine.cancel(&mut instance).unwrap();
        assert_eq!(instance.state, WorkflowState::Cancelled); assert!(instance.state.terminal());
    }

    #[test]
    fn ready_requests_do_not_execute_work() {
        let engine = ExecutionEngine::default(); let instance = { let mut value = workflow(); engine.start(&mut value).unwrap(); value };
        let requests = ready_requests(&instance, 123); assert_eq!(requests.len(), 1); assert_eq!(requests[0].step_id, "reserve"); assert_eq!(requests[0].attempt, 1); assert_eq!(instance.definition.steps[0].state, StepState::Ready);
    }

    #[test]
    fn claim_step_changes_state_before_dispatch() {
        let engine = ExecutionEngine::default(); let mut instance = workflow(); engine.start(&mut instance).unwrap(); let request = claim_step(&engine, &mut instance, "reserve").unwrap();
        assert_eq!(request.step_id, "reserve"); assert_eq!(request.attempt, 1); assert_eq!(instance.definition.steps[0].state, StepState::Running);
    }
}
