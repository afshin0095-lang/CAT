use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ModelId(pub String);
impl ModelId { pub fn new(value: impl Into<String>) -> Self { Self(value.into()) } }

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ProviderId(pub String);
impl ProviderId { pub fn new(value: impl Into<String>) -> Self { Self(value.into()) } }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role { System, User, Assistant, Tool }

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Message { pub role: Role, pub content: String }
impl Message {
    pub fn system(content: impl Into<String>) -> Self { Self { role: Role::System, content: content.into() } }
    pub fn user(content: impl Into<String>) -> Self { Self { role: Role::User, content: content.into() } }
    pub fn assistant(content: impl Into<String>) -> Self { Self { role: Role::Assistant, content: content.into() } }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafetyClass { Unclassified, Standard, Sensitive, Restricted }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub request_id: Uuid,
    pub model: ModelId,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_output_tokens: u32,
    pub safety: SafetyClass,
    pub correlation_id: Option<Uuid>,
}
impl GenerationRequest {
    pub fn new(model: ModelId, messages: Vec<Message>) -> Self {
        Self { request_id: Uuid::now_v7(), model, messages, temperature: 0.0, max_output_tokens: 1024, safety: SafetyClass::Standard, correlation_id: None }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Usage { pub input_tokens: u64, pub output_tokens: u64, pub total_tokens: u64 }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerationChunk {
    pub request_id: Uuid,
    pub provider: ProviderId,
    pub model: ModelId,
    pub delta: String,
    pub usage: Option<Usage>,
    pub finish_reason: Option<String>,
}
impl GenerationChunk {
    pub fn delta(request_id: Uuid, provider: ProviderId, model: ModelId, delta: impl Into<String>) -> Self {
        Self { request_id, provider, model, delta: delta.into(), usage: None, finish_reason: None }
    }
    pub fn terminal(mut self, usage: Usage, finish_reason: impl Into<String>) -> Self {
        self.usage = Some(usage);
        self.finish_reason = Some(finish_reason.into());
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerationResponse {
    pub request_id: Uuid,
    pub provider: ProviderId,
    pub model: ModelId,
    pub content: String,
    pub usage: Usage,
    pub finish_reason: String,
}
