use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrchestratorError {
    #[error("workflow {0} not found")]
    WorkflowNotFound(String),
    #[error("workflow step {step} is not ready")]
    StepNotReady { step: String },
    #[error("lease {lease_id} is not owned by {owner}")]
    LeaseOwnerMismatch { lease_id: String, owner: String },
    #[error("lease {lease_id} has expired")]
    LeaseExpired { lease_id: String },
    #[error("workflow {workflow_id} is not in a runnable state")]
    InvalidState { workflow_id: String },
    #[error("workflow dependency cycle detected")]
    DependencyCycle,
    #[error("workflow serialization failed: {0}")]
    Serialization(String),
}

pub type OrchestratorResult<T> = Result<T, OrchestratorError>;
