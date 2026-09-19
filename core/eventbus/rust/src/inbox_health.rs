use crate::{DeliveryState, EventBusMetricsSnapshot, EventBusResult, InboxStore, MetricsInbox};

/// A read-only health and delivery view over an inbox implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboxHealthSnapshot {
    pub event_id: uuid::Uuid,
    pub state: Option<DeliveryState>,
    pub duplicate_suppressed: bool,
    pub terminal: bool,
}

/// Diagnostic adapter that makes consumer-side idempotency observable without
/// coupling callers to a concrete inbox implementation.
#[derive(Debug)]
pub struct InboxHealthDiagnostics<'a, S: InboxStore> {
    store: &'a S,
}

impl<'a, S: InboxStore> InboxHealthDiagnostics<'a, S> {
    pub fn new(store: &'a S) -> Self {
        Self { store }
    }

    pub fn inspect(&self, event_id: uuid::Uuid) -> InboxHealthSnapshot {
        let state = self.store.state(event_id);
        InboxHealthSnapshot {
            event_id,
            state,
            duplicate_suppressed: matches!(
                state,
                Some(DeliveryState::Succeeded) | Some(DeliveryState::InFlight)
            ),
            terminal: matches!(
                state,
                Some(DeliveryState::Succeeded) | Some(DeliveryState::DeadLettered)
            ),
        }
    }
}

/// Convenience adapter for the standard metrics-decorated inbox.
pub struct MetricsInboxHealth<'a, S: InboxStore> {
    inbox: &'a MetricsInbox<S>,
}

impl<'a, S: InboxStore> MetricsInboxHealth<'a, S> {
    pub fn new(inbox: &'a MetricsInbox<S>) -> Self {
        Self { inbox }
    }

    pub fn metrics(&self) -> EventBusMetricsSnapshot {
        self.inbox.metrics().snapshot()
    }

    pub fn inspect(&self, event_id: uuid::Uuid) -> EventBusResult<InboxHealthSnapshot> {
        Ok(InboxHealthSnapshot {
            event_id,
            state: self.inbox.state(event_id),
            duplicate_suppressed: matches!(
                self.inbox.state(event_id),
                Some(DeliveryState::Succeeded) | Some(DeliveryState::InFlight)
            ),
            terminal: matches!(
                self.inbox.state(event_id),
                Some(DeliveryState::Succeeded) | Some(DeliveryState::DeadLettered)
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{InMemoryInbox, InboxStore};

    #[test]
    fn exposes_retry_state_without_mutation() {
        let mut inbox = InMemoryInbox::default();
        let id = uuid::Uuid::now_v7();
        inbox.accept(id).unwrap();
        inbox.mark_failed(id).unwrap();
        let snapshot = InboxHealthDiagnostics::new(&inbox).inspect(id);
        assert_eq!(snapshot.state, Some(DeliveryState::RetryScheduled));
        assert!(!snapshot.duplicate_suppressed);
        assert!(!snapshot.terminal);
    }

    #[test]
    fn succeeded_event_is_terminal_and_duplicate_suppressed() {
        let mut inbox = InMemoryInbox::default();
        let id = uuid::Uuid::now_v7();
        inbox.accept(id).unwrap();
        inbox.mark_succeeded(id).unwrap();
        let snapshot = InboxHealthDiagnostics::new(&inbox).inspect(id);
        assert!(snapshot.duplicate_suppressed);
        assert!(snapshot.terminal);
    }
}
