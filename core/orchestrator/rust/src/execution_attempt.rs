use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Durable identity of one worker execution attempt.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ExecutionAttemptKey {
    pub execution_id: Uuid,
    pub workflow_id: Uuid,
    pub attempt: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ExecutionAttemptStatus {
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionAttemptHealth {
    Healthy,
    Stale,
    Completed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExecutionAttempt {
    pub execution_id: Uuid,
    pub workflow_id: Uuid,
    pub step_id: String,
    pub attempt: u32,
    pub status: ExecutionAttemptStatus,
    pub owner: String,
    pub fencing_token: u64,
    pub started_at_ms: u64,
    pub heartbeat_at_ms: u64,
    pub finished_at_ms: Option<u64>,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl ExecutionAttemptStatus {
    pub const fn terminal(self) -> bool {
        matches!(self, Self::Succeeded | Self::Failed | Self::Cancelled)
    }
}

impl ExecutionAttempt {
    pub fn key(&self) -> ExecutionAttemptKey {
        ExecutionAttemptKey {
            execution_id: self.execution_id,
            workflow_id: self.workflow_id,
            attempt: self.attempt,
        }
    }

    pub fn health(&self, now_ms: u64, heartbeat_timeout_ms: u64) -> ExecutionAttemptHealth {
        match self.status {
            ExecutionAttemptStatus::Succeeded
            | ExecutionAttemptStatus::Failed
            | ExecutionAttemptStatus::Cancelled => ExecutionAttemptHealth::Completed,
            ExecutionAttemptStatus::Running => {
                let deadline = self.heartbeat_at_ms.saturating_add(heartbeat_timeout_ms);
                if deadline < now_ms {
                    ExecutionAttemptHealth::Stale
                } else {
                    ExecutionAttemptHealth::Healthy
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn running() -> ExecutionAttempt {
        ExecutionAttempt {
            execution_id: Uuid::now_v7(),
            workflow_id: Uuid::now_v7(),
            step_id: "publish".into(),
            attempt: 1,
            status: ExecutionAttemptStatus::Running,
            owner: "worker-1".into(),
            fencing_token: 7,
            started_at_ms: 1_000,
            heartbeat_at_ms: 2_000,
            finished_at_ms: None,
            result: None,
            error: None,
        }
    }

    #[test]
    fn running_attempt_becomes_stale_after_timeout() {
        assert_eq!(
            running().health(3_001, 1_000),
            ExecutionAttemptHealth::Stale
        );
    }

    #[test]
    fn running_attempt_is_healthy_inside_timeout() {
        assert_eq!(
            running().health(2_999, 1_000),
            ExecutionAttemptHealth::Healthy
        );
    }

    #[test]
    fn terminal_attempt_is_completed() {
        let mut attempt = running();
        attempt.status = ExecutionAttemptStatus::Succeeded;
        attempt.finished_at_ms = Some(2_500);
        assert_eq!(attempt.health(99_000, 1), ExecutionAttemptHealth::Completed);
    }
}
