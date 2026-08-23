use crate::model::{StepState, WorkflowInstance};

/// Returns compensation work in reverse successful execution order.
pub fn compensation_order(workflow: &WorkflowInstance) -> Vec<String> {
    workflow.definition.steps.iter()
        .filter(|step| step.state == StepState::Succeeded && step.compensation_step.is_some())
        .rev()
        .filter_map(|step| step.compensation_step.clone())
        .collect()
}

pub fn begin_compensation(workflow: &mut WorkflowInstance) {
    workflow.state = crate::model::WorkflowState::Compensating;
    workflow.revision = workflow.revision.saturating_add(1);
}
