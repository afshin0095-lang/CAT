# CAT Memory Core

`cat-memory` is the first implementation slice of the CAT Memory System Bible's P0 foundation: governed memory objects, immutable identity, provenance, consent, retention, lifecycle state, deterministic policy checks, and a deterministic in-memory store.

The crate deliberately does **not** make memory authoritative over domain truth. It provides a governed state boundary that can later be backed by PostgreSQL or another durable store without changing the object and policy contracts.

## Current scope

- versioned `MemoryObject` model
- typed classification and memory kinds
- provenance and consent metadata
- retention and legal-hold metadata
- lifecycle-state validation
- policy authorization boundary
- deterministic in-memory store for tests and local execution
- store-level validation helpers

Production persistence, retrieval/RAG, federation, and domain-specific memory schemas remain separate implementation stages.
