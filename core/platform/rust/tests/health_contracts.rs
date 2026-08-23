use cat_platform::{HealthState, PlatformHealthSnapshot};

#[test]
fn empty_scheduler_is_healthy_and_ready() {
    let snapshot = PlatformHealthSnapshot::from_queue_depth(0);
    assert_eq!(snapshot.state, HealthState::Healthy);
    assert!(snapshot.is_ready());
    assert_eq!(snapshot.queue_depth, 0);
    assert_eq!(snapshot.components.len(), 1);
}

#[test]
fn queued_work_is_degraded_but_still_ready() {
    let snapshot = PlatformHealthSnapshot::from_queue_depth(3);
    assert_eq!(snapshot.state, HealthState::Degraded);
    assert!(snapshot.is_ready());
    assert_eq!(snapshot.components[0].name, "scheduler");
}

#[test]
fn unhealthy_components_block_readiness() {
    let mut snapshot = PlatformHealthSnapshot::from_queue_depth(0);
    snapshot.state = HealthState::Unhealthy;
    snapshot.components[0].state = HealthState::Unhealthy;
    assert!(!snapshot.is_ready());
}
