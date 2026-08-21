use cat_eventbus::{
    AckDecision, EndpointTransport, EventEnvelope, EventKind, EventRouter, EventTransport,
    RecordingAcker, RecordingTransport, TransportAcker, TransportEndpoint, TransportHealth,
    TransportId, TransportRegistry, TransportRoute,
};
use uuid::Uuid;

fn envelope(event_type: &str, producer: &str) -> EventEnvelope {
    EventEnvelope {
        event_id: Uuid::now_v7(),
        event_type: event_type.to_owned(),
        version: 1,
        kind: EventKind::Integration,
        occurred_at_ms: 1,
        producer: producer.to_owned(),
        correlation_id: None,
        causation_id: None,
        subject_id: None,
        payload: serde_json::json!({"value": 1}),
    }
}

#[test]
fn router_publishes_to_selected_endpoint() {
    let id = TransportId::new("recording").unwrap();
    let mut registry = TransportRegistry::new();
    registry
        .register(EndpointTransport::new(
            TransportEndpoint::new(id.clone(), "cat."),
            RecordingTransport::default(),
        ))
        .unwrap();

    let mut router = EventRouter::new(registry);
    router.add_route(TransportRoute::exact_event("affiliate.conversion", id.clone()));

    let event = envelope("affiliate.conversion", "affiliate-engine");
    assert_eq!(router.publish(&event).unwrap(), id);
}

#[test]
fn unavailable_endpoint_is_rejected_before_network_call() {
    let id = TransportId::new("unavailable").unwrap();
    let mut endpoint = TransportEndpoint::new(id, "cat.");
    endpoint.health = TransportHealth::Unavailable;
    let mut transport = EndpointTransport::new(endpoint, RecordingTransport::default());

    let result = transport.publish(&envelope("test.event", "test"));
    assert!(result.is_err());
}

#[test]
fn acknowledgement_is_explicit_and_idempotency_sensitive() {
    let event_id = Uuid::now_v7();
    let mut acker = RecordingAcker::default();

    assert_eq!(AckDecision::Ack, AckDecision::Ack);
    acker.ack(event_id).unwrap();
    assert!(acker.ack(event_id).is_err());
    acker.retry(event_id).unwrap();
    acker.dead_letter(event_id).unwrap();

    assert_eq!(acker.acked, vec![event_id]);
    assert_eq!(acker.retried, vec![event_id]);
    assert_eq!(acker.dead_lettered, vec![event_id]);
}
