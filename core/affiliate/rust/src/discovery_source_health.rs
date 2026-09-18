//! Domain-level discovery source health.
//!
//! Tracks success/failure observations per discovery source behind a
//! persistence-neutral contract. This is *not* a distributed circuit breaker:
//! Sprint 0 only records and classifies what has been observed. Consumers
//! (revalidation planning, opportunity health, source selection) read
//! snapshots; a breaker policy can be layered on later without changing this
//! contract.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Consecutive failures after which a source is considered unavailable.
pub const UNAVAILABLE_AFTER_CONSECUTIVE_FAILURES: u32 = 5;
/// Consecutive failures after which a source is considered degraded.
pub const DEGRADED_AFTER_CONSECUTIVE_FAILURES: u32 = 2;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceHealthState {
    /// No success or failure has been observed yet.
    #[default]
    Unknown,
    Healthy,
    Degraded,
    Unavailable,
}

impl SourceHealthState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Point-in-time health of one discovery source.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct SourceHealthSnapshot {
    pub source: String,
    pub state: SourceHealthState,
    pub consecutive_failures: u32,
    pub total_successes: u64,
    pub total_failures: u64,
    pub last_success_at_ms: Option<u64>,
    pub last_failure_at_ms: Option<u64>,
    /// Most recent observed success latency in milliseconds.
    pub last_success_latency_ms: Option<u64>,
    /// Availability in basis points over all observations
    /// (`None` when nothing has been observed).
    pub availability_bps: Option<u16>,
}

/// Persistence-neutral health observation contract.
pub trait SourceHealthStore {
    /// Records one successful interaction. Latency must be a checked,
    /// non-negative duration supplied by the caller.
    fn record_success(&mut self, source: &str, at_ms: u64, latency_ms: u64);
    /// Records one failed interaction. `reason` is a short, bounded,
    /// non-sensitive label; raw provider payloads are never accepted here.
    fn record_failure(&mut self, source: &str, at_ms: u64, reason: Option<&str>);
    fn snapshot(&self, source: &str) -> SourceHealthSnapshot;
    /// All snapshots, deterministically ordered by source identifier.
    fn all_snapshots(&self) -> Vec<SourceHealthSnapshot>;
}

/// In-memory implementation with deterministic iteration order.
#[derive(Clone, Debug, Default)]
pub struct InMemorySourceHealthStore {
    sources: BTreeMap<String, SourceHealthSnapshot>,
    last_failure_reasons: BTreeMap<String, String>,
}

impl InMemorySourceHealthStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Short, bounded label of the last recorded failure, if any.
    pub fn last_failure_reason(&self, source: &str) -> Option<&str> {
        self.last_failure_reasons
            .get(source)
            .map(|reason| reason.as_str())
    }
}

impl SourceHealthStore for InMemorySourceHealthStore {
    fn record_success(&mut self, source: &str, at_ms: u64, latency_ms: u64) {
        let entry = self.sources.entry(source.to_owned()).or_default();
        entry.source = source.to_owned();
        entry.consecutive_failures = 0;
        entry.total_successes = entry.total_successes.saturating_add(1);
        entry.last_success_at_ms = Some(at_ms);
        entry.last_success_latency_ms = Some(latency_ms);
        refresh_availability(entry);
    }

    fn record_failure(&mut self, source: &str, at_ms: u64, reason: Option<&str>) {
        let entry = self.sources.entry(source.to_owned()).or_default();
        entry.source = source.to_owned();
        entry.consecutive_failures = entry.consecutive_failures.saturating_add(1);
        entry.total_failures = entry.total_failures.saturating_add(1);
        entry.last_failure_at_ms = Some(at_ms);
        refresh_availability(entry);
        if let Some(reason) = reason {
            const MAX_REASON_CHARS: usize = 128;
            let bounded: String = reason
                .chars()
                .map(|character| {
                    if character.is_control() {
                        ' '
                    } else {
                        character
                    }
                })
                .take(MAX_REASON_CHARS)
                .collect();
            self.last_failure_reasons.insert(source.to_owned(), bounded);
        }
    }

    fn snapshot(&self, source: &str) -> SourceHealthSnapshot {
        self.sources
            .get(source)
            .cloned()
            .unwrap_or(SourceHealthSnapshot {
                source: source.to_owned(),
                state: SourceHealthState::Unknown,
                ..SourceHealthSnapshot::default()
            })
    }

    fn all_snapshots(&self) -> Vec<SourceHealthSnapshot> {
        self.sources.values().cloned().collect()
    }
}

fn refresh_availability(entry: &mut SourceHealthSnapshot) {
    let total = entry.total_successes.saturating_add(entry.total_failures);
    entry.availability_bps = if total == 0 {
        None
    } else {
        // u128 intermediate: (u64::MAX successes) * 10_000 cannot overflow.
        let bps = (u128::from(entry.total_successes) * 10_000 / u128::from(total)) as u16;
        Some(bps)
    };
    entry.state = if entry.consecutive_failures >= UNAVAILABLE_AFTER_CONSECUTIVE_FAILURES {
        SourceHealthState::Unavailable
    } else if entry.consecutive_failures >= DEGRADED_AFTER_CONSECUTIVE_FAILURES {
        SourceHealthState::Degraded
    } else if total == 0 {
        SourceHealthState::Unknown
    } else {
        SourceHealthState::Healthy
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_sources_report_unknown_state() {
        let store = InMemorySourceHealthStore::new();
        let snapshot = store.snapshot("never-seen");
        assert_eq!(snapshot.state, SourceHealthState::Unknown);
        assert_eq!(snapshot.availability_bps, None);
        assert_eq!(snapshot.total_successes, 0);
    }

    #[test]
    fn successes_keep_sources_healthy() {
        let mut store = InMemorySourceHealthStore::new();
        store.record_success("network-a", 1_000, 120);
        store.record_failure("network-a", 1_100, Some("timeout"));
        let snapshot = store.snapshot("network-a");
        assert_eq!(
            snapshot.state,
            SourceHealthState::Healthy,
            "one failure cannot degrade"
        );
        assert_eq!(snapshot.consecutive_failures, 0);
        assert_eq!(snapshot.last_success_latency_ms, Some(120));
        assert_eq!(snapshot.availability_bps, Some(5_000));
    }

    #[test]
    fn consecutive_failures_degrade_then_mark_unavailable() {
        let mut store = InMemorySourceHealthStore::new();
        for at in 0..DEGRADED_AFTER_CONSECUTIVE_FAILURES {
            store.record_failure("network-b", u64::from(at), None);
        }
        assert_eq!(
            store.snapshot("network-b").state,
            SourceHealthState::Degraded
        );
        for at in DEGRADED_AFTER_CONSECUTIVE_FAILURES..UNAVAILABLE_AFTER_CONSECUTIVE_FAILURES {
            store.record_failure("network-b", u64::from(at), None);
        }
        assert_eq!(
            store.snapshot("network-b").state,
            SourceHealthState::Unavailable
        );

        // A single success resets the streak without erasing history.
        store.record_success("network-b", 9_000, 80);
        let snapshot = store.snapshot("network-b");
        assert_eq!(snapshot.state, SourceHealthState::Healthy);
        assert_eq!(snapshot.consecutive_failures, 0);
        assert_eq!(
            snapshot.total_failures,
            u64::from(UNAVAILABLE_AFTER_CONSECUTIVE_FAILURES)
        );
    }

    #[test]
    fn failure_reasons_are_bounded_and_sanitized() {
        let mut store = InMemorySourceHealthStore::new();
        let payload = "x".repeat(500);
        store.record_failure("network-c", 1_000, Some(&format!("boom\n{payload}")));
        let reason = store
            .last_failure_reason("network-c")
            .expect("reason recorded");
        assert!(reason.len() <= 128);
        assert!(!reason.contains('\n'));
    }

    #[test]
    fn all_snapshots_are_sorted_by_source() {
        let mut store = InMemorySourceHealthStore::new();
        store.record_success("zeta", 1, 1);
        store.record_success("alpha", 1, 1);
        let snapshots = store.all_snapshots();
        let sources: Vec<&str> = snapshots
            .iter()
            .map(|snapshot| snapshot.source.as_str())
            .collect();
        assert_eq!(sources, vec!["alpha", "zeta"]);
    }

    #[test]
    fn availability_never_overflows_at_extremes() {
        let mut store = InMemorySourceHealthStore::new();
        for at in 0_u64..1_000 {
            store.record_success("network-d", at, 1);
            store.record_failure("network-d", at, None);
        }
        let snapshot = store.snapshot("network-d");
        assert_eq!(snapshot.availability_bps, Some(5_000));
    }
}
