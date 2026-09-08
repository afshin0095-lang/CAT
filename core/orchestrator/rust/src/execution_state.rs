use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ExecutionId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus { Pending, Running, Paused, Completed, Failed, Cancelled }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepExecutionStatus { Pending, Ready, Running, WaitingApproval, Succeeded, Failed, Skipped, Cancelled }

/// Backward-compatible name for the step execution status contract.
pub type ExecutionStepState = StepExecutionStatus;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionState { pub execution_id: ExecutionId, pub plan_id: Uuid, pub plan_version: u32, pub status: ExecutionStatus }
impl ExecutionState { pub fn new(plan_id: Uuid, plan_version: u32) -> Self { Self { execution_id: ExecutionId(Uuid::now_v7()), plan_id, plan_version, status: ExecutionStatus::Pending } } }
