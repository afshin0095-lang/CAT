use thiserror::Error;

#[derive(Debug, Error)]
pub enum RagError {
    #[error("invalid embedding: {0}")]
    InvalidEmbedding(String),
    #[error("retrieval query requires an embedding for vector search")]
    MissingEmbedding,
    #[error("tenant isolation violation")]
    TenantIsolation,
    #[error("invalid retrieval limit")]
    InvalidLimit,
    #[error("retrieval query must not be empty")]
    EmptyQuery,
    #[error("retrieval score must be finite")]
    InvalidScore,
}

pub type RagResult<T> = Result<T, RagError>;
