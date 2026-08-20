use std::time::Duration;

/// Retry configuration is explicit and deterministic; adapters decide how to schedule it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier_millis: u32,
}

impl RetryPolicy {
    pub const fn new(max_attempts: u32, initial_delay: Duration, max_delay: Duration) -> Self {
        Self {
            max_attempts,
            initial_delay,
            max_delay,
            multiplier_millis: 2000,
        }
    }

    pub fn delay_for(&self, attempt: u32) -> Duration {
        if attempt <= 1 {
            return self.initial_delay.min(self.max_delay);
        }
        let mut delay = self.initial_delay;
        for _ in 1..attempt {
            let millis = delay.as_millis().saturating_mul(self.multiplier_millis as u128) / 1000;
            delay = Duration::from_millis(millis.min(self.max_delay.as_millis()) as u64);
        }
        delay.min(self.max_delay)
    }

    pub fn exhausted(&self, attempt: u32) -> bool {
        attempt >= self.max_attempts
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::new(5, Duration::from_millis(250), Duration::from_secs(30))
    }
}

/// Delivery state shared by inbox/outbox adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryState {
    Pending,
    InFlight,
    Succeeded,
    RetryScheduled,
    DeadLettered,
}

/// Stable processing record. The event id is the idempotency boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliveryRecord {
    pub event_id: uuid::Uuid,
    pub attempt: u32,
    pub state: DeliveryState,
}
