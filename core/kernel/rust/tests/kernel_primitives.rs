use cat_kernel::{EntityId, unix_millis};

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
fn infrastructure_clock_returns_a_nonzero_timestamp() {
    let timestamp = unix_millis().expect("clock must be available");
    assert!(timestamp > 0);
}
