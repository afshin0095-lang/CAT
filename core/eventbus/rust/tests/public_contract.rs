use cat_eventbus::{
    CatEvent, Compatibility, EventBus, EventBusError, EventContract, EventEnvelope, EventRegistry,
    EventKind, IdempotencyStore, InMemoryIdempotency, InMemoryInbox, InMemoryOutbox, InboxStore,
    OutboxStore, PublishOutcome, RetryPolicy,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TestEvent {
    value: String,
}

impl CatEvent for TestEvent {
    const TYPE: &'static str = "test.contract.event";
    const VERSION: u16 = 1;
}

fn envelope(id: Uuid) -> EventEnvelope {
    TestEvent { value: "hello".into() }
        .into_envelope("public-contract-test")
        .unwrap()
        .with_correlation_id(id)
        .with_causation_id(id)
        .with_subject_id(cat_kernel::EntityId::new())
}

#[test]
fn typed_event_conversion_preserves_contract_metadata() {
    let event = TestEvent { value: "hello".into() };
    let envelope = event.into_envelope("producer").unwrap();

    assert_eq!(envelope.event_type, TestEvent::TYPE);
    assert_eq!(envelope.version, TestEvent::VERSION);
    assert_eq!(envelope.kind, EventKind::Domain);
    assert_eq!(envelope.producer, "producer");
    assert_eq!(envelope.payload["value"], "hello");
}

#[test]
fn correlation_causation_and_subject_are_transport_independent() {
    let id = Uuid::now_v7();
    let envelope = envelope(id);

    assert_eq!(envelope.correlation_id, Some(id));
    assert_eq!(envelope.causation_id, Some(id));
    assert!(envelope.subject_id.is_some());
}

#[test]
fn registry_requires_exact_contract_version() {
    let bus = EventBus::new();
    let mut registry = EventRegistry::default();
    registry
        .register(EventContract::new(
            TestEvent::TYPE,
            1,
            "schema.test.contract.event.v1",
            Compatibility::Full,
        ))
        .unwrap();

    let accepted = bus.publish_registered(envelope(Uuid::now_v7()), &registry).unwrap();
    assert_eq!(accepted, PublishOutcome::Published { handlers_called: 0 });

    let mut wrong_version = envelope(Uuid::now_v7());
    wrong_version.version = 2;
    assert!(matches!(
        bus.publish_registered(wrong_version, &registry),
        Err(EventBusError::UnknownContract { .. })
    ));
}

#[test]
fn duplicate_publication_is_suppressed_after_success() {
    let bus = EventBus::new();
    let event = envelope(Uuid::now_v7());

    assert_eq!(bus.publish(event.clone()).unwrap(), PublishOutcome::Published { handlers_called: 0 });
    assert_eq!(bus.publish(event).unwrap(), PublishOutcome::DuplicateSuppressed);
}

#[test]
fn inbox_rejects_replay_and_allows_retry_after_failure() {
    let mut inbox = InMemoryInbox::default();
    let id = Uuid::now_v7();

    assert!(inbox.accept(id).unwrap());
    inbox.mark_failed(id).unwrap();
    assert!(inbox.accept(id).unwrap());
    inbox.mark_succeeded(id).unwrap();
    assert!(!inbox.accept(id).unwrap());
}

#[test]
fn idempotency_claim_complete_lifecycle_is_stable() {
    let mut store = InMemoryIdempotency::default();
    let id = Uuid::now_v7();

    assert!(store.claim(id).unwrap());
    assert!(!store.claim(id).unwrap());
    assert_eq!(store.state(id), Some(cat_eventbus::DeliveryState::InFlight));
    store.complete(id).unwrap();
    assert_eq!(store.state(id), Some(cat_eventbus::DeliveryState::Succeeded));
}

#[test]
fn outbox_enqueue_next_acknowledge_and_bounded_retry_are_public_contracts() {
    let mut outbox = InMemoryOutbox::default();
    let event = envelope(Uuid::now_v7());
    let id = event.event_id;
    let policy = RetryPolicy::new(3);

    outbox.enqueue(event.clone()).unwrap();
    assert_eq!(outbox.len(), 1);
    assert_eq!(outbox.next().unwrap().unwrap().event_id, id);
    assert_eq!(outbox.len(), 0);
    assert_eq!(outbox.fail(id, 1, &policy).unwrap(), cat_eventbus::DeliveryState::RetryScheduled);
    assert_eq!(outbox.fail(id, 3, &policy).unwrap(), cat_eventbus::DeliveryState::DeadLettered);
    outbox.acknowledge(id).unwrap();
}
