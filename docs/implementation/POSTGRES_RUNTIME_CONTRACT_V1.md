# CAT PostgreSQL Runtime Contract V1

## Scope

This contract defines the production PostgreSQL boundary for CAT durable execution. The Rust domain model remains independent of a specific SQL client, while the Orchestrator PostgreSQL adapter uses an asynchronous connection pool.

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

If a process crashes after commit and before acknowledgement, the outbox remains available for another dispatcher instance.

## Outbox claiming

Production dispatchers use row-level locking with `FOR UPDATE SKIP LOCKED` semantics. A claim has an owner and expiration. A worker whose claim expires must not be allowed to acknowledge the event under a different owner.

## Fencing

`cat_execution_leases.fencing_token` increases monotonically for each resource. A lease turnover produces a new token. Side-effect-capable adapters must reject stale tokens rather than trusting lease ownership alone.

## Retry and dead letters

Publication failures increment the outbox attempt counter. Backoff is bounded by the CAT retry policy. Once retries are exhausted, the event leaves the normal retry path and may be routed to a dead-letter workflow.

## Migrations

The canonical schema migration for this runtime is:

`core/orchestrator/rust/migrations/0001_durable_execution.sql`

The migration creates:

- `cat_workflows`
- `cat_workflow_outbox`
- `cat_execution_leases`

## Deployment rules

1. Schema migration must complete before enabling durable execution workers.
2. Database clocks should be synchronized because lease expiration and retry scheduling depend on time.
3. Connection pools must expose bounded concurrency and timeout behavior.
4. PostgreSQL remains an implementation choice behind the durable contract; a future strongly consistent store may implement the same semantics.
