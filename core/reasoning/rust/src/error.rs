use thiserror::Error;

pub type ReasoningOutcome<T> = Result<T, ReasoningError>;

#[derive(Debug, Error)]
pub enum ReasoningError {
    #[error("invalid reasoning request: {0}")]
    InvalidRequest(String),
    #[error("evidence set is empty")]
    EmptyEvidence,
    #[error("evidence confidence must be between 0 and 1")]
    InvalidConfidence,
    #[error("reasoning policy rejected execution: {0}")]
    PolicyRejected(String),
    #[error("reasoning provider failed: {0}")]
    Provider(String),
    #[error("serialization failed: {0}")]
    Serialization(String),
}
