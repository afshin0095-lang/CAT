use std::time::Duration;
use crate::{CancellationToken, TaskId, TaskLease};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionState { Ready, Running, Completed, Cancelled, TimedOut }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExecutionError { AlreadyFinished, Cancelled, TimedOut, LeaseExpired, NotOwner }

#[derive(Clone, Debug)]
pub struct ExecutionContext {
    pub task_id: TaskId,
    pub attempt: u32,
    pub owner: String,
    pub deadline_ms: Option<u64>,
    pub state: ExecutionState,
    cancellation: CancellationToken,
}

impl ExecutionContext {
    pub fn start(task_id: TaskId, attempt: u32, lease: &TaskLease, now_ms: u64, timeout: Option<Duration>) -> Result<Self, ExecutionError> {
        if lease.is_expired_at(now_ms) { return Err(ExecutionError::LeaseExpired); }
        let deadline_ms = timeout.map(|d| now_ms.saturating_add(d.as_millis() as u64));
        Ok(Self { task_id, attempt, owner: lease.owner.clone(), deadline_ms, state: ExecutionState::Running, cancellation: CancellationToken::new() })
    }

    pub fn token(&self) -> CancellationToken { self.cancellation.child() }

    pub fn cancel(&mut self) -> Result<(), ExecutionError> {
        if self.is_terminal() { return Err(ExecutionError::AlreadyFinished); }
        self.cancellation.cancel();
        self.state = ExecutionState::Cancelled;
        Ok(())
    }

    pub fn checkpoint(&mut self, now_ms: u64) -> Result<(), ExecutionError> {
        if self.state == ExecutionState::Cancelled || self.cancellation.is_cancelled() {
            self.state = ExecutionState::Cancelled;
            return Err(ExecutionError::Cancelled);
        }
        if self.deadline_ms.is_some_and(|deadline| now_ms >= deadline) {
            self.state = ExecutionState::TimedOut;
            return Err(ExecutionError::TimedOut);
        }
        Ok(())
    }

    pub fn complete(&mut self, now_ms: u64) -> Result<(), ExecutionError> {
        self.checkpoint(now_ms)?;
        self.state = ExecutionState::Completed;
        Ok(())
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self.state, ExecutionState::Completed | ExecutionState::Cancelled | ExecutionState::TimedOut)
    }
}
