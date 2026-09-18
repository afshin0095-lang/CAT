#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod ack;
mod async_contract_tests;
mod async_delivery;
mod async_delivery_tests;
mod async_in_memory;
mod async_in_memory_outbox;
mod async_inbox;
#[cfg(test)]
mod async_inbox_diagnostics_tests;
mod async_inbox_metrics;
#[cfg(test)]
mod async_inbox_metrics_tests;
mod async_inbox_worker;
mod async_nats_consumer;
mod async_outbox;
#[cfg(test)]
mod async_outbox_diagnostics_tests;
mod async_outbox_metrics;
mod async_tokio_in_memory_outbox;
mod async_worker;
mod bus;
mod causality;
mod delivery;
mod durable;
mod envelope;
mod error;
mod event_id;
mod in_memory;
#[cfg(test)]
mod inbox_contract_tests;
mod inbox_health;
mod inbox_metrics;
mod metrics;
mod nats;
mod nats_consumer;
mod nats_stream;
mod ports;
mod processor;
mod registry;
mod reliability;
mod router;
mod transport;
mod worker;

#[cfg(test)]
mod eventbus_contract_tests;

pub use ack::{AckDecision, RecordingAcker, TransportAcker};
pub use async_delivery::{AsyncDeliveryOutcome, AsyncOutboxDispatcher, RecordingAsyncTransport};
pub use async_in_memory::AsyncInMemoryInbox;
pub use async_in_memory_outbox::AsyncInMemoryOutbox;
pub use async_inbox::AsyncInboxStore;
pub use async_inbox_metrics::{
    AsyncInboxDiagnostics, MetricsAsyncInbox, MetricsAsyncInboxSnapshot,
};
pub use async_inbox_worker::{AsyncInboxDeliveryOutcome, AsyncInboxDeliveryWorker};
pub use async_nats_consumer::AsyncNatsJetStreamConsumer;
pub use async_outbox::AsyncOutboxStore;
pub use async_outbox_metrics::{
    AsyncOutboxDiagnostics, MetricsAsyncOutbox, MetricsAsyncOutboxSnapshot,
};
pub use async_tokio_in_memory_outbox::AsyncTokioInMemoryOutbox;
pub use async_worker::{AsyncDeliveryWorker, AsyncDeliveryWorkerConfig};
pub use bus::{EventBus, EventHandler, PublishOutcome, SubscriptionId};
pub use causality::EventCausality;
pub use delivery::{DeliveryOutcome, OutboxDispatcher, RecordingTransport};
pub use durable::{
    DeadLetter, DeadLetterStore, EventCodec, InMemoryDeadLetterStore, JsonEventCodec,
};
pub use envelope::{EventEnvelope, EventKind};
pub use error::{EventBusError, EventBusResult};
pub use event_id::EventId;
pub use in_memory::{InMemoryIdempotency, InMemoryInbox, InMemoryOutbox};
pub use inbox_health::{InboxHealthDiagnostics, InboxHealthSnapshot, MetricsInboxHealth};
pub use inbox_metrics::{InboxDiagnostics, MetricsInbox, MetricsInboxSnapshot};
pub use metrics::{EventBusMetrics, EventBusMetricsSnapshot};
pub use nats::{AsyncEventTransport, NatsJetStreamTransport, subject_for_prefix};
pub use nats_consumer::{AsyncEventHandler, NatsConsumerConfig, NatsJetStreamConsumer};
pub use nats_stream::{NatsStreamConfig, ensure_stream};
pub use ports::{EventTransport, IdempotencyStore, InboxStore, OutboxStore};
pub use processor::{DeliveryProcessor, ProcessingDecision};
pub use registry::{Compatibility, EventContract, EventRegistry};
pub use reliability::{DeliveryRecord, DeliveryState, RetryPolicy};
pub use router::{EventRouter, TransportRoute};
pub use transport::{
    EndpointTransport, TransportEndpoint, TransportHealth, TransportId, TransportRegistry,
};
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
