use crate::{AsyncOutboxStore, DeliveryState, EventBusResult, EventEnvelope, RetryPolicy};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Bounded, deterministic outbox implementation for async worker tests.
///
/// Unlike the synchronous test double, this implementation never holds a
/// blocking mutex across an await boundary. It is useful for Tokio-based
/// integration tests and documents the exact producer-side lifecycle.
#[derive(Clone, Debug, Default)]
pub struct AsyncTokioInMemoryOutbox {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug, Default)]
struct Inner {
    pending: VecDeque<EventEnvelope>,
    in_flight: std::collections::HashMap<uuid::Uuid, EventEnvelope>,
    states: std::collections::HashMap<uuid::Uuid, DeliveryState>,
    dead_letters: Vec<EventEnvelope>,
}

impl AsyncTokioInMemoryOutbox {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn pending_len(&self) -> usize {
        self.inner.lock().await.pending.len()
    }

    pub async fn state_snapshot(&self, event_id: uuid::Uuid) -> Option<DeliveryState> {
        self.inner.lock().await.states.get(&event_id).copied()
    }

    pub async fn dead_letters(&self) -> Vec<EventEnvelope> {
        self.inner.lock().await.dead_letters.clone()
    }
}

#[async_trait::async_trait]
impl AsyncOutboxStore for AsyncTokioInMemoryOutbox {
    async fn enqueue(&self, event: EventEnvelope) -> EventBusResult<()> {
        let mut inner = self.inner.lock().await;
        if inner.states.contains_key(&event.event_id) {
            return Err(crate::EventBusError::Storage(format!(
                "event {} already exists in async Tokio outbox",
                event.event_id
            )));
        }

        inner.states.insert(event.event_id, DeliveryState::Pending);
        inner.pending.push_back(event);
        Ok(())
    }

    async fn claim_next(&self) -> EventBusResult<Option<EventEnvelope>> {
        let mut inner = self.inner.lock().await;
        let Some(event) = inner.pending.pop_front() else {
            return Ok(None);
        };

        inner.states.insert(event.event_id, DeliveryState::InFlight);
        inner.in_flight.insert(event.event_id, event.clone());
        Ok(Some(event))
    }

    async fn acknowledge(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        let mut inner = self.inner.lock().await;
        if inner.in_flight.remove(&event_id).is_none() {
            return Err(crate::EventBusError::UnknownEvent(event_id));
        }

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
        let mut inner = self.inner.lock().await;
        let event = inner
            .in_flight
            .remove(&event_id)
            .ok_or(crate::EventBusError::UnknownEvent(event_id))?;

        if policy.exhausted(attempt) {
            inner.states.insert(event_id, DeliveryState::DeadLettered);
            inner.dead_letters.push(event);
            return Ok(DeliveryState::DeadLettered);
        }

        inner.states.insert(event_id, DeliveryState::RetryScheduled);
        inner.pending.push_back(event);
        Ok(DeliveryState::RetryScheduled)
    }

    async fn state(&self, event_id: uuid::Uuid) -> EventBusResult<Option<DeliveryState>> {
        Ok(self.state_snapshot(event_id).await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AsyncOutboxDispatcher, EventKind, RecordingAsyncTransport};

    fn event() -> EventEnvelope {
        EventEnvelope {
            event_id: uuid::Uuid::now_v7(),
            event_type: "cat.event.tokio-outbox.v1".to_owned(),
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
    async fn dispatcher_completes_tokio_outbox_lifecycle() {
        let outbox = AsyncTokioInMemoryOutbox::new();
        let event = event();
        let id = event.event_id;
        outbox.enqueue(event).await.unwrap();

        let dispatcher = AsyncOutboxDispatcher {
            outbox: outbox.clone(),
            transport: RecordingAsyncTransport::default(),
            retry_policy: RetryPolicy::default(),
        };

        assert!(matches!(
            dispatcher.dispatch_one(0).await.unwrap(),
            crate::AsyncDeliveryOutcome::Delivered
        ));
        assert_eq!(
            outbox.state_snapshot(id).await,
            Some(DeliveryState::Succeeded)
        );
        assert_eq!(outbox.pending_len().await, 0);
    }
}
