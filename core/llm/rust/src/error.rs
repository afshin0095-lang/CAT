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
    #[error("invalid prompt contract: {0}")]
    InvalidPrompt(String),
    #[error("invalid prompt evaluation: {0}")]
    InvalidEvaluation(String),
    #[error("invalid tool contract: {0}")]
    InvalidTool(String),
    #[error("invalid LLM policy: {0}")]
    InvalidPolicy(String),
    #[error("LLM policy denied the operation: {0}")]
    PolicyDenied(String),
    #[error("invalid provider authorization policy: {0}")]
    InvalidProviderPolicy(String),
    #[error("provider authorization denied the operation: {0}")]
    ProviderPolicyDenied(String),
}
