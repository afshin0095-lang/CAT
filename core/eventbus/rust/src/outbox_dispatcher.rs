use crate::{AsyncOutboxStore, EventBusResult, EventEnvelope, EventTransport, RetryPolicy};

/// Result of one dispatcher attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DispatchOutcome {
    Idle,
    Delivered { event_id: uuid::Uuid },
    RetryScheduled {
        event_id: uuid::Uuid,
        attempt: u32,
        error: String,
    },
    DeadLettered {
        event_id: uuid::Uuid,
        attempt: u32,
        error: String,
    },
}

/// Small orchestration boundary for transactional-outbox delivery.
///
/// The dispatcher does not own business transactions or canonical state. It only
/// moves already-committed envelopes from an outbox to a transport and records
/// the resulting delivery state. This keeps at-least-once delivery explicit:
/// consumers must remain idempotent.
pub struct OutboxDispatcher<'a, O, T>
where
    O: AsyncOutboxStore,
    T: EventTransport,
{
    outbox: &'a O,
    transport: &'a mut T,
    retry_policy: RetryPolicy,
}

impl<'a, O, T> OutboxDispatcher<'a, O, T>
where
    O: AsyncOutboxStore,
    T: EventTransport,
{
    pub fn new(outbox: &'a O, transport: &'a mut T, retry_policy: RetryPolicy) -> Self {
        Self {
            outbox,
            transport,
            retry_policy,
        }
    }

    pub async fn dispatch_once(&mut self, attempt: u32) -> EventBusResult<DispatchOutcome> {
        let Some(event) = self.outbox.claim_next().await? else {
            return Ok(DispatchOutcome::Idle);
        };

        self.deliver(event, attempt).await
    }

    async fn deliver(
        &mut self,
        event: EventEnvelope,
        attempt: u32,
    ) -> EventBusResult<DispatchOutcome> {
        let event_id = event.event_id;

        match self.transport.publish(&event) {
            Ok(()) => {
                self.outbox.acknowledge(event_id).await?;
                Ok(DispatchOutcome::Delivered { event_id })
            }
            Err(error) => {
                let error_message = error.to_string();
                let state = self
                    .outbox
                    .fail(event_id, attempt, &self.retry_policy, &error_message)
                    .await?;

                if matches!(state, crate::DeliveryState::DeadLettered) {
                    Ok(DispatchOutcome::DeadLettered {
                        event_id,
                        attempt,
                        error: error_message,
                    })
                } else {
                    Ok(DispatchOutcome::RetryScheduled {
                        event_id,
                        attempt,
                        error: error_message,
                    })
                }
            }
        }
    }

    pub fn retry_policy(&self) -> &RetryPolicy {
        &self.retry_policy
    }
}
