# CAT RAG Core

`cat-rag` is CAT's provider-neutral retrieval boundary. It keeps embedding, indexing, and retrieval contracts independent from any specific vector database, embedding vendor, or model provider.

## Boundary

- `Embedding` represents validated model output without selecting a provider.
- `DocumentChunk` is tenant-scoped retrieval material with provenance metadata.
- `RetrievalQuery` carries the tenant boundary, query, optional embedding, limit, and score threshold.
- `Embedder` is the model/provider adapter boundary.
- `Retriever` is the query boundary.
- `DocumentIndex` is the indexing lifecycle boundary.
- `InMemoryIndex` is the deterministic local implementation used for unit tests and early integration work.

Canonical knowledge truth remains outside this crate. RAG results are derived evidence and MUST NOT silently become canonical domain truth.

## Security invariants

1. Retrieval is tenant-isolated.
2. Embeddings reject non-finite values and dimension mismatches.
3. Provider-specific implementations belong behind the ports.
4. Retrieval results are scored evidence, not decisions.
5. Production vector stores can replace `InMemoryIndex` without changing the query contract.

## Current scope

This first implementation intentionally stops at the retrieval contract and deterministic in-memory index. Provider adapters, persistent vector storage, hybrid lexical/vector search, reranking, and GraphRAG orchestration should be added behind these interfaces rather than leaking provider APIs into CAT domains.
