# Sprint 0 Manifest — Affiliate Opportunity Platform Foundation

**Implementation base:** merged through `main` and extended by the Sprint 1
revalidation coordinator in `cbed531`.

## VERDICT: INCOMPLETE — Sprint 0 is NOT closed

The Affiliate Opportunity implementation covers the intended P0 domain surface,
but it has not passed the verification gate. The active CI workflow checks
formatting, metadata, Clippy, the 15-package Rust matrix, PostgreSQL-backed tests,
and a fail-closed summary. It currently omits the workspace member `cat-prompt`;
Go Gateway validation is not yet part of CI. The latest run is
[35332655214](https://github.com/afshin0095-lang/CAT/actions/runs/35332655214)
on `cbed531` and finished with 20 failed jobs and 14 successful jobs.

The sandbox cannot retrieve the detailed Actions runner log because the GitHub
log host is blocked by the egress proxy. It also has no `cargo`, `rustc`, or
`go`, so local execution is not claimed. The next verification must run from
a machine or runner with the toolchains and accessible logs.

Current gates:

| Gate | Current status |
|---|---|
| Rust formatting | FAILED in latest run |
| Workspace metadata | PASSED in latest run |
| Affiliate Clippy | FAILED in latest run |
| Rust package checks | MIXED; multiple failures |
| Rust package tests | MIXED; multiple failures |
| PostgreSQL-backed tests | FAILED in latest run |
| Sprint 0 closure | BLOCKED until a clean run |

## Requirement table

CI column legend — **PROVEN** = observed green in GitHub Actions on this PR chain ·
**FAILED** = observed red · **UNVERIFIED** = not yet proven by a clean run (the latest run is red) · **STATIC** = verified by manual + automated static audit only.

| Requirement | Implementation | Tests | Documentation | CI |
|---|---|---|---|---|
| Canonical freshness engine (3 thresholds, bounded) | `core/affiliate/rust/src/opportunity_freshness.rs` | `tests/opportunity_freshness.rs`, `src/opportunity_freshness.rs` tests | `docs/implementation/AFFILIATE_OPPORTUNITY_LIFECYCLE_FRESHNESS_P0.md` | UNVERIFIED |
| Lifecycle as derived projection delegating to freshness | `src/opportunity_lifecycle.rs` | `tests/opportunity_lifecycle.rs` | same doc | UNVERIFIED (base `Test cat-affiliate` PROVEN green at 6791ace; Sprint 0 delta UNVERIFIED) |
| Injectable clock, no hidden system time, no sleeps | `src/clock.rs` (`SystemClock`/`FixedClock`/`FnClock`) | `tests/clock.rs`, `src/clock.rs` tests | module docs | UNVERIFIED |
| Monotonic revisions, checked arithmetic, no wraparound | `src/opportunity_version.rs` | `src/opportunity_version.rs` tests, `tests/opportunity_store.rs` | `docs/implementation/AFFILIATE_OPPORTUNITY_PERSISTENCE_DEDUP_P0.md` | UNVERIFIED |
| Revision on records/results, idempotent re-observation | `src/opportunity_store.rs` | `tests/opportunity_store.rs`, in-crate tests | same doc | UNVERIFIED |
| Transactional upserts + CAS (`FOR UPDATE`), checked conversions, corrupt-row fail-closed | `src/opportunity_postgres.rs` | `tests/opportunity_postgres.rs` (env-gated) | same doc | UNVERIFIED (requires a clean PostgreSQL-backed run) |
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
| CI: fmt/clippy/postgres gates added | Active `.github/workflows/rust-workspace.yml`: `fmt`, `clippy -D warnings` for `cat-affiliate`, and PostgreSQL service with `CAT_TEST_DATABASE_URL` | — | `docs/implementation/CI_HARDENING.md` | FAILED in latest run 35332655214 |
| Existing workspace matrix intact | `.github/workflows/rust-workspace.yml` retains the package check/test matrices and adds the hardening gates | — | — | FAILED in latest run 35332655214 |
| `cargo fmt --all -- --check` | Active workflow `fmt` gate | — | — | local execution not available; latest workflow failed |
| `cargo clippy -p cat-affiliate --all-targets --all-features -- -D warnings` | Active workflow `clippy` gate | — | — | local execution not available; latest workflow failed |
| `cargo check -p cat-affiliate` / `cargo test -p cat-affiliate` | committed CI runs them | — | — | **FAILED** in latest workflow run 35332655214; detailed compiler log unavailable from the sandbox |

## Run evidence

| Item | Value |
|---|---|
| Current implementation HEAD | `cbed531e7633d7749ee140f7543e989b717aa8e5` |
| Latest workflow | [Run 35332655214](https://github.com/afshin0095-lang/CAT/actions/runs/35332655214) |
| Latest workflow result | 20 failed jobs, 14 successful jobs |
| Log availability | Detailed runner log unavailable from the sandbox; annotations expose only exit codes |
| Local toolchain | Not installed in the Arena sandbox (`cargo`, `rustc`, and `go` unavailable) |
| Closure status | INCOMPLETE; requires a clean active workflow run |

## CI hardening run record

| Gate / action | Result | Evidence |
|---|---|---|
| Active workflow integration | PARTIAL | `.github/workflows/rust-workspace.yml` contains the hardened `fmt`, `clippy`, PostgreSQL service, and fail-closed summary, but its matrices omit `cat-prompt` and it has no Go Gateway job. |
| Local formatting gate | NOT RUN | No Rust toolchain is installed in the current sandbox. |
| Local clippy gate | NOT RUN | No Rust toolchain is installed in the current sandbox. |
| Workflow log retrieval | BLOCKED | GitHub Actions log host is unreachable from the current sandbox; the latest run itself completed and is recorded above. |

Once the PR exists in GitHub, inspect the `Workspace summary` job. It is the
required status gate and fails when any of `fmt`, `metadata`, `clippy`, `check`,
or `test` is not successful.

## Known limitations (product-level, documented in code)

- ASCII-oriented `canonical_key` normalization; non-ASCII identities fail closed (`src/discovery.rs` docs).
- Event emission boundary deferred to Sprint 1 (contracts only).
- Source health is process-local; no durable health adapter yet.
- Revalidation execution (attempts/results) deferred to Sprint 1 by design.
