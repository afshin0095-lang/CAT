use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PromptError {
    #[error("prompt document has no blocks")]
    EmptyDocument,
    #[error("prompt variable `{0}` is missing")]
    MissingVariable(String),
    #[error("prompt variable `{0}` is not allowed in this policy context")]
    ForbiddenVariable(String),
    #[error("prompt block `{0}` is empty")]
    EmptyBlock(String),
    #[error("prompt contains an unresolved template token: {0}")]
    UnresolvedToken(String),
}

pub type PromptResult<T> = Result<T, PromptError>;
