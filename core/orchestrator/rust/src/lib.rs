#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod compensation;
mod error;
mod lease;
mod model;
mod retry;
mod scheduler;

pub use compensation::{begin_compensation, compensation_order};
pub use error::{OrchestratorError, OrchestratorResult};
pub use lease::Lease;
pub use model::{StepState, WorkflowDefinition, WorkflowInstance, WorkflowState, WorkflowStep};
pub use retry::RetryPolicy;
pub use scheduler::{ScheduleRequest, Scheduler};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_starts_pending_and_advances_revision_on_compensation() {
        let workflow = WorkflowInstance::new(WorkflowDefinition {
            workflow_type: "affiliate.settlement".into(),
            version: 1,
            steps: vec![
                WorkflowStep { id: "reserve".into(), dependencies: vec![], state: StepState::Succeeded, attempt: 1, max_attempts: 3, compensation_step: Some("release_reserve".into()) },
            ],
        });
        assert_eq!(workflow.state, WorkflowState::Pending);
        assert_eq!(compensation_order(&workflow), vec!["release_reserve"]);
        let mut running = workflow;
        begin_compensation(&mut running);
        assert_eq!(running.state, WorkflowState::Compensating);
        assert_eq!(running.revision, 1);
    }
}
