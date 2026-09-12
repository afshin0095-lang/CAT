use cat_eventbus::{EventBusMetrics, EventBusMetricsSnapshot};

#[test]
fn metrics_are_zero_initialized() {
    let metrics = EventBusMetrics::default();
    assert_eq!(metrics.snapshot(), EventBusMetricsSnapshot::default());
}

#[test]
fn metrics_snapshot_tracks_delivery_lifecycle() {
    let metrics = EventBusMetrics::default();
    metrics.record_published();
    metrics.record_delivered();
    metrics.record_acknowledged();
    metrics.record_retried();
    metrics.record_dead_lettered();
    metrics.record_rejected();
    assert_eq!(
        metrics.snapshot(),
        EventBusMetricsSnapshot {
            published: 1,
            delivered: 1,
            acknowledged: 1,
            retried: 1,
            dead_lettered: 1,
            rejected: 1
        }
    );
}

#[test]
fn reset_returns_previous_snapshot_and_clears_counters() {
    let metrics = EventBusMetrics::default();
    metrics.record_published();
    metrics.record_published();
    metrics.record_retried();
    assert_eq!(
        metrics.reset(),
        EventBusMetricsSnapshot {
            published: 2,
            retried: 1,
            ..EventBusMetricsSnapshot::default()
        }
    );
    assert_eq!(metrics.snapshot(), EventBusMetricsSnapshot::default());
}

#[test]
fn metrics_are_safe_for_concurrent_updates() {
    use std::sync::Arc;
    use std::thread;
    let metrics = Arc::new(EventBusMetrics::default());
    let mut workers = Vec::new();
    for _ in 0..8 {
        let metrics = Arc::clone(&metrics);
        workers.push(thread::spawn(move || {
            for _ in 0..1_000 {
                metrics.record_published();
                metrics.record_delivered();
            }
        }));
    }
    for worker in workers {
        worker.join().expect("metrics worker must finish");
    }
    let snapshot = metrics.snapshot();
    assert_eq!(snapshot.published, 8_000);
    assert_eq!(snapshot.delivered, 8_000);
}
