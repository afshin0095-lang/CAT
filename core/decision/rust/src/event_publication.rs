use std::sync::Arc;

use cat_eventbus::{EventBus, EventBusError, EventBusResult, EventEnvelope, PublishOutcome};

/// Transaction-facing publication contract.
///
/// A producer persists canonical state first. The same transaction must record
/// the event in an outbox (or equivalent durable relay) before commit. Only
/// committed events are eligible for publication. This adapter then publishes
/// the already-committed envelope and keeps duplicate delivery idempotent.
///
/// This boundary deliberately does not execute decisions or mutate decision
/// state. It only transports immutable decision trace facts.
#[derive(Clone)]
pub struct DecisionTraceEventPublisher {
    bus: Arc<EventBus>,
}

impl DecisionTraceEventPublisher {
    pub fn new(bus: Arc<EventBus>) -> Self {
        Self { bus }
    }

    pub fn publish_committed(&self, envelope: EventEnvelope) -> EventBusResult<PublishOutcome> {
        if envelope.event_type != "cat.decision.trace.recorded" {
            return Err(EventBusError::InvalidConfiguration(
                "decision trace publisher accepts only cat.decision.trace.recorded events".into(),
            ));
        }

        self.bus.publish(envelope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cat_eventbus::EventKind;
    use cat_kernel::EntityId;
    use serde_json::json;

    fn envelope(event_type: &str) -> EventEnvelope {
        EventEnvelope {
            event_id: uuid::Uuid::now_v7(),
            event_type: event_type.into(),
            version: 1,
            kind: EventKind::Domain,
            occurred_at_ms: 1,
            producer: "cat-decision".into(),
            correlation_id: Some(uuid::Uuid::now_v7()),
            causation_id: None,
            subject_id: Some(EntityId::new()),
            payload: json!({"immutable": true}),
        }
    }

    #[test]
    fn rejects_non_trace_event_types() {
        let publisher = DecisionTraceEventPublisher::new(Arc::new(EventBus::new()));
        assert!(
            publisher
                .publish_committed(envelope("cat.decision.executed"))
                .is_err()
        );
    }

    #[test]
    fn duplicate_committed_delivery_is_suppressed() {
        let publisher = DecisionTraceEventPublisher::new(Arc::new(EventBus::new()));
        let event = envelope("cat.decision.trace.recorded");

        assert!(matches!(
            publisher.publish_committed(event.clone()).unwrap(),
            PublishOutcome::Published { .. }
        ));
        assert_eq!(
            publisher.publish_committed(event).unwrap(),
            PublishOutcome::DuplicateSuppressed
        );
    }
}
