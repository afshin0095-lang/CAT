//! Injectable clock abstraction for Affiliate Core.
//!
//! Deterministic domain functions never read the system clock. Where real
//! time is required (adapters, ingestion orchestration), a `Clock` is
//! injected so tests can substitute [`FixedClock`] and never sleep.
//!
//! Rules:
//! - no global mutable clock;
//! - no `std::time` reads inside pure domain logic;
//! - all arithmetic on clock output is checked by consumers.

use serde::{Deserialize, Serialize};

/// Source of wall-clock milliseconds since the Unix epoch.
pub trait Clock: Send + Sync {
    /// Milliseconds since the Unix epoch. Monotonicity is best-effort for
    /// [`SystemClock`] and guaranteed only by contract for [`FixedClock`].
    fn now_ms(&self) -> u64;
}

/// Production clock backed by the system wall clock.
///
/// If the platform clock reports a time before the Unix epoch (a broken
/// environment), this clock fails closed by returning `0`; consumers treat
/// `0` as "no trustworthy time" rather than as a valid early timestamp.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SystemClock;

impl SystemClock {
    pub fn new() -> Self {
        Self
    }
}

impl Clock for SystemClock {
    fn now_ms(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
            .unwrap_or(0)
    }
}

/// Deterministic clock for tests and replayable evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FixedClock {
    now_ms: u64,
}

impl FixedClock {
    pub fn new(now_ms: u64) -> Self {
        Self { now_ms }
    }

    /// Move the clock forward. Clock regression is rejected: the fixed time
    /// only advances (`saturating`), never rewinds.
    pub fn advance(&mut self, delta_ms: u64) {
        self.now_ms = self.now_ms.saturating_add(delta_ms);
    }

    pub fn set(&mut self, now_ms: u64) {
        self.now_ms = self.now_ms.max(now_ms);
    }
}

impl Clock for FixedClock {
    fn now_ms(&self) -> u64 {
        self.now_ms
    }
}

impl Default for FixedClock {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Blanket adapter so any `Fn() -> u64` closure can act as a [`Clock`].
///
/// Note: closures used through this adapter must themselves be deterministic
/// or system-backed; production code should prefer [`SystemClock`].
pub struct FnClock<F>
where
    F: Fn() -> u64 + Send + Sync,
{
    function: F,
}

impl<F> FnClock<F>
where
    F: Fn() -> u64 + Send + Sync,
{
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<F> Clock for FnClock<F>
where
    F: Fn() -> u64 + Send + Sync,
{
    fn now_ms(&self) -> u64 {
        (self.function)()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_clock_is_deterministic() {
        let mut clock = FixedClock::new(1_000);
        assert_eq!(clock.now_ms(), 1_000);
        clock.advance(500);
        assert_eq!(clock.now_ms(), 1_500);
    }

    #[test]
    fn fixed_clock_never_rewinds() {
        let mut clock = FixedClock::new(1_000);
        clock.advance(u64::MAX);
        assert_eq!(clock.now_ms(), u64::MAX);
        clock.advance(1);
        assert_eq!(clock.now_ms(), u64::MAX);
        clock.set(1);
        assert_eq!(clock.now_ms(), u64::MAX);
    }

    #[test]
    fn system_clock_reports_epoch_milliseconds() {
        // Sanity only: the system clock must produce a plausible modern
        // timestamp (between 2020 and 2100) without being pinned, so this
        // test never flakes on slow runners.
        let now = SystemClock::new().now_ms();
        assert!(now >= 1_577_836_800_000, "system clock before 2020: {now}");
        assert!(now <= 4_102_444_800_000, "system clock after 2100: {now}");
    }

    #[test]
    fn closure_clock_is_supported() {
        let clock = FnClock::new(|| 42);
        assert_eq!(clock.now_ms(), 42);
    }
}
