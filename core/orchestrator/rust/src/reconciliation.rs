use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    AsyncPostgresExecutionStore, ExecutionAttempt, ExecutionAttemptStatus, OrchestratorResult,
    ProviderExecutionRecord, ProviderOutcomeState, ReconciliationAction,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationReport {
    pub execution_id: Uuid,
    pub action: ReconciliationAction,
    pub provider_result: Option<ProviderExecutionRecord>,
}

#[async_trait]
pub trait ExecutionReconciliationStore: AsyncPostgresExecutionStore {
    async fn load_execution_attempt(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<ExecutionAttempt>;
    async fn load_provider_result(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<Option<ProviderExecutionRecord>>;
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
                ExecutionAttemptStatus::Succeeded => ReconciliationAction::ConfirmSuccess,
                ExecutionAttemptStatus::Failed | ExecutionAttemptStatus::Cancelled => {
                    ReconciliationAction::ConfirmFailure
                }
                ExecutionAttemptStatus::Running => ReconciliationAction::ManualReview,
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
    use crate::ProviderExecutionRecord;

    #[test]
    fn provider_success_is_authoritative() {
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
    fn unknown_provider_outcome_requires_review() {
        let record = ProviderExecutionRecord {
            execution_id: Uuid::now_v7(),
            provider: "affiliate-api".into(),
            provider_execution_id: "remote-456".into(),
            request_hash: "sha256:def".into(),
            submitted_at_ms: 100,
            outcome: Some(ProviderOutcomeState::Unknown),
            observed_at_ms: Some(300),
            result: None,
            error: Some("provider unavailable".into()),
        };
        assert_eq!(record.action(), ReconciliationAction::ManualReview);
    }
}
