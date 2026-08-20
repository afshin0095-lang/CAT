use cat_kernel::{
    parse_uuid, require_non_nil, Clock, CorrelationId, EntityId, ExecutionContext, FixedClock,
    TenantId, TimestampMs, Uuid,
};

#[test]
fn entity_ids_are_unique() {
    let first = EntityId::new();
    let second = EntityId::new();
    assert_ne!(first, second);
}

#[test]
fn entity_id_round_trips_through_uuid() {
    let original = EntityId::new();
    let restored = EntityId::from_uuid(original.as_uuid());
    assert_eq!(original, restored);
}

#[test]
fn timestamp_ordering_is_explicit_and_strict() {
    let first = TimestampMs::new(100);
    let second = TimestampMs::new(101);
    assert_eq!(second.checked_after(first).unwrap().as_u64(), 101);
    assert!(second.checked_after(second).is_err());
    assert!(first.checked_after(second).is_err());
}

#[test]
fn fixed_clock_is_deterministic() {
    let clock = FixedClock::new(TimestampMs::new(42));
    assert_eq!(clock.now().unwrap().as_u64(), 42);
    assert_eq!(clock.now().unwrap().as_u64(), 42);
}

#[test]
fn execution_context_preserves_correlation_and_causation() {
    let tenant = TenantId::from_uuid(Uuid::now_v7());
    let correlation = CorrelationId::from_uuid(Uuid::now_v7());
    let causation = cat_kernel::CausationId::from_uuid(Uuid::now_v7());
    let actor = EntityId::new();

    let context = ExecutionContext::new(tenant, correlation, actor, TimestampMs::new(7))
        .with_causation(causation);

    assert_eq!(context.tenant_id, tenant);
    assert_eq!(context.correlation_id, correlation);
    assert_eq!(context.causation_id, Some(causation));
    assert_eq!(context.issued_at.as_u64(), 7);
}

#[test]
fn uuid_validation_rejects_nil_and_accepts_valid_values() {
    assert!(require_non_nil(Uuid::nil()).is_err());
    let value = Uuid::now_v7();
    assert_eq!(require_non_nil(value).unwrap(), value);
    assert_eq!(parse_uuid(&value.to_string()).unwrap(), value);
    assert!(parse_uuid("not-a-uuid").is_err());
}
