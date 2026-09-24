use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrchestratorError {
    #[error("workflow {0} not found")]
    WorkflowNotFound(String),
    #[error("workflow step {step} is not ready")]
    StepNotReady { step: String },
    #[error("workflow step {step_id} does not exist")]
    UnknownStep { step_id: String },
    #[error("invalid workflow transition from {from} to {to}")]
    InvalidStateTransition { from: String, to: String },
    #[error("invalid step {step_id} transition from {from} to {to}")]
    InvalidStepTransition {
        step_id: String,
        from: String,
        to: String,
    },
    #[error("lease {lease_id} is not owned by {owner}")]
    LeaseOwnerMismatch { lease_id: String, owner: String },
    #[error("lease {lease_id} has expired")]
    LeaseExpired { lease_id: String },
    #[error("lease is unavailable for resource {resource}")]
    LeaseUnavailable { resource: String },
    #[error("workflow {workflow_id} is not in a runnable state")]
    InvalidState { workflow_id: String },
    #[error("workflow dependency cycle detected")]
    DependencyCycle,
    #[error("workflow {workflow_id} revision conflict: expected {expected}, actual {actual}")]
    RevisionConflict {
        workflow_id: String,
        expected: u64,
        actual: u64,
    },
    #[error("authorization input is invalid: {0}")]
    InvalidAuthorizationInput(String),
    #[error("capability authorization denied for {capability}: {reasons:?}")]
    CapabilityAuthorizationDenied { capability: String, reasons: Vec<String> },
    #[error("capability authorization requires approval for {capability}: {reasons:?}")]
    CapabilityApprovalRequired { capability: String, reasons: Vec<String> },
    #[error("workflow serialization failed: {0}")]
    Serialization(String),
}

pub type OrchestratorResult<T> = Result<T, OrchestratorError>;
