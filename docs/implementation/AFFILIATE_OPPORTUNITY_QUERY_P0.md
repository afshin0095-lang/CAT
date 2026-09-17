# Affiliate Opportunity Query P0

## Purpose

A persistence-neutral query boundary over opportunities: composable filters, deterministic sorting, stable pagination, and a rich read model (`OpportunityStatusView`) that future services, UIs, and agents share.

## Scope

- `core/affiliate/rust/src/opportunity_query.rs` — `OpportunityFilter`, `OpportunityQuery`, `OpportunityQueryService`, results/errors.
- `core/affiliate/rust/src/opportunity_projection.rs` — status projection read model.
- `core/affiliate/rust/src/opportunity_ranking.rs` — deterministic weighted ranking.
- `core/affiliate/rust/src/opportunity_version.rs` — revision contract used by the read model.

## Architecture

```text
OpportunityQuery (filter + sort + limit/offset)
        │ OpportunityQueryService::execute(records, query, now_ms)
        ├─ filter pass on record fields + derived lifecycle state (cheap, pushdown-able)
        ├─ deterministic sort (always identity-ascending final tiebreak)
        └─ project ONLY the returned page into OpportunityStatusView
        ▼
OpportunityQueryResult { items, total_matched, offset, limit }
```

The reference implementation evaluates over record slices; storage adapters translate the same filter/sort semantics natively. No dynamic SQL strings are built anywhere.

## Data model

`OpportunityStatusView` exposes: identity, ids, merchant/product/category, `lifecycle_state` (derived), `freshness`, `last_observed_at_ms`, `observation_age_ms`, `best_source`, `best_score`, `best_observation` (currency/price/commission of the best observation), `revision`, `health`, `needs_revalidation`, `revalidation_reasons`, `revalidation_blocked`.

## API contracts

- `OpportunityFilter`: merchant, source, category, min_score, lifecycle state, observed-after/before, currency, price range, commission range — composable builder methods, validated (`observed_after <= observed_before`, ordered ranges, `min_score <= 10_000`).
- `OpportunityQuery`: filter + `OpportunitySort` (field, direction) + `limit (1..=MAX_QUERY_LIMIT)` + `offset`.
- `OpportunityQueryService::execute` → `OpportunityQueryResult::has_more()`.
- `OpportunityStatusProjector::project` / `project_with_availability`.
- `OpportunityRanker` with a validated basis-point weight profile.

## State transitions

None — read-only boundary over derived state.

## Invariants

1. Deterministic output: equal inputs produce equal results; `OpportunityIdentity` ascending is always the final sort tiebreak.
2. Pagination windows never wrap; out-of-range offsets saturate at the match-set size.
3. `total_matched` is the pre-pagination match count.
4. Clock regression for any candidate record fails the query (fail closed), consistent with lifecycle evaluation.
5. Ranking weights must sum to exactly 10_000 bps; scores are bounded `0..=10_000`.
6. Filters match observations without mutating or normalizing identity data.

## Error handling

`OpportunityQueryError { InvalidFilter(String), InvalidLimit, ClockBeforeObservation }`; `OpportunityProjectionError { ClockBeforeObservation, InvalidPolicy }`; `OpportunityRankingError { InvalidProfile, ClockBeforeObservation }` — all `Validation` category.

## Persistence behavior

None at the domain boundary. The Postgres store persists the facts the projection reads; adapters may push down filters later without changing this contract.

## Concurrency behavior

Queries operate on snapshots; concurrent upserts are serialized by the store's revision machinery and simply appear in later query results.

## Idempotency

Queries are pure reads; repeated execution is stable given unchanged facts and `now_ms`.

## Testing strategy

Filter-combination integration tests, observation-scoped filters (any-source matching), deterministic sort + tiebreak tests, pagination window/saturation tests, clock-regression failures, ranking boundary/overflow/determinism tests, projection serialization round trips.

## Security considerations

Read-only domain objects; no I/O; no provider data exposure.

## Future extension points

- Native SQL pushdown of `OpportunityFilter` in `PostgresOpportunityStore`.
- Cursor-based pagination layered over the deterministic identity tiebreak.
- Ranking profile versioning once AI agents contribute factor models (through contracts, never hardcoded providers).

## Known limitations

- In-memory execution is O(n) per query; database-backed deployments should push down filters before page projection.
