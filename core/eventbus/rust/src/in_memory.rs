use std::collections::{HashMap, VecDeque};

use crate::{
    DeliveryState, EventEnvelope, EventBusError, EventBusResult, IdempotencyStore, InboxStore,
    OutboxStore, RetryPolicy,
};

#[derive(Debug, Default)]
pub struct InMemoryIdempotency {
    states: HashMap<uuid::Uuid, DeliveryState>,
}

impl IdempotencyStore for InMemoryIdempotency {
    fn claim(&mut self, event_id: uuid::Uuid) -> EventBusResult<bool> {
        if self.states.contains_key(&event_id) {
            return Ok(false);
        }
        self.states.insert(event_id, DeliveryState::InFlight);
        Ok(true)
    }

    fn complete(&mut self, event_id: uuid::Uuid) -> EventBusResult<()> {
        if !self.states.contains_key(&event_id) {
            return Err(EventBusError::UnknownEvent(event_id));
        }
        self.states.insert(event_id, DeliveryState::Succeeded);
        Ok(())
    }

    fn state(&self, event_id: uuid::Uuid) -> Option<DeliveryState> {
        self.states.get(&event_id).copied()
    }
}

#[derive(Debug, Default)]
pub struct InMemoryInbox {
    accepted: HashMap<uuid::Uuid, DeliveryState>,
}

impl InboxStore for InMemoryInbox {
    fn accept(&mut self, event_id: uuid::Uuid) -> EventBusResult<bool> {
        match self.accepted.get(&event_id).copied() {
            Some(DeliveryState::Succeeded) | Some(DeliveryState::InFlight) => Ok(false),
            Some(DeliveryState::RetryScheduled) | Some(DeliveryState::DeadLettered) => {
                self.accepted.insert(event_id, DeliveryState::InFlight);
                Ok(true)
            }
            None => {
                self.accepted.insert(event_id, DeliveryState::InFlight);
                Ok(true)
            }
        }
    }

    fn mark_succeeded(&mut self, event_id: uuid::Uuid) -> EventBusResult<()> {
        if !self.accepted.contains_key(&event_id) {
            return Err(EventBusError::UnknownEvent(event_id));
        }
        self.accepted.insert(event_id, DeliveryState::Succeeded);
        Ok(())
    }

    fn mark_failed(&mut self, event_id: uuid::Uuid) -> EventBusResult<()> {
        if !self.accepted.contains_key(&event_id) {
            return Err(EventBusError::UnknownEvent(event_id));
        }
        self.accepted.insert(event_id, DeliveryState::RetryScheduled);
        Ok(())
    }

    fn state(&self, event_id: uuid::Uuid) -> Option<DeliveryState> {
        self.accepted.get(&event_id).copied()
    }
}

#[derive(Debug, Default)]
pub struct InMemoryOutbox {
    pending: VecDeque<EventEnvelope>,
    attempts: HashMap<uuid::Uuid, u32>,
    dead_letters: Vec<EventEnvelope>,
}

impl InMemoryOutbox {
    pub fn dead_letters(&self) -> &[EventEnvelope] {
        &self.dead_letters
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }
}

impl OutboxStore for InMemoryOutbox {
    fn enqueue(&mut self, event: EventEnvelope) -> EventBusResult<()> {
        self.pending.push_back(event);
        Ok(())
    }

    fn next(&mut self) -> EventBusResult<Option<EventEnvelope>> {
        Ok(self.pending.pop_front())
    }

    fn acknowledge(&mut self, event_id: uuid::Uuid) -> EventBusResult<()> {
        self.attempts.remove(&event_id);
        Ok(())
    }

    fn fail(&mut self, event_id: uuid::Uuid, attempt: u32, policy: &RetryPolicy) -> EventBusResult<DeliveryState> {
        if policy.exhausted(attempt) {
            return Ok(DeliveryState::DeadLettered);
        }
        self.attempts.insert(event_id, attempt);
        Ok(DeliveryState::RetryScheduled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_scheduled_event_can_be_claimed_again() {
        let mut inbox = InMemoryInbox::default();
        let id = uuid::Uuid::now_v7();

        assert!(inbox.accept(id).unwrap());
        inbox.mark_failed(id).unwrap();
        assert_eq!(inbox.state(id), Some(DeliveryState::RetryScheduled));
        assert!(inbox.accept(id).unwrap());
    }

    #[test]
    fn succeeded_event_is_not_reprocessed() {
        let mut inbox = InMemoryInbox::default();
        let id = uuid::Uuid::now_v7();

        assert!(inbox.accept(id).unwrap());
        inbox.mark_succeeded(id).unwrap();
        assert!(!inbox.accept(id).unwrap());
    }
}
