use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum DecisionError {
    #[error("invalid decision request: {0}")]
    InvalidRequest(String),
    #[error("invalid alternative confidence")]
    InvalidConfidence,
    #[error("no candidate alternative is available")]
    NoCandidate,
    #[error("policy error: {0}")]
    Policy(#[from] crate::policy::PolicyError),
}

pub type DecisionResult<T> = Result<T, DecisionError>;
