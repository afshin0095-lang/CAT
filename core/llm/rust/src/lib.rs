pub mod adapters;
pub mod authorization;
pub mod error;
pub mod evaluation;
pub mod execution_policy;
pub mod health;
mod http;
pub mod model;
pub mod policy;
pub mod prompt;
pub mod provider;
pub mod provider_authorization;
pub mod stream;
pub mod tool;

pub use adapters::{AnthropicProvider, OpenAiCompatibleProvider};
pub use authorization::{
    PolicyDecision, PolicyId, PolicyVersion, PromptToolPolicy, ToolGrant,
    safety_allowed as policy_safety_allowed,
};
pub use error::LlmError;
pub use evaluation::{NonEmptyOutputEvaluator, PromptEvaluation, PromptEvaluator};
pub use execution_policy::{
    AuthorizedToolExecution, ExecutionAuthorizationRequest, ExecutionAuthorizationResult,
    ExecutionPolicyGate,
};
pub use health::{
    ProviderHealthConfig, ProviderHealthRegistry, ProviderHealthSnapshot, ProviderHealthState,
};
pub use model::{
    GenerationChunk, GenerationRequest, GenerationResponse, Message, ModelId, ProviderId, Role,
    SafetyClass, Usage,
};
pub use policy::{LlmRouter, ModelRoute, RoutingPolicy};
pub use prompt::{PromptId, PromptRegistry, PromptTemplate, PromptVersion, RenderedPrompt};
pub use provider::{DeterministicProvider, LlmProvider};
pub use provider_authorization::{ProviderToolAuthorizationRegistry, ProviderToolPolicy};
pub use stream::{LlmGenerationStream, collect_stream};
pub use tool::{
    ToolCall, ToolDefinition, ToolExecution, ToolExecutionStatus, ToolExecutor,
    ToolExecutorRegistry, ToolId, ToolRegistry, ToolVersion,
};
