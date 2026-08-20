use cat_kernel::{
    parse_uuid, require_non_nil, Clock, CorrelationId, EntityId, EventEnvelope, ExecutionContext,
    FixedClock, SequenceNumber, TenantId, TimestampMs, Uuid,
};

#[test]
fn entity_ids_are_unique() { assert_ne!(EntityId::new(), EntityId::new()); }

#[test]
fn entity_id_round_trips_through_uuid() {
    let original = EntityId::new();
    assert_eq!(original, EntityId::from_uuid(original.as_uuid()));
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
fn sequence_is_strict_and_overflow_safe() {
    let zero = SequenceNumber::ZERO;
    assert_eq!(zero.next().unwrap().as_u64(), 1);
    assert!(SequenceNumber::new(1).checked_after(zero).is_ok());
    assert!(SequenceNumber::new(1).checked_after(SequenceNumber::new(1)).is_err());
    assert!(SequenceNumber::new(u64::MAX).next().is_err());
}

#[test]
fn execution_context_preserves_correlation_and_causation() {
    let tenant = TenantId::new();
    let correlation = CorrelationId::new();
    let causation = cat_kernel::CausationId::new();
    let actor = EntityId::new();
    let context = ExecutionContext::new(tenant, correlation, actor, TimestampMs::new(7))
        .with_causation(causation);
    assert_eq!(context.tenant_id, tenant);
    assert_eq!(context.correlation_id, correlation);
    assert_eq!(context.causation_id, Some(causation));
}

#[test]
fn event_envelope_rejects_invalid_contract_metadata() {
    let base = || (TenantId::new(), CorrelationId::new(), None, EntityId::new(), TimestampMs::new(10), SequenceNumber::new(1));
    let (tenant, correlation, causation, actor, occurred, sequence) = base();
    assert!(EventEnvelope::new("", 1, tenant, correlation, causation, actor, occurred, sequence, "payload").is_err());
    let (tenant, correlation, causation, actor, occurred, sequence) = base();
    assert!(EventEnvelope::new("cat.test", 0, tenant, correlation, causation, actor, occurred, sequence, "payload").is_err());
}

#[test]
fn event_envelope_preserves_typed_payload_and_metadata() {
    let tenant = TenantId::new();
    let correlation = CorrelationId::new();
    let actor = EntityId::new();
    let envelope = EventEnvelope::new("affiliate.partner.created", 1, tenant, correlation, None, actor, TimestampMs::new(123), SequenceNumber::new(7), 42u64).unwrap();
    assert_eq!(envelope.event_type, "affiliate.partner.created");
    assert_eq!(envelope.event_version, 1);
    assert_eq!(envelope.tenant_id, tenant);
    assert_eq!(envelope.correlation_id, correlation);
    assert_eq!(envelope.occurred_at.as_u64(), 123);
    assert_eq!(envelope.sequence.as_u64(), 7);
    assert_eq!(envelope.payload, 42);
}

#[test]
fn uuid_validation_rejects_nil_and_accepts_valid_values() {
    assert!(require_non_nil(Uuid::nil()).is_err());
    let value = Uuid::now_v7();
    assert_eq!(require_non_nil(value).unwrap(), value);
    assert_eq!(parse_uuid(&value.to_string()).unwrap(), value);
    assert!(parse_uuid("not-a-uuid").is_err());
}
