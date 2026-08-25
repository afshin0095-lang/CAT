use crate::{DeliveryState, EventBusError, EventBusResult, RetryPolicy};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeliveryDecision {
    Retry { after: Duration },
    DeadLetter,
}

#[derive(Clone, Debug)]
pub struct DeliveryAttempt {
    pub event_id: Uuid,
    pub attempt: u32,
    pub state: DeliveryState,
    pub next_attempt_at_ms: Option<u64>,
    pub error: Option<String>,
}

impl DeliveryAttempt {
    pub fn new(event_id: Uuid) -> Self {
        Self { event_id, attempt: 0, state: DeliveryState::Pending, next_attempt_at_ms: None, error: None }
    }

    pub fn failed(&mut self, error: impl Into<String>, policy: &RetryPolicy) -> EventBusResult<DeliveryDecision> {
        self.attempt = self.attempt.saturating_add(1);
        self.error = Some(error.into());
        self.state = DeliveryState::Failed;
        if self.attempt >= policy.max_attempts {
            self.state = DeliveryState::DeadLetter;
            self.next_attempt_at_ms = None;
            return Ok(DeliveryDecision::DeadLetter);
        }
        let delay = policy.delay_for(self.attempt);
        self.next_attempt_at_ms = Some(unix_millis()?.saturating_add(delay.as_millis() as u64));
        self.state = DeliveryState::RetryScheduled;
        Ok(DeliveryDecision::Retry { after: delay })
    }

    pub fn succeeded(&mut self) {
        self.state = DeliveryState::Succeeded;
        self.next_attempt_at_ms = None;
        self.error = None;
    }
}

fn unix_millis() -> EventBusResult<u64> {
    SystemTime::now().duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .map_err(|e| EventBusError::Clock(e.to_string()))
}
