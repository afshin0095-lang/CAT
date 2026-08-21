#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod ack;
mod bus;
mod delivery;
mod durable;
mod envelope;
mod error;
mod in_memory;
mod nats;
mod ports;
mod registry;
mod reliability;
mod router;
mod transport;
mod worker;

pub use ack::{AckDecision, RecordingAcker, TransportAcker};
pub use bus::{EventBus, EventHandler, PublishOutcome, SubscriptionId};
pub use delivery::{DeliveryOutcome, OutboxDispatcher, RecordingTransport};
pub use durable::{DeadLetter, DeadLetterStore, EventCodec, InMemoryDeadLetterStore, JsonEventCodec};
pub use envelope::{EventEnvelope, EventKind};
pub use error::{EventBusError, EventBusResult};
pub use in_memory::{InMemoryIdempotency, InMemoryInbox, InMemoryOutbox};
pub use nats::{subject_for_prefix, AsyncEventTransport, NatsJetStreamTransport};
pub use ports::{EventTransport, IdempotencyStore, InboxStore, OutboxStore};
pub use registry::{Compatibility, EventContract, EventRegistry};
pub use reliability::{DeliveryRecord, DeliveryState, RetryPolicy};
pub use router::{EventRouter, TransportRoute};
pub use transport::{EndpointTransport, TransportEndpoint, TransportHealth, TransportId, TransportRegistry};
pub use worker::{DeliveryWorker, DeliveryWorkerConfig};

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
