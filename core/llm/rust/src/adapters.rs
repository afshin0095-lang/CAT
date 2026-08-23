use async_trait::async_trait;
use serde_json::{json, Value};

use crate::{GenerationRequest, GenerationResponse, LlmError, LlmProvider, Message, ProviderId, Role, Usage};
use crate::http::HttpLlmClient;

#[derive(Clone)]
pub struct OpenAiCompatibleProvider {
    client: HttpLlmClient,
}

impl OpenAiCompatibleProvider {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self, LlmError> {
        Ok(Self { client: HttpLlmClient::new(base_url, api_key)? })
    }

    fn request_body(request: &GenerationRequest) -> Value {
        json!({
            "model": request.model.0,
            "messages": request.messages.iter().map(message_json).collect::<Vec<_>>(),
            "temperature": request.temperature,
            "max_tokens": request.max_output_tokens,
        })
    }

    fn response_body(&self, request: &GenerationRequest, body: Value) -> Result<GenerationResponse, LlmError> {
        let content = body["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| LlmError::ProviderRejected("OpenAI-compatible response has no choices[0].message.content".to_owned()))?;
        Ok(GenerationResponse {
            request_id: request.request_id,
            provider: self.id(),
            model: request.model.clone(),
            content: content.to_owned(),
            usage: usage_from_openai(&body),
            finish_reason: body["choices"][0]["finish_reason"].as_str().unwrap_or("stop").to_owned(),
        })
    }
}

#[async_trait]
impl LlmProvider for OpenAiCompatibleProvider {
    fn id(&self) -> ProviderId { ProviderId::new("openai.compatible") }

    async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, LlmError> {
        let body = self.client.post_json("/chat/completions", Self::request_body(&request)).await?;
        self.response_body(&request, body)
    }
}

#[derive(Clone)]
pub struct AnthropicProvider {
    client: HttpLlmClient,
}

impl AnthropicProvider {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self, LlmError> {
        Ok(Self { client: HttpLlmClient::new(base_url, api_key)? })
    }

    fn request_body(request: &GenerationRequest) -> Value {
        let system = request.messages.iter()
            .filter(|message| message.role == Role::System)
            .map(|message| message.content.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");
        let messages = request.messages.iter()
            .filter(|message| message.role != Role::System)
            .map(|message| json!({
                "role": match message.role { Role::Assistant => "assistant", _ => "user" },
                "content": message.content,
            }))
            .collect::<Vec<_>>();
        json!({
            "model": request.model.0,
            "system": system,
            "messages": messages,
            "temperature": request.temperature,
            "max_tokens": request.max_output_tokens,
        })
    }

    fn response_body(&self, request: &GenerationRequest, body: Value) -> Result<GenerationResponse, LlmError> {
        let content = body["content"][0]["text"]
            .as_str()
            .ok_or_else(|| LlmError::ProviderRejected("Anthropic response has no content[0].text".to_owned()))?;
        let input_tokens = body["usage"]["input_tokens"].as_u64().unwrap_or(0);
        let output_tokens = body["usage"]["output_tokens"].as_u64().unwrap_or(0);
        Ok(GenerationResponse {
            request_id: request.request_id,
            provider: self.id(),
            model: request.model.clone(),
            content: content.to_owned(),
            usage: Usage { input_tokens, output_tokens, total_tokens: input_tokens + output_tokens },
            finish_reason: body["stop_reason"].as_str().unwrap_or("stop").to_owned(),
        })
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    fn id(&self) -> ProviderId { ProviderId::new("anthropic") }

    async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, LlmError> {
        let body = self.client.post_anthropic("/v1/messages", Self::request_body(&request)).await?;
        self.response_body(&request, body)
    }
}

fn message_json(message: &Message) -> Value {
    json!({
        "role": match message.role {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::Tool => "tool",
        },
        "content": message.content,
    })
}

fn usage_from_openai(body: &Value) -> Usage {
    let input_tokens = body["usage"]["prompt_tokens"].as_u64().unwrap_or(0);
    let output_tokens = body["usage"]["completion_tokens"].as_u64().unwrap_or(0);
    Usage { input_tokens, output_tokens, total_tokens: input_tokens + output_tokens }
}
