#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        AsyncInMemoryInbox, AsyncInboxDiagnostics, AsyncInboxStore, DeliveryState, EventBusMetrics,
        MetricsAsyncInbox,
    };

    #[tokio::test]
    async fn diagnostics_report_retry_scheduled_state_and_counter_history() {
        let metrics = Arc::new(EventBusMetrics::default());
        let inbox = MetricsAsyncInbox::new(AsyncInMemoryInbox::new(), Arc::clone(&metrics));
        let event_id = uuid::Uuid::now_v7();

        assert!(inbox.accept(event_id).await.unwrap());
        inbox.mark_failed(event_id).await.unwrap();

        let snapshot = inbox.diagnostics(event_id).await.unwrap();
        assert_eq!(snapshot.state, Some(DeliveryState::RetryScheduled));
        assert_eq!(snapshot.metrics.delivered, 1);
        assert_eq!(snapshot.metrics.retried, 1);
        assert_eq!(snapshot.metrics.acknowledged, 0);

        assert!(inbox.accept(event_id).await.unwrap());
        let reclaimed = inbox.diagnostics(event_id).await.unwrap();
        assert_eq!(reclaimed.state, Some(DeliveryState::InFlight));
        assert_eq!(reclaimed.metrics.delivered, 2);
    }
}
