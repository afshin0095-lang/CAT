use cat_orchestrator::{
    ExecutionEngine, OrchestratorError, StepState, WorkflowDefinition, WorkflowInstance,
    WorkflowState, WorkflowStep,
};

fn workflow() -> WorkflowInstance {
    WorkflowInstance::new(WorkflowDefinition {
        workflow_type: "affiliate.attribution".into(),
        version: 1,
        steps: vec![
            WorkflowStep {
                id: "ingest".into(),
                dependencies: vec![],
                state: StepState::Pending,
                attempt: 0,
                max_attempts: 3,
                compensation_step: None,
            },
            WorkflowStep {
                id: "attribute".into(),
                dependencies: vec!["ingest".into()],
                state: StepState::Pending,
                attempt: 0,
                max_attempts: 3,
                compensation_step: None,
            },
        ],
    })
}

#[test]
fn public_execution_contract_preserves_dependency_order() {
    let engine = ExecutionEngine::default();
    let mut workflow = workflow();

    engine.start(&mut workflow).unwrap();
    assert_eq!(workflow.state, WorkflowState::Running);
    assert_eq!(workflow.definition.steps[0].state, StepState::Ready);
    assert_eq!(workflow.definition.steps[1].state, StepState::Pending);

    engine.begin_step(&mut workflow, "ingest").unwrap();
    engine.succeed_step(&mut workflow, "ingest").unwrap();

    assert_eq!(workflow.definition.steps[1].state, StepState::Ready);
}

#[test]
fn public_execution_contract_rejects_terminal_cancel() {
    let engine = ExecutionEngine::default();
    let mut workflow = workflow();
    engine.start(&mut workflow).unwrap();
    engine.cancel(&mut workflow).unwrap();

    let error = engine.cancel(&mut workflow).unwrap_err();
    assert!(matches!(
        error,
        OrchestratorError::InvalidStateTransition { .. }
    ));
}
