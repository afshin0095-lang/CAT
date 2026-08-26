use uuid::Uuid;

/// Correlation context propagated across related CAT events.
///
/// A correlation id identifies one logical workflow, while causation identifies the
/// immediate event that caused the current event to be emitted.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EventCausality {
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
}

impl EventCausality {
    pub const fn root(correlation_id: Uuid) -> Self {
        Self {
            correlation_id: Some(correlation_id),
            causation_id: None,
        }
    }

    pub const fn caused_by(correlation_id: Uuid, causation_id: Uuid) -> Self {
        Self {
            correlation_id: Some(correlation_id),
            causation_id: Some(causation_id),
        }
    }

    pub fn apply(self, envelope: crate::EventEnvelope) -> crate::EventEnvelope {
        let envelope = match self.correlation_id {
            Some(id) => envelope.with_correlation_id(id),
            None => envelope,
        };
        match self.causation_id {
            Some(id) => envelope.with_causation_id(id),
            None => envelope,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CatEvent, EventCausality};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    struct TestEvent;

    impl CatEvent for TestEvent {
        const TYPE: &'static str = "cat.eventbus.causality.test";
        const VERSION: u16 = 1;
    }

    #[test]
    fn causality_is_applied_without_replacing_event_identity() {
        let correlation = Uuid::now_v7();
        let cause = Uuid::now_v7();
        let event = TestEvent.into_envelope("test").expect("serialize event");
        let event_id = event.event_id;
        let event = EventCausality::caused_by(correlation, cause).apply(event);

        assert_eq!(event.event_id, event_id);
        assert_eq!(event.correlation_id, Some(correlation));
        assert_eq!(event.causation_id, Some(cause));
    }
}
