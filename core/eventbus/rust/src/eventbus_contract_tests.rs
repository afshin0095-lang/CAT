use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::{
    CatEvent, Compatibility, EventBus, EventBusError, EventContract, EventEnvelope, EventKind,
    EventRegistry, InMemoryIdempotency, InMemoryInbox, InMemoryOutbox, PublishOutcome, RetryPolicy,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AffiliateCreated { affiliate_id: String }

impl CatEvent for AffiliateCreated {
    const TYPE: &'static str = "affiliate.created";
    const VERSION: u16 = 1;
}

fn envelope(event_id: Uuid, event_type: &str, version: u16) -> EventEnvelope {
    EventEnvelope {
        event_id, event_type: event_type.to_owned(), version,
        kind: EventKind::Domain, occurred_at_ms: 1,
        producer: "integration-test".to_owned(), correlation_id: None,
        causation_id: None, subject_id: None, payload: serde_json::json!({"ok": true}),
    }
}

#[test]
fn typed_event_creates_stable_contract_metadata() {
    let envelope = AffiliateCreated { affiliate_id: "aff-1".into() }
        .into_envelope("affiliate-core").unwrap();
    assert_eq!(envelope.event_type, AffiliateCreated::TYPE);
    assert_eq!(envelope.version, AffiliateCreated::VERSION);
    assert_eq!(envelope.payload["affiliate_id"], "aff-1");
}

#[test]
fn envelope_preserves_context() {
    let correlation = Uuid::now_v7();
    let causation = Uuid::now_v7();
    let subject = cat_kernel::EntityId::new();
    let envelope = envelope(Uuid::now_v7(), "affiliate.created", 1)
        .with_kind(EventKind::Integration)
        .with_correlation_id(correlation)
        .with_causation_id(causation)
        .with_subject_id(subject);
    assert_eq!(envelope.kind, EventKind::Integration);
    assert_eq!(envelope.correlation_id, Some(correlation));
    assert_eq!(envelope.causation_id, Some(causation));
    assert_eq!(envelope.subject_id, Some(subject));
}

#[test]
fn registry_rejects_duplicate_and_unknown_versions() {
    let mut registry = EventRegistry::default();
    registry.register(EventContract::new("affiliate.created", 1, "affiliate.created.v1", Compatibility::Full)).unwrap();
    assert!(matches!(registry.register(EventContract::new("affiliate.created", 1, "duplicate", Compatibility::Breaking)), Err(EventBusError::ContractConflict { .. })));
    assert!(registry.require("affiliate.created", 1).is_ok());
    assert!(matches!(registry.require("affiliate.created", 2), Err(EventBusError::UnknownContract { .. })));
}

#[test]
fn event_bus_suppresses_successful_replay() {
    let bus = EventBus::new();
    let calls = Arc::new(Mutex::new(0usize));
    let calls_for_handler = Arc::clone(&calls);
    bus.subscribe("affiliate.created", Arc::new(move |_| {
        *calls_for_handler.lock().unwrap() += 1;
        Ok(())
    })).unwrap();
    let event = envelope(Uuid::now_v7(), "affiliate.created", 1);
    assert_eq!(bus.publish(event.clone()).unwrap(), PublishOutcome::Published { handlers_called: 1 });
    assert_eq!(bus.publish(event).unwrap(), PublishOutcome::DuplicateSuppressed);
    assert_eq!(*calls.lock().unwrap(), 1);
}

#[test]
fn retry_policy_is_bounded() {
    let policy = RetryPolicy::new(4, Duration::from_millis(100), Duration::from_millis(350));
    assert_eq!(policy.delay_for(1), Duration::from_millis(100));
    assert_eq!(policy.delay_for(2), Duration::from_millis(200));
    assert_eq!(policy.delay_for(3), Duration::from_millis(350));
    assert_eq!(policy.delay_for(20), Duration::from_millis(350));
    assert!(!policy.exhausted(3));
    assert!(policy.exhausted(4));
}

#[test]
fn idempotency_claims_once_and_completes() {
    let event_id = Uuid::now_v7();
    let mut store = InMemoryIdempotency::default();
    assert!(store.claim(event_id).unwrap());
    assert!(!store.claim(event_id).unwrap());
    store.complete(event_id).unwrap();
    assert_eq!(store.state(event_id), Some(crate::DeliveryState::Succeeded));
}

#[test]
fn inbox_rejects_replay() {
    let event_id = Uuid::now_v7();
    let mut inbox = InMemoryInbox::default();
    assert!(inbox.accept(event_id).unwrap());
    assert!(!inbox.accept(event_id).unwrap());
    inbox.mark_succeeded(event_id).unwrap();
}

#[test]
fn outbox_round_trip_preserves_identity() {
    let event = envelope(Uuid::now_v7(), "affiliate.created", 1);
    let event_id = event.event_id;
    let mut outbox = InMemoryOutbox::default();
    outbox.enqueue(event).unwrap();
    assert_eq!(outbox.next().unwrap().unwrap().event_id, event_id);
    outbox.acknowledge(event_id).unwrap();
    assert!(outbox.next().unwrap().is_none());
}

#[test]
fn correlation_and_causation_form_a_replayable_lineage() {
    let root = Uuid::now_v7();
    let child = envelope(Uuid::now_v7(), "affiliate.created", 1)
        .with_correlation_id(root)
        .with_causation_id(root);
    assert_eq!(child.correlation_id, Some(root));
    assert_eq!(child.causation_id, Some(root));
    assert_ne!(child.event_id, root);
}

#[test]
fn integration_events_remain_distinguishable_from_domain_events() {
    let event = envelope(Uuid::now_v7(), "affiliate.created", 1)
        .with_kind(EventKind::Integration);
    assert_eq!(event.kind, EventKind::Integration);
}
