#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod bus;
mod durable;
mod envelope;
mod error;
mod in_memory;
mod ports;
mod registry;
mod reliability;

pub use bus::{EventBus, EventHandler, PublishOutcome, SubscriptionId};
pub use durable::{DeadLetter, DeadLetterStore, EventCodec, InMemoryDeadLetterStore, JsonEventCodec};
pub use envelope::{EventEnvelope, EventKind};
pub use error::{EventBusError, EventBusResult};
pub use in_memory::{InMemoryIdempotency, InMemoryInbox, InMemoryOutbox};
pub use ports::{EventTransport, IdempotencyStore, InboxStore, OutboxStore};
pub use registry::{Compatibility, EventContract, EventRegistry};
pub use reliability::{DeliveryRecord, DeliveryState, RetryPolicy};

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
