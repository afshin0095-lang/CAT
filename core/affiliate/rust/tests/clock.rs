use cat_affiliate::{Clock, FixedClock, SystemClock};

#[test]
fn clock_contract_supports_deterministic_and_system_backends() {
    // Deterministic backend: stable across reads, advance-only.
    let mut fixed = FixedClock::new(1_000);
    assert_eq!(fixed.now_ms(), 1_000);
    assert_eq!(fixed.now_ms(), 1_000);
    fixed.advance(1);
    assert_eq!(fixed.now_ms(), 1_001);

    // System backend: plausible epoch milliseconds (wide bounds, never flaky).
    let now = SystemClock::new().now_ms();
    assert!(now > 1_600_000_000_000, "unexpectedly old system clock: {now}");
}

#[test]
fn clocks_are_object_safe_for_dependency_injection() {
    fn read_via_trait(clock: &dyn Clock) -> u64 {
        clock.now_ms()
    }
    assert_eq!(read_via_trait(&FixedClock::new(7)), 7);
    assert!(read_via_trait(&SystemClock::new()) > 0);
}
