# Affiliate Opportunity Health P0

## Purpose

Health answers a question freshness cannot: **can the current observation still be trusted and acted on?** Sprint 0 introduces opportunity-level health assessment and source-level health tracking as two deliberately independent dimensions from lifecycle.

## Scope

- `core/affiliate/rust/src/opportunity_health.rs` — `OpportunityHealth`, `OpportunityHealthAssessor`, `HealthConcern`, `OpportunityHealthAssessment`.
- `core/affiliate/rust/src/discovery_source_health.rs` — `SourceHealthStore`, `InMemorySourceHealthStore`, `SourceHealthSnapshot`, `SourceHealthState`.
- `core/affiliate/rust/src/opportunity_projection.rs` — health surfaced on `OpportunityStatusView`.

## Architecture

Two orthogonal dimensions:

```text
Lifecycle (age of facts)      Health (trust in facts)
  Active / Stale / Expired      Healthy / Degraded / Unavailable / NeedsRevalidation
```

Every combination is representable: `Active + Degraded` (fresh observation, struggling source) and `Stale + Healthy` (old observation, fine source) are both valid and both tested. Never merge the enums.

## Data model

- `SourceHealthSnapshot { source, state, consecutive_failures, total_successes, total_failures, last_success_at_ms, last_failure_at_ms, last_success_latency_ms, availability_bps: Option<u16> }`.
- Health is not persisted in Sprint 0; snapshots derive from observation history (`SourceHealthStore` is the persistence-neutral contract).

## API contracts

- `OpportunityHealthAssessor::assess(record, freshness, Option<&SourceHealthSnapshot>, revalidation_pending) -> OpportunityHealthAssessment`.
- `SourceHealthStore`: `record_success(source, at_ms, latency_ms)`, `record_failure(source, at_ms, bounded_reason)`, `snapshot(source)`, `all_snapshots()`.

## State transitions

Assessment precedence (documented, deterministic):

1. no observations → `Unavailable`;
2. best source `Unavailable` → `Unavailable`;
3. best source `Degraded` → `Degraded`;
4. `revalidation_pending && otherwise Healthy` → `NeedsRevalidation`;
5. otherwise → `Healthy`.

Source health state machine: `Unknown → Healthy` on first success; ≥ 2 consecutive failures → `Degraded`; ≥ 5 → `Unavailable`; any success resets the streak (history preserved).

## Invariants

1. Health never derives from observation age; staleness never implies unhealthiness.
2. `needs_revalidation` is an explicit flag, never inferred from the health enum.
3. Missing or `Unknown` source health never penalizes an opportunity (benefit of the doubt), but `Unknown` is reported as a concern.
4. `availability_bps` uses `u128` intermediates; no overflow at extreme counts.
5. Failure reasons are bounded (128 chars) and control-character sanitized.
6. `all_snapshots()` iterates in deterministic (identifier) order.

## Error handling

Pure functions; no error paths. Invalid inputs (empty records) map to `Unavailable` + `NoObservations` concern.

## Persistence behavior

None in Sprint 0. The `SourceHealthStore` contract accepts future durable adapters without changing consumers.

## Concurrency behavior

`InMemorySourceHealthStore` is single-owner (`&mut self`); concurrent deployments shard per source or wrap with external synchronization at the adapter layer.

## Idempotency

Replaying identical success/failure observations converges the counters to the same state.

## Testing strategy

Full lifecycle × health matrix tests (`Active+Degraded`, `Stale+Healthy`, `Expired+NeedsRevalidation`), precedence tests (revalidation never masks degradation or unavailability), source journey tests (`Unknown → Healthy → Unavailable → Healthy`), serialization round trips, sanitization bounds.

## Security considerations

Failure reasons accept only short operator labels; raw provider payloads are rejected by contract and sanitized defensively.

## Future extension points

- Durable source-health adapters and dashboards via `observability` metric samples.
- Circuit-breaker policies layered over snapshots (explicitly out of Sprint 0 scope).

## Known limitations

- Health is process-local until a durable store adapter exists.
- `NeedsRevalidation` is an aggregate label for convenience; the authoritative signal is the `needs_revalidation` flag.
