use crate::IntegrationTarget;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("invalid integration command: {0}")]
    InvalidCommand(String),
    #[error("platform adapter not found: {0}")]
    AdapterNotFound(String),
    #[error("adapter target mismatch: expected {expected:?}, got {actual:?}")]
    TargetMismatch { expected: IntegrationTarget, actual: IntegrationTarget },
}

pub type PlatformResult<T> = Result<T, PlatformError>;
