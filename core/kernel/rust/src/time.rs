use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::{KernelError, KernelResult};

/// Monotonic domain timestamp represented as Unix epoch milliseconds.
///
/// The kernel stores timestamps explicitly so domain operations can be deterministic
/// and replayable. Wall-clock acquisition belongs to infrastructure adapters.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TimestampMs(u64);

impl TimestampMs {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }

    pub const fn as_i64(self) -> i64 {
        self.0 as i64
    }

    pub fn checked_after(self, previous: Self) -> KernelResult<Self> {
        if self.0 <= previous.0 {
            return Err(KernelError::InvalidTimestamp(
                "timestamp must be strictly after the previous timestamp".to_owned(),
            ));
        }
        Ok(self)
    }
}

impl Default for TimestampMs {
    fn default() -> Self {
        Self(0)
    }
}

/// Returns the current wall-clock Unix time in milliseconds.
///
/// This function is an infrastructure-facing convenience; deterministic domain
/// logic should inject a `Clock` instead of calling it directly.
pub fn unix_millis() -> KernelResult<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .map_err(|error| KernelError::InvalidTimestamp(error.to_string()))
}
