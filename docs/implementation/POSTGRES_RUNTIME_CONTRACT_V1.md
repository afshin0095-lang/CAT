# CAT PostgreSQL Runtime Contract V1

## Scope

This contract defines the production PostgreSQL boundary for CAT durable execution. The Rust domain model remains independent of a specific SQL client, while the Orchestrator PostgreSQL adapter uses an asynchronous connection pool.

## Schema bootstrap

`PostgresExecutionStore::ensure_schema()` runs the canonical SQLx migration set from:

`core/orchestrator/rust/migrations/`

Durable execution workers must not become active until schema migration succeeds.

## Authoritative state

`cat_workflows.revision` is the optimistic concurrency version. A workflow update must use:

```sql
UPDATE cat_workflows
SET state = $new_state,
    revision = $new_revision,
    updated_at = NOW()
WHERE id = $workflow_id
  AND revision = $expected_revision;
```

Exactly one affected row is required. A zero-row update is a concurrency failure and must not be treated as success.

## Transactional outbox

The workflow update and all resulting `cat_workflow_outbox` inserts belong to one database transaction. The transaction is committed before any external EventBus publication is attempted.

This gives the system restart safety:

```text
DB transaction
   ├─ workflow revision N -> N+1
   └─ outbox event(s)
          ↓
       COMMIT
          ↓
   dispatcher publishes
          ↓
   acknowledgement
```

The full event envelope metadata is persisted with the outbox record: event kind, producer, correlation ID, causation ID, subject ID, type, version, timestamp, and payload.

If a process crashes after commit and before acknowledgement, the outbox remains available for another dispatcher instance.

## Outbox claiming

Production dispatchers use row-level locking with `FOR UPDATE SKIP LOCKED` semantics. A claim has an owner and expiration. Expired claims can be reclaimed, while acknowledgement still requires the original claim owner.

## Fencing lifecycle

A fenced lease has three explicit operations:

1. `acquire_fenced_lease` obtains a monotonically increasing token for a resource.
2. `renew_fenced_lease` extends the lease only when resource, owner, token, and current lease validity all match.
3. `validate_fencing_token` verifies that the token is still current and the lease has not expired before a side-effect-capable operation.

Stale tokens are rejected. Ownership alone is never sufficient protection against an old worker continuing after lease turnover.

## Retry and dead letters

Publication failures increment the outbox attempt counter. Backoff is bounded by the CAT retry policy. Once retries are exhausted, the normal retry path ends and the event is reported as dead-lettered. A future durable DLQ workflow can consume that disposition without changing the outbox contract.

## Migration

The canonical schema migration creates:

- `cat_workflows`
- `cat_workflow_outbox`
- `cat_execution_leases`
- `cat_execution_attempts`
- `cat_execution_authorizations`

The migration also includes compatibility-safe column additions for already-created outbox tables.

## Deployment rules

1. Schema migration must complete before enabling durable execution workers.
2. Database clocks should be synchronized because lease expiration and retry scheduling depend on time.
3. Connection pools must expose bounded concurrency and timeout behavior.
4. Side-effect adapters must validate fencing before committing externally visible work.
5. PostgreSQL remains an implementation choice behind the durable contract; a future strongly consistent store may implement the same semantics.
