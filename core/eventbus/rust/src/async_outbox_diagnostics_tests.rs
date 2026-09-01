#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        AsyncOutboxDiagnostics, AsyncOutboxStore, AsyncInMemoryOutbox, DeliveryState,
        EventBusMetrics, EventEnvelope, EventKind, MetricsAsyncOutbox, RetryPolicy,
    };

    fn event() -> EventEnvelope {
        EventEnvelope {
            event_id: uuid::Uuid::now_v7(),
            event_type: "cat.event.outbox.diagnostics.v1".to_owned(),
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
    async fn diagnostics_tracks_retry_and_dead_letter_history() {
        let metrics = Arc::new(EventBusMetrics::default());
        let outbox = MetricsAsyncOutbox::new(AsyncInMemoryOutbox::new(), metrics);
        let event = event();
        let id = event.event_id;

        outbox.enqueue(event).await.unwrap();
        outbox.claim_next().await.unwrap();
        let state = outbox
            .fail(id, 1, &RetryPolicy::default(), "temporary")
            .await
            .unwrap();

        assert_eq!(state, DeliveryState::RetryScheduled);
        let retry = outbox.diagnostics(id).await.unwrap();
        assert_eq!(retry.state, Some(DeliveryState::RetryScheduled));
        assert_eq!(retry.metrics.published, 1);
        assert_eq!(retry.metrics.retried, 1);

        outbox.claim_next().await.unwrap();
        let exhausted = RetryPolicy::new(
            2,
            std::time::Duration::from_millis(1),
            std::time::Duration::from_millis(1),
        );
        let state = outbox.fail(id, 2, &exhausted, "poison").await.unwrap();

        assert_eq!(state, DeliveryState::DeadLettered);
        let dead = outbox.diagnostics(id).await.unwrap();
        assert_eq!(dead.state, Some(DeliveryState::DeadLettered));
        assert_eq!(dead.metrics.dead_lettered, 1);
    }
}
