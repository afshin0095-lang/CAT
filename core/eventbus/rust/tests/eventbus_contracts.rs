use cat_eventbus::{
    Compatibility, EventBus, EventBusError, EventContract, EventEnvelope, EventKind,
    EventRegistry, InMemoryIdempotency, InMemoryInbox, InMemoryOutbox, OutboxStore,
    PublishOutcome, RetryPolicy,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OrderCreated {
    order_id: String,
    amount_minor: i64,
}

impl cat_eventbus::CatEvent for OrderCreated {
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
    let event = OrderCreated { order_id: "ord-1".into(), amount_minor: 4200 };
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
    let mut registry = EventRegistry::default();
    registry
        .register(EventContract::new(
            "order.created",
            1,
            "schema.order.created.v1",
            Compatibility::Full,
        ))
        .unwrap();

    let id = Uuid::now_v7();
    assert!(bus.publish_registered(envelope(id, "order.created", 2), &registry).is_err());
    assert_eq!(
        bus.publish_registered(envelope(id, "order.created", 1), &registry).unwrap(),
        PublishOutcome::Published { handlers_called: 0 }
    );
}

#[test]
fn unsubscribe_removes_only_the_selected_subscription() {
    let bus = EventBus::new();
    let calls = Arc::new(Mutex::new(0u32));
    let first = Arc::clone(&calls);
    let second = Arc::clone(&calls);

    let first_id = bus.subscribe("order.created", Arc::new(move |_| {
        *first.lock().unwrap() += 1;
        Ok(())
    })).unwrap();
    bus.subscribe("order.created", Arc::new(move |_| {
        *second.lock().unwrap() += 10;
        Ok(())
    })).unwrap();

    assert!(bus.unsubscribe(first_id).unwrap());
    assert_eq!(bus.publish(envelope(Uuid::now_v7(), "order.created", 1)).unwrap(),
        PublishOutcome::Published { handlers_called: 1 });
    assert_eq!(*calls.lock().unwrap(), 10);
    assert!(matches!(bus.unsubscribe(first_id), Err(EventBusError::UnknownSubscription(_))));
}

#[test]
fn inbox_and_idempotency_boundaries_suppress_replay() {
    let id = Uuid::now_v7();
    let mut inbox = InMemoryInbox::default();
    assert!(inbox.accept(id).unwrap());
    assert!(!inbox.accept(id).unwrap());
    inbox.mark_succeeded(id).unwrap();

    let mut idempotency = InMemoryIdempotency::default();
    assert!(idempotency.claim(id).unwrap());
    assert!(!idempotency.claim(id).unwrap());
    idempotency.complete(id).unwrap();
}

#[test]
fn outbox_retry_policy_transitions_to_dead_letter() {
    let mut outbox = InMemoryOutbox::default();
    let id = Uuid::now_v7();
    outbox.enqueue(envelope(id, "order.created", 1)).unwrap();
    assert_eq!(outbox.len(), 1);

    let policy = RetryPolicy::new(2, 0);
    assert!(matches!(outbox.fail(id, 1, &policy).unwrap(), cat_eventbus::DeliveryState::RetryScheduled));
    assert!(matches!(outbox.fail(id, 2, &policy).unwrap(), cat_eventbus::DeliveryState::DeadLettered));
}
