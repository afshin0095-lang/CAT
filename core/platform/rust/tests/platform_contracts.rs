use cat_orchestrator::ScheduleRequest;
use cat_platform::{IntegrationCommand, IntegrationContext, IntegrationTarget, PlatformRuntime};
use uuid::Uuid;

#[test]
fn command_context_preserves_correlation_and_workflow_identity() {
    let workflow_id = Uuid::now_v7();
    let context = IntegrationContext::new("integration-test").with_workflow(workflow_id);
    let correlation_id = context.correlation_id;
    let command = IntegrationCommand::new(IntegrationTarget::EventBus, "publish", context);

    assert_eq!(command.context.correlation_id, correlation_id);
    assert_eq!(command.context.workflow_id, Some(workflow_id));
}

#[test]
fn runtime_exposes_deterministic_scheduler_boundary() {
    let mut runtime = PlatformRuntime::default();
    let workflow_id = Uuid::now_v7();
    runtime.schedule(ScheduleRequest {
        workflow_id,
        not_before_ms: 500,
        priority: 7,
    });

    assert_eq!(runtime.queue_depth(), 1);
    assert!(runtime.ready_work(499).is_none());
    let ready = runtime.ready_work(500).expect("work should become ready");
    assert_eq!(ready.workflow_id, workflow_id);
    assert_eq!(ready.priority, 7);
    assert_eq!(runtime.queue_depth(), 0);
}

#[test]
fn command_validation_rejects_missing_actor_or_operation() {
    let runtime = PlatformRuntime::default();
    let mut context = IntegrationContext::new(" ");
    let command = IntegrationCommand::new(IntegrationTarget::Retrieval, "search", context.clone());
    assert!(runtime.accept_command(&command).is_err());

    context.actor = "platform".into();
    let command = IntegrationCommand::new(IntegrationTarget::Retrieval, " ", context);
    assert!(runtime.accept_command(&command).is_err());
}
