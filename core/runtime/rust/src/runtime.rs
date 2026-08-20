use cat_eventbus::{EventBus, EventBusError, EventBusResult, EventEnvelope, EventHandler, EventKind, PublishOutcome};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeState {
    Created,
    Running,
    Draining,
    Stopped,
}

impl RuntimeState {
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Stopped)
    }
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("invalid runtime transition from {from:?} to {to:?}")]
    InvalidTransition { from: RuntimeState, to: RuntimeState },
    #[error("event bus error: {0}")]
    EventBus(#[from] EventBusError),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeStarted {
    pub runtime: String,
}

impl cat_eventbus::CatEvent for RuntimeStarted {
    const TYPE: &'static str = "cat.runtime.started";
    const VERSION: u16 = 1;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeDraining {
    pub runtime: String,
}

impl cat_eventbus::CatEvent for RuntimeDraining {
    const TYPE: &'static str = "cat.runtime.draining";
    const VERSION: u16 = 1;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeStopped {
    pub runtime: String,
}

impl cat_eventbus::CatEvent for RuntimeStopped {
    const TYPE: &'static str = "cat.runtime.stopped";
    const VERSION: u16 = 1;
}

pub struct Runtime {
    name: String,
    state: RuntimeState,
    bus: Arc<EventBus>,
}

impl Runtime {
    pub fn new(name: impl Into<String>, bus: Arc<EventBus>) -> Self {
        Self { name: name.into(), state: RuntimeState::Created, bus }
    }

    pub fn name(&self) -> &str { &self.name }

    pub const fn state(&self) -> RuntimeState { self.state }

    pub fn bus(&self) -> Arc<EventBus> { Arc::clone(&self.bus) }

    pub fn start(&mut self) -> RuntimeResult<PublishOutcome> {
        self.transition(RuntimeState::Running)?;
        let envelope = RuntimeStarted { runtime: self.name.clone() }.into_envelope(self.name.clone())?;
        Ok(self.bus.publish(envelope)?)
    }

    pub fn drain(&mut self) -> RuntimeResult<PublishOutcome> {
        self.transition(RuntimeState::Draining)?;
        let envelope = RuntimeDraining { runtime: self.name.clone() }.into_envelope(self.name.clone())?;
        Ok(self.bus.publish(envelope)?)
    }

    pub fn stop(&mut self) -> RuntimeResult<PublishOutcome> {
        self.transition(RuntimeState::Stopped)?;
        let envelope = RuntimeStopped { runtime: self.name.clone() }.into_envelope(self.name.clone())?;
        Ok(self.bus.publish(envelope)?)
    }

    pub fn subscribe(&self, event_type: impl Into<String>, handler: EventHandler) -> EventBusResult<cat_eventbus::SubscriptionId> {
        self.bus.subscribe(event_type, handler)
    }

    pub fn publish(&self, envelope: EventEnvelope) -> EventBusResult<PublishOutcome> {
        self.bus.publish(envelope)
    }

    fn transition(&mut self, next: RuntimeState) -> RuntimeResult<()> {
        let valid = matches!((self.state, next),
            (RuntimeState::Created, RuntimeState::Running) |
            (RuntimeState::Running, RuntimeState::Draining) |
            (RuntimeState::Running, RuntimeState::Stopped) |
            (RuntimeState::Draining, RuntimeState::Stopped)
        );
        if !valid {
            return Err(RuntimeError::InvalidTransition { from: self.state, to: next });
        }
        self.state = next;
        Ok(())
    }
}

pub fn lifecycle_event_type(kind: EventKind) -> &'static str {
    match kind {
        EventKind::Domain => "domain",
        EventKind::Integration => "integration",
    }
}
