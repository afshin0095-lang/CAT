use std::sync::Arc;

use crate::{DeliveryState, EventBusMetrics, EventBusMetricsSnapshot, EventBusResult, InboxStore};

/// Read-only diagnostics snapshot for a synchronous inbox.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MetricsInboxSnapshot {
    pub state: Option<DeliveryState>,
    pub metrics: EventBusMetricsSnapshot,
}

/// Read-only diagnostics boundary for synchronous inbox implementations.
///
/// Diagnostics are derived observability data. Implementations must not mutate
/// canonical inbox delivery state while answering an inspection request.
pub trait InboxDiagnostics: Send + Sync {
    fn diagnostics(&self, event_id: uuid::Uuid) -> EventBusResult<MetricsInboxSnapshot>;
}

/// Metrics-aware decorator for synchronous inbox implementations.
///
/// The wrapper preserves the inner inbox state machine and records only
/// observability counters. It does not become an additional source of truth.
pub struct MetricsInbox<I> {
    inner: I,
    metrics: Arc<EventBusMetrics>,
}

impl<I> MetricsInbox<I> {
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

impl<I> InboxStore for MetricsInbox<I>
where
    I: InboxStore,
{
    fn accept(&mut self, event_id: uuid::Uuid) -> EventBusResult<bool> {
        let accepted = self.inner.accept(event_id)?;
        if accepted {
            self.metrics.record_delivered();
        } else {
            self.metrics.record_rejected();
        }
        Ok(accepted)
    }

    fn mark_succeeded(&mut self, event_id: uuid::Uuid) -> EventBusResult<()> {
        self.inner.mark_succeeded(event_id)?;
        self.metrics.record_acknowledged();
        Ok(())
    }

    fn mark_failed(&mut self, event_id: uuid::Uuid) -> EventBusResult<()> {
        self.inner.mark_failed(event_id)?;
        self.metrics.record_retried();
        Ok(())
    }

    fn state(&self, event_id: uuid::Uuid) -> Option<DeliveryState> {
        self.inner.state(event_id)
    }
}

impl<I> InboxDiagnostics for MetricsInbox<I>
where
    I: InboxStore,
{
    fn diagnostics(&self, event_id: uuid::Uuid) -> EventBusResult<MetricsInboxSnapshot> {
        Ok(MetricsInboxSnapshot {
            state: self.inner.state(event_id),
            metrics: self.metrics.snapshot(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryInbox;

    #[test]
    fn diagnostics_combines_delivery_state_and_metrics_without_mutation() {
        let metrics = Arc::new(EventBusMetrics::default());
        let mut inbox = MetricsInbox::new(InMemoryInbox::default(), Arc::clone(&metrics));
        let event_id = uuid::Uuid::now_v7();

        assert!(inbox.accept(event_id).unwrap());

        let snapshot = inbox.diagnostics(event_id).unwrap();
        assert_eq!(snapshot.state, Some(DeliveryState::InFlight));
        assert_eq!(snapshot.metrics.delivered, 1);
        assert_eq!(snapshot.metrics.acknowledged, 0);
        assert_eq!(inbox.state(event_id), Some(DeliveryState::InFlight));
    }

    #[test]
    fn duplicate_and_retry_history_are_reflected_in_metrics() {
        let metrics = Arc::new(EventBusMetrics::default());
        let mut inbox = MetricsInbox::new(InMemoryInbox::default(), metrics);
        let event_id = uuid::Uuid::now_v7();

        assert!(inbox.accept(event_id).unwrap());
        assert!(!inbox.accept(event_id).unwrap());
        inbox.mark_failed(event_id).unwrap();
        assert!(inbox.accept(event_id).unwrap());

        let snapshot = inbox.diagnostics(event_id).unwrap();
        assert_eq!(snapshot.state, Some(DeliveryState::InFlight));
        assert_eq!(snapshot.metrics.delivered, 2);
        assert_eq!(snapshot.metrics.rejected, 1);
        assert_eq!(snapshot.metrics.retried, 1);
    }
}
