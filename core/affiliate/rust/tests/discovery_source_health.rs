//! Integration coverage for the persistence-neutral source health contract.

use cat_affiliate::{InMemorySourceHealthStore, SourceHealthState, SourceHealthStore};

#[test]
fn health_journey_unknown_to_healthy_to_unavailable_to_healthy() {
    let mut store = InMemorySourceHealthStore::new();

    let snapshot = store.snapshot("network-a");
    assert_eq!(snapshot.state, SourceHealthState::Unknown);

    store.record_success("network-a", 1_000, 100);
    let snapshot = store.snapshot("network-a");
    assert_eq!(snapshot.state, SourceHealthState::Healthy);
    assert_eq!(snapshot.last_success_latency_ms, Some(100));

    for at in 0..cat_affiliate::UNAVAILABLE_AFTER_CONSECUTIVE_FAILURES {
        store.record_failure("network-a", 1_100 + u64::from(at), Some("timeout"));
    }
    let snapshot = store.snapshot("network-a");
    assert_eq!(snapshot.state, SourceHealthState::Unavailable);
    assert_eq!(
        snapshot.total_failures,
        u64::from(cat_affiliate::UNAVAILABLE_AFTER_CONSECUTIVE_FAILURES)
    );

    // One success recovers trust without erasing history.
    store.record_success("network-a", 2_000, 150);
    let snapshot = store.snapshot("network-a");
    assert_eq!(snapshot.state, SourceHealthState::Healthy);
    assert_eq!(snapshot.consecutive_failures, 0);
}

#[test]
fn multiple_sources_are_tracked_independently() {
    let mut store = InMemorySourceHealthStore::new();
    store.record_success("alpha", 1_000, 10);
    for _ in 0..cat_affiliate::UNAVAILABLE_AFTER_CONSECUTIVE_FAILURES {
        store.record_failure("beta", 1_000, None);
    }
    assert_eq!(store.snapshot("alpha").state, SourceHealthState::Healthy);
    assert_eq!(store.snapshot("beta").state, SourceHealthState::Unavailable);

    let snapshots = store.all_snapshots();
    let sources: Vec<&str> = snapshots
        .iter()
        .map(|snapshot| snapshot.source.as_str())
        .collect();
    assert_eq!(
        sources,
        vec!["alpha", "beta"],
        "iteration order is deterministic"
    );
}

#[test]
fn snapshots_are_plain_data() {
    let mut store = InMemorySourceHealthStore::new();
    store.record_success("network-a", 1_000, 42);
    let snapshot = store.snapshot("network-a");
    let encoded = serde_json::to_string(&snapshot).expect("serialize");
    let decoded: cat_affiliate::SourceHealthSnapshot =
        serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded, snapshot);
}
