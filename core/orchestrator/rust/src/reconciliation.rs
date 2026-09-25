use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    AsyncPostgresExecutionStore, ExecutionAttempt, ExecutionAttemptStatus, ExecutionAuditEvidence,
    ExecutionAuthorizationRecord, ExecutionAttemptStore, OrchestratorResult,
    ProviderExecutionRecord, ProviderOutcomeState, ReconciliationAction,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationReport {
    pub execution_id: Uuid,
    pub action: ReconciliationAction,
    pub authorization: Option<ExecutionAuthorizationRecord>,
    pub provider_result: Option<ProviderExecutionRecord>,
}

impl ReconciliationReport {
    /// Build the derived audit view only when durable authorization evidence is present.
    ///
    /// Missing authorization evidence is intentionally not synthesized from workflow state.
    pub fn audit_evidence(
        &self,
        attempt: &ExecutionAttempt,
    ) -> Option<ExecutionAuditEvidence> {
        self.authorization.clone().and_then(|authorization| {
            ExecutionAuditEvidence::from_parts(
                attempt,
                authorization,
                self.provider_result.clone(),
            )
        })
    }

    pub async fn persist_audit<S: crate::ExecutionAuditStore>(
        &self,
        attempt: &ExecutionAttempt,
        store: &S,
        event_key: &str,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<crate::ExecutionAuditEvent> {
        let evidence = self.audit_evidence(attempt).ok_or_else(|| {
            OrchestratorError::Serialization(
                "audit evidence is unavailable because durable authorization evidence is missing"
                    .into(),
            )
        })?;

        store
            .append_audit_event(event_key, &evidence, self.action, recorded_at_ms)
            .await
    }
}

#[async_trait]
pub trait ExecutionReconciliationStore:
    AsyncPostgresExecutionStore + ExecutionAttemptStore
{
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
        let authorization = self
            .store
            .load_execution_authorization(execution_id)
            .await?;
        let provider_result = self.store.load_provider_result(execution_id).await?;

        let action = classify_reconciliation(
            attempt.status,
            authorization.is_some(),
            provider_result.as_ref(),
        );

        Ok(ReconciliationReport {
            execution_id,
            action,
            authorization,
            provider_result,
        })
    }
}

fn classify_reconciliation(
    status: ExecutionAttemptStatus,
    authorization_present: bool,
    provider_result: Option<&ProviderExecutionRecord>,
) -> ReconciliationAction {
    if !authorization_present {
        return ReconciliationAction::ManualReview;
    }

    match provider_result {
        Some(result) => result.action(),
        None if status.terminal() => match status {
            ExecutionAttemptStatus::Succeeded => ReconciliationAction::ConfirmSuccess,
            ExecutionAttemptStatus::Failed | ExecutionAttemptStatus::Cancelled => {
                ReconciliationAction::ConfirmFailure
            }
            ExecutionAttemptStatus::Running => ReconciliationAction::ManualReview,
        },
        None => ReconciliationAction::Continue,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProviderExecutionRecord;

    #[test]
    fn provider_success_cannot_substitute_missing_authorization() {
        let record = ProviderExecutionRecord {
            execution_id: Uuid::now_v7(),
            provider: "affiliate-api".into(),
            provider_execution_id: "remote-789".into(),
            request_hash: "sha256:ghi".into(),
            submitted_at_ms: 100,
            outcome: Some(ProviderOutcomeState::Succeeded),
            observed_at_ms: Some(400),
            result: Some(serde_json::json!({"accepted": true})),
            error: None,
        };

        assert_eq!(
            classify_reconciliation(
                ExecutionAttemptStatus::Running,
                false,
                Some(&record),
            ),
            ReconciliationAction::ManualReview
        );
    }

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
