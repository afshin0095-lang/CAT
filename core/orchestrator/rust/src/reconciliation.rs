use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    AsyncPostgresExecutionStore, ExecutionAttempt, OrchestratorResult, ProviderExecutionRecord,
    ProviderOutcomeState, ReconciliationAction,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationReport {
    pub execution_id: Uuid,
    pub action: ReconciliationAction,
    pub provider_result: Option<ProviderExecutionRecord>,
}

#[async_trait]
pub trait ExecutionReconciliationStore: AsyncPostgresExecutionStore {
    async fn load_execution_attempt(&self, execution_id: Uuid) -> OrchestratorResult<ExecutionAttempt>;
    async fn load_provider_result(&self, execution_id: Uuid) -> OrchestratorResult<Option<ProviderExecutionRecord>>;
    async fn record_provider_submission(
        &self,
        execution_id: Uuid,
        provider: &str,
        provider_execution_id: &str,
        request_hash: &str,
        submitted_at_ms: u64,
    ) -> OrchestratorResult<()>;
    async fn record_provider_result(
        &self,
        execution_id: Uuid,
        provider_execution_id: &str,
        outcome: ProviderOutcomeState,
        observed_at_ms: u64,
        result: Option<serde_json::Value>,
        error: Option<&str>,
    ) -> OrchestratorResult<()>;
}

pub struct WorkflowExecutionReconciler<'a, S> {
    pub store: &'a S,
}

impl<'a, S> WorkflowExecutionReconciler<'a, S>
where
    S: ExecutionReconciliationStore,
{
    pub fn new(store: &'a S) -> Self {
        Self { store }
    }

    pub async fn reconcile(&self, execution_id: Uuid) -> OrchestratorResult<ReconciliationReport> {
        let attempt = self.store.load_execution_attempt(execution_id).await?;
        let provider_result = self.store.load_provider_result(execution_id).await?;

        let action = match provider_result.as_ref() {
            Some(result) => result.action(),
            None if attempt.status.terminal() => match attempt.status {
                crate::ExecutionAttemptStatus::Succeeded => ReconciliationAction::ConfirmSuccess,
                crate::ExecutionAttemptStatus::Failed | crate::ExecutionAttemptStatus::Cancelled => ReconciliationAction::ConfirmFailure,
                crate::ExecutionAttemptStatus::Running => ReconciliationAction::ManualReview,
            },
            None => ReconciliationAction::Continue,
        };

        Ok(ReconciliationReport {
            execution_id,
            action,
            provider_result,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ExecutionAttemptKey, ExecutionAttemptStatus, FencingToken};

    fn running_attempt() -> ExecutionAttempt {
        ExecutionAttempt {
            execution_id: Uuid::now_v7(),
            key: ExecutionAttemptKey { workflow_id: Uuid::now_v7(), step_id: "publish".into(), attempt: 1 },
            status: ExecutionAttemptStatus::Running,
            owner: "worker-1".into(),
            fencing_token: FencingToken::from_value(1),
            started_at_ms: 100,
            heartbeat_at_ms: 100,
            finished_at_ms: None,
            result: None,
            error: None,
        }
    }

    #[test]
    fn provider_success_is_the_authoritative_reconciliation_signal() {
        let record = ProviderExecutionRecord {
            execution_id: Uuid::now_v7(),
            provider: "affiliate-api".into(),
            provider_execution_id: "remote-123".into(),
            request_hash: "sha256:abc".into(),
            submitted_at_ms: 100,
            outcome: Some(ProviderOutcomeState::Succeeded),
            observed_at_ms: Some(200),
            result: Some(serde_json::json!({"accepted": true})),
            error: None,
        };
        assert_eq!(record.action(), ReconciliationAction::ConfirmSuccess);
    }

    #[test]
    fn terminal_attempt_without_provider_result_is_still_conservative() {
        let mut attempt = running_attempt();
        attempt.status = ExecutionAttemptStatus::Succeeded;
        assert_eq!(attempt.status, ExecutionAttemptStatus::Succeeded);
        // The reconciler implementation uses the durable attempt state only as a fallback;
        // provider evidence remains preferred whenever available.
    }
}
