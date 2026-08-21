# CAT PostgreSQL Event Store Adapter

This crate is the first durable infrastructure adapter for the kernel event-store contract.

## Guarantees

- append-only event rows per stream
- explicit per-stream sequence numbers
- optimistic concurrency using the kernel `ExpectedVersion` contract
- idempotency receipts persisted transactionally
- JSONB envelope preservation without imposing a domain payload schema
- deterministic ordered stream reads
- schema creation through an explicit SQL migration

The adapter is intentionally separate from `cat-kernel`: the kernel owns event semantics while infrastructure owns persistence. PostgreSQL transactions provide the all-or-nothing boundary required for the stream version, event row, and idempotency record to commit together. PostgreSQL documents transactions as atomic and isolated, and SQLx provides the asynchronous PostgreSQL driver used by this adapter. 
