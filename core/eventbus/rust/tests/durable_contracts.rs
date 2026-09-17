use std::time::Duration;

use cat_eventbus::{
    Compatibility, DeadLetterStore, EventCodec, EventContract, EventEnvelope, EventKind,
    EventRegistry, IdempotencyStore, InMemoryDeadLetterStore, InMemoryIdempotency, InMemoryInbox,
    InMemoryOutbox, InboxStore, JsonEventCodec, OutboxStore, RetryPolicy,
};
use serde_json::json;
use uuid::Uuid;

fn sample_event() -> EventEnvelope {
    EventEnvelope {
        event_id: Uuid::now_v7(),
        event_type: "affiliate.campaign.started".to_owned(),
        version: 1,
        kind: EventKind::Domain,
        occurred_at_ms: 1,
        producer: "test".to_owned(),
        correlation_id: None,
        causation_id: None,
        subject_id: None,
        payload: json!({"campaign_id": "cmp-1"}),
    }
}

#[test]
fn registry_rejects_duplicate_contract_version() {
    let mut registry = EventRegistry::default();
    let contract = EventContract::new(
        "affiliate.campaign.started",
        1,
        "cat.affiliate.campaign.started.v1",
        Compatibility::Full,
    );
    registry.register(contract.clone()).unwrap();
    assert!(registry.register(contract).is_err());
    assert_eq!(registry.len(), 1);
}

#[test]
fn json_codec_round_trips_envelope() {
    let codec = JsonEventCodec;
    let event = sample_event();
    let bytes = codec.encode(&event).unwrap();
    let decoded = codec.decode(&bytes).unwrap();
    assert_eq!(decoded.event_id, event.event_id);
    assert_eq!(decoded.event_type, event.event_type);
    assert_eq!(decoded.payload, event.payload);
}

#[test]
fn inbox_and_idempotency_suppress_duplicates() {
    let event_id = Uuid::now_v7();
    let mut idempotency = InMemoryIdempotency::default();
    assert!(idempotency.claim(event_id).unwrap());
    assert!(!idempotency.claim(event_id).unwrap());
    idempotency.complete(event_id).unwrap();

    let mut inbox = InMemoryInbox::default();
    assert!(inbox.accept(event_id).unwrap());
    assert!(!inbox.accept(event_id).unwrap());
    inbox.mark_succeeded(event_id).unwrap();
}

#[test]
fn outbox_preserves_enqueue_order() {
    let mut outbox = InMemoryOutbox::default();
    let first = sample_event();
    let mut second = sample_event();
    second.event_id = Uuid::now_v7();
    outbox.enqueue(first.clone()).unwrap();
    outbox.enqueue(second.clone()).unwrap();
    assert_eq!(outbox.next().unwrap().unwrap().event_id, first.event_id);
    assert_eq!(outbox.next().unwrap().unwrap().event_id, second.event_id);
}

#[test]
fn retry_policy_exhaustion_is_deterministic() {
    let policy = RetryPolicy::new(3, Duration::from_millis(100), Duration::from_secs(1));
    assert_eq!(policy.delay_for(1), Duration::from_millis(100));
    assert_eq!(policy.delay_for(2), Duration::from_millis(200));
    assert!(!policy.exhausted(2));
    assert!(policy.exhausted(3));
}

#[test]
fn dead_letter_store_is_explicit_not_silent() {
    let mut store = InMemoryDeadLetterStore::default();
    store.park(sample_event(), "max attempts exceeded").unwrap();
    assert_eq!(store.len(), 1);
    assert_eq!(store.entries()[0].reason, "max attempts exceeded");
}
