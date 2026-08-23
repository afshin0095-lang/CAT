#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("no model route is available")]
    NoRoute,
    #[error("provider rejected the request: {0}")]
    ProviderRejected(String),
    #[error("provider execution failed: {0}")]
    ProviderFailure(String),
    #[error("invalid generation request: {0}")]
    InvalidRequest(String),
}
