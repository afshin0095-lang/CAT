use cat_runtime::{CancellationToken, ExecutionContext, ExecutionError, ExecutionState, TaskId, TaskLease};
use std::time::Duration;

fn lease(owner: &str) -> TaskLease { TaskLease::acquire(owner, 1, Duration::from_secs(30)).unwrap() }

#[test]
fn cancellation_propagates_to_child_tokens() {
    let token = CancellationToken::new();
    let child = token.child();
    assert!(!child.is_cancelled());
    token.cancel();
    assert!(child.is_cancelled());
}

#[test]
fn execution_times_out_at_deadline() {
    let lease = lease("worker-a");
    let mut ctx = ExecutionContext::start(TaskId(7), 1, &lease, 1_000, Some(Duration::from_millis(50))).unwrap();
    assert_eq!(ctx.state, ExecutionState::Running);
    assert_eq!(ctx.checkpoint(1_049), Ok(()));
    assert_eq!(ctx.checkpoint(1_050), Err(ExecutionError::TimedOut));
    assert_eq!(ctx.state, ExecutionState::TimedOut);
}

#[test]
fn cancellation_blocks_completion() {
    let lease = lease("worker-a");
    let mut ctx = ExecutionContext::start(TaskId(8), 2, &lease, 2_000, None).unwrap();
    ctx.cancel().unwrap();
    assert_eq!(ctx.complete(2_001), Err(ExecutionError::Cancelled));
    assert!(ctx.is_terminal());
}

#[test]
fn completion_is_terminal_and_idempotent_is_rejected() {
    let lease = lease("worker-a");
    let mut ctx = ExecutionContext::start(TaskId(9), 1, &lease, 3_000, None).unwrap();
    ctx.complete(3_001).unwrap();
    assert_eq!(ctx.state, ExecutionState::Completed);
    assert_eq!(ctx.cancel(), Err(ExecutionError::AlreadyFinished));
}
