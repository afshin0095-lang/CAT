use thiserror::Error;

pub type EventBusResult<T> = Result<T, EventBusError>;

#[derive(Debug, Error)]
pub enum EventBusError {
    #[error("event serialization failed: {0}")]
    Serialization(String),

    #[error("event type '{0}' is already registered")]
    DuplicateEventType(String),

    #[error("event contract '{event_type}' v{version} conflicts with an existing registration")]
    ContractConflict { event_type: String, version: u16 },

    #[error("event contract '{event_type}' v{version} is not registered")]
    UnknownContract { event_type: String, version: u16 },

    #[error("event {0} is not known to the reliability store")]
    UnknownEvent(uuid::Uuid),

    #[error("subscription {0} is not registered")]
    UnknownSubscription(u64),

    #[error("handler failed for event '{event_type}': {message}")]
    HandlerFailure { event_type: String, message: String },
}
