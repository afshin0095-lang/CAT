use cat_eventbus::{CatEvent, EventEnvelope, EventKind};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ContractEvent {
    value: String,
}

impl CatEvent for ContractEvent {
    const TYPE: &'static str = "test.contract.event";
    const VERSION: u16 = 1;
}

#[test]
fn typed_event_envelope_preserves_contract_identity() {
    let envelope = EventEnvelope::from_typed(
        ContractEvent {
            value: "deterministic".to_owned(),
        },
        "eventbus-contract-test",
    )
    .expect("typed event must serialize");

    assert_eq!(envelope.event_type, ContractEvent::TYPE);
    assert_eq!(envelope.version, ContractEvent::VERSION);
    assert_eq!(envelope.kind, EventKind::Domain);
    assert_eq!(envelope.producer, "eventbus-contract-test");
    assert_eq!(envelope.payload["value"], "deterministic");
    assert!(envelope.correlation_id.is_none());
    assert!(envelope.causation_id.is_none());
}

#[test]
fn envelope_chain_metadata_is_explicit_and_round_trippable() {
    let correlation_id = Uuid::now_v7();
    let causation_id = Uuid::now_v7();

    let envelope = EventEnvelope::from_typed(
        ContractEvent {
            value: "child".to_owned(),
        },
        "worker",
    )
    .expect("typed event must serialize")
    .with_kind(EventKind::Integration)
    .with_correlation_id(correlation_id)
    .with_causation_id(causation_id);

    assert_eq!(envelope.kind, EventKind::Integration);
    assert_eq!(envelope.correlation_id, Some(correlation_id));
    assert_eq!(envelope.causation_id, Some(causation_id));
    assert_ne!(envelope.event_id, correlation_id);
    assert_ne!(envelope.event_id, causation_id);
}

#[test]
fn envelope_serialization_is_json_object_with_stable_top_level_fields() {
    let envelope = EventEnvelope::from_typed(
        ContractEvent {
            value: "payload".to_owned(),
        },
        "producer",
    )
    .expect("typed event must serialize");

    let encoded = serde_json::to_value(&envelope).expect("envelope must serialize to JSON");
    for field in [
        "event_id",
        "event_type",
        "version",
        "kind",
        "occurred_at_ms",
        "producer",
        "correlation_id",
        "causation_id",
        "subject_id",
        "payload",
    ] {
        assert!(
            encoded.get(field).is_some(),
            "missing envelope field: {field}"
        );
    }

    assert_eq!(encoded["event_type"], ContractEvent::TYPE);
    assert_eq!(encoded["version"], ContractEvent::VERSION);
    assert_eq!(encoded["producer"], "producer");
}
