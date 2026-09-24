use serde::Serialize;
use uuid::Uuid;

use crate::ExecutionRequest;

/// Worker input is executable only because it contains a kernel-backed authorization receipt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WorkerExecutionInput {
    execution_id: Uuid,
    workflow_id: Uuid,
    step_id: String,
    attempt: u32,
    authorization: cat_kernel::ExecutionAuthorization,
}

impl WorkerExecutionInput {
    pub(crate) fn from_request(execution_id: Uuid, request: &ExecutionRequest) -> Self {
        Self {
            execution_id,
            workflow_id: request.workflow_id(),
            step_id: request.step_id().to_owned(),
            attempt: request.attempt(),
            authorization: request.authorization().clone(),
        }
    }

    pub fn execution_id(&self) -> Uuid {
        self.execution_id
    }

    pub fn workflow_id(&self) -> Uuid {
        self.workflow_id
    }

    pub fn step_id(&self) -> &str {
        &self.step_id
    }

    pub fn attempt(&self) -> u32 {
        self.attempt
    }

    pub fn authorization(&self) -> &cat_kernel::ExecutionAuthorization {
        &self.authorization
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerExecutionOutcome {
    Succeeded,
    Failed,
    WaitingApproval,
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WorkerExecutionResult {
    pub outcome: WorkerExecutionOutcome,
    pub output: serde_json::Value,
    pub error_code: Option<String>,
}

impl WorkerExecutionResult {
    pub fn success(output: serde_json::Value) -> Self {
        Self {
            outcome: WorkerExecutionOutcome::Succeeded,
            output,
            error_code: None,
        }
    }
    pub fn failure(error_code: impl Into<String>, output: serde_json::Value) -> Self {
        Self {
            outcome: WorkerExecutionOutcome::Failed,
            output,
            error_code: Some(error_code.into()),
        }
    }
}

/// Boundary for real workers. The orchestrator supplies governed execution intent; the worker owns I/O.
pub trait WorkerExecutor {
    fn execute(&mut self, input: WorkerExecutionInput) -> WorkerExecutionResult;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CapabilityAdmission, CapabilityAdmissionResult, ExecutionIntent, ExecutionRequest,
    };
    use cat_kernel::{
        AgentContract, AgentId, ApprovalContext, CapabilityContract, CapabilityId,
        CapabilityLifecycle, CapabilityRegistry, EntityId, ExecutionContext, IdempotencyKey,
        IdempotencyPolicy, CorrelationId, SideEffectClass, TenantId, TimestampMs, InvocationRequest,
    };

    fn governed_request() -> ExecutionRequest {
        let capability_id = CapabilityId::new("cat.capability.test.execute.v1").unwrap();
        let mut capability =
            CapabilityContract::new(capability_id.clone(), "test", "Execute test work").unwrap();
        capability.inputs.push("input".into());
        capability.outputs.push("output".into());
        capability.failure_model.push("typed failure".into());
        capability.observability.push("worker.execute".into());
        capability.evaluation.push("deterministic".into());
        capability.lifecycle = CapabilityLifecycle::Validating;

        let mut registry = CapabilityRegistry::new();
        registry.register(capability).unwrap();
        registry
            .transition(&capability_id, CapabilityLifecycle::Active)
            .unwrap();

        let agent_id = AgentId::new();
        let agent = AgentContract::new(agent_id, "test.agent", "test mission")
            .unwrap()
            .with_capability(capability_id.as_str())
            .unwrap()
            .enable();

        let invocation = InvocationRequest::new(
            agent_id,
            capability_id.as_str(),
            ExecutionContext::new(
                TenantId::new(),
                CorrelationId::new(),
                EntityId::new(),
                TimestampMs::new(1_000),
            ),
            IdempotencyKey::new("worker-test-1").unwrap(),
            serde_json::json!({"input":true}),
            SideEffectClass::S0,
            TimestampMs::new(1_000),
        )
        .unwrap();

        let admission = CapabilityAdmission::new(&registry);
        let workflow_id = Uuid::now_v7();
        let intent = ExecutionIntent::new(workflow_id, "research", 3, 100);
        let authorization = match admission
            .admit(
                workflow_id,
                intent.step_id(),
                intent.attempt(),
                &agent,
                &invocation,
                ApprovalContext::none(),
                100,
            )
            .unwrap()
        {
            CapabilityAdmissionResult::Admitted(receipt) => receipt,
            other => panic!("expected admission, got {other:?}"),
        };

        ExecutionRequest::from_authorized(intent, authorization).unwrap()
    }

    #[test]
    fn worker_input_preserves_execution_identity_and_authorization() {
        let execution_id = Uuid::now_v7();
        let request = governed_request();
        let input = WorkerExecutionInput::from_request(execution_id, &request);

        assert_eq!(input.execution_id(), execution_id);
        assert_eq!(input.workflow_id(), request.workflow_id());
        assert_eq!(input.step_id(), "research");
        assert_eq!(input.attempt(), 3);
        assert_eq!(input.authorization().capability_id(), request.authorization().capability_id());
    }

    #[test]
    fn result_helpers_are_machine_readable() {
        assert_eq!(
            WorkerExecutionResult::success(serde_json::json!({})).outcome,
            WorkerExecutionOutcome::Succeeded
        );
        let failure = WorkerExecutionResult::failure("timeout", serde_json::json!({}));
        assert_eq!(failure.outcome, WorkerExecutionOutcome::Failed);
        assert_eq!(failure.error_code.as_deref(), Some("timeout"));
    }
}
