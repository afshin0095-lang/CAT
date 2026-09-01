use crate::{
    AsyncOutboxStore, DeliveryState, EventBusMetrics, EventBusMetricsSnapshot, EventBusResult,
    EventEnvelope, RetryPolicy,
};
use std::sync::Arc;

/// Snapshot of one metrics-aware asynchronous outbox.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MetricsAsyncOutboxSnapshot {
    pub state: Option<DeliveryState>,
    pub metrics: EventBusMetricsSnapshot,
}

/// Read-only diagnostics facade for an asynchronous outbox.
#[async_trait::async_trait]
pub trait AsyncOutboxDiagnostics: Send + Sync {
    async fn diagnostics(
        &self,
        event_id: uuid::Uuid,
    ) -> EventBusResult<MetricsAsyncOutboxSnapshot>;
}

/// Metrics-aware asynchronous outbox decorator.
///
/// Canonical delivery state remains owned by the wrapped store. Metrics are derived
/// observability data and diagnostics never mutate outbox state.
#[derive(Clone)]
pub struct MetricsAsyncOutbox<O> {
    inner: O,
    metrics: Arc<EventBusMetrics>,
}

impl<O> MetricsAsyncOutbox<O> {
    pub fn new(inner: O, metrics: Arc<EventBusMetrics>) -> Self {
        Self { inner, metrics }
    }

    pub fn metrics(&self) -> Arc<EventBusMetrics> {
        Arc::clone(&self.metrics)
    }

    pub fn into_inner(self) -> O {
        self.inner
    }
}

#[async_trait::async_trait]
impl<O> AsyncOutboxStore for MetricsAsyncOutbox<O>
where
    O: AsyncOutboxStore,
{
    async fn enqueue(&self, event: EventEnvelope) -> EventBusResult<()> {
        match self.inner.enqueue(event).await {
            Ok(()) => {
                self.metrics.record_published();
                Ok(())
            }
            Err(error) => {
                self.metrics.record_rejected();
                Err(error)
            }
        }
    }

    async fn claim_next(&self) -> EventBusResult<Option<EventEnvelope>> {
        self.inner.claim_next().await
    }

    async fn acknowledge(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        self.inner.acknowledge(event_id).await?;
        self.metrics.record_acknowledged();
        Ok(())
    }

    async fn fail(
        &self,
        event_id: uuid::Uuid,
        attempt: u32,
        policy: &RetryPolicy,
        error: &str,
    ) -> EventBusResult<DeliveryState> {
        let state = self.inner.fail(event_id, attempt, policy, error).await?;
        match state {
            DeliveryState::DeadLettered => self.metrics.record_dead_lettered(),
            DeliveryState::RetryScheduled => self.metrics.record_retried(),
            _ => {}
        }
        Ok(state)
    }

    async fn state(&self, event_id: uuid::Uuid) -> EventBusResult<Option<DeliveryState>> {
        self.inner.state(event_id).await
    }
}

#[async_trait::async_trait]
impl<O> AsyncOutboxDiagnostics for MetricsAsyncOutbox<O>
where
    O: AsyncOutboxStore,
{
    async fn diagnostics(
        &self,
        event_id: uuid::Uuid,
    ) -> EventBusResult<MetricsAsyncOutboxSnapshot> {
        Ok(MetricsAsyncOutboxSnapshot {
            state: self.inner.state(event_id).await?,
            metrics: self.metrics.snapshot(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AsyncInMemoryOutbox, EventKind};

    fn event() -> EventEnvelope {
        EventEnvelope {
            event_id: uuid::Uuid::now_v7(),
            event_type: "cat.event.outbox.metrics.v1".to_owned(),
            version: 1,
            kind: EventKind::Integration,
            occurred_at_ms: 1,
            producer: "eventbus-test".to_owned(),
            correlation_id: None,
            causation_id: None,
            subject_id: None,
            payload: serde_json::json!({}),
        }
    }

    #[tokio::test]
    async fn diagnostics_reports_state_and_metrics_without_mutation() {
        let metrics = Arc::new(EventBusMetrics::default());
        let outbox = MetricsAsyncOutbox::new(AsyncInMemoryOutbox::new(), metrics);
        let event = event();
        let id = event.event_id;

        outbox.enqueue(event).await.unwrap();
        let snapshot = outbox.diagnostics(id).await.unwrap();

        assert_eq!(snapshot.state, Some(DeliveryState::Pending));
        assert_eq!(snapshot.metrics.published, 1);
        assert_eq!(snapshot.metrics.acknowledged, 0);
        assert_eq!(
            outbox.state(id).await.unwrap(),
            Some(DeliveryState::Pending)
        );
    }
}
