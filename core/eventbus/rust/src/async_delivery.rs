use crate::{
    AsyncEventTransport, AsyncOutboxStore, DeliveryState, EventBusResult, EventEnvelope, RetryPolicy,
};
use std::time::Duration;

/// Result of one asynchronous durable outbox delivery attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AsyncDeliveryOutcome {
    Idle,
    Delivered,
    RetryScheduled { attempt: u32, delay: Duration },
    DeadLettered { attempt: u32 },
}

/// Async orchestration layer for durable publication.
///
/// Storage remains the source of truth for pending work. The dispatcher only
/// coordinates claim -> publish -> acknowledge/fail and therefore stays usable
/// with local tests, PostgreSQL outboxes, NATS, or future transports.
pub struct AsyncOutboxDispatcher<O, T> {
    pub outbox: O,
    pub transport: T,
    pub retry_policy: RetryPolicy,
}

impl<O, T> AsyncOutboxDispatcher<O, T>
where
    O: AsyncOutboxStore,
    T: AsyncEventTransport,
{
    pub async fn dispatch_one(&self, attempt: u32) -> EventBusResult<AsyncDeliveryOutcome> {
        let Some(event) = self.outbox.claim_next().await? else {
            return Ok(AsyncDeliveryOutcome::Idle);
        };

        let event_id = event.event_id;
        match self.transport.publish(&event).await {
            Ok(()) => {
                self.outbox.acknowledge(event_id).await?;
                Ok(AsyncDeliveryOutcome::Delivered)
            }
            Err(error) => {
                let next_attempt = attempt.saturating_add(1);
                match self
                    .outbox
                    .fail(event_id, next_attempt, &self.retry_policy, &error.to_string())
                    .await?
                {
                    DeliveryState::DeadLettered => {
                        Ok(AsyncDeliveryOutcome::DeadLettered { attempt: next_attempt })
                    }
                    _ => Ok(AsyncDeliveryOutcome::RetryScheduled {
                        attempt: next_attempt,
                        delay: self.retry_policy.delay_for(next_attempt),
                    }),
                }
            }
        }
    }
}

/// Minimal deterministic async transport for local integration tests.
#[derive(Clone, Debug, Default)]
pub struct RecordingAsyncTransport {
    inner: std::sync::Arc<std::sync::Mutex<Vec<EventEnvelope>>>,
}

impl RecordingAsyncTransport {
    pub fn published(&self) -> Vec<EventEnvelope> {
        self.inner
            .lock()
            .expect("recording async transport mutex poisoned")
            .clone()
    }
}

#[async_trait::async_trait]
impl AsyncEventTransport for RecordingAsyncTransport {
    async fn publish(&self, event: &EventEnvelope) -> EventBusResult<()> {
        self.inner
            .lock()
            .expect("recording async transport mutex poisoned")
            .push(event.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AsyncInMemoryOutbox, EventKind};

    fn event() -> EventEnvelope {
        EventEnvelope {
            event_id: uuid::Uuid::now_v7(),
            event_type: "cat.event.dispatch.v1".to_owned(),
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
    async fn dispatcher_delivers_and_acknowledges() {
        let outbox = AsyncInMemoryOutbox::new();
        let event = event();
        let id = event.event_id;
        outbox.enqueue(event).await.unwrap();

        let dispatcher = AsyncOutboxDispatcher {
            outbox: outbox.clone(),
            transport: RecordingAsyncTransport::default(),
            retry_policy: RetryPolicy::default(),
        };

        assert_eq!(
            dispatcher.dispatch_one(0).await.unwrap(),
            AsyncDeliveryOutcome::Delivered
        );
        assert_eq!(outbox.state(id), Some(DeliveryState::Succeeded));
        assert_eq!(dispatcher.transport.published().len(), 1);
    }

    #[tokio::test]
    async fn dispatcher_reports_idle_without_work() {
        let dispatcher = AsyncOutboxDispatcher {
            outbox: AsyncInMemoryOutbox::new(),
            transport: RecordingAsyncTransport::default(),
            retry_policy: RetryPolicy::default(),
        };

        assert_eq!(
            dispatcher.dispatch_one(0).await.unwrap(),
            AsyncDeliveryOutcome::Idle
        );
    }
}
