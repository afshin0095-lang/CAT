use serde::{Deserialize, Serialize};

use crate::{
    AgentId, ContractVersion, EntityId, ExecutionContext, IdempotencyKey, InvocationStatus,
    KernelError, KernelResult, OutcomeStatus, SideEffectClass, TimestampMs,
};

/// Stable identity of one logical agent invocation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InvocationId(EntityId);

impl InvocationId {
    pub fn new() -> Self {
        Self(EntityId::new())
    }

    pub const fn from_entity_id(value: EntityId) -> Self {
        Self(value)
    }

    pub const fn as_entity_id(self) -> EntityId {
        self.0
    }
}

impl Default for InvocationId {
    fn default() -> Self {
        Self::new()
    }
}

/// Reference to evidence supporting an invocation outcome.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub evidence_type: String,
    pub reference: String,
    pub source: Option<String>,
}

impl EvidenceRef {
    pub fn new(
        evidence_type: impl Into<String>,
        reference: impl Into<String>,
    ) -> KernelResult<Self> {
        let evidence_type = evidence_type.into();
        let reference = reference.into();
        if evidence_type.trim().is_empty() {
            return Err(KernelError::InvalidInput(
                "evidence type must not be empty".to_owned(),
            ));
        }
        if reference.trim().is_empty() {
            return Err(KernelError::InvalidInput(
                "evidence reference must not be empty".to_owned(),
            ));
        }
        Ok(Self {
            evidence_type,
            reference,
            source: None,
        })
    }

    pub fn with_source(mut self, source: impl Into<String>) -> KernelResult<Self> {
        let source = source.into();
        if source.trim().is_empty() {
            return Err(KernelError::InvalidInput(
                "evidence source must not be empty".to_owned(),
            ));
        }
        self.source = Some(source);
        Ok(self)
    }
}

/// Canonical request submitted to the CAT execution boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvocationRequest {
    pub contract_version: ContractVersion,
    pub invocation_id: InvocationId,
    pub agent_id: AgentId,
    pub capability: String,
    pub context: ExecutionContext,
    pub idempotency_key: IdempotencyKey,
    pub input: serde_json::Value,
    pub requested_side_effect: SideEffectClass,
    pub requested_at: TimestampMs,
}

impl InvocationRequest {
    pub fn new(
        agent_id: AgentId,
        capability: impl Into<String>,
        context: ExecutionContext,
        idempotency_key: IdempotencyKey,
        input: serde_json::Value,
        requested_side_effect: SideEffectClass,
        requested_at: TimestampMs,
    ) -> KernelResult<Self> {
        let capability = capability.into();
        if capability.trim().is_empty() {
            return Err(KernelError::InvalidInput(
                "invocation capability must not be empty".to_owned(),
            ));
        }
        Ok(Self {
            contract_version: ContractVersion::V1,
            invocation_id: InvocationId::new(),
            agent_id,
            capability,
            context,
            idempotency_key,
            input,
            requested_side_effect,
            requested_at,
        })
    }

    pub fn validate(&self) -> KernelResult<()> {
        if self.contract_version.major == 0 {
            return Err(KernelError::InvalidInput(
                "invocation contract major version must be greater than zero".to_owned(),
            ));
        }
        if self.capability.trim().is_empty() {
            return Err(KernelError::InvalidInput(
                "invocation capability must not be empty".to_owned(),
            ));
        }
        if self.idempotency_key.as_str().trim().is_empty() {
            return Err(KernelError::InvalidIdentifier(
                "invocation idempotency key must not be empty".to_owned(),
            ));
        }
        Ok(())
    }
}

/// Canonical result emitted after an invocation attempt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvocationOutcome {
    pub contract_version: ContractVersion,
    pub invocation_id: InvocationId,
    pub status: OutcomeStatus,
    pub lifecycle_status: InvocationStatus,
    pub evidence: Vec<EvidenceRef>,
    pub output: Option<serde_json::Value>,
    pub error_code: Option<String>,
    pub completed_at: TimestampMs,
}

impl InvocationOutcome {
    pub fn validate(&self) -> KernelResult<()> {
        if self.contract_version.major == 0 {
            return Err(KernelError::InvalidInput(
                "outcome contract major version must be greater than zero".to_owned(),
            ));
        }
        if !self.lifecycle_status.is_terminal() {
            return Err(KernelError::InvalidInput(
                "invocation outcome requires a terminal lifecycle status".to_owned(),
            ));
        }
        match self.status {
            OutcomeStatus::Succeeded if !self.lifecycle_status.is_success() => {
                Err(KernelError::InvalidInput(
                    "successful outcome requires succeeded lifecycle status".to_owned(),
                ))
            }
            OutcomeStatus::Failed if self.lifecycle_status != InvocationStatus::Failed => {
                Err(KernelError::InvalidInput(
                    "failed outcome requires failed lifecycle status".to_owned(),
                ))
            }
            OutcomeStatus::Unknown if self.lifecycle_status != InvocationStatus::Unknown => {
                Err(KernelError::InvalidInput(
                    "unknown outcome requires unknown lifecycle status".to_owned(),
                ))
            }
            _ => Ok(()),
        }
    }

    pub fn succeeded(
        invocation_id: InvocationId,
        output: serde_json::Value,
        completed_at: TimestampMs,
    ) -> Self {
        Self {
            contract_version: ContractVersion::V1,
            invocation_id,
            status: OutcomeStatus::Succeeded,
            lifecycle_status: InvocationStatus::Succeeded,
            evidence: Vec::new(),
            output: Some(output),
            error_code: None,
            completed_at,
        }
    }

    pub fn failed(
        invocation_id: InvocationId,
        error_code: impl Into<String>,
        completed_at: TimestampMs,
    ) -> Self {
        Self {
            contract_version: ContractVersion::V1,
            invocation_id,
            status: OutcomeStatus::Failed,
            lifecycle_status: InvocationStatus::Failed,
            evidence: Vec::new(),
            output: None,
            error_code: Some(error_code.into()),
            completed_at,
        }
    }

    pub fn unknown(invocation_id: InvocationId, completed_at: TimestampMs) -> Self {
        Self {
            contract_version: ContractVersion::V1,
            invocation_id,
            status: OutcomeStatus::Unknown,
            lifecycle_status: InvocationStatus::Unknown,
            evidence: Vec::new(),
            output: None,
            error_code: None,
            completed_at,
        }
    }

    pub fn with_evidence(mut self, evidence: EvidenceRef) -> Self {
        self.evidence.push(evidence);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CorrelationId, EntityId, TenantId};

    fn context() -> ExecutionContext {
        ExecutionContext::new(
            TenantId::new(),
            CorrelationId::new(),
            EntityId::new(),
            TimestampMs::new(1_000),
        )
    }

    #[test]
    fn request_requires_a_capability() {
        let result = InvocationRequest::new(
            AgentId::new(),
            "  ",
            context(),
            IdempotencyKey::new("request-1").unwrap(),
            serde_json::json!({}),
            SideEffectClass::S0,
            TimestampMs::new(1_000),
        );
        assert!(result.is_err());
    }

    #[test]
    fn request_round_trips_through_json() {
        let request = InvocationRequest::new(
            AgentId::new(),
            "affiliate.discovery",
            context(),
            IdempotencyKey::new("request-1").unwrap(),
            serde_json::json!({"query":"laptop"}),
            SideEffectClass::S0,
            TimestampMs::new(1_000),
        )
        .unwrap();
        let json = serde_json::to_string(&request).unwrap();
        let decoded: InvocationRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, request);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn unknown_outcome_is_not_a_failed_outcome() {
        let outcome = InvocationOutcome::unknown(
            InvocationId::new(),
            TimestampMs::new(2_000),
        );
        assert_eq!(outcome.status, OutcomeStatus::Unknown);
        assert!(outcome.validate().is_ok());
    }

    #[test]
    fn outcome_status_and_lifecycle_must_match() {
        let mut outcome = InvocationOutcome::succeeded(
            InvocationId::new(),
            serde_json::json!({"ok":true}),
            TimestampMs::new(2_000),
        );
        outcome.lifecycle_status = InvocationStatus::Failed;
        assert!(outcome.validate().is_err());
    }

    #[test]
    fn evidence_requires_type_and_reference() {
        assert!(EvidenceRef::new("", "ref").is_err());
        assert!(EvidenceRef::new("source", "").is_err());
        assert!(EvidenceRef::new("source", "ref").is_ok());
    }
}
