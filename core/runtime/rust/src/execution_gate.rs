use std::time::Duration;

use crate::task_lease::{LeaseError, TaskLease};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DispatchDecision {
    Execute,
    RejectLease(LeaseError),
    RejectAttemptBudget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionGate {
    pub max_attempts: u32,
}

impl ExecutionGate {
    pub fn new(max_attempts: u32) -> Option<Self> {
        (max_attempts > 0).then_some(Self { max_attempts })
    }

    pub fn admit(&self, lease: &TaskLease, owner: &str, attempt: u32, now_ms: u64) -> DispatchDecision {
        if attempt == 0 || attempt > self.max_attempts {
            return DispatchDecision::RejectAttemptBudget;
        }
        if lease.is_expired_at(now_ms) {
            return DispatchDecision::RejectLease(LeaseError::Expired);
        }
        if lease.owner != owner {
            return DispatchDecision::RejectLease(LeaseError::NotOwner);
        }
        DispatchDecision::Execute
    }

    pub fn default_lease_ttl() -> Duration {
        Duration::from_secs(30)
    }
}
