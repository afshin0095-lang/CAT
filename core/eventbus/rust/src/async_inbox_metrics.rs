use crate::{AsyncInboxStore, DeliveryState, EventBusMetrics, EventBusMetricsSnapshot, EventBusResult};
use std::sync::Arc;

/// Snapshot of one metrics-aware asynchronous inbox.
///
/// The snapshot deliberately exposes both delivery-state and observability
/// counters without allowing callers to mutate the underlying inbox.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MetricsAsyncInboxSnapshot {
    pub state: Option<DeliveryState>,
    pub metrics: EventBusMetricsSnapshot,
}

/// Read-only diagnostics facade for an asynchronous inbox.
///
/// CAT workers can use this trait to inspect delivery progress at a stable
/// boundary while the concrete store remains responsible for canonical state.
#[async_trait::async_trait]
pub trait AsyncInboxDiagnostics: Send + Sync {
    async fn diagnostics(
        &self,
        event_id: uuid::Uuid,
    ) -> EventBusResult<MetricsAsyncInboxSnapshot>;
}

/// Metrics-aware inbox decorator with a read-only diagnostics surface.
///
/// The wrapper preserves the inner inbox state machine and keeps metrics as
/// derived observability data. No diagnostic operation mutates delivery state.
#[derive(Clone)]
pub struct MetricsAsyncInbox<I> {
    inner: I,
    metrics: Arc<EventBusMetrics>,
}

impl<I> MetricsAsyncInbox<I> {
    pub fn new(inner: I, metrics: Arc<EventBusMetrics>) -> Self {
        Self { inner, metrics }
    }

    pub fn metrics(&self) -> Arc<EventBusMetrics> {
        Arc::clone(&self.metrics)
    }

    pub fn into_inner(self) -> I {
        self.inner
    }
}

#[async_trait::async_trait]
impl<I> AsyncInboxStore for MetricsAsyncInbox<I>
where
    I: AsyncInboxStore,
{
    async fn accept(&self, event_id: uuid::Uuid) -> EventBusResult<bool> {
        let accepted = self.inner.accept(event_id).await?;
        if accepted {
            self.metrics.record_delivered();
        } else {
            self.metrics.record_rejected();
        }
        Ok(accepted)
    }

    async fn mark_succeeded(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        self.inner.mark_succeeded(event_id).await?;
        self.metrics.record_acknowledged();
        Ok(())
    }

    async fn mark_failed(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        self.inner.mark_failed(event_id).await?;
        self.metrics.record_retried();
        Ok(())
    }

    async fn state(&self, event_id: uuid::Uuid) -> EventBusResult<Option<DeliveryState>> {
        self.inner.state(event_id).await
    }
}

#[async_trait::async_trait]
impl<I> AsyncInboxDiagnostics for MetricsAsyncInbox<I>
where
    I: AsyncInboxStore,
{
    async fn diagnostics(
        &self,
        event_id: uuid::Uuid,
    ) -> EventBusResult<MetricsAsyncInboxSnapshot> {
        Ok(MetricsAsyncInboxSnapshot {
            state: self.inner.state(event_id).await?,
            metrics: self.metrics.snapshot(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsyncInMemoryInbox;

    #[tokio::test]
    async fn diagnostics_combines_state_and_metrics_without_mutation() {
        let metrics = Arc::new(EventBusMetrics::default());
        let inbox = MetricsAsyncInbox::new(AsyncInMemoryInbox::new(), metrics);
        let event_id = uuid::Uuid::now_v7();

        assert!(inbox.accept(event_id).await.unwrap());

        let snapshot = inbox.diagnostics(event_id).await.unwrap();
        assert_eq!(snapshot.state, Some(DeliveryState::InFlight));
        assert_eq!(snapshot.metrics.delivered, 1);
        assert_eq!(snapshot.metrics.acknowledged, 0);

        assert_eq!(
            inbox.state(event_id).await.unwrap(),
            Some(DeliveryState::InFlight)
        );
    }
}
