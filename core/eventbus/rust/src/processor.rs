use crate::{
    DeadLetterStore, DeliveryState, EventBusError, EventBusResult, EventEnvelope, InboxStore,
    RetryPolicy,
};
use uuid::Uuid;

/// Broker-independent result of applying CAT's delivery policy to an event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessingDecision {
    Accepted,
    Duplicate,
    Retry { delay_ms: u64 },
    DeadLettered,
}

/// Deterministic application-level delivery coordinator.
pub struct DeliveryProcessor<I, D> {
    inbox: I,
    dead_letters: D,
    retry: RetryPolicy,
}

impl<I, D> DeliveryProcessor<I, D>
where
    I: InboxStore,
    D: DeadLetterStore,
{
    pub fn new(inbox: I, dead_letters: D, retry: RetryPolicy) -> Self {
        Self {
            inbox,
            dead_letters,
            retry,
        }
    }

    pub fn inbox(&self) -> &I {
        &self.inbox
    }
    pub fn inbox_mut(&mut self) -> &mut I {
        &mut self.inbox
    }
    pub fn dead_letters(&self) -> &D {
        &self.dead_letters
    }
    pub fn dead_letters_mut(&mut self) -> &mut D {
        &mut self.dead_letters
    }
    pub fn retry_policy(&self) -> RetryPolicy {
        self.retry
    }

    pub fn begin(&mut self, event_id: Uuid) -> EventBusResult<ProcessingDecision> {
        if self.inbox.accept(event_id)? {
            Ok(ProcessingDecision::Accepted)
        } else {
            Ok(ProcessingDecision::Duplicate)
        }
    }

    pub fn succeed(&mut self, event_id: Uuid) -> EventBusResult<DeliveryState> {
        self.inbox.mark_succeeded(event_id)?;
        Ok(DeliveryState::Succeeded)
    }

    pub fn fail(
        &mut self,
        event: EventEnvelope,
        attempt: u32,
        reason: impl Into<String>,
    ) -> EventBusResult<ProcessingDecision> {
        self.inbox.mark_failed(event.event_id)?;
        if self.retry.exhausted(attempt) {
            self.dead_letters.park(event, reason)?;
            return Ok(ProcessingDecision::DeadLettered);
        }
        Ok(ProcessingDecision::Retry {
            delay_ms: self.retry.delay_for(attempt + 1).as_millis() as u64,
        })
    }

    pub fn require_retryable(state: DeliveryState) -> EventBusResult<()> {
        match state {
            DeliveryState::RetryScheduled | DeliveryState::InFlight | DeliveryState::Pending => {
                Ok(())
            }
            DeliveryState::Succeeded | DeliveryState::DeadLettered => {
                Err(EventBusError::InvalidConfiguration(
                    "terminal delivery state cannot be retried".to_owned(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EventEnvelope, EventKind, InMemoryDeadLetterStore, InMemoryInbox};
    use serde_json::json;
    use std::time::Duration;

    fn event() -> EventEnvelope {
        EventEnvelope {
            event_id: Uuid::now_v7(),
            event_type: "cat.test".to_owned(),
            version: 1,
            kind: EventKind::Domain,
            occurred_at_ms: 1,
            producer: "test".to_owned(),
            correlation_id: None,
            causation_id: None,
            subject_id: None,
            payload: json!({"ok": true}),
        }
    }

    #[test]
    fn duplicate_delivery_is_not_reprocessed() {
        let mut p = DeliveryProcessor::new(
            InMemoryInbox::default(),
            InMemoryDeadLetterStore::default(),
            RetryPolicy::new(3, Duration::from_millis(10), Duration::from_millis(100)),
        );
        let e = event();
        assert_eq!(p.begin(e.event_id).unwrap(), ProcessingDecision::Accepted);
        assert_eq!(p.begin(e.event_id).unwrap(), ProcessingDecision::Duplicate);
    }

    #[test]
    fn exhausted_attempt_is_parked_in_dlq() {
        let mut p = DeliveryProcessor::new(
            InMemoryInbox::default(),
            InMemoryDeadLetterStore::default(),
            RetryPolicy::new(2, Duration::from_millis(10), Duration::from_millis(100)),
        );
        let e = event();
        p.begin(e.event_id).unwrap();
        assert_eq!(
            p.fail(e, 2, "poison event").unwrap(),
            ProcessingDecision::DeadLettered
        );
        assert_eq!(p.dead_letters().len(), 1);
    }

    #[test]
    fn failed_attempt_returns_bounded_retry_delay() {
        let mut p = DeliveryProcessor::new(
            InMemoryInbox::default(),
            InMemoryDeadLetterStore::default(),
            RetryPolicy::new(5, Duration::from_millis(10), Duration::from_millis(40)),
        );
        let e = event();
        p.begin(e.event_id).unwrap();
        assert_eq!(
            p.fail(e, 1, "temporary failure").unwrap(),
            ProcessingDecision::Retry { delay_ms: 20 }
        );
    }
}
