use crate::execution_state::{ExecutionId, ExecutionStatus, StepExecutionStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionEventKind {
    Started,
    Paused,
    Resumed,
    Cancelled,
    Completed,
    Failed,
    StepReady,
    StepStarted,
    StepSucceeded,
    StepFailed,
    StepSkipped,
    ApprovalRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionEvent {
    pub execution_id: ExecutionId,
    pub plan_id: Uuid,
    pub plan_version: u32,
    pub kind: ExecutionEventKind,
    pub step_id: Option<Uuid>,
    pub step_status: Option<StepExecutionStatus>,
    pub execution_status: Option<ExecutionStatus>,
    pub sequence: u64,
}

impl ExecutionEvent {
    pub fn new(
        execution_id: ExecutionId,
        plan_id: Uuid,
        plan_version: u32,
        kind: ExecutionEventKind,
        sequence: u64,
    ) -> Self {
        Self {
            execution_id,
            plan_id,
            plan_version,
            kind,
            step_id: None,
            step_status: None,
            execution_status: None,
            sequence,
        }
    }
}
