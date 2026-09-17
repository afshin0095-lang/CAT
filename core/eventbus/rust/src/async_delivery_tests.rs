#[cfg(test)]
mod tests {
    use crate::{
        AsyncDeliveryOutcome, AsyncInMemoryOutbox, AsyncOutboxDispatcher, AsyncOutboxStore,
        DeliveryState, EventBusError, EventBusResult, EventEnvelope, EventKind,
        RecordingAsyncTransport, RetryPolicy,
    };
    use async_trait::async_trait;

    fn event() -> EventEnvelope {
        EventEnvelope {
            event_id: uuid::Uuid::now_v7(),
            event_type: "cat.event.contract.v1".to_owned(),
            version: 1,
            kind: EventKind::Domain,
            occurred_at_ms: 1,
            producer: "contract-test".to_owned(),
            correlation_id: None,
            causation_id: None,
            subject_id: None,
            payload: serde_json::json!({}),
        }
    }

    #[derive(Clone, Debug)]
    struct RejectingTransport;

    #[async_trait]
    impl crate::AsyncEventTransport for RejectingTransport {
        async fn publish(&self, _event: &EventEnvelope) -> EventBusResult<()> {
            Err(EventBusError::TransportUnavailable(
                "simulated outage".to_owned(),
            ))
        }
    }

    #[tokio::test]
    async fn transient_transport_failure_returns_work_to_the_outbox() {
        let outbox = AsyncInMemoryOutbox::new();
        let event = event();
        let id = event.event_id;
        outbox.enqueue(event).await.unwrap();

        let dispatcher = AsyncOutboxDispatcher {
            outbox: outbox.clone(),
            transport: RejectingTransport,
            retry_policy: RetryPolicy::default(),
        };

        assert!(matches!(
            dispatcher.dispatch_one(0).await.unwrap(),
            AsyncDeliveryOutcome::RetryScheduled { attempt: 1, .. }
        ));
        assert_eq!(outbox.state(id), Some(DeliveryState::RetryScheduled));
        assert_eq!(outbox.pending_len(), 1);
    }

    #[tokio::test]
    async fn exhausted_transport_failure_dead_letters_the_event() {
        let outbox = AsyncInMemoryOutbox::new();
        let event = event();
        let id = event.event_id;
        outbox.enqueue(event).await.unwrap();

        let dispatcher = AsyncOutboxDispatcher {
            outbox: outbox.clone(),
            transport: RejectingTransport,
            retry_policy: RetryPolicy::new(
                1,
                std::time::Duration::from_millis(1),
                std::time::Duration::from_millis(1),
            ),
        };

        assert_eq!(
            dispatcher.dispatch_one(0).await.unwrap(),
            AsyncDeliveryOutcome::DeadLettered { attempt: 1 }
        );
        assert_eq!(outbox.state(id), Some(DeliveryState::DeadLettered));
        assert_eq!(outbox.dead_letters().len(), 1);
    }
}
