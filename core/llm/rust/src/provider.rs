use async_trait::async_trait;

use crate::{GenerationRequest, GenerationResponse, LlmError, ProviderId};

#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn id(&self) -> ProviderId;
    async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, LlmError>;
}

#[derive(Clone, Debug, Default)]
pub struct DeterministicProvider;

#[async_trait]
impl LlmProvider for DeterministicProvider {
    fn id(&self) -> ProviderId { ProviderId::new("local.deterministic") }

    async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, LlmError> {
        let prompt = request.messages.last().map(|message| message.content.as_str()).unwrap_or("");
        let content = if prompt.is_empty() {
            "CAT deterministic provider: empty input".to_owned()
        } else {
            format!("CAT deterministic provider: {prompt}")
        };
        let input_tokens = request.messages.iter().map(|m| m.content.split_whitespace().count() as u64).sum();
        let output_tokens = content.split_whitespace().count() as u64;
        Ok(GenerationResponse {
            request_id: request.request_id,
            provider: self.id(),
            model: request.model,
            content,
            usage: crate::Usage { input_tokens, output_tokens, total_tokens: input_tokens + output_tokens },
            finish_reason: "stop".to_owned(),
        })
    }
}
