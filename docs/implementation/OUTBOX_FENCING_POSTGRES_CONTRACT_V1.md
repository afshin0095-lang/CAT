# CAT Outbox, Fencing, and PostgreSQL Contract V1

## Purpose

This contract extends durable execution from a state-and-outbox boundary into a restart-safe delivery model suitable for clustered workers.

## Outbox invariants

1. The workflow transaction writes canonical state and its integration/domain events before publication.
2. Each outbox event has a stable `event_id` and is safe to publish more than once because downstream consumers must remain idempotent.
3. A dispatcher claims work, publishes it, then acknowledges it.
4. A failed publication clears the claim and schedules bounded retry until the retry policy is exhausted.
5. Exhausted events leave the retry path and may be routed to a dead-letter workflow.
6. Persistence must ensure two active dispatchers do not acknowledge the same claim without a valid ownership check.

## Fencing invariants

A lease alone is not sufficient when a slow worker can continue after its lease expires. CAT therefore uses a monotonically increasing fencing token per resource.

```text
worker A obtains token 41
      |
      | lease expires
      v
worker B obtains token 42
      |
      +----> side effects must accept 42 and reject 41
```

The token must be persisted with the lease and checked at the side-effect boundary. A stale worker must not be able to commit merely because it still holds an old in-memory lease object.

## PostgreSQL mapping

The reference schema reserves three tables:

- `cat_workflows`: authoritative workflow state and revision.
- `cat_workflow_outbox`: durable event delivery queue with claim/retry metadata.
- `cat_execution_leases`: lease owner, expiration, and fencing token.

A production implementation should use a transaction for workflow state plus outbox writes, optimistic revision checks on workflow updates, and row-level locking/claim semantics for outbox delivery.

## Delivery flow

```text
transaction
   |
   +--> workflow row revision N -> N+1
   +--> outbox row INSERT
   |
   v
commit
   |
   v
OutboxDispatcher
   |
   +--> claim
   +--> publish to EventBus / NATS
   +--> acknowledge
   |
   +--> failure -> bounded retry -> dead letter
```

## Separation of concerns

The Orchestrator crate intentionally does not select `sqlx`, `tokio-postgres`, Diesel, a particular connection pool, or a particular distributed lease service. Those remain adapters selected through CAT technology decision records.

The current implementations are deterministic contract/test implementations. They are not production database or cluster coordination implementations.
