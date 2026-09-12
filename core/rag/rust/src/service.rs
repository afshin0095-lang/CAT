use crate::{Embedder, RagError, RagResult, RetrievalHit, RetrievalQuery, Retriever};
use uuid::Uuid;

/// Provider-neutral retrieval orchestration.
///
/// The service owns query preparation only: embedding generation, tenant identity,
/// and retrieval dispatch. It deliberately does not own ranking policy or storage.
pub struct RetrievalService<E, R> {
    embedder: E,
    retriever: R,
}

impl<E, R> RetrievalService<E, R> {
    pub fn new(embedder: E, retriever: R) -> Self {
        Self {
            embedder,
            retriever,
        }
    }

    pub fn embedder(&self) -> &E {
        &self.embedder
    }

    pub fn retriever(&self) -> &R {
        &self.retriever
    }

    pub fn into_parts(self) -> (E, R) {
        (self.embedder, self.retriever)
    }
}

impl<E, R> RetrievalService<E, R>
where
    E: Embedder,
    R: Retriever,
{
    /// Embeds the query and executes vector retrieval for exactly one tenant.
    pub fn retrieve(
        &self,
        tenant_id: Uuid,
        query: impl Into<String>,
        limit: usize,
        min_score: Option<f32>,
    ) -> RagResult<Vec<RetrievalHit>> {
        let query = query.into();
        if query.trim().is_empty() {
            return Err(RagError::EmptyQuery);
        }
        if limit == 0 {
            return Err(RagError::InvalidLimit);
        }
        if min_score.is_some_and(|score| !score.is_finite()) {
            return Err(RagError::InvalidScore);
        }

        let embedding = self.embedder.embed(&query)?;
        embedding.validate().map_err(RagError::InvalidEmbedding)?;

        let request = RetrievalQuery::new(tenant_id, query)
            .with_embedding(embedding)
            .with_limit(limit)
            .with_min_score(min_score.unwrap_or(f32::NEG_INFINITY));

        self.retriever.retrieve(&request)
    }
}
