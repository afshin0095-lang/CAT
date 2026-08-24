use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Embedding {
    pub model: String,
    pub dimensions: u32,
    pub values: Vec<f32>,
}

impl Embedding {
    pub fn new(model: impl Into<String>, values: Vec<f32>) -> Self {
        let dimensions = values.len() as u32;
        Self { model: model.into(), dimensions, values }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.values.len() as u32 != self.dimensions {
            return Err("embedding dimensions do not match vector length".into());
        }
        if self.values.iter().any(|value| !value.is_finite()) {
            return Err("embedding contains a non-finite value".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DocumentChunk {
    pub id: Uuid,
    pub document_id: Uuid,
    pub tenant_id: Uuid,
    pub ordinal: u32,
    pub text: String,
    pub content_hash: String,
    pub metadata: serde_json::Value,
    pub embedding: Option<Embedding>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RetrievalQuery {
    pub tenant_id: Uuid,
    pub query: String,
    pub embedding: Option<Embedding>,
    pub limit: usize,
    pub min_score: Option<f32>,
}

impl RetrievalQuery {
    pub fn new(tenant_id: Uuid, query: impl Into<String>) -> Self {
        Self { tenant_id, query: query.into(), embedding: None, limit: 10, min_score: None }
    }

    pub fn with_embedding(mut self, embedding: Embedding) -> Self {
        self.embedding = Some(embedding);
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit.max(1);
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RetrievalHit {
    pub chunk: DocumentChunk,
    pub score: f32,
}
