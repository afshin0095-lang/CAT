use crate::{AsyncOutboxStore, DeliveryState, EventBusError, EventBusResult, EventEnvelope, RetryPolicy};
use async_trait::async_trait;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// Async in-memory outbox that mirrors the durable outbox lifecycle.
///
/// It is intentionally deterministic and suitable for contract/integration tests:
/// pending -> in-flight -> succeeded, retry-scheduled -> pending, or dead-lettered.
#[derive(Clone, Debug, Default)]
pub struct AsyncInMemoryOutbox {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug, Default)]
struct Inner {
    pending: VecDeque<EventEnvelope>,
    in_flight: HashMap<uuid::Uuid, EventEnvelope>,
    attempts: HashMap<uuid::Uuid, u32>,
    states: HashMap<uuid::Uuid, DeliveryState>,
    dead_letters: Vec<EventEnvelope>,
}

impl AsyncInMemoryOutbox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pending_len(&self) -> usize {
        self.inner.lock().expect("async outbox mutex poisoned").pending.len()
    }

    pub fn in_flight_len(&self) -> usize {
        self.inner.lock().expect("async outbox mutex poisoned").in_flight.len()
    }

    pub fn state(&self, event_id: uuid::Uuid) -> Option<DeliveryState> {
        self.inner.lock().expect("async outbox mutex poisoned").states.get(&event_id).copied()
    }

    pub fn dead_letters(&self) -> Vec<EventEnvelope> {
        self.inner.lock().expect("async outbox mutex poisoned").dead_letters.clone()
    }
}

#[async_trait]
impl AsyncOutboxStore for AsyncInMemoryOutbox {
    async fn enqueue(&self, event: EventEnvelope) -> EventBusResult<()> {
        let mut inner = self.inner.lock().expect("async outbox mutex poisoned");
        if inner.states.contains_key(&event.event_id) {
            return Err(EventBusError::Storage(format!(
                "event {} already exists in async outbox",
                event.event_id
            )));
        }

        inner.states.insert(event.event_id, DeliveryState::Pending);
        inner.pending.push_back(event);
        Ok(())
    }

    async fn claim_next(&self) -> EventBusResult<Option<EventEnvelope>> {
        let mut inner = self.inner.lock().expect("async outbox mutex poisoned");
        let Some(event) = inner.pending.pop_front() else {
            return Ok(None);
        };

        inner.states.insert(event.event_id, DeliveryState::InFlight);
        inner.in_flight.insert(event.event_id, event.clone());
        Ok(Some(event))
    }

    async fn acknowledge(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        let mut inner = self.inner.lock().expect("async outbox mutex poisoned");
        if inner.in_flight.remove(&event_id).is_none() {
            return Err(EventBusError::UnknownEvent(event_id));
        }

        inner.attempts.remove(&event_id);
        inner.states.insert(event_id, DeliveryState::Succeeded);
        Ok(())
    }

    async fn fail(
        &self,
        event_id: uuid::Uuid,
        attempt: u32,
        policy: &RetryPolicy,
        _error: &str,
    ) -> EventBusResult<DeliveryState> {
        let mut inner = self.inner.lock().expect("async outbox mutex poisoned");
        let event = inner
            .in_flight
            .remove(&event_id)
            .ok_or(EventBusError::UnknownEvent(event_id))?;

        inner.attempts.insert(event_id, attempt);
        if policy.exhausted(attempt) {
            inner.states.insert(event_id, DeliveryState::DeadLettered);
            inner.dead_letters.push(event);
            return Ok(DeliveryState::DeadLettered);
        }

        inner.states.insert(event_id, DeliveryState::RetryScheduled);
        inner.pending.push_back(event);
        Ok(DeliveryState::RetryScheduled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EventKind;

    fn event() -> EventEnvelope {
        EventEnvelope {
            event_id: uuid::Uuid::now_v7(),
            event_type: "cat.event.test.v1".to_owned(),
            version: 1,
            kind: EventKind::Domain,
            occurred_at_ms: 1,
            producer: "eventbus-test".to_owned(),
            correlation_id: None,
            causation_id: None,
            subject_id: None,
            payload: serde_json::json!({"ok": true}),
        }
    }

    #[tokio::test]
    async fn retry_requeues_the_claimed_event() {
        let outbox = AsyncInMemoryOutbox::new();
        let event = event();
        let id = event.event_id;

        outbox.enqueue(event).await.unwrap();
        assert_eq!(outbox.claim_next().await.unwrap().unwrap().event_id, id);

        assert_eq!(
            outbox.fail(id, 1, &RetryPolicy::default(), "temporary").await.unwrap(),
            DeliveryState::RetryScheduled
        );
        assert_eq!(outbox.state(id), Some(DeliveryState::RetryScheduled));
        assert_eq!(outbox.pending_len(), 1);
        assert_eq!(outbox.claim_next().await.unwrap().unwrap().event_id, id);
    }

    #[tokio::test]
    async fn exhausted_event_is_parked() {
        let outbox = AsyncInMemoryOutbox::new();
        let event = event();
        let id = event.event_id;
        let policy = RetryPolicy::new(
            1,
            std::time::Duration::from_millis(1),
            std::time::Duration::from_millis(1),
        );

        outbox.enqueue(event).await.unwrap();
        outbox.claim_next().await.unwrap();

        assert_eq!(
            outbox.fail(id, 1, &policy, "poison").await.unwrap(),
            DeliveryState::DeadLettered
        );
        assert_eq!(outbox.state(id), Some(DeliveryState::DeadLettered));
        assert_eq!(outbox.dead_letters().len(), 1);
    }
}
