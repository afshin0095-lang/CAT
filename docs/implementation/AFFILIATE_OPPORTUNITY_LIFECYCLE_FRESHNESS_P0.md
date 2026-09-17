# Affiliate Opportunity Lifecycle & Freshness P0

## Purpose

Defines the storage-neutral lifecycle projection for persisted affiliate opportunities: whether an opportunity is **Active**, **Stale**, or **Expired**, derived deterministically from the age of its most recent observation. Sprint 0 hardens the original P0 by moving the derivation into a single canonical engine (`opportunity_freshness`) and adding aggregated batch evaluation.

## Scope

- `core/affiliate/rust/src/opportunity_freshness.rs` — canonical policy/state engine.
- `core/affiliate/rust/src/opportunity_lifecycle.rs` — lifecycle-facing vocabulary and batch evaluation.
- `core/affiliate/rust/src/clock.rs` — injectable clock for real-time evaluation.
- `core/affiliate/rust/tests/opportunity_freshness.rs`, `tests/opportunity_lifecycle.rs` — contract tests.

## Architecture

Freshness is a **derived projection** over durable observation facts. Exactly one module owns the derivation:

```text
FreshnessPolicy (3 thresholds)
        │ evaluate(record, evaluation_time)
        ▼
FreshnessEvaluation { state, age_ms, is_fresh }
        │ From<FreshnessState>
        ▼
OpportunityLifecycleState { Active, Stale, Expired }
```

`OpportunityLifecyclePolicy` remains the two-threshold public API (`stale_after_ms`, `expire_after_ms`) and maps losslessly onto `FreshnessPolicy` with `active_threshold_ms == stale_after_ms`. The two vocabularies can never disagree because neither computes state independently.

## Data model

No lifecycle state is persisted. Persisted facts remain:

- `cat_affiliate_opportunities.first_observed_at_ms`, `last_observed_at_ms`, `version`
- `cat_affiliate_opportunity_observations.observed_at_ms`

## API contracts

- `FreshnessPolicy::new(active, stale, expiration) -> Result<FreshnessPolicy, FreshnessPolicyError>`
- `FreshnessPolicy::evaluate_age(age_ms) -> FreshnessEvaluation`
- `FreshnessPolicy::evaluate(record, evaluation_time_ms) -> Result<RecordFreshnessEvaluation, FreshnessPolicyError>`
- `OpportunityLifecycleEvaluator::evaluate(record, now_ms) -> Result<OpportunityLifecycleSnapshot, OpportunityLifecycleError>` (unchanged public API)
- `evaluate_records(records, evaluator, now_ms)` (unchanged) and `evaluate_records_batch(...) -> LifecycleEvaluationBatch` with deterministic `counts` and identity-sorted `evaluations`.
- `Clock` / `SystemClock` / `FixedClock` for injected time.

## State transitions

| Condition (age of last observation) | State |
|---|---|
| `age < stale_threshold_ms` | Active |
| `stale_threshold_ms <= age < expiration_threshold_ms` | Stale |
| `age >= expiration_threshold_ms` | Expired |

Transitions are pure re-derivations; there is no stored state machine.

## Invariants

1. Freshness derives from `last_observed_at_ms`, never creation time.
2. Exact threshold equality is deterministic (`>=` boundaries).
3. Clock regression (`evaluation_time < last_observed_at_ms`) fails closed.
4. Integer milliseconds only; no floating point; checked arithmetic.
5. Evaluation is side-effect free and repeatable.
6. Batch ordering is by identity, never storage iteration order.
7. Thresholds are bounded by `MAX_THRESHOLD_MS`; `active <= stale < expiration`.
8. No hidden system-clock reads in domain functions; real time arrives via injected `Clock`.

## Error handling

`FreshnessPolicyError { ActiveBeyondStale, StaleBeyondExpiration, ThresholdTooLarge, ClockBeforeObservation }`; the lifecycle module maps these onto its stable two-variant error. All are `Validation` category per `ErrorClassification`.

## Persistence behavior

None by design (projection). Persistence layers keep observation facts; indexed freshness queries are a later optimization that can reuse `last_observed_at_ms` directly.

## Concurrency behavior

Evaluation takes owned/copied inputs; concurrent re-observation changes facts and every later evaluation reflects them. No read-modify-write cycles exist.

## Idempotency

Repeated evaluation of the same record at the same timestamp yields identical output.

## Testing strategy

Table-driven threshold-boundary tests, age-monotonicity sweep (0..=300ms), clock-regression rejection across threshold shapes, lifecycle↔freshness vocabulary equivalence over parameterized policies, serialization round trips, and batch determinism/count aggregation.

## Security considerations

Pure computation, no I/O, no provider data in errors.

## Future extension points

- Persisted fact `last_revalidated_at_ms` (Sprint 1) feeding freshness if product requires revalidation-aware aging.
- Event emission boundary: a **detection boundary** may emit `OpportunityBecameStale` / `OpportunityBecameExpired` facts once, when it first observes a transition; read queries must never emit them.

## Known limitations (documented, deliberate)

- `canonical_key` normalization remains ASCII-oriented; non-ASCII merchant/product names fail closed rather than silently merging identities. See `discovery::canonical_key` documentation and the ingestion doc.
