use cat_kernel::{
    AgentId, CapabilityId, CorrelationId, IdempotencyKey, InvocationId, SideEffectClass,
};
use serde::{Deserialize, Serialize};

use crate::ExecutionAuthorization;

/// Durable projection of an authorization receipt.
///
/// This is persistence data, not a second authorization mechanism. The source receipt
/// remains the result of the kernel authorization boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionAuthorizationRecord {
    pub invocation_id: InvocationId,
    pub agent_id: AgentId,
    pub capability_id: CapabilityId,
    pub requested_side_effect: SideEffectClass,
    pub required_policies: Vec<String>,
    pub approval_reference: Option<String>,
    pub idempotency_key: IdempotencyKey,
    pub correlation_id: CorrelationId,
    pub admitted_at_ms: u64,
}

impl ExecutionAuthorizationRecord {
    pub fn from_receipt(receipt: &ExecutionAuthorization) -> Self {
        Self {
            invocation_id: receipt.invocation_id(),
            agent_id: receipt.agent_id(),
            capability_id: receipt.capability_id().clone(),
            requested_side_effect: receipt.requested_side_effect(),
            required_policies: receipt.required_policies().to_vec(),
            approval_reference: receipt.approval_reference().map(str::to_owned),
            idempotency_key: receipt.idempotency_key().clone(),
            correlation_id: receipt.correlation_id(),
            admitted_at_ms: receipt.admitted_at_ms(),
        }
    }

    pub fn validate(&self) -> bool {
        !self.required_policies.iter().any(|policy| policy.trim().is_empty())
            && !self.idempotency_key.as_str().trim().is_empty()
            && !self.capability_id.as_str().trim().is_empty()
            && self.invocation_id.as_entity_id().as_uuid() != uuid::Uuid::nil()
            && self.agent_id.as_entity_id().as_uuid() != uuid::Uuid::nil()
            && self.correlation_id.as_uuid() != uuid::Uuid::nil()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CapabilityAdmission, CapabilityAdmissionResult, ExecutionIntent};
    use cat_kernel::{
        AgentContract, ApprovalContext, CapabilityContract, CapabilityLifecycle, CapabilityRegistry,
        EntityId, ExecutionContext, ContractVersion, CorrelationId, TimestampMs, TenantId,
    };

    fn receipt() -> ExecutionAuthorization {
        let capability_id = CapabilityId::new("cat.capability.test.record.v1").unwrap();
        let mut capability =
            CapabilityContract::new(capability_id.clone(), "test", "Record governed execution")
                .unwrap();
        capability.contract_version = ContractVersion::V1;
        capability.inputs.push("input".into());
        capability.outputs.push("output".into());
        capability.failure_model.push("typed".into());
        capability.observability.push("record".into());
        capability.evaluation.push("deterministic".into());
        capability.lifecycle = CapabilityLifecycle::Validating;

        let mut registry = CapabilityRegistry::new();
        registry.register(capability).unwrap();
        registry.transition(&capability_id, CapabilityLifecycle::Active).unwrap();

        let agent_id = AgentId::new();
        let agent = AgentContract::new(agent_id, "test.agent", "record execution")
            .unwrap()
            .with_capability(capability_id.as_str())
            .unwrap()
            .enable();

        let invocation = cat_kernel::InvocationRequest::new(
            agent_id,
            capability_id.as_str(),
            ExecutionContext::new(
                TenantId::new(),
                CorrelationId::new(),
                EntityId::new(),
                TimestampMs::new(1_000),
            ),
            IdempotencyKey::new("authorization-record-1").unwrap(),
            serde_json::json!({"ok":true}),
            SideEffectClass::S0,
            TimestampMs::new(1_000),
        )
        .unwrap();

        let workflow_id = uuid::Uuid::now_v7();
        let intent = ExecutionIntent::new(workflow_id, "record", 1, 1_000);
        let admission = CapabilityAdmission::new(&registry);
        match admission
            .admit(
                workflow_id,
                intent.step_id(),
                intent.attempt(),
                &agent,
                &invocation,
                ApprovalContext::none(),
                2_000,
            )
            .unwrap()
        {
            CapabilityAdmissionResult::Admitted(receipt) => receipt,
            other => panic!("expected admitted receipt, got {other:?}"),
        }
    }

    #[test]
    fn record_round_trips_through_json() {
        let record = ExecutionAuthorizationRecord::from_receipt(&receipt());
        let json = serde_json::to_string(&record).unwrap();
        let decoded: ExecutionAuthorizationRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, record);
        assert!(decoded.validate());
    }
}
