use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use cat_eventbus::{CatEvent, EventBus, EventEnvelope, EventKind, PublishOutcome};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct UserRegistered {
    user_id: Uuid,
}

impl CatEvent for UserRegistered {
    const TYPE: &'static str = "identity.user_registered";
    const VERSION: u16 = 1;
}

#[test]
fn typed_event_becomes_versioned_domain_envelope() {
    let user_id = Uuid::now_v7();
    let envelope = UserRegistered { user_id }
        .into_envelope("identity-core")
        .unwrap();

    assert_eq!(envelope.event_type, "identity.user_registered");
    assert_eq!(envelope.version, 1);
    assert_eq!(envelope.kind, EventKind::Domain);
    assert_eq!(envelope.producer, "identity-core");
    assert_eq!(envelope.subject_id, None);
    assert_eq!(envelope.payload["user_id"], user_id.to_string());
}

#[test]
fn envelope_preserves_correlation_causation_and_subject_context() {
    let event_id = Uuid::now_v7();
    let correlation_id = Uuid::now_v7();
    let causation_id = Uuid::now_v7();
    let subject_id = cat_kernel::EntityId::new();

    let envelope = EventEnvelope {
        event_id,
        event_type: "affiliate.conversion_recorded".into(),
        version: 2,
        kind: EventKind::Domain,
        occurred_at_ms: 1_000,
        producer: "affiliate-core".into(),
        correlation_id: None,
        causation_id: None,
        subject_id: None,
        payload: serde_json::json!({"amount": 1250}),
    }
    .with_kind(EventKind::Integration)
    .with_correlation_id(correlation_id)
    .with_causation_id(causation_id)
    .with_subject_id(subject_id);

    assert_eq!(envelope.event_id, event_id);
    assert_eq!(envelope.kind, EventKind::Integration);
    assert_eq!(envelope.correlation_id, Some(correlation_id));
    assert_eq!(envelope.causation_id, Some(causation_id));
    assert_eq!(envelope.subject_id, Some(subject_id));
}

#[test]
fn bus_dispatches_in_registration_order() {
    let bus = EventBus::new();
    let seen = Arc::new(Mutex::new(Vec::new()));

    for marker in ["first", "second", "third"] {
        let seen = Arc::clone(&seen);
        bus.subscribe(
            "test.event",
            Arc::new(move |_| {
                seen.lock().unwrap().push(marker);
                Ok(())
            }),
        )
        .unwrap();
    }

    let envelope = EventEnvelope {
        event_id: Uuid::now_v7(),
        event_type: "test.event".into(),
        version: 1,
        kind: EventKind::Domain,
        occurred_at_ms: 1,
        producer: "test".into(),
        correlation_id: None,
        causation_id: None,
        subject_id: None,
        payload: serde_json::json!({}),
    };

    assert_eq!(
        bus.publish(envelope).unwrap(),
        PublishOutcome::Published { handlers_called: 3 }
    );
    assert_eq!(*seen.lock().unwrap(), vec!["first", "second", "third"]);
}

#[test]
fn bus_suppresses_duplicate_event_delivery() {
    let bus = EventBus::new();
    let calls = Arc::new(Mutex::new(0usize));
    let calls_for_handler = Arc::clone(&calls);

    bus.subscribe(
        "test.duplicate",
        Arc::new(move |_| {
            *calls_for_handler.lock().unwrap() += 1;
            Ok(())
        }),
    )
    .unwrap();

    let event_id = Uuid::now_v7();
    let envelope = EventEnvelope {
        event_id,
        event_type: "test.duplicate".into(),
        version: 1,
        kind: EventKind::Integration,
        occurred_at_ms: 1,
        producer: "test".into(),
        correlation_id: None,
        causation_id: None,
        subject_id: None,
        payload: serde_json::json!({"event_id": event_id}),
    };

    assert!(matches!(
        bus.publish(envelope.clone()).unwrap(),
        PublishOutcome::Published { .. }
    ));
    assert_eq!(
        bus.publish(envelope).unwrap(),
        PublishOutcome::DuplicateSuppressed
    );
    assert_eq!(*calls.lock().unwrap(), 1);
}

#[test]
fn unsubscribe_removes_only_the_selected_subscription() {
    let bus = EventBus::new();
    let calls = Arc::new(Mutex::new(Vec::new()));

    let first_calls = Arc::clone(&calls);
    let first = bus
        .subscribe(
            "test.unsubscribe",
            Arc::new(move |_| {
                first_calls.lock().unwrap().push("first");
                Ok(())
            }),
        )
        .unwrap();

    let second_calls = Arc::clone(&calls);
    bus.subscribe(
        "test.unsubscribe",
        Arc::new(move |_| {
            second_calls.lock().unwrap().push("second");
            Ok(())
        }),
    )
    .unwrap();

    assert!(bus.unsubscribe(first).unwrap());
    assert!(bus.unsubscribe(first).is_err());

    bus.publish(EventEnvelope {
        event_id: Uuid::now_v7(),
        event_type: "test.unsubscribe".into(),
        version: 1,
        kind: EventKind::Domain,
        occurred_at_ms: 1,
        producer: "test".into(),
        correlation_id: None,
        causation_id: None,
        subject_id: None,
        payload: serde_json::json!({}),
    })
    .unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["second"]);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OrderCreated {
    order_id: String,
    amount_minor: i64,
}

impl CatEvent for OrderCreated {
    const TYPE: &'static str = "order.created";
    const VERSION: u16 = 1;
}

fn envelope(event_id: Uuid, event_type: &str, version: u16) -> EventEnvelope {
    EventEnvelope {
        event_id,
        event_type: event_type.to_owned(),
        version,
        kind: EventKind::Domain,
        occurred_at_ms: 1,
        producer: "integration-test".into(),
        correlation_id: None,
        causation_id: None,
        subject_id: None,
        payload: serde_json::json!({"ok": true}),
    }
}

#[test]
fn typed_event_builds_stable_envelope() {
    let event = OrderCreated {
        order_id: "ord-1".into(),
        amount_minor: 4200,
    };
    let envelope = event.into_envelope("orders").unwrap();

    assert_eq!(envelope.event_type, "order.created");
    assert_eq!(envelope.version, 1);
    assert_eq!(envelope.producer, "orders");
    assert_eq!(envelope.payload["order_id"], "ord-1");
}

#[test]
fn event_bus_is_idempotent_and_preserves_handler_order() {
    let bus = EventBus::new();
    let calls = Arc::new(Mutex::new(Vec::new()));

    for marker in ["first", "second"] {
        let calls = Arc::clone(&calls);
        bus.subscribe(
            "order.created",
            Arc::new(move |_| {
                calls.lock().unwrap().push(marker);
                Ok(())
            }),
        )
        .unwrap();
    }

    let id = Uuid::now_v7();
    assert_eq!(
        bus.publish(envelope(id, "order.created", 1)).unwrap(),
        PublishOutcome::Published { handlers_called: 2 }
    );
    assert_eq!(
        bus.publish(envelope(id, "order.created", 1)).unwrap(),
        PublishOutcome::DuplicateSuppressed
    );
    assert_eq!(*calls.lock().unwrap(), vec!["first", "second"]);
}

#[test]
fn registered_publish_requires_exact_contract_version() {
    let bus = EventBus::new();
    let mut registry = cat_eventbus::EventRegistry::default();
    registry
        .register(cat_eventbus::EventContract::new(
            "order.created",
            1,
            "schema.order.created.v1",
            cat_eventbus::Compatibility::Full,
        ))
        .unwrap();

    let id = Uuid::now_v7();
    assert!(
        bus.publish_registered(envelope(id, "order.created", 2), &registry)
            .is_err()
    );
    assert_eq!(
        bus.publish_registered(envelope(id, "order.created", 1), &registry)
            .unwrap(),
        PublishOutcome::Published { handlers_called: 0 }
    );
}

#[test]
fn inbox_and_idempotency_boundaries_suppress_replay() {
    let id = Uuid::now_v7();
    let mut inbox = cat_eventbus::InMemoryInbox::default();
    assert!(cat_eventbus::InboxStore::accept(&mut inbox, id).unwrap());
    assert!(!cat_eventbus::InboxStore::accept(&mut inbox, id).unwrap());
    cat_eventbus::InboxStore::mark_succeeded(&mut inbox, id).unwrap();

    let mut idempotency = cat_eventbus::InMemoryIdempotency::default();
    assert!(cat_eventbus::IdempotencyStore::claim(&mut idempotency, id).unwrap());
    assert!(!cat_eventbus::IdempotencyStore::claim(&mut idempotency, id).unwrap());
    cat_eventbus::IdempotencyStore::complete(&mut idempotency, id).unwrap();
}

#[test]
fn outbox_retry_policy_transitions_to_dead_letter() {
    let mut outbox = cat_eventbus::InMemoryOutbox::default();
    let id = Uuid::now_v7();
    cat_eventbus::OutboxStore::enqueue(&mut outbox, envelope(id, "order.created", 1)).unwrap();
    assert_eq!(outbox.len(), 1);

    let policy = cat_eventbus::RetryPolicy::new(2, Duration::ZERO, Duration::ZERO);
    assert!(matches!(
        cat_eventbus::OutboxStore::fail(&mut outbox, id, 1, &policy).unwrap(),
        cat_eventbus::DeliveryState::RetryScheduled
    ));
    assert!(matches!(
        cat_eventbus::OutboxStore::fail(&mut outbox, id, 2, &policy).unwrap(),
        cat_eventbus::DeliveryState::DeadLettered
    ));
}

#[test]
fn envelope_json_round_trip_preserves_wire_contract() {
    let correlation_id = Uuid::now_v7();
    let causation_id = Uuid::now_v7();
    let subject_id = cat_kernel::EntityId::new();
    let original = envelope(Uuid::now_v7(), "affiliate.conversion_recorded", 3)
        .with_kind(EventKind::Integration)
        .with_correlation_id(correlation_id)
        .with_causation_id(causation_id)
        .with_subject_id(subject_id);

    let encoded = serde_json::to_vec(&original).unwrap();
    let decoded: EventEnvelope = serde_json::from_slice(&encoded).unwrap();

    assert_eq!(decoded.event_id, original.event_id);
    assert_eq!(decoded.event_type, original.event_type);
    assert_eq!(decoded.version, original.version);
    assert_eq!(decoded.kind, original.kind);
    assert_eq!(decoded.occurred_at_ms, original.occurred_at_ms);
    assert_eq!(decoded.producer, original.producer);
    assert_eq!(decoded.correlation_id, Some(correlation_id));
    assert_eq!(decoded.causation_id, Some(causation_id));
    assert_eq!(decoded.subject_id, Some(subject_id));
    assert_eq!(decoded.payload, original.payload);
}

#[test]
fn envelope_json_uses_stable_snake_case_event_kind() {
    let encoded = serde_json::to_value(EventEnvelope {
        event_id: Uuid::now_v7(),
        event_type: "test.kind".into(),
        version: 1,
        kind: EventKind::Integration,
        occurred_at_ms: 7,
        producer: "test".into(),
        correlation_id: None,
        causation_id: None,
        subject_id: None,
        payload: serde_json::json!({"value": true}),
    })
    .unwrap();

    assert_eq!(encoded["kind"], "integration");
    assert_eq!(encoded["event_type"], "test.kind");
    assert_eq!(encoded["version"], 1);
    assert_eq!(encoded["payload"]["value"], true);
}
