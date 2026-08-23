use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("invalid integration command: {0}")]
    InvalidCommand(String),
}

pub type PlatformResult<T> = Result<T, PlatformError>;
