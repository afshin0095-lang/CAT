use serde::{Deserialize, Serialize};

use crate::{RetryDecision, WorkerExecutionResult};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DispatchAction {
    Complete,
    Retry { delay_ms: u64 },
    WaitForApproval,
    Cancel,
    Fail,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DispatchResult {
    pub action: DispatchAction,
    pub result: WorkerExecutionResult,
}

impl DispatchResult {
    pub fn from_worker(result: WorkerExecutionResult, retry: Option<RetryDecision>) -> Self {
        let action = match result.outcome {
            crate::WorkerExecutionOutcome::Succeeded => DispatchAction::Complete,
            crate::WorkerExecutionOutcome::WaitingApproval => DispatchAction::WaitForApproval,
            crate::WorkerExecutionOutcome::Cancelled => DispatchAction::Cancel,
            crate::WorkerExecutionOutcome::Failed => match retry {
                Some(RetryDecision::Retry { delay_ms }) => DispatchAction::Retry { delay_ms },
                _ => DispatchAction::Fail,
            },
        };
        Self { action, result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_result_can_become_retry() {
        let result = WorkerExecutionResult::failure("timeout", serde_json::json!({}));
        let dispatch =
            DispatchResult::from_worker(result, Some(RetryDecision::Retry { delay_ms: 500 }));
        assert_eq!(dispatch.action, DispatchAction::Retry { delay_ms: 500 });
    }

    #[test]
    fn success_result_completes() {
        let result = WorkerExecutionResult::success(serde_json::json!({"ok": true}));
        let dispatch = DispatchResult::from_worker(result, None);
        assert_eq!(dispatch.action, DispatchAction::Complete);
    }
}
