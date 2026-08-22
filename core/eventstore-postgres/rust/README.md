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
- monotonic projection checkpoints isolated from canonical event history
- equal-sequence checkpoint writes cannot replace the canonical event identity

The adapter is intentionally separate from `cat-kernel`: the kernel owns event semantics while infrastructure owns persistence. PostgreSQL transactions provide the all-or-nothing boundary required for the stream version, event row, and idempotency record to commit together. PostgreSQL documents transactions as atomic and isolated, and SQLx provides the asynchronous PostgreSQL driver used by this adapter.

## Projection Checkpoints

Projection checkpoints are derived execution state. They identify a `(projection_id, stream_id)` pair and persist the last accepted `(sequence, event_id, phase)`.

The persistence contract is deliberately monotonic:

1. A higher sequence advances the checkpoint.
2. A replay of the same sequence and event is idempotent.
3. A different event at an already committed sequence is ignored rather than replacing canonical identity.
4. A stale lower sequence is ignored.
5. Checkpoints are independent from the canonical event log and may be rebuilt.

The PostgreSQL implementation lives in `src/checkpoint.rs`; the schema is created by the projection-checkpoint migration.

### Integration Contract Tests

The ignored PostgreSQL integration tests in `tests/checkpoint.rs` cover:

- monotonic advancement and live-phase transition
- idempotent replay of the same checkpoint
- protection against equal-sequence event identity replacement
- rejection of stale checkpoint advancement
- isolation between independent projection/stream pairs

Run them against a disposable PostgreSQL instance with:

```bash
CAT_TEST_DATABASE_URL='postgres://cat:cat@localhost:5432/cat_test' \
  cargo test -p cat-eventstore-postgres --test checkpoint -- --ignored
```

The tests run the crate migrations before exercising the store and clean up their checkpoint rows afterward.
