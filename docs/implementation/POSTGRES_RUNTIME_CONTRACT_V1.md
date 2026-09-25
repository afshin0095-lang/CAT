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

## Outbox claiming

Production dispatchers use row-level locking with `FOR UPDATE SKIP LOCKED` semantics. A claim has an owner and expiration. Expired claims can be reclaimed, while acknowledgement still requires the original claim owner.

## Fencing lifecycle

A fenced lease has three explicit operations:

1. `acquire_fenced_lease` obtains a monotonically increasing token for a resource.
2. `renew_fenced_lease` extends the lease only when resource, owner, token, and current lease validity all match.
3. `validate_fencing_token` verifies that the token is still current and the lease has not expired before a side-effect-capable operation.

## Execution and governance evidence

The canonical durable execution tables are:

- `cat_execution_attempts`
- `cat_execution_authorizations`

Attempt start and authorization evidence are committed in one transaction before worker dispatch.

## Provider journaling

The provider subsystem adds:

- `cat_provider_execution_results`: current provider execution state;
- `cat_provider_execution_journal`: append-only provider execution history.

Provider submission/observation mutations update current state and append the corresponding journal record in the same transaction. Event keys provide idempotent replay safety, while execution/provider identities are conflict-checked.

## Operator audit

The audit subsystem adds:

- `cat_execution_audit_events`: append-only audit evidence;
- `cat_execution_audit_read_model`: latest operator-facing view per execution.

An audit append transaction updates the read model with the source audit sequence. The read model is derived and can be rebuilt from the audit log.

Operator queries are bounded and filtered by normalized governance fields such as agent ID, capability ID, and reconciliation action. Query authorization belongs to the higher-level Control Plane; the storage layer does not treat a read query as permission to modify execution.

## Retry and dead letters

Publication failures increment the outbox attempt counter. Backoff is bounded by the CAT retry policy. Once retries are exhausted, the normal retry path ends and the event is reported as dead-lettered.

## Deployment rules

1. Schema migration must complete before enabling durable execution workers.
2. Database clocks should be synchronized because lease expiration and retry scheduling depend on time.
3. Connection pools must expose bounded concurrency and timeout behavior.
4. Side-effect adapters must validate fencing before committing externally visible work.
5. PostgreSQL remains an implementation choice behind the durable contract; a future strongly consistent store may implement the same semantics.
