use async_trait::async_trait;
use futures_util::stream;

use crate::{
    GenerationChunk, GenerationRequest, GenerationResponse, LlmError, LlmGenerationStream,
    ProviderId, Usage,
};

#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn id(&self) -> ProviderId;
    async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, LlmError>;

    /// Provider-neutral streaming boundary. Implementations may override this
    /// with native provider streaming; the default preserves compatibility by
    /// exposing one terminal chunk derived from `generate`.
    async fn generate_stream(
        &self,
        request: GenerationRequest,
    ) -> Result<LlmGenerationStream, LlmError> {
        let response = self.generate(request).await?;
        let chunk = GenerationChunk {
            request_id: response.request_id,
            provider: response.provider,
            model: response.model,
            delta: response.content,
            usage: Some(response.usage),
            finish_reason: Some(response.finish_reason),
        };
        Ok(Box::pin(stream::iter([Ok(chunk)])))
    }
}

#[derive(Clone, Debug, Default)]
pub struct DeterministicProvider;

#[async_trait]
impl LlmProvider for DeterministicProvider {
    fn id(&self) -> ProviderId {
        ProviderId::new("local.deterministic")
    }

    async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, LlmError> {
        let prompt = request
            .messages
            .last()
            .map(|message| message.content.as_str())
            .unwrap_or("");
        let content = if prompt.is_empty() {
            "CAT deterministic provider: empty input".to_owned()
        } else {
            format!("CAT deterministic provider: {prompt}")
        };
        let input_tokens = request
            .messages
            .iter()
            .map(|m| m.content.split_whitespace().count() as u64)
            .sum();
        let output_tokens = content.split_whitespace().count() as u64;
        Ok(GenerationResponse {
            request_id: request.request_id,
            provider: self.id(),
            model: request.model,
            content,
            usage: Usage {
                input_tokens,
                output_tokens,
                total_tokens: input_tokens + output_tokens,
            },
            finish_reason: "stop".to_owned(),
        })
    }

    async fn generate_stream(
        &self,
        request: GenerationRequest,
    ) -> Result<LlmGenerationStream, LlmError> {
        let response = self.generate(request).await?;
        let words: Vec<String> = response
            .content
            .split_whitespace()
            .map(ToOwned::to_owned)
            .collect();
        let last = words.len().saturating_sub(1);
        let chunks = words.into_iter().enumerate().map(move |(index, word)| {
            let delta = if index == last {
                word
            } else {
                format!("{word} ")
            };
            let terminal = index == last;
            let usage = terminal.then(|| response.usage.clone());
            let finish_reason = terminal.then(|| response.finish_reason.clone());
            Ok(GenerationChunk {
                request_id: response.request_id,
                provider: response.provider.clone(),
                model: response.model.clone(),
                delta,
                usage,
                finish_reason,
            })
        });
        Ok(Box::pin(stream::iter(chunks)))
    }
}
