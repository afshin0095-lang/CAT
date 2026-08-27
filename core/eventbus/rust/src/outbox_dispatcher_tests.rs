#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Mutex;
    use std::time::Duration;

    use async_trait::async_trait;
    use cat_eventbus::{
        AsyncOutboxStore, DeliveryState, EventBusError, EventBusResult, EventEnvelope,
        EventKind, EventTransport, OutboxDispatcher, RetryPolicy,
    };
    use serde_json::json;
    use uuid::Uuid;

    #[derive(Default)]
    struct RecordingOutbox {
        events: Mutex<VecDeque<EventEnvelope>>,
        acknowledged: Mutex<Vec<Uuid>>,
        failures: Mutex<Vec<(Uuid, u32, String)>>,
    }

    #[async_trait]
    impl AsyncOutboxStore for RecordingOutbox {
        async fn enqueue(&self, event: EventEnvelope) -> EventBusResult<()> {
            self.events.lock().unwrap().push_back(event);
            Ok(())
        }

        async fn claim_next(&self) -> EventBusResult<Option<EventEnvelope>> {
            Ok(self.events.lock().unwrap().pop_front())
        }

        async fn acknowledge(&self, event_id: Uuid) -> EventBusResult<()> {
            self.acknowledged.lock().unwrap().push(event_id);
            Ok(())
        }

        async fn fail(
            &self,
            event_id: Uuid,
            attempt: u32,
            policy: &RetryPolicy,
            error: &str,
        ) -> EventBusResult<DeliveryState> {
            self.failures
                .lock()
                .unwrap()
                .push((event_id, attempt, error.to_owned()));
            Ok(if policy.exhausted(attempt) {
                DeliveryState::DeadLettered
            } else {
                DeliveryState::RetryScheduled
            })
        }
    }

    #[derive(Default)]
    struct RecordingTransport {
        fail: bool,
        published: Vec<Uuid>,
    }

    impl EventTransport for RecordingTransport {
        fn publish(&mut self, event: &EventEnvelope) -> EventBusResult<()> {
            if self.fail {
                return Err(EventBusError::Transport("injected failure".into()));
            }
            self.published.push(event.event_id);
            Ok(())
        }
    }

    fn event() -> EventEnvelope {
        EventEnvelope {
            event_id: Uuid::now_v7(),
            event_type: "test.outbox".into(),
            version: 1,
            kind: EventKind::Integration,
            occurred_at_ms: 1,
            producer: "test".into(),
            correlation_id: None,
            causation_id: None,
            subject_id: None,
            payload: json!({"ok": true}),
        }
    }

    #[tokio::test]
    async fn dispatcher_delivers_and_acknowledges() {
        let outbox = RecordingOutbox::default();
        let event = event();
        let id = event.event_id;
        outbox.enqueue(event).await.unwrap();

        let mut transport = RecordingTransport::default();
        let policy = RetryPolicy::new(3, Duration::from_millis(1), Duration::from_millis(5));
        let mut dispatcher = OutboxDispatcher::new(&outbox, &mut transport, policy);

        assert!(matches!(
            dispatcher.dispatch_once(1).await.unwrap(),
            cat_eventbus::DispatchOutcome::Delivered { event_id } if event_id == id
        ));
        assert_eq!(transport.published, vec![id]);
        assert_eq!(*outbox.acknowledged.lock().unwrap(), vec![id]);
    }

    #[tokio::test]
    async fn dispatcher_records_retry_and_dead_letter() {
        let outbox = RecordingOutbox::default();
        let first = event();
        let first_id = first.event_id;
        outbox.enqueue(first).await.unwrap();

        let mut transport = RecordingTransport { fail: true, published: Vec::new() };
        let policy = RetryPolicy::new(2, Duration::from_millis(1), Duration::from_millis(5));
        let mut dispatcher = OutboxDispatcher::new(&outbox, &mut transport, policy);

        assert!(matches!(
            dispatcher.dispatch_once(1).await.unwrap(),
            cat_eventbus::DispatchOutcome::RetryScheduled { event_id, attempt: 1, .. } if event_id == first_id
        ));

        let second = event();
        let second_id = second.event_id;
        outbox.enqueue(second).await.unwrap();

        assert!(matches!(
            dispatcher.dispatch_once(2).await.unwrap(),
            cat_eventbus::DispatchOutcome::DeadLettered { event_id, attempt: 2, .. } if event_id == second_id
        ));
        assert_eq!(outbox.failures.lock().unwrap().len(), 2);
    }
}
