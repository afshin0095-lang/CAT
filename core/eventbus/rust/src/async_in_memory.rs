use crate::{AsyncInboxStore, DeliveryState, EventBusError, EventBusResult};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Deterministic asynchronous inbox for development and contract tests.
///
/// The implementation models the durable consumer state machine without
/// coupling handlers to a particular database:
/// Pending/unknown -> InFlight -> RetryScheduled -> InFlight -> Succeeded.
///
/// A successfully completed event is never accepted again, while a failed
/// event may be claimed again. This makes retry semantics explicit and keeps
/// consumer-side deduplication aligned with at-least-once broker delivery.
#[derive(Clone, Debug, Default)]
pub struct AsyncInMemoryInbox {
    inner: Arc<Mutex<HashMap<uuid::Uuid, DeliveryState>>>,
}

impl AsyncInMemoryInbox {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl AsyncInboxStore for AsyncInMemoryInbox {
    async fn accept(&self, event_id: uuid::Uuid) -> EventBusResult<bool> {
        let mut inner = self.inner.lock().expect("in-memory inbox mutex poisoned");
        match inner.get(&event_id).copied() {
            None | Some(DeliveryState::RetryScheduled) => {
                inner.insert(event_id, DeliveryState::InFlight);
                Ok(true)
            }
            Some(DeliveryState::InFlight)
            | Some(DeliveryState::Succeeded)
            | Some(DeliveryState::DeadLettered)
            | Some(DeliveryState::Pending) => Ok(false),
        }
    }

    async fn mark_succeeded(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        let mut inner = self.inner.lock().expect("in-memory inbox mutex poisoned");
        match inner.get(&event_id).copied() {
            Some(DeliveryState::InFlight) => {
                inner.insert(event_id, DeliveryState::Succeeded);
                Ok(())
            }
            _ => Err(EventBusError::UnknownEvent(event_id)),
        }
    }

    async fn mark_failed(&self, event_id: uuid::Uuid) -> EventBusResult<()> {
        let mut inner = self.inner.lock().expect("in-memory inbox mutex poisoned");
        match inner.get(&event_id).copied() {
            Some(DeliveryState::InFlight) => {
                inner.insert(event_id, DeliveryState::RetryScheduled);
                Ok(())
            }
            _ => Err(EventBusError::UnknownEvent(event_id)),
        }
    }

    async fn state(&self, event_id: uuid::Uuid) -> EventBusResult<Option<DeliveryState>> {
        let inner = self.inner.lock().expect("in-memory inbox mutex poisoned");
        Ok(inner.get(&event_id).copied())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn failed_event_can_be_reaccepted_for_retry() {
        let inbox = AsyncInMemoryInbox::new();
        let event_id = uuid::Uuid::now_v7();

        assert!(inbox.accept(event_id).await.unwrap());
        inbox.mark_failed(event_id).await.unwrap();
        assert_eq!(
            inbox.state(event_id).await.unwrap(),
            Some(DeliveryState::RetryScheduled)
        );

        assert!(inbox.accept(event_id).await.unwrap());
        assert_eq!(
            inbox.state(event_id).await.unwrap(),
            Some(DeliveryState::InFlight)
        );
    }

    #[tokio::test]
    async fn succeeded_event_remains_deduplicated() {
        let inbox = AsyncInMemoryInbox::new();
        let event_id = uuid::Uuid::now_v7();

        assert!(inbox.accept(event_id).await.unwrap());
        inbox.mark_succeeded(event_id).await.unwrap();
        assert!(!inbox.accept(event_id).await.unwrap());
        assert_eq!(
            inbox.state(event_id).await.unwrap(),
            Some(DeliveryState::Succeeded)
        );
    }
}
