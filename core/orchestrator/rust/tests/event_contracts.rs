use cat_eventbus::CatEvent;
use cat_orchestrator::{
    StepState, WorkflowCompleted, WorkflowEventFactory, WorkflowStarted,
    WorkflowState, WorkflowStepStateChanged,
};
use uuid::Uuid;

#[test]
fn workflow_event_contracts_have_stable_names_and_versions() {
    assert_eq!(WorkflowStarted::TYPE, "orchestrator.workflow.started");
    assert_eq!(WorkflowStepStateChanged::TYPE, "orchestrator.workflow.step.state_changed");
    assert_eq!(WorkflowCompleted::TYPE, "orchestrator.workflow.completed");
    assert_eq!(WorkflowStarted::VERSION, 1);
    assert_eq!(WorkflowStepStateChanged::VERSION, 1);
    assert_eq!(WorkflowCompleted::VERSION, 1);
}

#[test]
fn started_event_preserves_workflow_identity_and_contract_metadata() {
    let workflow_id = Uuid::now_v7();
    let envelope = WorkflowEventFactory::started(
        workflow_id,
        "affiliate.settlement",
        3,
        "orchestrator",
    )
    .unwrap();

    assert_eq!(envelope.event_type, WorkflowStarted::TYPE);
    assert_eq!(envelope.version, WorkflowStarted::VERSION);
    assert_eq!(envelope.producer, "orchestrator");
    assert_eq!(envelope.payload["workflow_id"], workflow_id.to_string());
    assert_eq!(envelope.payload["workflow_type"], "affiliate.settlement");
    assert_eq!(envelope.payload["workflow_version"], 3);
}

#[test]
fn step_event_preserves_state_attempt_and_revision() {
    let workflow_id = Uuid::now_v7();
    let envelope = WorkflowEventFactory::step_state_changed(
        workflow_id,
        "reserve",
        StepState::Running,
        2,
        7,
        "orchestrator",
    )
    .unwrap();

    assert_eq!(envelope.event_type, WorkflowStepStateChanged::TYPE);
    assert_eq!(envelope.payload["workflow_id"], workflow_id.to_string());
    assert_eq!(envelope.payload["step_id"], "reserve");
    assert_eq!(envelope.payload["state"], "running");
    assert_eq!(envelope.payload["attempt"], 2);
    assert_eq!(envelope.payload["revision"], 7);
}

#[test]
fn completed_event_is_explicitly_terminal_and_versioned() {
    let workflow_id = Uuid::now_v7();
    let envelope = WorkflowEventFactory::completed(
        workflow_id,
        WorkflowState::Succeeded,
        9,
        "orchestrator",
    )
    .unwrap();

    assert_eq!(envelope.event_type, WorkflowCompleted::TYPE);
    assert_eq!(envelope.version, 1);
    assert_eq!(envelope.payload["workflow_id"], workflow_id.to_string());
    assert_eq!(envelope.payload["state"], "succeeded");
    assert_eq!(envelope.payload["revision"], 9);
}
