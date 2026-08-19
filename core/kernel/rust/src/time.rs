use std::time::{SystemTime, UNIX_EPOCH};

use crate::{KernelError, KernelResult};

/// Returns Unix epoch milliseconds for infrastructure metadata.
///
/// Domain modules must not use wall-clock time to derive business truth.
/// They should receive explicit timestamps at their boundary instead.
pub fn unix_millis() -> KernelResult<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .map_err(|error| KernelError::InvalidTimestamp(error.to_string()))
}
