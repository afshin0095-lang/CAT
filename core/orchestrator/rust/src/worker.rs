use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ExecutionRequest;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkerExecutionInput {
    pub execution_id: Uuid,
    pub workflow_id: Uuid,
    pub step_id: String,
    pub attempt: u32,
}

impl WorkerExecutionInput {
    pub fn from_request(execution_id: Uuid, request: &ExecutionRequest) -> Self {
        Self { execution_id, workflow_id: request.workflow_id, step_id: request.step_id.clone(), attempt: request.attempt }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerExecutionOutcome { Succeeded, Failed, WaitingApproval, Cancelled }

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkerExecutionResult {
    pub outcome: WorkerExecutionOutcome,
    pub output: serde_json::Value,
    pub error_code: Option<String>,
}

impl WorkerExecutionResult {
    pub fn success(output: serde_json::Value) -> Self { Self { outcome: WorkerExecutionOutcome::Succeeded, output, error_code: None } }
    pub fn failure(error_code: impl Into<String>, output: serde_json::Value) -> Self { Self { outcome: WorkerExecutionOutcome::Failed, output, error_code: Some(error_code.into()) } }
}

/// Boundary for real workers. The orchestrator supplies intent; the worker owns I/O.
pub trait WorkerExecutor {
    fn execute(&mut self, input: WorkerExecutionInput) -> WorkerExecutionResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worker_input_preserves_execution_identity() {
        let workflow_id = Uuid::now_v7();
        let execution_id = Uuid::now_v7();
        let request = ExecutionRequest::new(workflow_id, "research", 3, 100);
        let input = WorkerExecutionInput::from_request(execution_id, &request);
        assert_eq!(input.execution_id, execution_id);
        assert_eq!(input.workflow_id, workflow_id);
        assert_eq!(input.step_id, "research");
        assert_eq!(input.attempt, 3);
    }

    #[test]
    fn result_helpers_are_machine_readable() {
        assert_eq!(WorkerExecutionResult::success(serde_json::json!({})).outcome, WorkerExecutionOutcome::Succeeded);
        let failure = WorkerExecutionResult::failure("timeout", serde_json::json!({}));
        assert_eq!(failure.outcome, WorkerExecutionOutcome::Failed);
        assert_eq!(failure.error_code.as_deref(), Some("timeout"));
    }
}
