use crate::{
    EventBusError, EventBusResult, EventEnvelope, EventTransport, TransportId, TransportRegistry,
};

/// Explicit routing policy for an event. The first matching route wins.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportRoute {
    pub event_type: Option<String>,
    pub producer_prefix: Option<String>,
    pub transport_id: TransportId,
}

impl TransportRoute {
    pub fn exact_event(event_type: impl Into<String>, transport_id: TransportId) -> Self {
        Self {
            event_type: Some(event_type.into()),
            producer_prefix: None,
            transport_id,
        }
    }

    pub fn producer_prefix(prefix: impl Into<String>, transport_id: TransportId) -> Self {
        Self {
            event_type: None,
            producer_prefix: Some(prefix.into()),
            transport_id,
        }
    }

    fn matches(&self, event: &EventEnvelope) -> bool {
        let event_match = self
            .event_type
            .as_ref()
            .map(|t| t == &event.event_type)
            .unwrap_or(true);
        let producer_match = self
            .producer_prefix
            .as_ref()
            .map(|p| event.producer.starts_with(p))
            .unwrap_or(true);
        event_match && producer_match
    }
}

/// Deterministic broker-neutral router. Routing is policy, not transport logic.
pub struct EventRouter<T> {
    pub registry: TransportRegistry<T>,
    routes: Vec<TransportRoute>,
}

impl<T> EventRouter<T> {
    pub fn new(registry: TransportRegistry<T>) -> Self {
        Self {
            registry,
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, route: TransportRoute) {
        self.routes.push(route);
    }

    pub fn route(&self, event: &EventEnvelope) -> EventBusResult<&TransportRoute> {
        self.routes
            .iter()
            .find(|route| route.matches(event))
            .ok_or_else(|| EventBusError::NoTransportRoute(event.event_type.clone()))
    }
}

impl<T: EventTransport> EventRouter<T> {
    pub fn publish(&mut self, event: &EventEnvelope) -> EventBusResult<TransportId> {
        let route = self.route(event)?.clone();
        let transport = self.registry.get_mut(&route.transport_id).ok_or_else(|| {
            EventBusError::TransportNotRegistered(route.transport_id.as_str().to_owned())
        })?;
        transport.publish(event)?;
        Ok(route.transport_id)
    }
}

/// Allows the router to sit directly behind `OutboxDispatcher` as its transport
/// boundary, keeping routing decisions out of durable persistence code.
impl<T: EventTransport> EventTransport for EventRouter<T> {
    fn publish(&mut self, event: &EventEnvelope) -> EventBusResult<()> {
        self.publish(event).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EndpointTransport, EventKind, RecordingTransport, TransportEndpoint, TransportId};
    use uuid::Uuid;

    fn event(event_type: &str, producer: &str) -> EventEnvelope {
        EventEnvelope {
            event_id: Uuid::now_v7(),
            event_type: event_type.into(),
            version: 1,
            kind: EventKind::Integration,
            occurred_at_ms: 1,
            producer: producer.into(),
            correlation_id: None,
            causation_id: None,
            subject_id: None,
            payload: serde_json::json!({"ok": true}),
        }
    }

    #[test]
    fn exact_event_route_wins_deterministically() {
        let kafka = TransportId::new("kafka").unwrap();
        let mut registry = TransportRegistry::new();
        registry
            .register(EndpointTransport::new(
                TransportEndpoint::new(kafka.clone(), "cat."),
                RecordingTransport::default(),
            ))
            .unwrap();
        let mut router = EventRouter::new(registry);
        router.add_route(TransportRoute::exact_event(
            "affiliate.conversion",
            kafka.clone(),
        ));
        assert_eq!(
            router
                .route(&event("affiliate.conversion", "affiliate"))
                .unwrap()
                .transport_id,
            kafka
        );
    }
}
