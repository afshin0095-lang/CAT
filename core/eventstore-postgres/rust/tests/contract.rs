use cat_eventstore_postgres::{DurableAppendReceipt, PostgresEventStore};
use cat_kernel::SequenceNumber;

#[test]
fn adapter_type_is_constructible_at_compile_time() {
    let _ = std::any::type_name::<PostgresEventStore>();
}

#[test]
fn durable_append_receipt_exposes_idempotent_replay_state() {
    let event_id = uuid::Uuid::now_v7();
    let first = DurableAppendReceipt {
        event_id,
        sequence: SequenceNumber::new(1),
        idempotent_replay: false,
    };
    let replay = DurableAppendReceipt {
        event_id,
        sequence: SequenceNumber::new(1),
        idempotent_replay: true,
    };

    assert_eq!(first.event_id, replay.event_id);
    assert_eq!(first.sequence, replay.sequence);
    assert!(!first.idempotent_replay);
    assert!(replay.idempotent_replay);
}
