use cat_eventbus::{EndpointTransport, EventEnvelope, EventKind, EventRouter, RecordingTransport, TransportEndpoint, TransportId, TransportRegistry, TransportRoute};
use uuid::Uuid;

#[test]
fn routed_transport_smoke() {
    let id = TransportId::new("smoke").unwrap();
    let mut registry = TransportRegistry::new();
    registry.register(EndpointTransport::new(TransportEndpoint::new(id.clone(), "cat."), RecordingTransport::default())).unwrap();
    let mut router = EventRouter::new(registry);
    router.add_route(TransportRoute::exact_event("cat.smoke", id.clone()));
    let event = EventEnvelope { event_id: Uuid::now_v7(), event_type: "cat.smoke".into(), version: 1, kind: EventKind::Integration, occurred_at_ms: 1, producer: "test".into(), correlation_id: None, causation_id: None, subject_id: None, payload: serde_json::json!({}) };
    assert_eq!(router.publish(&event).unwrap(), id);
}
