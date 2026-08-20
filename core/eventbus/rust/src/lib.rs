#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod bus;
mod envelope;
mod error;

pub use bus::{EventBus, EventHandler, PublishOutcome, SubscriptionId};
pub use envelope::{EventEnvelope, EventKind};
pub use error::{EventBusError, EventBusResult};

/// Trait implemented by typed events crossing a CAT event boundary.
pub trait CatEvent: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static {
    const TYPE: &'static str;
    const VERSION: u16;

    fn into_envelope(self, producer: impl Into<String>) -> EventBusResult<EventEnvelope>
    where
        Self: Sized,
    {
        EventEnvelope::from_typed(self, producer)
    }
}
