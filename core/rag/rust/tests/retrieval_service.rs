use cat_rag::{DocumentChunk, Embedding, Embedder, InMemoryIndex, RagError, RetrievalService, DocumentIndex};
use serde_json::json;
use uuid::Uuid;

#[derive(Clone)]
struct StaticEmbedder;

impl Embedder for StaticEmbedder {
    fn embed(&self, _text: &str) -> Result<Embedding, RagError> {
        Ok(Embedding::new("test-model", vec![1.0, 0.0]))
    }
}

fn chunk(tenant_id: Uuid, ordinal: u32, values: Vec<f32>) -> DocumentChunk {
    DocumentChunk {
        id: Uuid::now_v7(),
        document_id: Uuid::now_v7(),
        tenant_id,
        ordinal,
        text: format!("chunk-{ordinal}"),
        content_hash: format!("hash-{ordinal}"),
        metadata: json!({"ordinal": ordinal}),
        embedding: Some(Embedding::new("test-model", values)),
    }
}

#[test]
fn service_embeds_and_retrieves_with_tenant_isolation() {
    let tenant = Uuid::now_v7();
    let other = Uuid::now_v7();
    let mut index = InMemoryIndex::default();
    index.upsert(chunk(tenant, 1, vec![1.0, 0.0])).unwrap();
    index.upsert(chunk(tenant, 2, vec![0.0, 1.0])).unwrap();
    index.upsert(chunk(other, 3, vec![1.0, 0.0])).unwrap();

    let service = RetrievalService::new(StaticEmbedder, index);
    let hits = service.retrieve(tenant, "affiliate attribution", 5, Some(0.5)).unwrap();

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].chunk.ordinal, 1);
    assert_eq!(hits[0].chunk.tenant_id, tenant);
}

#[test]
fn service_rejects_empty_queries_before_embedding() {
    let service = RetrievalService::new(StaticEmbedder, InMemoryIndex::default());
    let error = service.retrieve(Uuid::now_v7(), "   ", 5, None).unwrap_err();
    assert!(matches!(error, RagError::EmptyQuery));
}

#[test]
fn service_rejects_non_finite_score_thresholds() {
    let service = RetrievalService::new(StaticEmbedder, InMemoryIndex::default());
    let error = service.retrieve(Uuid::now_v7(), "query", 5, Some(f32::NAN)).unwrap_err();
    assert!(matches!(error, RagError::InvalidScore));
}

#[test]
fn service_rejects_zero_limit() {
    let service = RetrievalService::new(StaticEmbedder, InMemoryIndex::default());
    let error = service.retrieve(Uuid::now_v7(), "query", 0, None).unwrap_err();
    assert!(matches!(error, RagError::InvalidLimit));
}
