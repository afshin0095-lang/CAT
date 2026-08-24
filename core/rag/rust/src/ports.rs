use crate::{Embedding, RetrievalHit, RetrievalQuery};

pub trait Embedder: Send + Sync {
    fn embed(&self, text: &str) -> Result<Embedding, crate::RagError>;
}

pub trait Retriever: Send + Sync {
    fn retrieve(&self, query: &RetrievalQuery) -> Result<Vec<RetrievalHit>, crate::RagError>;
}

pub trait DocumentIndex: Send + Sync {
    fn upsert(&mut self, chunk: crate::DocumentChunk) -> Result<(), crate::RagError>;
    fn delete(&mut self, chunk_id: uuid::Uuid) -> Result<bool, crate::RagError>;
}
