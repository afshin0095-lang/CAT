#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod adapters;
mod context;
mod core_adapters;
mod error;
mod runtime;

pub use adapters::{AdapterRequest, AdapterRegistry, AdapterResponse, PassthroughAdapter, PlatformAdapter};
pub use context::{IntegrationCommand, IntegrationContext, IntegrationTarget};
pub use core_adapters::{
    default_core_adapter_registry, execute_typed, CoreAdapter, CoreCommand, TypedCoreCommand,
    TypedCoreResponse, DECISION_ADAPTER, EVENT_BUS_ADAPTER, KNOWLEDGE_ADAPTER, LLM_ADAPTER,
    MEMORY_ADAPTER, ORCHESTRATOR_ADAPTER, PLANNING_ADAPTER, REASONING_ADAPTER, RETRIEVAL_ADAPTER,
};
pub use error::{PlatformError, PlatformResult};
pub use runtime::{PlatformRuntime, ReadyWork};

#[cfg(test)]
mod tests {
    use super::*;
    use cat_orchestrator::ScheduleRequest;
    use cat_planning::{validate_plan, PlanBuilder, StepKind};
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn platform_composes_decision_planning_and_orchestration() {
        let runtime = PlatformRuntime::default();
        let plan = PlanBuilder::new("publish a verified affiliate offer")
            .metadata("domain", serde_json::json!("affiliate"));
        let mut plan_builder = plan;
        let step = plan_builder.step("approve", StepKind::Approval);
        let plan = plan_builder.build();
        assert!(validate_plan(&plan).is_valid());

        let workflow_id = Uuid::now_v7();
        let context = IntegrationContext::new("platform-test").with_workflow(workflow_id);
        let command = IntegrationCommand::new(IntegrationTarget::Planning, "validate_plan", context);
        assert_eq!(command.target, IntegrationTarget::Planning);
        assert_eq!(step, plan.steps[0].id);

        let mut runtime = runtime;
        runtime.schedule(ScheduleRequest { workflow_id, not_before_ms: 10, priority: 10 });
        assert_eq!(runtime.ready_work(9), None);
        assert_eq!(runtime.ready_work(10).unwrap().workflow_id, workflow_id);
    }

    #[test]
    fn default_core_adapters_are_available_from_the_composition_root() {
        let registry = default_core_adapter_registry().unwrap();
        assert_eq!(registry.len(), 9);
    }

    #[test]
    fn typed_execution_is_available_from_the_composition_root() {
        let registry = default_core_adapter_registry().unwrap();
        let command = TypedCoreCommand::EventBus(CoreCommand::new(
            Uuid::now_v7(),
            "publish",
            IntegrationContext::new("platform-test"),
            json!({"event_type":"affiliate.conversion"}),
        ));

        let response = execute_typed(&registry, command).unwrap();
        assert_eq!(response.target, IntegrationTarget::EventBus);
        assert_eq!(response.operation, "publish");
        assert!(response.accepted);
    }
}
