use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use crate::{EventBusError, EventBusResult, EventEnvelope, EventRegistry};

pub type EventHandler = Arc<dyn Fn(&EventEnvelope) -> EventBusResult<()> + Send + Sync>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SubscriptionId(u64);

impl SubscriptionId {
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublishOutcome {
    Published { handlers_called: usize },
    DuplicateSuppressed,
}

#[derive(Default)]
struct EventBusState {
    next_subscription_id: u64,
    handlers: HashMap<String, Vec<(SubscriptionId, EventHandler)>>,
    processing_events: HashSet<uuid::Uuid>,
    processed_events: HashSet<uuid::Uuid>,
}

/// Deterministic in-memory EventBus used as the local development implementation.
/// Handler execution preserves registration order. Processing is marked completed
/// only after every handler succeeds, so transient handler failures remain retryable.
pub struct EventBus {
    state: RwLock<EventBusState>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(EventBusState::default()),
        }
    }

    pub fn subscribe(
        &self,
        event_type: impl Into<String>,
        handler: EventHandler,
    ) -> EventBusResult<SubscriptionId> {
        let event_type = event_type.into();
        if event_type.trim().is_empty() {
            return Err(EventBusError::InvalidConfiguration(
                "event subscription type cannot be empty".into(),
            ));
        }
        let mut state = state_write(&self.state)?;
        state.next_subscription_id =
            state.next_subscription_id.checked_add(1).ok_or_else(|| {
                EventBusError::InvalidConfiguration("subscription id overflow".into())
            })?;
        let id = SubscriptionId(state.next_subscription_id);
        state
            .handlers
            .entry(event_type)
            .or_default()
            .push((id, handler));
        Ok(id)
    }

    pub fn unsubscribe(&self, subscription_id: SubscriptionId) -> EventBusResult<bool> {
        let mut state = state_write(&self.state)?;
        for handlers in state.handlers.values_mut() {
            if let Some(position) = handlers.iter().position(|(id, _)| *id == subscription_id) {
                handlers.remove(position);
                return Ok(true);
            }
        }
        Err(EventBusError::UnknownSubscription(subscription_id.value()))
    }

    pub fn publish_registered(
        &self,
        envelope: EventEnvelope,
        registry: &EventRegistry,
    ) -> EventBusResult<PublishOutcome> {
        registry.require(&envelope.event_type, envelope.version)?;
        self.publish(envelope)
    }

    pub fn publish(&self, envelope: EventEnvelope) -> EventBusResult<PublishOutcome> {
        if envelope.event_type.trim().is_empty() {
            return Err(EventBusError::InvalidConfiguration(
                "event type cannot be empty".into(),
            ));
        }

        let handlers = {
            let mut state = state_write(&self.state)?;
            if state.processed_events.contains(&envelope.event_id)
                || !state.processing_events.insert(envelope.event_id)
            {
                return Ok(PublishOutcome::DuplicateSuppressed);
            }
            state
                .handlers
                .get(&envelope.event_type)
                .map(|items| {
                    items
                        .iter()
                        .map(|(_, handler)| Arc::clone(handler))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        };

        let mut called = 0;
        for handler in handlers {
            if let Err(error) = handler(&envelope) {
                let mut state = state_write(&self.state)?;
                state.processing_events.remove(&envelope.event_id);
                return Err(EventBusError::HandlerFailure {
                    event_type: envelope.event_type.clone(),
                    message: error.to_string(),
                });
            }
            called += 1;
        }

        let mut state = state_write(&self.state)?;
        state.processing_events.remove(&envelope.event_id);
        state.processed_events.insert(envelope.event_id);
        Ok(PublishOutcome::Published {
            handlers_called: called,
        })
    }
}

fn state_write(
    lock: &RwLock<EventBusState>,
) -> EventBusResult<std::sync::RwLockWriteGuard<'_, EventBusState>> {
    lock.write()
        .map_err(|_| EventBusError::Storage("event bus state lock poisoned".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Compatibility, EventContract};

    fn envelope(event_id: uuid::Uuid, event_type: &str) -> EventEnvelope {
        EventEnvelope {
            event_id,
            event_type: event_type.into(),
            version: 1,
            kind: crate::EventKind::Domain,
            occurred_at_ms: 1,
            producer: "test".into(),
            correlation_id: None,
            causation_id: None,
            subject_id: None,
            payload: serde_json::json!({}),
        }
    }

    #[test]
    fn registered_publish_rejects_unknown_contract() {
        let bus = EventBus::new();
        let registry = EventRegistry::default();
        assert!(matches!(
            bus.publish_registered(envelope(uuid::Uuid::now_v7(), "missing.event"), &registry),
            Err(EventBusError::UnknownContract { .. })
        ));
    }

    #[test]
    fn registered_publish_accepts_exact_contract_version() {
        let bus = EventBus::new();
        let mut registry = EventRegistry::default();
        registry
            .register(EventContract::new(
                "test.event",
                1,
                "schema.test.event.v1",
                Compatibility::Full,
            ))
            .unwrap();
        assert_eq!(
            bus.publish_registered(envelope(uuid::Uuid::now_v7(), "test.event"), &registry)
                .unwrap(),
            PublishOutcome::Published { handlers_called: 0 }
        );
    }

    #[test]
    fn failed_handler_does_not_poison_event_for_retry() {
        let bus = EventBus::new();
        let attempts = Arc::new(RwLock::new(0usize));
        let attempts_for_handler = Arc::clone(&attempts);
        bus.subscribe(
            "retry.event",
            Arc::new(move |_| {
                let mut attempts = attempts_for_handler.write().unwrap();
                *attempts += 1;
                if *attempts == 1 {
                    return Err(EventBusError::Storage("transient failure".into()));
                }
                Ok(())
            }),
        )
        .unwrap();

        let event = envelope(uuid::Uuid::now_v7(), "retry.event");
        assert!(matches!(
            bus.publish(event.clone()),
            Err(EventBusError::HandlerFailure { .. })
        ));
        assert_eq!(
            bus.publish(event).unwrap(),
            PublishOutcome::Published { handlers_called: 1 }
        );
        assert_eq!(*attempts.read().unwrap(), 2);
    }
}
