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
}

pub type RagResult<T> = Result<T, RagError>;
