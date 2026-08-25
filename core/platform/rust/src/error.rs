use crate::{IntegrationTarget, ProviderFailure};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("invalid integration command: {0}")]
    InvalidCommand(String),
    #[error("platform adapter not found: {0}")]
    AdapterNotFound(String),
    #[error("adapter target mismatch: expected {expected:?}, got {actual:?}")]
    TargetMismatch { expected: IntegrationTarget, actual: IntegrationTarget },
    #[error("provider failure: {0}")]
    ProviderFailure(#[from] ProviderFailure),
    #[error("provider response serialization failed: {0}")]
    Serialization(String),
    #[error("provider transport unavailable: {0}")]
    TransportUnavailable(String),
}

pub type PlatformResult<T> = Result<T, PlatformError>;
