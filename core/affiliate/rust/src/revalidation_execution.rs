use async_trait::async_trait;
use cat_eventbus::EventBus;
use cat_orchestrator::ExecutionIntent;
use uuid::Uuid;

use crate::{
    AsyncRevalidationRequestStore, OpportunityRevalidated, OpportunityRevalidationFailed,
    PostgresOpportunityStoreError, RevalidationRequestRecord, RevalidationStatus,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RevalidationExecutionResult {
    pub observed_at_ms: u64,
    pub revision: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum RevalidationExecutionError {
    #[error("revalidation persistence error: {0}")]
    Persistence(#[from] PostgresOpportunityStoreError),
    #[error("revalidation executor failed: {0}")]
    Executor(String),
    #[error("event publication failed: {0}")]
    EventPublication(String),
}

#[async_trait]
pub trait RevalidationExecutor: Send + Sync {
    async fn execute(
        &self,
        request: &RevalidationRequestRecord,
        execution: &ExecutionIntent,
    ) -> Result<RevalidationExecutionResult, String>;
}

pub struct RevalidationExecutionCoordinator<S, E> {
    store: S,
    executor: E,
    event_bus: EventBus,
}

impl<S, E> RevalidationExecutionCoordinator<S, E>
where
    S: AsyncRevalidationRequestStore,
    E: RevalidationExecutor,
{
    pub fn new(store: S, executor: E, event_bus: EventBus) -> Self {
        Self { store, executor, event_bus }
    }

    pub fn store(&self) -> &S {
        &self.store
    }

    pub async fn run_due(
        &self,
        now_ms: u64,
        limit: u32,
    ) -> Result<Vec<RevalidationExecutionReport>, RevalidationExecutionError> {
        let claimed = self.store.claim_due(now_ms, limit).await?;
        let mut reports = Vec::with_capacity(claimed.len());

        for request in claimed {
            reports.push(self.execute_claimed(request, now_ms).await?);
        }

        Ok(reports)
    }

    async fn execute_claimed(
        &self,
        request: RevalidationRequestRecord,
        now_ms: u64,
    ) -> Result<RevalidationExecutionReport, RevalidationExecutionError> {
        let running = self
            .store
            .transition(request.request.request_id, RevalidationStatus::Running, now_ms, None)
            .await?;

        let execution = execution_request_for(&running, now_ms);

        match self.executor.execute(&running, &execution).await {
            Ok(result) => {
                let completed = self
                    .store
                    .transition(
                        running.request.request_id,
                        RevalidationStatus::Succeeded,
                        result.observed_at_ms,
                        None,
                    )
                    .await?;

                let event = OpportunityRevalidated {
                    request_id: completed.request.request_id,
                    opportunity_id: completed.request.opportunity_id,
                    identity: completed.request.target.identity.clone(),
                    source: completed.request.target.source.clone(),
                    observed_at_ms: result.observed_at_ms,
                    revision: result.revision,
                };

                publish_event(&self.event_bus, event)?;
                Ok(RevalidationExecutionReport::Succeeded {
                    request_id: completed.request.request_id,
                    attempt: completed.attempt,
                    revision: result.revision,
                })
            }
            Err(error) => {
                let failed = self
                    .store
                    .transition(
                        running.request.request_id,
                        RevalidationStatus::Failed,
                        now_ms,
                        Some(error.clone()),
                    )
                    .await?;

                let event = OpportunityRevalidationFailed::new(
                    failed.request.request_id,
                    failed.request.opportunity_id,
                    failed.request.target.identity.clone(),
                    failed.request.target.source.clone(),
                    failed.request.reason.clone(),
                    now_ms,
                    error,
                );

                publish_event(&self.event_bus, event)?;
                Ok(RevalidationExecutionReport::Failed {
                    request_id: failed.request.request_id,
                    attempt: failed.attempt,
                })
            }
        }
    }
}

pub fn execution_request_for(
    request: &RevalidationRequestRecord,
    requested_at_ms: u64,
) -> ExecutionIntent {
    ExecutionIntent::new(
        request.request.request_id,
        format!("affiliate.revalidation:{}", request.request.target.source),
        request.attempt.max(1),
        requested_at_ms,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RevalidationExecutionReport {
    Succeeded { request_id: Uuid, attempt: u32, revision: u64 },
    Failed { request_id: Uuid, attempt: u32 },
}

fn publish_event<T: cat_eventbus::CatEvent>(
    event_bus: &EventBus,
    event: T,
) -> Result<(), RevalidationExecutionError> {
    let envelope = event
        .into_envelope("affiliate-revalidation")
        .map_err(|error| RevalidationExecutionError::EventPublication(error.to_string()))?;
    event_bus
        .publish(envelope)
        .map_err(|error| RevalidationExecutionError::EventPublication(error.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RevalidationPriority, RevalidationReason, RevalidationRequest, RevalidationTarget};

    fn record() -> RevalidationRequestRecord {
        RevalidationRequestRecord {
            request: RevalidationRequest::new(
                Uuid::nil(),
                RevalidationTarget::new("acme:widget", "network-a"),
                RevalidationReason::Stale,
                RevalidationPriority::High,
                1_000,
                2_000,
            ),
            status: RevalidationStatus::Running,
            attempt: 2,
            created_at_ms: 1_000,
            started_at_ms: Some(2_000),
            completed_at_ms: None,
            last_error: None,
        }
    }

    #[test]
    fn execution_intent_is_stable_and_source_scoped() {
        let request = record();
        let execution = execution_request_for(&request, 3_000);

        assert_eq!(execution.workflow_id, request.request.request_id);
        assert_eq!(execution.step_id, "affiliate.revalidation:network-a");
        assert_eq!(execution.attempt, 2);
        assert_eq!(execution.requested_at_ms, 3_000);
    }

    #[test]
    fn execution_intent_never_uses_zero_attempt() {
        let mut request = record();
        request.attempt = 0;
        assert_eq!(execution_request_for(&request, 3_000).attempt, 1);
    }
}
