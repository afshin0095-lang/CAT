#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod context;
mod error;
mod runtime;

pub use context::{IntegrationCommand, IntegrationContext, IntegrationTarget};
pub use error::{PlatformError, PlatformResult};
pub use runtime::{PlatformRuntime, ReadyWork};

#[cfg(test)]
mod tests {
    use super::*;
    use cat_orchestrator::ScheduleRequest;
    use cat_planning::{PlanBuilder, StepKind, validate_plan};
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
}
