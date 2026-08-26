#[path = "../src/task_lease.rs"]
mod task_lease;
#[path = "../src/execution_gate.rs"]
mod execution_gate;

use execution_gate::{DispatchDecision, ExecutionGate};
use std::time::Duration;
use task_lease::{LeaseError, TaskLease};

#[test]
fn execution_requires_a_live_lease_owned_by_the_dispatching_worker() {
    let gate = ExecutionGate::new(3).unwrap();
    let lease = TaskLease::acquire("worker-a", 11, Duration::from_secs(30)).unwrap();
    assert_eq!(
        gate.admit(&lease, "worker-a", 1, lease.expires_at_ms - 1),
        DispatchDecision::Execute
    );
    assert_eq!(
        gate.admit(&lease, "worker-b", 1, lease.expires_at_ms - 1),
        DispatchDecision::RejectLease(LeaseError::NotOwner)
    );
}

#[test]
fn execution_rejects_expired_and_out_of_budget_attempts() {
    let gate = ExecutionGate::new(2).unwrap();
    let lease = TaskLease::acquire("worker-a", 2, Duration::from_secs(5)).unwrap();

    assert_eq!(
        gate.admit(&lease, "worker-a", 0, lease.expires_at_ms - 1),
        DispatchDecision::RejectAttemptBudget
    );
    assert_eq!(
        gate.admit(&lease, "worker-a", 3, lease.expires_at_ms - 1),
        DispatchDecision::RejectAttemptBudget
    );
    assert_eq!(
        gate.admit(&lease, "worker-a", 1, lease.expires_at_ms),
        DispatchDecision::RejectLease(LeaseError::Expired)
    );
}

#[test]
fn default_lease_ttl_is_explicit_and_nonzero() {
    assert_eq!(ExecutionGate::default_lease_ttl(), Duration::from_secs(30));
    assert!(ExecutionGate::new(0).is_none());
}
