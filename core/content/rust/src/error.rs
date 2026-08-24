use thiserror::Error;

pub type ContentDomainResult<T> = Result<T, ContentDomainError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ContentDomainError {
    #[error("content title cannot be empty")]
    EmptyTitle,
    #[error("content body cannot be empty")]
    EmptyBody,
    #[error("content version must be greater than zero")]
    InvalidVersion,
    #[error("content is not in the required state: {0}")]
    InvalidState(&'static str),
    #[error("content record already exists")]
    AlreadyExists,
    #[error("content record not found")]
    NotFound,
    #[error("repository error: {0}")]
    Repository(String),
}
