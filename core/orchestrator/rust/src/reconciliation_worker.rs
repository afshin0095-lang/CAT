use crate::{
    ExecutionReconciliationStore, OrchestratorResult, ProviderExecutionAdapter,
    ProviderOutcomeState, ReconciliationAction, ReconciliationReport, WorkflowExecutionReconciler,
};
use uuid::Uuid;

pub struct ReconciliationWorker<'a, S, A> {
    pub store: &'a S,
    pub adapter: &'a A,
}

impl<'a, S, A> ReconciliationWorker<'a, S, A>
where
    S: ExecutionReconciliationStore,
    A: ProviderExecutionAdapter,
{
    pub fn new(store: &'a S, adapter: &'a A) -> Self {
        Self { store, adapter }
    }

    pub async fn reconcile_once(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<ReconciliationReport> {
        let reconciler = WorkflowExecutionReconciler::new(self.store);
        let initial = reconciler.reconcile(execution_id).await?;

        let Some(existing) = initial.provider_result else {
            return Ok(initial);
        };

        if existing.provider != self.adapter.provider_name() || existing.outcome.is_some() {
            return Ok(ReconciliationReport {
                execution_id,
                action: existing.action(),
                provider_result: Some(existing),
            });
        }

        let observed = self.adapter.lookup(&existing.provider_execution_id).await?;
        if let Some(remote) = observed
            && let (Some(outcome), Some(observed_at_ms)) = (remote.outcome, remote.observed_at_ms)
        {
                self.store
                    .record_provider_result(
                        execution_id,
                        &existing.provider_execution_id,
                        outcome,
                        observed_at_ms,
                        remote.result.clone(),
                        remote.error.as_deref(),
                    )
                    .await?;
            }
        }

        reconciler.reconcile(execution_id).await
    }

    pub fn action_from_outcome(outcome: ProviderOutcomeState) -> ReconciliationAction {
        match outcome {
            ProviderOutcomeState::Succeeded => ReconciliationAction::ConfirmSuccess,
            ProviderOutcomeState::Failed => ReconciliationAction::ConfirmFailure,
            ProviderOutcomeState::Unknown => ReconciliationAction::ManualReview,
        }
    }
}
