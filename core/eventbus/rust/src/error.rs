use thiserror::Error;

pub type EventBusResult<T> = Result<T, EventBusError>;

#[derive(Debug, Error)]
pub enum EventBusError {
    #[error("event serialization failed: {0}")]
    Serialization(String),

    #[error("event type '{0}' is already registered")]
    DuplicateEventType(String),

    #[error("subscription {0} is not registered")]
    UnknownSubscription(u64),

    #[error("handler failed for event '{event_type}': {message}")]
    HandlerFailure { event_type: String, message: String },
}
