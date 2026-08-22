use std::sync::{Arc, Mutex};

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

    assert!(matches!(bus.publish(envelope.clone()).unwrap(), PublishOutcome::Published { .. }));
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
