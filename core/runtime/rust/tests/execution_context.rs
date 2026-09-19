use cat_runtime::{ExecutionContext, ExecutionError, ExecutionState, TaskId, TaskLease};
use std::time::Duration;

#[test]
fn execution_context_rejects_expired_lease() {
    let lease = TaskLease::acquire("worker-a", 1, Duration::from_secs(10)).unwrap();
    let result = ExecutionContext::start(TaskId(1), 1, &lease, lease.expires_at_ms, None);
    assert_eq!(result.unwrap_err(), ExecutionError::LeaseExpired);
}

#[test]
fn execution_context_cancels_and_propagates_to_child_token() {
    let lease = TaskLease::acquire("worker-a", 1, Duration::from_secs(10)).unwrap();
    let mut context = ExecutionContext::start(TaskId(1), 1, &lease, 0, None).unwrap();
    let child = context.token();
    context.cancel().unwrap();
    assert!(child.is_cancelled());
    assert_eq!(context.state, ExecutionState::Cancelled);
}

#[test]
fn execution_context_times_out_at_deadline() {
    let lease = TaskLease::acquire("worker-a", 1, Duration::from_secs(10)).unwrap();
    let mut context =
        ExecutionContext::start(TaskId(1), 1, &lease, 100, Some(Duration::from_millis(50)))
            .unwrap();
    assert_eq!(context.checkpoint(149), Ok(()));
    assert_eq!(context.checkpoint(150), Err(ExecutionError::TimedOut));
    assert_eq!(context.state, ExecutionState::TimedOut);
}
