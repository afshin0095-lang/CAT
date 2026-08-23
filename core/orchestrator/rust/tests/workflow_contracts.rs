use cat_orchestrator::{compensation_order, Lease, RetryPolicy, ScheduleRequest, Scheduler, StepState, WorkflowDefinition, WorkflowInstance, WorkflowState, WorkflowStep};
use uuid::Uuid;

#[test]
fn lease_rejects_wrong_owner() {
    let lease = Lease::acquire("workflow:1", "worker-a", 100, 1000);
    assert!(lease.valid_for("worker-b", 200).is_err());
}

#[test]
fn scheduler_is_deterministic_for_ready_work() {
    let mut scheduler = Scheduler::default();
    let first = Uuid::now_v7();
    let second = Uuid::now_v7();
    scheduler.schedule(ScheduleRequest { workflow_id: second, not_before_ms: 200, priority: 5 });
    scheduler.schedule(ScheduleRequest { workflow_id: first, not_before_ms: 100, priority: 1 });
    assert_eq!(scheduler.pop_ready(100).unwrap().workflow_id, first);
    assert_eq!(scheduler.pop_ready(199), None);
    assert_eq!(scheduler.pop_ready(200).unwrap().workflow_id, second);
}

#[test]
fn compensation_is_reverse_success_order() {
    let workflow = WorkflowInstance::new(WorkflowDefinition {
        workflow_type: "commerce.order".into(),
        version: 1,
        steps: vec![
            WorkflowStep { id: "reserve".into(), dependencies: vec![], state: StepState::Succeeded, attempt: 1, max_attempts: 3, compensation_step: Some("release".into()) },
            WorkflowStep { id: "charge".into(), dependencies: vec!["reserve".into()], state: StepState::Succeeded, attempt: 1, max_attempts: 3, compensation_step: Some("refund".into()) },
            WorkflowStep { id: "notify".into(), dependencies: vec!["charge".into()], state: StepState::Failed, attempt: 3, max_attempts: 3, compensation_step: None },
        ],
    });
    assert_eq!(compensation_order(&workflow), vec!["refund", "release"]);
}

#[test]
fn retry_policy_stops_at_attempt_limit() {
    let policy = RetryPolicy::default();
    assert!(policy.retryable(1));
    assert!(!policy.retryable(policy.max_attempts));
}

#[test]
fn terminal_states_are_explicit() {
    assert!(WorkflowState::Succeeded.terminal());
    assert!(WorkflowState::Failed.terminal());
    assert!(WorkflowState::Cancelled.terminal());
    assert!(!WorkflowState::Running.terminal());
}
