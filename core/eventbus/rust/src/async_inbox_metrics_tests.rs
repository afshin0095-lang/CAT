#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        AsyncInMemoryInbox, AsyncInboxStore, EventBusMetrics, EventBusMetricsSnapshot,
        MetricsAsyncInbox,
    };

    #[tokio::test]
    async fn wrapper_preserves_successful_delivery_state_machine() {
        let metrics = Arc::new(EventBusMetrics::default());
        let inbox = MetricsAsyncInbox::new(AsyncInMemoryInbox::new(), Arc::clone(&metrics));
        let event_id = uuid::Uuid::now_v7();

        assert!(inbox.accept(event_id).await.unwrap());
        inbox.mark_succeeded(event_id).await.unwrap();
        assert!(!inbox.accept(event_id).await.unwrap());

        assert_eq!(
            metrics.snapshot(),
            EventBusMetricsSnapshot {
                published: 0,
                delivered: 1,
                acknowledged: 1,
                retried: 0,
                dead_lettered: 0,
                rejected: 1,
            }
        );
    }

    #[tokio::test]
    async fn wrapper_makes_retry_reclaim_visible_without_breaking_reprocessing() {
        let metrics = Arc::new(EventBusMetrics::default());
        let inbox = MetricsAsyncInbox::new(AsyncInMemoryInbox::new(), Arc::clone(&metrics));
        let event_id = uuid::Uuid::now_v7();

        assert!(inbox.accept(event_id).await.unwrap());
        inbox.mark_failed(event_id).await.unwrap();
        assert!(inbox.accept(event_id).await.unwrap());

        assert_eq!(
            metrics.snapshot(),
            EventBusMetricsSnapshot {
                published: 0,
                delivered: 2,
                acknowledged: 0,
                retried: 1,
                dead_lettered: 0,
                rejected: 0,
            }
        );
    }
}
