use thiserror::Error;

pub type KernelResult<T> = Result<T, KernelError>;

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),

    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(String),
}
