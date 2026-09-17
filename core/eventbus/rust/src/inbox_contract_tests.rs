#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        AsyncInMemoryInbox, AsyncInboxStore, DeliveryState, EventBusMetrics, InMemoryInbox,
        InboxDiagnostics, InboxStore, MetricsAsyncInbox, MetricsInbox,
    };

    #[tokio::test]
    async fn sync_and_async_metrics_inboxes_preserve_equivalent_retry_semantics() {
        let event_id = uuid::Uuid::now_v7();

        let mut sync = MetricsInbox::new(
            InMemoryInbox::default(),
            Arc::new(EventBusMetrics::default()),
        );
        let async_inbox = MetricsAsyncInbox::new(
            AsyncInMemoryInbox::new(),
            Arc::new(EventBusMetrics::default()),
        );

        assert!(sync.accept(event_id).unwrap());
        assert!(async_inbox.accept(event_id).await.unwrap());

        sync.mark_failed(event_id).unwrap();
        async_inbox.mark_failed(event_id).await.unwrap();

        assert_eq!(
            sync.diagnostics(event_id).unwrap().state,
            Some(DeliveryState::RetryScheduled)
        );
        assert_eq!(
            async_inbox.diagnostics(event_id).await.unwrap().state,
            Some(DeliveryState::RetryScheduled)
        );

        assert!(sync.accept(event_id).unwrap());
        assert!(async_inbox.accept(event_id).await.unwrap());

        sync.mark_succeeded(event_id).unwrap();
        async_inbox.mark_succeeded(event_id).await.unwrap();

        assert_eq!(
            sync.diagnostics(event_id).unwrap().state,
            Some(DeliveryState::Succeeded)
        );
        assert_eq!(
            async_inbox.diagnostics(event_id).await.unwrap().state,
            Some(DeliveryState::Succeeded)
        );

        let sync_metrics = sync.diagnostics(event_id).unwrap().metrics;
        let async_metrics = async_inbox.diagnostics(event_id).await.unwrap().metrics;

        assert_eq!(sync_metrics.delivered, async_metrics.delivered);
        assert_eq!(sync_metrics.retried, async_metrics.retried);
        assert_eq!(sync_metrics.acknowledged, async_metrics.acknowledged);
    }
}
