use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use crate::{EventBusError, EventBusResult, EventEnvelope};

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
    processed_events: HashSet<uuid::Uuid>,
}

/// Deterministic in-memory EventBus used as the local development implementation.
///
/// Handler execution is serial and registration order is preserved. The transport
/// boundary is intentionally isolated behind this API so production deployments
/// can replace it with a durable broker without changing domain contracts.
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

    pub fn subscribe(&self, event_type: impl Into<String>, handler: EventHandler) -> EventBusResult<SubscriptionId> {
        let event_type = event_type.into();
        let mut state = self.state.write().expect("event bus lock poisoned");
        state.next_subscription_id += 1;
        let id = SubscriptionId(state.next_subscription_id);
        state.handlers.entry(event_type).or_default().push((id, handler));
        Ok(id)
    }

    pub fn unsubscribe(&self, subscription_id: SubscriptionId) -> EventBusResult<bool> {
        let mut state = self.state.write().expect("event bus lock poisoned");
        for handlers in state.handlers.values_mut() {
            if let Some(position) = handlers.iter().position(|(id, _)| *id == subscription_id) {
                handlers.remove(position);
                return Ok(true);
            }
        }
        Err(EventBusError::UnknownSubscription(subscription_id.value()))
    }

    pub fn publish(&self, envelope: EventEnvelope) -> EventBusResult<PublishOutcome> {
        let handlers = {
            let mut state = self.state.write().expect("event bus lock poisoned");
            if !state.processed_events.insert(envelope.event_id) {
                return Ok(PublishOutcome::DuplicateSuppressed);
            }
            state
                .handlers
                .get(&envelope.event_type)
                .map(|items| items.iter().map(|(_, handler)| Arc::clone(handler)).collect::<Vec<_>>())
                .unwrap_or_default()
        };

        let mut called = 0;
        for handler in handlers {
            handler(&envelope).map_err(|error| EventBusError::HandlerFailure {
                event_type: envelope.event_type.clone(),
                message: error.to_string(),
            })?;
            called += 1;
        }

        Ok(PublishOutcome::Published { handlers_called: called })
    }
}
