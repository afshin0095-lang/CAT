use crate::{
    DeadLetterStore, EventBusResult, EventEnvelope, EventTransport, OutboxStore, RetryPolicy,
};
use std::time::Duration;

/// Result of one durable outbox delivery attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryOutcome {
    Idle,
    Delivered,
    RetryScheduled { attempt: u32, delay: Duration },
    DeadLettered { attempt: u32 },
}

/// Small orchestration layer shared by durable transport workers.
///
/// The dispatcher deliberately does not own persistence. The outbox remains the
/// source of delivery work, the transport owns network semantics, and the DLQ
/// owns poison-event quarantine. This keeps the delivery loop deterministic and
/// replaceable while allowing PostgreSQL, Redis, or another durable adapter to
/// implement the storage boundaries.
pub struct OutboxDispatcher<O, T, D> {
    pub outbox: O,
    pub transport: T,
    pub dead_letters: D,
    pub retry_policy: RetryPolicy,
}

impl<O, T, D> OutboxDispatcher<O, T, D>
where
    O: OutboxStore,
    T: EventTransport,
    D: DeadLetterStore,
{
    pub fn dispatch_one(&mut self, attempt: u32) -> EventBusResult<DeliveryOutcome> {
        let Some(event) = self.outbox.next()? else {
            return Ok(DeliveryOutcome::Idle);
        };

        let event_id = event.event_id;
        match self.transport.publish(&event) {
            Ok(()) => {
                self.outbox.acknowledge(event_id)?;
                Ok(DeliveryOutcome::Delivered)
            }
            Err(_) => {
                let next_attempt = attempt.saturating_add(1);
                let state = self
                    .outbox
                    .fail(event_id, next_attempt, &self.retry_policy)?;
                match state {
                    crate::DeliveryState::DeadLettered => {
                        self.dead_letters.park(event, "retry budget exhausted")?;
                        Ok(DeliveryOutcome::DeadLettered {
                            attempt: next_attempt,
                        })
                    }
                    _ => Ok(DeliveryOutcome::RetryScheduled {
                        attempt: next_attempt,
                        delay: self.retry_policy.delay_for(next_attempt),
                    }),
                }
            }
        }
    }
}

/// A transport that records the envelopes it accepted. Useful for deterministic
/// integration tests and local orchestration without a broker.
#[derive(Debug, Default)]
pub struct RecordingTransport {
    pub published: Vec<EventEnvelope>,
}

impl EventTransport for RecordingTransport {
    fn publish(&mut self, event: &EventEnvelope) -> EventBusResult<()> {
        self.published.push(event.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryOutbox;

    #[test]
    fn dispatcher_reports_idle_when_outbox_is_empty() {
        let mut dispatcher = OutboxDispatcher {
            outbox: InMemoryOutbox::default(),
            transport: RecordingTransport::default(),
            dead_letters: crate::InMemoryDeadLetterStore::default(),
            retry_policy: RetryPolicy::default(),
        };

        assert_eq!(dispatcher.dispatch_one(0).unwrap(), DeliveryOutcome::Idle);
    }
}
