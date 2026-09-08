use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    ConcreteCoreRuntime, CoreCommand, IntegrationTarget, PlatformError,
    PlatformResult, ProviderAdapterRegistry, ProviderId, RemainingCoreRuntime,
    TypedCoreCommand, TypedCoreResponse, WorkflowRuntime,
};

#[derive(Default)]
pub struct PlatformAdapterLayer { concrete: ConcreteCoreRuntime, remaining: RemainingCoreRuntime, workflow: WorkflowRuntime, providers: ProviderAdapterRegistry }
impl PlatformAdapterLayer {
    pub fn new() -> Self { Self::default() }
    pub fn execute(&mut self, command: TypedCoreCommand) -> PlatformResult<TypedCoreResponse> {
        match command.target() {
            IntegrationTarget::EventBus | IntegrationTarget::Knowledge | IntegrationTarget::Memory => self.concrete.execute(command),
            IntegrationTarget::Llm | IntegrationTarget::Reasoning | IntegrationTarget::Decision | IntegrationTarget::Retrieval => self.remaining.execute(command),
            IntegrationTarget::Planning | IntegrationTarget::Orchestrator => self.execute_workflow(command),
        }
    }
    pub fn execute_provider(&self, provider: &ProviderId, request: &crate::AdapterRequest) -> PlatformResult<crate::AdapterResponse> { self.providers.execute(provider, request) }
    pub fn providers(&self) -> &ProviderAdapterRegistry { &self.providers }
    pub fn providers_mut(&mut self) -> &mut ProviderAdapterRegistry { &mut self.providers }
    fn execute_workflow(&mut self, command: TypedCoreCommand) -> PlatformResult<TypedCoreResponse> {
        let target = command.target();
        let core = match command { TypedCoreCommand::Planning(core) | TypedCoreCommand::Orchestrator(core) => core, _ => unreachable!("target was checked before workflow dispatch") };
        match (target, core.operation.as_str()) {
            (IntegrationTarget::Planning, "validate_plan") => {
                let plan: cat_planning::Plan = serde_json::from_value(core.payload).map_err(|error| invalid(format!("invalid plan payload: {error}")))?;
                let receipt = self.workflow.validate_plan(&core.context, &plan)?;
                Ok(response(core.context.request_id, target, "validate_plan", serde_json::to_value(receipt).map_err(|error| invalid(format!("receipt serialization failed: {error}")))?))
            }
            (IntegrationTarget::Orchestrator, "schedule") => {
                let request: ScheduleCommand = serde_json::from_value(core.payload).map_err(|error| invalid(format!("invalid schedule payload: {error}")))?;
                self.workflow.schedule(&core.context, request.workflow_id, request.not_before_ms, request.priority)?;
                Ok(response(core.context.request_id, target, "schedule", json!({"workflow_id": request.workflow_id, "queued": true, "queue_depth": self.workflow.queue_depth(), "correlation_id": core.context.correlation_id, "causation_id": core.context.causation_id})))
            }
            (IntegrationTarget::Orchestrator, "status") => Ok(response(core.context.request_id, target, "status", json!({"queue_depth": self.workflow.queue_depth(), "ready": false, "correlation_id": core.context.correlation_id}))),
            _ => Err(invalid(format!("unsupported workflow adapter operation: {:?}/{}", target, core.operation))),
        }
    }
    pub fn queue_depth(&self) -> usize { self.workflow.queue_depth() }
}

#[derive(Clone, Debug, Deserialize)]
struct ScheduleCommand { workflow_id: Uuid, not_before_ms: u64, priority: i32 }
fn invalid(message: impl Into<String>) -> PlatformError { PlatformError::InvalidCommand(message.into()) }
fn response(request_id: Uuid, target: IntegrationTarget, operation: &str, payload: Value) -> TypedCoreResponse { TypedCoreResponse { request_id, target, operation: operation.to_owned(), accepted: true, payload } }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AdapterRequest, IntegrationCommand, IntegrationContext};
    use cat_planning::{PlanBuilder, StepKind};
    use std::sync::Arc;
    #[test] fn routes_stateful_cores_to_concrete_runtime() { let mut layer = PlatformAdapterLayer::new(); let command = TypedCoreCommand::Memory(CoreCommand::new(Uuid::now_v7(), "validate", IntegrationContext::new("adapter-test"), Value::Null)); let error = layer.execute(command).unwrap_err(); assert!(matches!(error, PlatformError::InvalidCommand(_))); }
    #[test] fn routes_ai_cores_to_remaining_runtime() { let mut layer = PlatformAdapterLayer::new(); let command = TypedCoreCommand::Llm(CoreCommand::new(Uuid::now_v7(), "health", IntegrationContext::new("adapter-test"), Value::Null)); let result = layer.execute(command).unwrap(); assert_eq!(result.target, IntegrationTarget::Llm); assert_eq!(result.payload["ready"], true); }
    #[test] fn planning_adapter_validates_through_planning_owner() { let mut builder = PlanBuilder::new("adapter test"); builder = builder.metadata("domain", json!("affiliate")); let _ = builder.step("approve", StepKind::Approval); let plan = builder.build(); let context = IntegrationContext::new("adapter-test").with_workflow(Uuid::now_v7()); let mut layer = PlatformAdapterLayer::new(); let command = TypedCoreCommand::Planning(CoreCommand::new(Uuid::now_v7(), "validate_plan", context, serde_json::to_value(plan).unwrap())); let result = layer.execute(command).unwrap(); assert!(result.accepted); assert_eq!(result.payload["validated"], true); }
    #[test] fn orchestrator_adapter_preserves_workflow_identity() { let workflow_id = Uuid::now_v7(); let context = IntegrationContext::new("adapter-test").with_workflow(workflow_id); let mut layer = PlatformAdapterLayer::new(); let command = TypedCoreCommand::Orchestrator(CoreCommand::new(Uuid::now_v7(), "schedule", context, json!({"workflow_id": workflow_id, "not_before_ms": 100, "priority": 7}))); let result = layer.execute(command).unwrap(); assert_eq!(result.payload["workflow_id"], json!(workflow_id)); assert_eq!(result.payload["queued"], true); assert_eq!(layer.queue_depth(), 1); }
    #[test] fn wrong_operation_is_rejected_at_adapter_boundary() { let mut layer = PlatformAdapterLayer::new(); let command = TypedCoreCommand::Retrieval(CoreCommand::new(Uuid::now_v7(), "delete_everything", IntegrationContext::new("adapter-test"), json!({}))); assert!(matches!(layer.execute(command), Err(PlatformError::InvalidCommand(_)))); }
    #[test] fn external_provider_is_checked_before_execution() { let mut layer = PlatformAdapterLayer::new(); let adapter = Arc::new(crate::DeterministicProviderAdapter::new("local-llm", IntegrationTarget::Llm, ["generate"]).unwrap()); layer.providers_mut().register(adapter).unwrap(); let command = IntegrationCommand::new(IntegrationTarget::Llm, "generate", IntegrationContext::new("provider-contract")); let request = AdapterRequest::from_command(command, json!({"prompt":"ping"})); let provider = ProviderId::new("local-llm").unwrap(); let result = layer.execute_provider(&provider, &request).unwrap(); assert!(result.accepted); assert_eq!(result.payload["provider"], "local-llm"); assert_eq!(result.payload["payload"]["prompt"], "ping"); }
}
