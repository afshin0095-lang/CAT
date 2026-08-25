use crate::EventEnvelope;

#[derive(Clone, Debug, Default)]
pub struct ReplayBuffer {
    events: Vec<EventEnvelope>,
}

impl ReplayBuffer {
    pub fn push(&mut self, event: EventEnvelope) {
        self.events.push(event);
    }

    pub fn len(&self) -> usize { self.events.len() }

    pub fn is_empty(&self) -> bool { self.events.is_empty() }

    pub fn iter(&self) -> impl Iterator<Item = &EventEnvelope> {
        self.events.iter()
    }

    pub fn since(&self, event_id: uuid::Uuid) -> impl Iterator<Item = &EventEnvelope> {
        let index = self.events.iter().position(|event| event.event_id == event_id).map(|i| i + 1).unwrap_or(0);
        self.events[index..].iter()
    }
}
