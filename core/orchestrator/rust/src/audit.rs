use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    ExecutionAttempt, ExecutionAttemptStatus, ExecutionAuthorizationRecord,
    ProviderExecutionRecord,
};

/// Stable audit projection assembled from durable execution facts.
///
/// This is a derived view: the attempt, authorization record, and provider record remain
/// the canonical sources. The projection exists so reconciliation, audit consumers, and
/// operators can inspect one coherent evidence object without trusting process memory.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionAuditEvidence {
    pub execution_id: Uuid,
    pub workflow_id: Uuid,
    pub step_id: String,
    pub attempt: u32,
    pub status: ExecutionAttemptStatus,
    pub authorization: ExecutionAuthorizationRecord,
    pub provider_result: Option<ProviderExecutionRecord>,
}

impl ExecutionAuditEvidence {
    pub fn from_parts(
        attempt: &ExecutionAttempt,
        authorization: ExecutionAuthorizationRecord,
        provider_result: Option<ProviderExecutionRecord>,
    ) -> Option<Self> {
        if attempt.execution_id.is_nil()
            || attempt.workflow_id.is_nil()
            || attempt.step_id.trim().is_empty()
            || attempt.attempt == 0
            || !authorization.validate()
        {
            return None;
        }

        Some(Self {
            execution_id: attempt.execution_id,
            workflow_id: attempt.workflow_id,
            step_id: attempt.step_id.clone(),
            attempt: attempt.attempt,
            status: attempt.status,
            authorization,
            provider_result,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cat_kernel::{
        AgentContract, AgentId, CapabilityContract, CapabilityId, CapabilityLifecycle,
        CapabilityRegistry, ContractVersion, CorrelationId, EntityId, ExecutionContext,
        IdempotencyKey, SideEffectClass, TenantId, TimestampMs,
    };
    use crate::{
        ApprovalContext, CapabilityAdmission, CapabilityAdmissionResult, ExecutionIntent,
    };

    fn authorization() -> ExecutionAuthorizationRecord {
        let capability_id = CapabilityId::new("cat.capability.test.audit.v1").unwrap();
        let mut capability =
            CapabilityContract::new(capability_id.clone(), "test", "Audit test execution").unwrap();
        capability.contract_version = ContractVersion::V1;
        capability.inputs.push("input".into());
        capability.outputs.push("output".into());
        capability.failure_model.push("typed".into());
        capability.observability.push("audit".into());
        capability.evaluation.push("deterministic".into());
        capability.lifecycle = CapabilityLifecycle::Validating;

        let mut registry = CapabilityRegistry::new();
        registry.register(capability).unwrap();
        registry
            .transition(&capability_id, CapabilityLifecycle::Active)
            .unwrap();

        let agent_id = AgentId::new();
        let agent = AgentContract::new(agent_id, "audit.agent", "audit test")
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
                TimestampMs::new(100),
            ),
            IdempotencyKey::new("audit-test-1").unwrap(),
            serde_json::json!({"ok": true}),
            SideEffectClass::S0,
            TimestampMs::new(100),
        )
        .unwrap();

        let workflow_id = Uuid::now_v7();
        let intent = ExecutionIntent::new(workflow_id, "audit", 1, 100);
        let admission = CapabilityAdmission::new(&registry);

        let receipt = match admission
            .admit(
                workflow_id,
                intent.step_id(),
                intent.attempt(),
                &agent,
                &invocation,
                ApprovalContext::none(),
                200,
            )
            .unwrap()
        {
            CapabilityAdmissionResult::Admitted(receipt) => receipt,
            other => panic!("expected admitted authorization, got {other:?}"),
        };

        ExecutionAuthorizationRecord::from_receipt(&receipt)
    }

    #[test]
    fn audit_projection_rejects_invalid_attempt_identity() {
        let attempt = ExecutionAttempt {
            execution_id: Uuid::nil(),
            workflow_id: Uuid::now_v7(),
            step_id: "audit".into(),
            attempt: 1,
            status: ExecutionAttemptStatus::Running,
            owner: "worker".into(),
            fencing_token: 1,
            started_at_ms: 1,
            heartbeat_at_ms: 1,
            finished_at_ms: None,
            result: None,
            error: None,
        };

        assert!(ExecutionAuditEvidence::from_parts(&attempt, authorization(), None).is_none());
    }
}
