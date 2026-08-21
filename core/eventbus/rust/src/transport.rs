use crate::{EventBusError, EventBusResult, EventEnvelope, EventTransport};
use std::collections::BTreeMap;

/// Stable transport identity used for routing and observability.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransportId(String);

impl TransportId {
    pub fn new(value: impl Into<String>) -> EventBusResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventBusError::InvalidConfiguration("transport id must not be empty".into()));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

/// Operational health reported by a transport endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportHealth {
    Healthy,
    Degraded,
    Unavailable,
}

/// Broker-neutral endpoint metadata. Concrete transports own connection details.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportEndpoint {
    pub id: TransportId,
    pub health: TransportHealth,
    pub subject_prefix: String,
}

impl TransportEndpoint {
    pub fn new(id: TransportId, subject_prefix: impl Into<String>) -> Self {
        Self { id, health: TransportHealth::Healthy, subject_prefix: subject_prefix.into() }
    }
}

/// A transport implementation that records endpoint metadata and delegates publication.
pub struct EndpointTransport<T> {
    pub endpoint: TransportEndpoint,
    pub inner: T,
}

impl<T> EndpointTransport<T> {
    pub fn new(endpoint: TransportEndpoint, inner: T) -> Self { Self { endpoint, inner } }
}

impl<T: EventTransport> EventTransport for EndpointTransport<T> {
    fn publish(&mut self, event: &EventEnvelope) -> EventBusResult<()> {
        if self.endpoint.health == TransportHealth::Unavailable {
            return Err(EventBusError::TransportUnavailable(self.endpoint.id.as_str().to_owned()));
        }
        self.inner.publish(event)
    }
}

/// In-process endpoint registry. It deliberately contains no broker-specific state.
pub struct TransportRegistry<T> {
    endpoints: BTreeMap<TransportId, EndpointTransport<T>>,
}

impl<T> Default for TransportRegistry<T> {
    fn default() -> Self { Self { endpoints: BTreeMap::new() } }
}

impl<T> TransportRegistry<T> {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, transport: EndpointTransport<T>) -> EventBusResult<()> {
        let id = transport.endpoint.id.clone();
        if self.endpoints.insert(id.clone(), transport).is_some() {
            return Err(EventBusError::InvalidConfiguration(format!("transport already registered: {}", id.as_str())));
        }
        Ok(())
    }

    pub fn get(&self, id: &TransportId) -> Option<&EndpointTransport<T>> { self.endpoints.get(id) }
    pub fn get_mut(&mut self, id: &TransportId) -> Option<&mut EndpointTransport<T>> { self.endpoints.get_mut(id) }
    pub fn len(&self) -> usize { self.endpoints.len() }
    pub fn is_empty(&self) -> bool { self.endpoints.is_empty() }
}
