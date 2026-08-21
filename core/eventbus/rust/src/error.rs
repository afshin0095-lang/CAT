use thiserror::Error;

pub type EventBusResult<T> = Result<T, EventBusError>;

#[derive(Debug, Error)]
pub enum EventBusError {
    #[error("event serialization failed: {0}")]
    Serialization(String),

    #[error("event storage operation failed: {0}")]
    Storage(String),

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

    #[error("transport '{0}' is not registered")]
    TransportNotRegistered(String),

    #[error("transport '{0}' is unavailable")]
    TransportUnavailable(String),

    #[error("no transport route exists for event type '{0}'")]
    NoTransportRoute(String),

    #[error("duplicate acknowledgement for event {0}")]
    DuplicateAcknowledgement(uuid::Uuid),

    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),
}
