# Sprint 0 Manifest — Affiliate Opportunity Platform Foundation

**PR:** [#40](https://github.com/afshin0095-lang/CAT/pull/40) · **Branch:** `arena/01a08d17-cat` (stacked on `feat/affiliate-opportunity-lifecycle-p0` @ `6791ace`)

## VERDICT: INCOMPLETE — Sprint 0 is NOT closed

The implementation scope exists and has passed repeated static audits, but the
**latest GitHub Actions run on this branch fails on `Check cat-affiliate` /
`Test cat-affiliate`**, and the failure could not be diagnosed or verified
fixed:

| Blocker | Detail |
|---|---|
| Unresolved compile failure | Runs on `79e48b4` and `73816b2` fail at `cargo check/test -p cat-affiliate`. Compiler output is not retrievable from the sandbox (Actions log hosts are blocked at the egress proxy; check-run annotations carry only `exit code 101`). Four full manual audits plus automated struct-field, name-resolution, use-path, and brace-balance checks found no defect; the probes proved the failure is real and located in the code that became live when `lib.rs` gained the Sprint 0 module declarations (commit `b86a9fd` and later). |
| CI minutes exhausted | The repository is **private**; the Actions minutes quota ran out during parallel bisect probing. Every run created afterwards fails at startup ("workflow file may be broken", zero steps) — including on previously working branches. CI validation cannot resume until quota resets. |
| Workflow write permission | The GitHub App cannot modify `.github/workflows/*` (missing `workflows` scope), so the hardened workflow is preserved at `docs/ci/rust-workspace-hardened.yml` instead of committed. |
| Local toolchain | `rustc`/`cargo` cannot be installed in the sandbox: `static.rust-lang.org`, `crates.io`, and `index.crates.io` are TLS-blocked by the proxy (only github.com and pypi are allowed). There is no tracked `Cargo.lock` to vendor from. |

**Path to closure:** restore Actions minutes → apply the hardened workflow →
re-run PR #40 CI → read the first `cargo check -p cat-affiliate` error from
the log (will be immediately visible) → fix → iterate until the FINAL GATE in
the Sprint 0 directive is met.

## Requirement table

CI column legend — **PROVEN** = observed green in GitHub Actions on this PR chain ·
**FAILED** = observed red · **UNVERIFIED** = could not be observed (log access blocked /
quota exhausted before a clean run) · **STATIC** = verified by manual + automated static audit only.

| Requirement | Implementation | Tests | Documentation | CI |
|---|---|---|---|---|
| Canonical freshness engine (3 thresholds, bounded) | `core/affiliate/rust/src/opportunity_freshness.rs` | `tests/opportunity_freshness.rs`, `src/opportunity_freshness.rs` tests | `docs/implementation/AFFILIATE_OPPORTUNITY_LIFECYCLE_FRESHNESS_P0.md` | UNVERIFIED |
| Lifecycle as derived projection delegating to freshness | `src/opportunity_lifecycle.rs` | `tests/opportunity_lifecycle.rs` | same doc | UNVERIFIED (base `Test cat-affiliate` PROVEN green at 6791ace; Sprint 0 delta UNVERIFIED) |
| Injectable clock, no hidden system time, no sleeps | `src/clock.rs` (`SystemClock`/`FixedClock`/`FnClock`) | `tests/clock.rs`, `src/clock.rs` tests | module docs | UNVERIFIED |
| Monotonic revisions, checked arithmetic, no wraparound | `src/opportunity_version.rs` | `src/opportunity_version.rs` tests, `tests/opportunity_store.rs` | `docs/implementation/AFFILIATE_OPPORTUNITY_PERSISTENCE_DEDUP_P0.md` | UNVERIFIED |
| Revision on records/results, idempotent re-observation | `src/opportunity_store.rs` | `tests/opportunity_store.rs`, in-crate tests | same doc | UNVERIFIED |
| Transactional upserts + CAS (`FOR UPDATE`), checked conversions, corrupt-row fail-closed | `src/opportunity_postgres.rs` | `tests/opportunity_postgres.rs` (env-gated) | same doc | UNVERIFIED (requires DB; job prepared, not committed) |
| Migration 0002 additive, facts only, no lifecycle persisted | `core/affiliate/rust/migrations/0002_opportunity_revalidation.sql` | covered by `tests/opportunity_postgres.rs` | header SQL reversal + same doc | UNVERIFIED |
| ensure_schema single-statement prepared DDL (idempotent) | `src/opportunity_postgres.rs` (`ensure_schema`, `ensure_revalidation_schema`) | same postgres tests | inline comment | UNVERIFIED |
| Revalidation REQUEST ≠ ATTEMPT ≠ RESULT; dedup identity; transition matrix | `src/opportunity_revalidation.rs` | `tests/opportunity_revalidation.rs`, in-crate tests | `docs/implementation/AFFILIATE_OPPORTUNITY_REVALIDATION_P0.md` | UNVERIFIED |
| Deterministic planner, no I/O, skip/block explicit | `src/revalidation_planner.rs` | `tests/revalidation_planner.rs`, in-crate tests | same doc | UNVERIFIED |
| Status projection read model (revision visible, health orthogonal) | `src/opportunity_projection.rs` | `tests/opportunity_projection.rs`, in-crate tests | `docs/implementation/AFFILIATE_OPPORTUNITY_QUERY_P0.md` | UNVERIFIED |
| Persistence-neutral query (deterministic sort/tiebreak, saturating pagination, total_matched) | `src/opportunity_query.rs` | `tests/opportunity_query.rs`, in-crate tests | same doc | UNVERIFIED |
| Basis-point ranking (validated sum, u128 intermediates, bounded) | `src/opportunity_ranking.rs` | `tests/opportunity_ranking.rs`, in-crate tests | same doc | UNVERIFIED |
| Health independent from freshness; no circuit breaker | `src/opportunity_health.rs`, `src/discovery_source_health.rs` | `tests/opportunity_health.rs`, `tests/discovery_source_health.rs` | `docs/implementation/AFFILIATE_OPPORTUNITY_HEALTH_P0.md` | UNVERIFIED |
| Neutral observability (names, samples, sink, converters) | `src/observability.rs` | `tests/observability.rs`, in-crate tests | `docs/implementation/AFFILIATE_OPPORTUNITY_OBSERVABILITY_P0.md` | UNVERIFIED |
| Typed event contracts (7 opportunity facts), forward-compatible reasons, bounded detail, malformed rejection | `src/events.rs` | `tests/events.rs`, in-crate tests | `docs/implementation/AFFILIATE_OPPORTUNITY_EVENTS_P0.md` | UNVERIFIED |
| No event emission from read paths (emission deferred to Sprint 1) | contract comments in `src/events.rs`; no publisher exists | enforced by absence | events doc §boundaries | STATIC |
| Discovery validation (URL syntax, bounds, control chars, degenerate identity fail-closed, documented ASCII limitation) | `src/discovery.rs` | `src/discovery.rs` tests, `tests/discovery_engine.rs` | doc comments at `canonical_key` | UNVERIFIED |
| Network adapter (commission table, pagination saturation, provider failure mapping, filter rejection) | `src/network_discovery_adapter.rs` | in-crate tests, `tests/network_discovery_adapter.rs` | module docs | UNVERIFIED |
| Ingestion isolation + accounting + batch identity | `src/opportunity_ingestion.rs` | in-crate tests | module docs | UNVERIFIED |
| Error classification (category/retryability) without API breakage | `src/error.rs` | used by adapter/network tests | doc comments | UNVERIFIED |
| Error classification for Postgres store errors | `src/opportunity_postgres.rs` (`PostgresOpportunityStoreError`) | in-crate tests | doc comments | UNVERIFIED |
| Parameterized SQL only; no secrets; no wildcard exports; idempotency | all `src/opportunity_postgres.rs` queries, `src/lib.rs` | — | `docs/implementation/CI_HARDENING.md` | STATIC |
| CI: fmt/clippy/postgres gates added | **NOT COMMITTED** — preserved at `docs/ci/rust-workspace-hardened.yml` | — | `docs/implementation/CI_HARDENING.md` | FAILED (cannot be applied by the integration; quota exhausted) |
| Existing workspace matrix intact (fmt-free committed workflow unchanged) | `.github/workflows/rust-workspace.yml` | — | — | PROVEN (runs #157/#159 executed it; matrices ran) |
| `cargo fmt --all -- --check` | not runnable locally; not in committed CI | — | — | UNVERIFIED |
| `cargo clippy -p cat-affiliate --all-targets --all-features -- -D warnings` | not runnable locally; not in committed CI | — | — | UNVERIFIED |
| `cargo check -p cat-affiliate` / `cargo test -p cat-affiliate` | committed CI runs them | — | — | **FAILED** on `79e48b4`, `73816b2` (unresolved compile error) |

## Run evidence

| Item | Value |
|---|---|
| Final implementation HEAD | `73816b2096ff2eb9519ae1a94de07a41102aabe9` |
| PR | #40 (`feat(affiliate): Sprint 0 opportunity platform foundation`) |
| Runs on this branch | #157 (`79e48b4`) FAILED, #159-era run on `73816b2` FAILED — both at `Check/Test cat-affiliate` (exit 101); pre-existing failures in `cat-eventbus`, `cat-eventstore-postgres`, `cat-decision`, `cat-planning`, `cat-orchestrator`, `cat-platform`, `Test cat-reasoning` exist on base `6791ace` as well (PROVEN via PR #39 rollup) and are NOT Sprint 0 regressions |
| Post-fix probes | c1–c6 bisect probes (PRs #41–#46, closed+deleted): all red — c1–c5 explained by `lib.rs` declarations lagging in the same commit; c6 (`b86a9fd`) red with a genuine `cargo check` failure — the defect sits in the Sprint 0 surface, unlocated |
| Successful required jobs | On base: `Check/Test cat-affiliate` PROVEN green (PR #39). On Sprint 0 heads: none beyond `Workspace metadata`/individual matrix jobs that completed before failures |
| Workflow hardening committed | **NO** — preserved at `docs/ci/rust-workspace-hardened.yml` (App lacks `workflows` permission) |
| Local cargo validation | **IMPOSSIBLE** — rust-lang.org/crates.io TLS-blocked in the sandbox; documented as the standing limitation |

## Known limitations (product-level, documented in code)

- ASCII-oriented `canonical_key` normalization; non-ASCII identities fail closed (`src/discovery.rs` docs).
- Event emission boundary deferred to Sprint 1 (contracts only).
- Source health is process-local; no durable health adapter yet.
- Revalidation execution (attempts/results) deferred to Sprint 1 by design.
