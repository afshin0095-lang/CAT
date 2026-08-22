use cat_eventbus::{DeliveryHealth, DeliveryMetrics, DeliveryMetricsSnapshot, DeliveryOutcome};
use std::time::Duration;

#[test]
fn delivery_metrics_snapshot_is_deterministic() {
    let metrics = DeliveryMetrics::default();
    metrics.record(DeliveryOutcome::Idle);
    metrics.record(DeliveryOutcome::Delivered);
    metrics.record(DeliveryOutcome::RetryScheduled { attempt: 1, delay: Duration::from_millis(250) });
    assert_eq!(metrics.snapshot(), DeliveryMetricsSnapshot { idle: 1, delivered: 1, retries: 1, dead_lettered: 0, total_attempts: 2 });
    assert_eq!(metrics.health().status, DeliveryHealth::Degraded);
}

#[test]
fn dead_lettered_delivery_is_unhealthy() {
    let metrics = DeliveryMetrics::default();
    metrics.record(DeliveryOutcome::DeadLettered { attempt: 5 });
    let health = metrics.health();
    assert_eq!(health.status, DeliveryHealth::Unhealthy);
    assert_eq!(health.dead_lettered, 1);
    assert_eq!(health.total_attempts, 1);
}
