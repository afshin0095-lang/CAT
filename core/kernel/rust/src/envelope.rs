use serde::{Deserialize, Serialize};

use crate::{
    CausationId, CorrelationId, EntityId, EventId, KernelResult, SequenceNumber, TenantId,
    TimestampMs,
};

/// Immutable event envelope shared by kernel and domain runtimes.
///
/// The envelope carries execution metadata and sequencing information while
/// leaving business meaning entirely to the payload type.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventEnvelope<T> {
    pub event_id: EventId,
    pub event_type: String,
    pub event_version: u16,
    pub tenant_id: TenantId,
    pub correlation_id: CorrelationId,
    pub causation_id: Option<CausationId>,
    pub actor_id: EntityId,
    pub occurred_at: TimestampMs,
    pub sequence: SequenceNumber,
    pub payload: T,
}

impl<T> EventEnvelope<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        event_type: impl Into<String>,
        event_version: u16,
        tenant_id: TenantId,
        correlation_id: CorrelationId,
        causation_id: Option<CausationId>,
        actor_id: EntityId,
        occurred_at: TimestampMs,
        sequence: SequenceNumber,
        payload: T,
    ) -> KernelResult<Self> {
        let event_type = event_type.into();
        if event_type.trim().is_empty() {
            return Err(crate::KernelError::InvalidIdentifier(
                "event type must not be empty".to_owned(),
            ));
        }
        if event_version == 0 {
            return Err(crate::KernelError::InvalidIdentifier(
                "event version must be greater than zero".to_owned(),
            ));
        }
        Ok(Self {
            event_id: EventId::new(),
            event_type,
            event_version,
            tenant_id,
            correlation_id,
            causation_id,
            actor_id,
            occurred_at,
            sequence,
            payload,
        })
    }
}
