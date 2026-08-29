use crate::{AsyncInboxStore, DeliveryState, EventBusMetrics, EventBusResult};
use std::sync::Arc;

/// Metrics-aware consumer inbox adapter.
///
/// This wrapper keeps the inbox state machine unchanged while making retry and
/// successful processing observable at the same boundary where state changes
/// are committed. The inner inbox remains the source of delivery state truth.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AsyncInMemoryInbox, EventBusMetricsSnapshot};

    #[tokio::test]
    async fn records_delivery_ack_retry_and_duplicate_rejection() {
        let metrics = Arc::new(EventBusMetrics::default());
        let inbox = MetricsAsyncInbox::new(AsyncInMemoryInbox::new(), Arc::clone(&metrics));
        let event_id = uuid::Uuid::now_v7();

        assert!(inbox.accept(event_id).await.unwrap());
        inbox.mark_failed(event_id).await.unwrap();
        assert!(inbox.accept(event_id).await.unwrap());
        inbox.mark_succeeded(event_id).await.unwrap();
        assert!(!inbox.accept(event_id).await.unwrap());

        assert_eq!(
            metrics.snapshot(),
            EventBusMetricsSnapshot {
                published: 0,
                delivered: 2,
                acknowledged: 1,
                retried: 1,
                dead_lettered: 0,
                rejected: 1,
            }
        );
    }
}
