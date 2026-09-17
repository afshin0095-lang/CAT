use std::cmp::Ordering;

use crate::{DocumentChunk, DocumentIndex, RagError, RetrievalHit, RetrievalQuery, Retriever};

#[derive(Default)]
pub struct InMemoryIndex {
    chunks: Vec<DocumentChunk>,
}

impl InMemoryIndex {
    pub fn len(&self) -> usize {
        self.chunks.len()
    }
    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    fn cosine(left: &[f32], right: &[f32]) -> Option<f32> {
        if left.len() != right.len() || left.is_empty() {
            return None;
        }
        let mut dot = 0.0;
        let mut left_norm = 0.0;
        let mut right_norm = 0.0;
        for (a, b) in left.iter().zip(right) {
            dot += a * b;
            left_norm += a * a;
            right_norm += b * b;
        }
        if left_norm == 0.0 || right_norm == 0.0 {
            return None;
        }
        Some(dot / (left_norm.sqrt() * right_norm.sqrt()))
    }
}

impl DocumentIndex for InMemoryIndex {
    fn upsert(&mut self, chunk: DocumentChunk) -> Result<(), RagError> {
        if let Some(existing) = self.chunks.iter_mut().find(|item| item.id == chunk.id) {
            *existing = chunk;
        } else {
            self.chunks.push(chunk);
        }
        Ok(())
    }

    fn delete(&mut self, chunk_id: uuid::Uuid) -> Result<bool, RagError> {
        let before = self.chunks.len();
        self.chunks.retain(|chunk| chunk.id != chunk_id);
        Ok(before != self.chunks.len())
    }
}

impl Retriever for InMemoryIndex {
    fn retrieve(&self, query: &RetrievalQuery) -> Result<Vec<RetrievalHit>, RagError> {
        if query.limit == 0 {
            return Err(RagError::InvalidLimit);
        }
        let embedding = query.embedding.as_ref().ok_or(RagError::MissingEmbedding)?;
        embedding.validate().map_err(RagError::InvalidEmbedding)?;

        let mut hits = Vec::new();
        for chunk in &self.chunks {
            if chunk.tenant_id != query.tenant_id {
                continue;
            }
            let Some(chunk_embedding) = chunk.embedding.as_ref() else {
                continue;
            };
            let Some(score) = Self::cosine(&embedding.values, &chunk_embedding.values) else {
                continue;
            };
            if query.min_score.map(|min| score >= min).unwrap_or(true) {
                hits.push(RetrievalHit {
                    chunk: chunk.clone(),
                    score,
                });
            }
        }
        hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
        hits.truncate(query.limit);
        Ok(hits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Embedding, RetrievalQuery};
    use serde_json::json;
    use uuid::Uuid;

    fn chunk(tenant_id: Uuid, ordinal: u32, values: Vec<f32>) -> DocumentChunk {
        DocumentChunk {
            id: Uuid::now_v7(),
            document_id: Uuid::now_v7(),
            tenant_id,
            ordinal,
            text: format!("chunk-{ordinal}"),
            content_hash: format!("hash-{ordinal}"),
            metadata: json!({"ordinal": ordinal}),
            embedding: Some(Embedding::new("test", values)),
        }
    }

    #[test]
    fn retrieval_is_tenant_isolated_and_ranked() {
        let tenant = Uuid::now_v7();
        let other = Uuid::now_v7();
        let mut index = InMemoryIndex::default();
        index.upsert(chunk(tenant, 1, vec![1.0, 0.0])).unwrap();
        index.upsert(chunk(tenant, 2, vec![0.7, 0.7])).unwrap();
        index.upsert(chunk(other, 3, vec![1.0, 0.0])).unwrap();

        let query = RetrievalQuery::new(tenant, "test")
            .with_embedding(Embedding::new("test", vec![1.0, 0.0]))
            .with_limit(2);
        let hits = index.retrieve(&query).unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].chunk.ordinal, 1);
        assert!(hits[0].score >= hits[1].score);
        assert!(hits.iter().all(|hit| hit.chunk.tenant_id == tenant));
    }

    #[test]
    fn upsert_replaces_existing_chunk() {
        let tenant = Uuid::now_v7();
        let mut index = InMemoryIndex::default();
        let mut first = chunk(tenant, 1, vec![1.0, 0.0]);
        let id = first.id;
        index.upsert(first.clone()).unwrap();
        first.text = "updated".into();
        index.upsert(first).unwrap();
        assert_eq!(index.len(), 1);
        assert_eq!(
            index.chunks.iter().find(|item| item.id == id).unwrap().text,
            "updated"
        );
    }
}
