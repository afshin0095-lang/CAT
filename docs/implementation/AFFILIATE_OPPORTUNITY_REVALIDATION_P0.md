# Affiliate Opportunity Revalidation P0

## Purpose

Defines the persistence-neutral revalidation contract: what should be revalidated, why, with what priority, against which source, and how requests deduplicate and persist. The domain decision layer remains pure, while Sprint 1 now provides an executable governed Orchestrator bridge.

## Scope

- `core/affiliate/rust/src/opportunity_revalidation.rs` — request/status/decision contracts + in-memory store.
- `core/affiliate/rust/src/revalidation_planner.rs` — deterministic planner.
- `core/affiliate/rust/src/opportunity_postgres.rs` — durable request persistence (`PostgresRevalidationStore`).
- `core/affiliate/rust/migrations/0002_opportunity_revalidation.sql` — additive migration;
- `core/affiliate/rust/src/governed_revalidation.rs` — capability/workflow/worker/runner bridge;
- `core/affiliate/rust/tests/governed_revalidation_postgres.rs` — end-to-end governed execution verification.

## Architecture

```text
OpportunityRecord + FreshnessEvaluation + source availability
        │ RevalidationPlanner (pure)
        ▼
RevalidationDecision { requests, skipped, blocked }
        │ store.insert (idempotent on dedup_key)
        ▼
cat_affiliate_revalidation_requests   ←── governed runner claims here
        │
        ▼
Capability Registry → Admission → Fenced Durable Execution → Revalidation Worker
```

The planner never performs I/O and never mutates the record. Execution (calling sources, merging results) is still outside this pure module. `GovernedRevalidationCoordinator` owns claim/transition/workflow registration/admission/dispatch bookkeeping; `GovernedRevalidationExecutor` remains the injected source-side implementation.

## Data model

`cat_affiliate_revalidation_requests` (see migration `0002`):

| Column | Meaning |
|---|---|
| `request_id` | UUID, unique per construction (not the idempotency identity) |
| `opportunity_id`, `identity`, `source` | revalidation target |
| `reason` | forward-compatible snake_case string (no CHECK — new versions may add reasons) |
| `priority` | `low < normal < high < critical` (CHECK-constrained closed set) |
| `status` | `pending/claimed/running/succeeded/failed/cancelled/dead_lettered` (CHECK-constrained) |
| `dedup_key` | `opp-revalidation:{identity}:{source}:{reason}:{window bucket}` |
| `created_at_ms`, `scheduled_at_ms`, `started_at_ms`, `completed_at_ms` | facts |
| `attempt`, `last_error` | bounded bookkeeping |

Partial unique index `uq_cat_affiliate_revalidation_active_dedup` enforces: **only active requests (pending/claimed/running) suppress duplicates; terminal requests never block re-issue.**

## API contracts

- `RevalidationReason` (8 known values + forward-compatible `Unknown(String)`; serde accepts unknown strings, `parse_strict` rejects them on write paths).
- `RevalidationPriority` (ordered; `Critical > High > Normal > Low`).
- `RevalidationRequest::new(...)` / `::with_dedup_window(...)`.
- `RevalidationDecision { requests, skipped, blocked }` with `requires_revalidation()` and deterministic `reasons()`.
- `RevalidationPlanner::plan` / `plan_for_record` / `RevalidationPlanningOutcome::plan_batch` (identity-ordered, globally deduplicated `unique_requests()`).
- `RevalidationRequestStore` (sync, in-memory) and `AsyncRevalidationRequestStore` (Postgres): `insert`, `get`, `transition`, `claim_due`.

## State transitions

```text
Pending   -> Claimed | Running | Cancelled
Claimed   -> Running | Cancelled
Running   -> Succeeded | Failed | DeadLettered | Cancelled
Failed    -> Pending (retry) | DeadLettered
Succeeded | Cancelled | DeadLettered            (terminal)
```

Everything else is rejected with `InvalidTransition` (fail closed), in memory and in SQL.

## Invariants

1. Planning is pure and deterministic; identical inputs produce identical requests (modulo `request_id`, which is deliberately not the identity).
2. Active-and-fresh opportunities produce zero requests.
3. Active past the freshness target → `PeriodicRefresh` (Low), scheduled at `last_observed + active_threshold` (never earlier than now).
4. Stale → `Stale` (High) against the best source; unavailable best sources are skipped explicitly.
5. Expired → `Expired` (Critical) against **every** known available source (independent confirmation before commercial re-use).
6. Expired opportunities are never deleted; records are never mutated.
7. Blocked decisions explain themselves (`NoKnownSources`, `AllSourcesUnavailable`).
8. Deduplication window default is 1 hour (`REVALIDATION_DEDUP_WINDOW_MS`); `window_ms == 0` disables bucketing.

## Error handling

`RevalidationStoreError { DuplicateRequest, NotFound, InvalidTransition, UnknownEnumValue, InvalidRequest }`; categories: Conflict / NotFound / Validation / Permanent / Validation.

## Persistence behavior

- `insert` is `INSERT ... ON CONFLICT (dedup_key) WHERE status IN (active) DO NOTHING`; zero rows affected ⇒ `DuplicateRequest`.
- `transition` locks the row (`SELECT ... FOR UPDATE`), validates the domain transition matrix, then updates; `attempt` increments on the first `Running`, `started_at_ms` is first-start, `completed_at_ms` is terminal.
- `claim_due` uses `FOR UPDATE SKIP LOCKED` with a deterministic CTE order (priority desc, scheduled asc, identity asc) so concurrent workers never share a request.

## Concurrency behavior

Multiple planners can race safely: the database dedup index collapses identical intents. Multiple claimers are isolated by `SKIP LOCKED`. Opportunity updates from revalidation results use the revision-checked upsert (`upsert_if_revision`).

## Idempotency

- **Identity**: `(identity, source, reason, scheduled window bucket)`.
- **Duplicate**: an *active* request with the same identity.
- **Validity**: dedup suppression holds while the earlier request is non-terminal; terminal requests free the key for re-issue.

## Testing strategy

In-memory store contract tests (dedup, transitions, claim order, bounded errors), planner decision-table tests per freshness state, batch stability/ordering tests, forward-compatible reason serialization, and PostgreSQL integration tests gated on `CAT_TEST_DATABASE_URL`/`DATABASE_URL` (schema idempotency, dedup via partial index, claim order, SQL transition matrix, terminal re-issue).

## Security considerations

- `last_error` is bounded (512 chars) and control-character sanitized; provider payloads, headers, and credentials never enter this column.
- No secrets in errors; `claim_due` and `transition` operate on typed enums decoded strictly (statuses/priorities) or forward-compatibly (reasons).

## Current executable governed path

`GovernedRevalidationPlan` binds a request to workflow ID, capability ID, tenant/project scope, idempotency key, and the governed worker contract. `GovernedRevalidationCoordinator` then performs:

```text
claim_due
   ↓
claimed → running
   ↓
scope resolution
   ↓
workflow registration
   ↓
CapabilityAdmission
   ↓
Fenced DurableExecutionCoordinator
   ↓
GovernedRevalidationExecutor
   ↓
running → succeeded / failed
```

The current runner deliberately does not silently requeue `DispatchAction::Retry`: the request/workflow identity model does not yet provide an independent durable workflow identity per retry attempt. A retry is therefore recorded as failure until retry-attempt identity is implemented explicitly.

## Future extension points

- Sprint 1: `RevalidationAttempt` ledger + result reconciliation feeding `OpportunityRevalidated` / `OpportunityRevalidationFailed` events through the Orchestrator boundary.
- Per-source cooldowns and budgets layered over `claim_due`.

## Known limitations

- Source-specific execution is still injected through `GovernedRevalidationExecutor`; no concrete network/provider adapter is hard-coded into the domain bridge.
- Automatic retry requeue is intentionally deferred until a durable retry-attempt/workflow identity mapping exists.
- `Unknown` reasons are preserved but planning treats them as `Stale`-equivalent urgency only via explicit caller mapping; unknown reasons never silently trigger Critical paths.
