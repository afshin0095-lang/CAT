use cat_orchestrator::ScheduleRequest;
use cat_planning::{PlanBuilder, StepKind};
use cat_platform::{IntegrationContext, WorkflowRuntime};
use uuid::Uuid;

#[test]
fn planning_and_orchestration_share_the_same_workflow_context() {
    let workflow_id = Uuid::now_v7();
    let context = IntegrationContext::new("integration-test").with_workflow(workflow_id);
    let mut builder = PlanBuilder::new("execute approved affiliate workflow");
    let _approval = builder.step("approval", StepKind::Approval);
    let plan = builder.build();

    let mut runtime = WorkflowRuntime::new();
    let receipt = runtime.validate_plan(&context, &plan).unwrap();
    assert_eq!(receipt.workflow_id, workflow_id);

    runtime.schedule(&context, workflow_id, 500, 10).unwrap();
    let ready = runtime.pop_ready(500).unwrap();
    assert_eq!(ready.workflow_id, workflow_id);
}

#[test]
fn scheduler_contract_remains_explicit_at_the_platform_boundary() {
    let workflow_id = Uuid::now_v7();
    let context = IntegrationContext::new("integration-test").with_workflow(workflow_id);
    let mut runtime = WorkflowRuntime::new();

    runtime.schedule(&context, workflow_id, 1000, 3).unwrap();
    assert_eq!(runtime.queue_depth(), 1);
    assert!(runtime.pop_ready(999).is_none());
    assert_eq!(runtime.pop_ready(1000), Some(ScheduleRequest { workflow_id, not_before_ms: 1000, priority: 3 }));
}
