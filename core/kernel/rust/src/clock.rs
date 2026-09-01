use std::time::{SystemTime, UNIX_EPOCH};

use crate::{KernelError, KernelResult, TimestampMs};

/// Clock boundary used by infrastructure and tests.
pub trait Clock: Send + Sync {
    fn now(&self) -> KernelResult<TimestampMs>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> KernelResult<TimestampMs> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| TimestampMs::new(duration.as_millis() as u64))
            .map_err(|error| KernelError::InvalidTimestamp(error.to_string()))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedClock {
    now: TimestampMs,
}

impl FixedClock {
    pub const fn new(now: TimestampMs) -> Self { Self { now } }
}

impl Clock for FixedClock {
    fn now(&self) -> KernelResult<TimestampMs> { Ok(self.now) }
}
