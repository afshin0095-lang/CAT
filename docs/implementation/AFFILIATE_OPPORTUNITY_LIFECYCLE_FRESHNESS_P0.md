# Affiliate Opportunity Lifecycle & Freshness P0

## Purpose

This stage adds a storage-neutral lifecycle projection for persisted affiliate opportunities. It determines whether an opportunity is **Active**, **Stale**, or **Expired** from the timestamp of its most recent observation.

## Contract

`OpportunityLifecyclePolicy` requires:

- `0 < stale_after_ms < expire_after_ms`
- timestamps are unsigned milliseconds
- evaluation never mutates the persisted `OpportunityRecord`

State rules are deterministic:

| Condition | State |
|---|---|
| `age < stale_after_ms` | Active |
| `stale_after_ms <= age < expire_after_ms` | Stale |
| `age >= expire_after_ms` | Expired |

An evaluation clock earlier than `last_observed_at_ms` is rejected. This prevents unsigned subtraction from wrapping into a false expired/active state.

## Batch behavior

`evaluate_records` evaluates every record against one `now_ms` and sorts output by `OpportunityIdentity`. Ordering therefore does not depend on database iteration order.

## Architectural boundary

This P0 deliberately does **not** persist lifecycle state. Lifecycle is a derived view over durable opportunity observations. The next persistence stage can add indexed queries, lifecycle events, and scheduled expiration processing without changing this domain contract.

## Safety invariants

1. Freshness is derived from `last_observed_at_ms`, not creation time.
2. Exact threshold equality is deterministic.
3. Clock regression fails closed.
4. Evaluation is side-effect free.
5. Batch ordering is stable.
6. The lifecycle policy is immutable after construction.

## Next stage

The next affiliate increment should connect this projection to durable persistence with indexed freshness queries and an expiration/revalidation workflow. Expiration must trigger revalidation rather than silently deleting commercial provenance.
