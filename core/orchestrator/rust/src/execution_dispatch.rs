use crate::{ExecutionEngine, ExecutionRequest, WorkflowInstance, cursor};

/// Pure dispatcher adapter: turns the current execution cursor into requests.
/// It does not perform I/O or execute the requested work.
pub fn ready_requests(workflow: &WorkflowInstance, requested_at_ms: u64) -> Vec<ExecutionRequest> {
    let view = cursor(workflow);
    view.ready_steps
        .into_iter()
        .filter_map(|step_id| {
            workflow
                .definition
                .steps
                .iter()
                .find(|step| step.id == step_id)
                .map(|step| {
                    ExecutionRequest::new(
                        workflow.id,
                        step.id.clone(),
                        step.attempt.saturating_add(1),
                        requested_at_ms,
                    )
                })
        })
        .collect()
}

/// Claim one ready step in the in-memory state machine before handing it to an external worker.
pub fn claim_step(
    engine: &ExecutionEngine,
    workflow: &mut WorkflowInstance,
    step_id: &str,
) -> crate::OrchestratorResult<ExecutionRequest> {
    let attempt = workflow
        .definition
        .steps
        .iter()
        .find(|step| step.id == step_id)
        .map(|step| step.attempt.saturating_add(1));
    engine.begin_step(workflow, step_id)?;
    Ok(ExecutionRequest::new(
        workflow.id,
        step_id,
        attempt.unwrap_or(1),
        0,
    ))
}
