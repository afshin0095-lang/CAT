# Sprint 0 Manifest — Affiliate Opportunity Platform Foundation

**Sprint 0 PR:** [#40](https://github.com/afshin0095-lang/CAT/pull/40) —
`feat(affiliate): Sprint 0 opportunity platform foundation`, head `fcdc654`,
**MERGED** into `feat/affiliate-opportunity-lifecycle-p0` (merge commit
`201c105`).

**Verification / closure branch (this session):** `arena/01a094aa-cat`,
fast-forwarded to `fcdc654` so it carries the complete Sprint 0 implementation
(`cda7a28..fcdc654`, 70 commits) plus this session's verification work.

---

## VERDICT: `SPRINT 0 INCOMPLETE — VERIFICATION BLOCKED`

Sprint 0's **implementation, tests and documentation are complete** and have
passed every audit that can be performed without executing code. **It cannot be
closed, because no execution evidence can be produced at all**: GitHub Actions
will not start jobs, and the sandbox cannot run Cargo or PostgreSQL.

No gate below is claimed green. Every claim is backed by the evidence shown.

---

## 1. Blockers (all external to the repository)

| # | Blocker | Evidence |
|---|---|---|
| **B1** | **GitHub Actions billing/payment block — no job will start.** | Every job on every branch since 2026-09-11T08:42Z fails with the check-run annotation: *"The job was not started because recent account payments have failed or your spending limit needs to be increased. Please check the 'Billing & plans' section in your settings."* Confirmed on: run `34660702949` (`feat/affiliate-opportunity-lifecycle-p0`, 2026-09-12T00:10Z), run `34614021525` (Sprint 0 HEAD `fcdc654`), run `34613823501`, `34613617853`, `34584130193`, `34583866192`. Jobs are created but never start, so **no gate is observable as green or red**. |
| **B2** | **Actions log retrieval blocked at the sandbox egress proxy.** | `results-receiver.actions.githubusercontent.com` and `objects.githubusercontent.com` are unreachable (TLS `SSL_ERROR_SYSCALL`). `gh run view --log-failed` returns `EOF`. Check-run annotations carry only *"Process completed with exit code 101."* — no compiler output. Even historical failures therefore cannot be diagnosed from here. |
| **B3** | **No Rust toolchain, and none is installable.** | `rustc`/`cargo` absent. Unreachable: `static.rust-lang.org`, `crates.io`, `index.crates.io`, `static.crates.io`, `deb.debian.org`, `security.debian.org`, `repo.anaconda.com`, `conda.anaconda.org`, `mirrors.kernel.org`, and Rust mirrors `rsproxy.cn`, `mirrors.tuna.tsinghua.edu.cn`, `mirrors.ustc.edu.cn`, `mirrors.aliyun.com`. Reachable: `github.com`, `api.github.com`, `codeload.github.com`, `pypi.org`, `files.pythonhosted.org`, `registry.npmjs.org` only. The allowlist blocks every source of a toolchain *and* every source of crate archives, so `cargo fmt`, `cargo metadata`, `cargo check`, `cargo clippy` and `cargo test` are **all unrunnable** — locally, by any means. |
| **B4** | **No PostgreSQL instance.** | Nothing listening on `127.0.0.1:55432`. PostgreSQL is not installed and cannot be installed (no apt, no conda, no docker). The Sprint 0 database-backed suite is unrunnable locally. |
| **B5** | **Three named local commits do not exist.** | `aa5335b`, `053efdf`, `3913b79` are absent from this workspace. All **818** reachable commits were scanned across every branch, tag and all 50 `refs/pull/*/head` refs — no object with those prefixes exists. They were local-only commits of an earlier, discarded sandbox. Their content-equivalents **are** present and pushed (§2). |
| **B6** | **The CI workflow cannot be modified — the credential lacks the `workflows` permission.** | The hardened workflow was committed locally and the push was rejected by GitHub: `! [remote rejected] arena/01a094aa-cat -> arena/01a094aa-cat (refusing to allow a GitHub App to create or update workflow .github/workflows/rust-workspace.yml without workflows permission)`. The branch was reset to `fcdc654`. Per the closure directive, the `fmt` / `clippy` / PostgreSQL gates therefore **do not exist in CI** and must not be claimed. The file is preserved at `docs/ci/rust-workspace-hardened.yml`; see `docs/implementation/CI_HARDENING.md`. **Additionally** B1 would block its execution even if it were committed. |

## 2. B5 — content-equivalents of the three lost commits

| Named SHA | Expected content | Equivalent present on `arena/01a094aa-cat` |
|---|---|---|
| `aa5335b` | Sprint 0 implementation | the 70-commit Sprint 0 series `cda7a28..fcdc654` (`8fe955f` … `fcdc654`) |
| `053efdf` | compile fix · formatting fix · test-quality fixes | `73816b2` *resolve compile and test-contract defects*; `b4c57c3` + `fcdc654` *apply rustfmt*; `0553a1b` *tokio::test convention* |
| `3913b79` | production panic-surface fix | `0553a1b` — poison-safe velocity lock in `tracking.rs` |

**Unpushed commits: none.** `arena/01a094aa-cat` → remote `fcdc6540c32feb45ca91506f0f7b960de8ee9e53`.

## 3. Last CI run in which jobs actually executed

Run **34533081445**, head `73816b2`, 2026-09-10T21:36Z — before B1 began.
8 of 15 crates failed with exit code 101:

`cat-affiliate`, `cat-eventbus`, `cat-platform`, `cat-orchestrator`,
`cat-planning`, `cat-decision`, `cat-eventstore-postgres`, `cat-reasoning`
(check and/or test).

Passed: `cat-kernel`, `cat-knowledge`, `cat-memory`, `cat-llm`, `cat-rag`,
`cat-runtime`, `cat-content`.

The breadth of that failure set points at a **workspace-wide cause** (dependency
resolution/build) rather than a Sprint 0-local defect — but the compiler output
is unobtainable (B2), so this is **not proven** and is recorded as an open
question, not as a finding. Commits `1691686` … `fcdc654` were pushed
afterwards and have **never been validated**: every run since is blocked by B1.

## 4. Requirement table

Legend — **STATIC-OK** = verified by static audit this session ·
**BLOCKED** = cannot be executed (B1–B6) · **UNVERIFIED** = never executed.

No row in this table is green. Nothing here has been observed passing.

| Requirement | Implementation | Tests | Documentation | Verification |
|---|---|---|---|---|
| Canonical freshness engine (3 thresholds, bounded) | `src/opportunity_freshness.rs` | `tests/opportunity_freshness.rs`, in-crate | `AFFILIATE_OPPORTUNITY_LIFECYCLE_FRESHNESS_P0.md` | BLOCKED |
| Lifecycle as derived projection delegating to freshness | `src/opportunity_lifecycle.rs` | `tests/opportunity_lifecycle.rs` | same doc | BLOCKED |
| Injectable clock, no hidden system time, no sleeps | `src/clock.rs` (`SystemClock`/`FixedClock`/`FnClock`) | `tests/clock.rs` | module docs | BLOCKED |
| Monotonic revisions, checked arithmetic, no wraparound | `src/opportunity_version.rs` | in-crate, `tests/opportunity_store.rs` | `AFFILIATE_OPPORTUNITY_PERSISTENCE_DEDUP_P0.md` | BLOCKED |
| Revision on records/results, idempotent re-observation | `src/opportunity_store.rs` | `tests/opportunity_store.rs` | same doc | BLOCKED |
| Transactional upserts + CAS (`FOR UPDATE`), checked conversions, corrupt-row fail-closed | `src/opportunity_postgres.rs` | `tests/opportunity_postgres.rs` (env-gated) | same doc | BLOCKED (**STATIC-OK**, §5) |
| Migration 0002 additive, facts only, no lifecycle persisted | `migrations/0002_opportunity_revalidation.sql` | covered by postgres tests | header SQL reversal + doc | BLOCKED (**STATIC-OK**, §5) |
| `ensure_schema` single-statement prepared DDL (idempotent) | `src/opportunity_postgres.rs` | same | inline comment | BLOCKED |
| Revalidation REQUEST ≠ ATTEMPT ≠ RESULT; dedup identity; transition matrix | `src/opportunity_revalidation.rs` | `tests/opportunity_revalidation.rs` | `AFFILIATE_OPPORTUNITY_REVALIDATION_P0.md` | BLOCKED |
| Deterministic planner, no I/O, skip/block explicit | `src/revalidation_planner.rs` | `tests/revalidation_planner.rs` | same doc | BLOCKED |
| Concurrent claiming — `FOR UPDATE SKIP LOCKED` | `src/opportunity_postgres.rs:710` | postgres tests | doc comment | BLOCKED (**STATIC-OK**, §5) |
| Status projection read model (revision visible, health orthogonal) | `src/opportunity_projection.rs` | `tests/opportunity_projection.rs` | `AFFILIATE_OPPORTUNITY_QUERY_P0.md` | BLOCKED |
| Persistence-neutral query (deterministic sort/tiebreak, saturating pagination, `total_matched`) | `src/opportunity_query.rs` | `tests/opportunity_query.rs` | same doc | BLOCKED |
| Basis-point ranking (validated sum, u128 intermediates, bounded, **no floats**) | `src/opportunity_ranking.rs` | `tests/opportunity_ranking.rs` | same doc | BLOCKED (**STATIC-OK**, §5) |
| Health independent from freshness; no circuit breaker | `src/opportunity_health.rs`, `src/discovery_source_health.rs` | `tests/opportunity_health.rs`, `tests/discovery_source_health.rs` | `AFFILIATE_OPPORTUNITY_HEALTH_P0.md` | BLOCKED |
| Neutral observability (names, samples, sink, converters) | `src/observability.rs` | `tests/observability.rs` | `AFFILIATE_OPPORTUNITY_OBSERVABILITY_P0.md` | BLOCKED |
| Typed event contracts (7 opportunity facts), forward-compatible reasons, bounded detail, malformed rejection | `src/events.rs` | `tests/events.rs` | `AFFILIATE_OPPORTUNITY_EVENTS_P0.md` | BLOCKED |
| No event emission from read paths (deferred to Sprint 1) | contract comments; no publisher exists | enforced by absence | events doc §boundaries | STATIC-OK |
| Discovery validation (URL syntax, bounds, control chars, degenerate identity fail-closed, documented ASCII limitation) | `src/discovery.rs` | in-crate, `tests/discovery_engine.rs` | `canonical_key` doc comments | BLOCKED |
| Discovery Source SPI (abstraction, registry, ingestion boundary, isolated failures, deterministic pagination) | `src/discovery_source.rs`, `src/opportunity_ingestion.rs`, `src/discovery_source_health.rs` | `tests/discovery_source.rs` (tokio), `tests/discovery_source_health.rs` | `AFFILIATE_DISCOVERY_SOURCE_SPI_V1.md` | BLOCKED (**STATIC-OK** §5) |
| Network adapter → DiscoverySource bridge (commission table, pagination saturation, provider failure mapping, filter rejection) | `src/network_discovery_adapter.rs` | `tests/network_discovery_adapter.rs` (tokio) | `AFFILIATE_NETWORK_DISCOVERY_ADAPTER_P0.md` | BLOCKED (**STATIC-OK** §5) |
| Error classification (category/retryability) without API breakage | `src/error.rs`, `PostgresOpportunityStoreError` | used by adapter/network tests | doc comments | BLOCKED |
| Parameterized SQL only; no secrets; no wildcard exports | all queries in `src/opportunity_postgres.rs`, `src/lib.rs` | — | `CI_HARDENING.md` | STATIC-OK |
| **CI: fmt / metadata / clippy / check / test / PostgreSQL gates** | **NOT COMMITTED** — blocked by the `workflows` permission (B6); preserved at `docs/ci/rust-workspace-hardened.yml` | — | `docs/implementation/CI_HARDENING.md` | **BLOCKED** (B6; B1 even if applied) |
| `cargo fmt --all -- --check` | **absent** from the committed workflow | — | — | BLOCKED (B6, B1, B3) |
| `cargo metadata --no-deps --format-version 1` | present in the committed workflow | — | — | UNVERIFIED (B1, B3) |
| `cargo clippy -p cat-affiliate --all-targets --all-features -- -D warnings` | **absent** from the committed workflow | — | — | BLOCKED (B6, B1, B3) |
| `cargo check` / `cargo test` — 15-package matrix | present in the committed workflow (no `--locked`; no database service) | — | — | UNVERIFIED on final HEAD (B1); red on `73816b2` (§3) |
| PostgreSQL integration (`tests/opportunity_postgres.rs`) | **absent** from the committed workflow; `postgres:16` service only in the uncommitted hardened file | — | — | BLOCKED (B6, B1, B4) |

## 5. Static audits performed this session (all passed)

| Audit | Method | Result |
|---|---|---|
| **Production `unsafe`** | scanned `core/*/rust/src/**/*.rs`, excluding `#[cfg(test)]` regions and test files | **0 occurrences.** The only `unsafe` in the workspace is inside `#[cfg(test)]` raw-waker helpers in `network_discovery_adapter.rs:276` and `opportunity_ingestion.rs:212/214` (test-only, permitted). |
| **Production panic surfaces** | same scan for `.unwrap(`/`.expect(`/`panic!(`/`unreachable!(`/`todo!(`/`unimplemented!(` | **33 occurrences across 12 files, none in Sprint 0 code.** 22 are pre-existing `lock().expect("… mutex poisoned")` in `cat-eventbus`, `cat-llm`, `cat-platform`; the rest are internal-invariant `expect`/`unwrap` in `cat-decision`, `cat-orchestrator`, `cat-planning`, `cat-platform`. **Not changed**: they are outside Sprint 0 scope and no edit can be compiled or tested here (B3). Listed as known debt. |
| **`tracking.rs` poisoned-mutex fix** | direct read | **Fixed and intact** — `self.buckets.lock().unwrap_or_else(\|poisoned\| poisoned.into_inner())` (`tracking.rs:170-173`). |
| **Raw-waker test executors** | grep for `RawWaker`/`raw_waker` in `tests/` | **Replaced.** `tests/discovery_source.rs` and `tests/network_discovery_adapter.rs` each use 2 `#[tokio::test]`; no waker code. `tokio` is a dev-dependency of `cat-affiliate`. |
| **Secrets** | scanned `*.rs`/`*.toml`/`*.yml`/`*.sql`/`*.md` for key/token/password/credential patterns and hardcoded DSNs | **No secrets.** All hits are benign (`api_key` constructor params, `max_tokens` fields, the literal test string `"secret"` in `opportunity_revalidation.rs:901`). No `postgresql://` literal in any production source. |
| **Lifecycle not persisted** | scanned both migrations | **Confirmed** — no lifecycle column; 0002 header states it explicitly. |
| **CAS / row locking** | read `opportunity_postgres.rs` | **Confirmed** — aggregate locked `FOR UPDATE` (`:223`), CAS path `SELECT version … FOR UPDATE` (`:395`) with `RevisionConflict { expected, actual }` + `is_revision_conflict()`; claim query uses `FOR UPDATE SKIP LOCKED` (`:710`). |
| **Checked conversions / overflow safety** | grep `checked_*`, `saturating_*`, `try_into`, `try_from` | **Confirmed** present across `opportunity_postgres`, `opportunity_ranking`, `discovery_source_health`, `opportunity_lifecycle`, `revalidation_planner`, `opportunity_version`, `opportunity_query`, `opportunity_freshness`, `clock`. |
| **No floating-point scoring** | grep `f32`/`f64` in `opportunity_ranking.rs`, `opportunity_query.rs`, `discovery.rs` | **Confirmed — zero floats.** Scoring is integer/basis-point only. |
| **Migrations additive + idempotent** | read both SQL files | **Confirmed** — every DDL statement is `CREATE TABLE/INDEX IF NOT EXISTS`; 0002 is purely additive; manual reversal documented in its header. |
| **CI workflow gates** | read the committed workflow; YAML-parsed and job-graph-inspected the uncommitted hardened file | **Gap confirmed.** The **committed** `.github/workflows/rust-workspace.yml` gates only `cargo metadata`, the 15-package `cargo check` matrix and the 15-package `cargo test` matrix behind a fail-closed summary — **no `fmt`, no `clippy`, no PostgreSQL service**. The required hardened file is valid YAML (`fmt`, `metadata`, `clippy`, `check`×15, `test`×15 + `postgres:16` on 55432, fail-closed summary requiring all five, no `--locked`) but **is not committed** (B6). |

## 6. Known limitations

Product-level (documented in code, unchanged):

- ASCII-oriented `canonical_key` normalization; non-ASCII identities fail closed.
- Event emission deferred to Sprint 1 (contracts only).
- Source health is process-local; no durable health adapter yet.
- Revalidation execution (attempts/results) deferred to Sprint 1 by design.

Verification-level (added this session):

- No execution evidence exists for any Sprint 0 gate.
- The `73816b2` failure set spans 8 crates and is not diagnosed.
- Pre-existing panic surfaces outside Sprint 0 remain (§5).
- Clippy is gated for `cat-affiliate` only, not `--workspace` (see `CI_HARDENING.md`).

## 7. Path to closure

0. Grant a credential with the `workflows` scope and apply
   `docs/ci/rust-workspace-hardened.yml` (B6) — otherwise Sprint 0 can only ever
   be closed without fmt/clippy/PostgreSQL gates in CI, which must be recorded
   as an accepted gap rather than as a passing gate.
1. Resolve the GitHub Actions billing/payment block (B1).
2. Push/re-run the workflow on `arena/01a094aa-cat`.
3. Read the first `cargo check -p cat-affiliate` error from the log (it will be
   immediately visible once jobs start) and fix the `73816b2`-era defect.
4. Iterate until `fmt`, `metadata`, `clippy`, `check`, `test` and the PostgreSQL
   suite are green on the exact latest HEAD.
5. Record results here and in `.ai/PROJECT_STATUS.md`, then close Sprint 0.

Steps 2–5 cannot be started from this sandbox.

## 8. Verification-session record (2026-09-12)

| Item | Value |
|---|---|
| Verification branch | `arena/01a094aa-cat` |
| Sprint 0 implementation HEAD | `fcdc654` (PR #40, **MERGED** into `feat/affiliate-opportunity-lifecycle-p0`) |
| Evidence commit | `70e9c46` — documentation only; no source or CI-behaviour change |
| Closure PR | [#51](https://github.com/afshin0095-lang/CAT/pull/51) |
| Unpushed commits | **none** |
| GitHub Actions run on the latest HEAD | **none exists** |

**Why no Actions run exists for the latest HEAD.** The committed workflow's
`pull_request` trigger is filtered to
`paths: ['**/*.rs', '**/Cargo.toml', '**/Cargo.lock', '.github/workflows/rust-workspace.yml']`
and its `push` trigger fires only on `main`. PR #51 changes documentation only,
so the workflow never runs for it — and any run would fail anyway (B1).
Triggering it would require modifying Rust or workflow files, which this
verification is not authorised to do (and B6 blocks the latter outright).

**Newest run anywhere in the repository:** `34660702949`
(`feat/affiliate-opportunity-lifecycle-p0` @ `201c105`, 2026-09-12T00:10Z) —
**failure**, billing block (B1). The most recent run on Sprint 0 code is
`34614021525` (head `fcdc654`) — **failure**, same cause.

**Sprint 0 cannot be closed from this environment.** Steps 0–1 of §7 are
account-level actions (granting the `workflows` scope; settling billing) that
cannot be performed from the repository or the sandbox.

## 9. Infrastructure-unblock pass (2026-09-12)

Both blockers were re-tested from scratch rather than assumed. **Neither has
changed.**

### B1 — billing block: STILL ACTIVE

| Item | Value |
|---|---|
| Run | `34660702949` |
| Branch / HEAD | `feat/affiliate-opportunity-lifecycle-p0` @ `201c105` |
| Created | 2026-09-12T00:10:03Z — five seconds after PR #40 merged (00:09:58Z) |
| Conclusion | **failure** |
| Jobs | `Workspace metadata` **failure**; `Check` / `Test` matrices **skipped** |
| Annotation (verbatim) | *"The job was not started because recent account payments have failed or your spending limit needs to be increased. Please check the 'Billing & plans' section in your settings"* |
| Re-checked | 2026-09-12T09:39Z — unchanged; still the newest run in the repository |

**Every job is prevented from starting**: the metadata job never runs and both
matrices are skipped as a consequence. This is not a workflow defect.

### B6 — `workflows` permission: STILL DENIED

Re-probed with an isolated throwaway branch so the closure branch was never at
risk:

```
git checkout -b probe/workflow-permission f8405e4
cp docs/ci/rust-workspace-hardened.yml .github/workflows/rust-workspace.yml
git push origin probe/workflow-permission

! [remote rejected] probe/workflow-permission -> probe/workflow-permission
  (refusing to allow a GitHub App to create or update workflow
   `.github/workflows/rust-workspace.yml` without `workflows` permission)
```

The scratch branch was deleted; it never existed on the remote. The committed
workflow is untouched — blob `d0c7023e1a1e3b262b220c6613e85726c1d3b907` at
`f8405e4`. The credential does have repository `contents: write` (the three
documentation commits pushed cleanly), so it is specifically the `workflows`
scope that is missing. No further push attempts were made and no bypass was
attempted (no renaming, no relocation, no generated stand-ins).

### Trigger design — no change required

The prepared `docs/ci/rust-workspace-hardened.yml` already carries the required
filters on both `push` and `pull_request`: `**/*.rs`, `**/Cargo.toml`,
`**/Cargo.lock`, `.github/workflows/rust-workspace.yml`. Because the workflow
path is itself in the filter, **applying the file would trigger a run** — the
correct way to obtain a latest-HEAD run, and exactly what B6 prevents.

### Why no run exists for the exact latest HEAD

`f8405e4` changes documentation only, so the committed workflow's paths filter
excludes it. Producing a run would require either modifying the workflow (B6,
denied) or changing Rust/Cargo files purely to force a trigger (forbidden).
Both remaining routes are account-level actions for the repository owner.

### PR #51

Left **OPEN** deliberately. It is an evidence/closure PR and must not be merged
while verification is blocked.

## 10. Main-landing analysis (branch-graph audit)

**Sprint 0 is NOT on `main`.** PR #40 merged into
`feat/affiliate-opportunity-lifecycle-p0`, not into `main`.

| Item | Value |
|---|---|
| `main` HEAD | `cda7a28` |
| Sprint 0 branch | `feat/affiliate-opportunity-lifecycle-p0` @ `201c105` |
| merge-base | `cda7a28` — **identical to `main` HEAD** |
| Commits ahead of `main` | **71** |
| Commits behind `main` | **0** |
| Fast-forward possible | **YES** — `main` is a strict ancestor, so no conflict is possible |
| Diff `main..201c105` | 295 files changed, +19 131 / −2 022 |

### PR chain topology (#34 → #40)

| PR | State | Base | Head | Contained in `201c105`? |
|---|---|---|---|---|
| #34 | OPEN | `main` | `feat/affiliate-discovery-p0` @ `c3fd834` | yes |
| #35 | **CLOSED** | `main` | `feat/affiliate-discovery-source-p0` @ `c57e621` | **no** — abandoned drafting branch |
| #36 | OPEN | `feat/affiliate-discovery-p0` | `feat/affiliate-discovery-source-p0-chain` @ `9d32f1a` | yes |
| #37 | OPEN | `feat/affiliate-discovery-source-p0` ⚠ | `feat/affiliate-network-discovery-adapter-p0` @ `6642a7c` | yes |
| #38 | OPEN | `feat/affiliate-network-discovery-adapter-p0` | `feat/affiliate-opportunity-persistence-p0` @ `01e0a3a` | yes |
| #39 | OPEN | `feat/affiliate-opportunity-persistence-p0` | **`feat/affiliate-opportunity-lifecycle-p0` @ `201c105`** | itself |
| #40 | MERGED | `feat/affiliate-opportunity-lifecycle-p0` | `arena/01a08d17-cat` @ `fcdc654` | yes |

### Irregularity — PR #37's base

PR #37 declares its base as `feat/affiliate-discovery-source-p0`, the branch of
the **closed** PR #35. Its head (`6642a7c`) does **not** descend from that
branch — it descends from the `-chain` branch (verified: `9d32f1a` *is* an
ancestor of `6642a7c`; `c57e621` is *not*). This is a base-configuration defect,
not a content defect, but merging #37 as configured would merge it into an
abandoned branch.

### No content is lost

The 7 commits unique to closed PR #35's branch are absent from `201c105` both
by SHA and by patch-id (all marked `+` by `git cherry`), because PR #36
deliberately re-created that work on top of #34. Content equivalence was
verified directly: every identifier introduced by the three commits that had no
subject match (`9070d13`, `cf7a6c8`, `c57e621`) is present on the Sprint 0
branch — `DiscoveryCandidate`, `DiscoveryEngine`, `DiscoveryError`,
`DiscoveryOpportunity`, `DiscoveryRequest`, `DiscoveryResult`,
`normalize_key_part`, `validate_score` — and `discovery.rs` has grown from 175
to 482 lines. The `-chain` branch is **100 % contained** in `201c105`
(0 commits missing).

### Required operation to land Sprint 0 on `main`

**One operation: fast-forward `main` to `201c105`.**

The PR carrying the complete implementation is **#39** — its head *is* the
lifecycle branch. Retarget **PR #39's base from
`feat/affiliate-opportunity-persistence-p0` to `main`** and merge it. Because
`main` is 0 commits behind and 71 commits ahead, this is a pure fast-forward:
it adds exactly 71 commits, **omits none and duplicates none**. Once it lands,
#34, #36, #37 and #38 are redundant (their content is already inside `201c105`)
and can be closed; #37's base defect then becomes moot.

**Not performed.** This audit did not retarget or merge anything. Verification
is still blocked (B1/B6), and landing implementation on `main` before CI
evidence exists would be premature. **PR #51 must not be used as a
substitute** — it is documentation/evidence only and contains no
implementation.

## 11. Reality-alignment audit (repository identity)

A later review raised the possibility that this workspace was a locally
reconstructed copy of CAT rather than the real repository. **It is not.**
`/home/user/CAT` is a real clone of `afshin0095-lang/CAT`:

| Check | Result |
|---|---|
| Remote | `https://github.com/afshin0095-lang/CAT.git` (fetch + push) |
| Repository identity (API) | `full_name=afshin0095-lang/CAT`, `private=true`, `default_branch=main` |
| Local SHAs exist on GitHub | `944d064`, `b88447f`, `fcdc654`, `201c105`, `cda7a28` — **all found** via `GET /repos/.../commits/{sha}` |
| Byte-identical content | local `HEAD^{tree}` = `439c019525aa2126535b82c91fe6d222f4488f4e`, identical to GitHub's tree for `944d064` |
| Branch tip known to GitHub | `refs/heads/arena/01a094aa-cat` = `944d064` = local HEAD |
| Other workspaces on disk | **none** — no second repository and no reconstructed tree exists |

No reconstruction was created in any session. There is therefore nothing to
quarantine, nothing to compare, and no reconstructed code has been merged.

### Deployed workflow vs. the hardened proposal

Verified through the GitHub Contents API (not from the working tree): on
`main`, on the Sprint 0 branch `201c105`, and on this branch `944d064`, the
deployed `.github/workflows/rust-workspace.yml` contains **no** `cargo fmt`,
**no** `cargo clippy` and **no** PostgreSQL service — it gates only
`metadata`, the `check` matrix and the `test` matrix. The hardened proposal
remains unapplied at `docs/ci/rust-workspace-hardened.yml`. No `--locked` is
used anywhere (the repository tracks no root `Cargo.lock`).

### Coding-standard checkpoints (`context/14_CODING_STANDARD.md`)

| Rule | Sprint 0 status |
|---|---|
| Sections 1–20 frozen, append-only | untouched |
| Monetary values never use binary floating point | compliant — integer/basis-point scoring only |
| Secrets never in source, logs or fixtures | compliant — secret scan clean |
| Migrations forward-tested, rollback-aware | compliant — additive, `IF NOT EXISTS`, reversal documented |
| Retried writes idempotent | compliant — upserts + partial-unique dedup index |
| Derived data reconstructible | compliant — lifecycle is derived, never persisted |
| Deterministic core; clock at explicit boundaries | compliant — injectable `SystemClock`/`FixedClock`/`FnClock` |
| Immutable values, single mutable owner | compliant |
| AI must not remove tests, weaken validators or invent APIs | compliant — nothing removed or weakened |
