use serde::{Deserialize, Serialize};

use crate::{CausationId, CorrelationId, EntityId, TenantId, TimestampMs};

/// Execution metadata propagated across CAT boundaries.
///
/// This context is transport-neutral: adapters may serialize it, but domain
/// modules receive the typed form and never infer identity from transport data.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub tenant_id: TenantId,
    pub correlation_id: CorrelationId,
    pub causation_id: Option<CausationId>,
    pub actor_id: EntityId,
    pub issued_at: TimestampMs,
}

impl ExecutionContext {
    pub const fn new(
        tenant_id: TenantId,
        correlation_id: CorrelationId,
        actor_id: EntityId,
        issued_at: TimestampMs,
    ) -> Self {
        Self {
            tenant_id,
            correlation_id,
            causation_id: None,
            actor_id,
            issued_at,
        }
    }

    pub const fn with_causation(mut self, causation_id: CausationId) -> Self {
        self.causation_id = Some(causation_id);
        self
    }
}
