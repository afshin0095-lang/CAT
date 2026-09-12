# CAT Project Status

> Official Development Dashboard
> Project: CAT (Commerce AI Trinity)
> Company: Omni System
> Repository Status: Active Development

---

# Project Information

| Item | Value |
|------|-------|
| Project | CAT (Commerce AI Trinity) |
| Company | Omni System |
| Status | Active Development |
| Version | 0.1.0 |
| Phase | Implementation Phase — Active |
| Repository | GitHub |
| Main Branch | main |

---

# Current Phase

## Phase B

Core implementation · Rust foundations · Event Bus · Event Store · Knowledge Graph · Memory Core · LLM / AI Core · Reasoning Core · Decision Core · Planning Core · Orchestrator · Retrieval Core · Platform Integration · Platform Adapter Layer

**Status:** Active Development

# Documentation Progress

| File | Status | Progress |
|------|--------|----------|
| 00_PROJECT_CONTEXT.md | Completed | 100% |
| 01_PROJECT_OVERVIEW.md | Completed | 100% |
| 02_PROJECT_RULES.md | Completed | 100% |
| 03_TECH_STACK.md | Completed | 100% |
| 04_ARCHITECTURE.md | Completed | 100% |
| 05_AGENTS.md | Completed | 100% |
| 06_KNOWLEDGE_ENGINE.md | Completed | 100% |
| 07_MEMORY_SYSTEM.md | Completed | 100% |
| 08_EVENTS_SYSTEM.md | Completed | 100% |
| 09_REASONING_ENGINE.md | Completed | 100% |
| 10_DECISION_ENGINE.md | Completed | 100% |
| 11_PLANNING_ENGINE.md | Completed | 100% |
| 07_TREASURY_CORE.md | Completed | 100% |
| 08_AFFILIATE_ENGINE.md | Completed | 100% |
| 09_CONTENT_ENGINE.md | Completed | 100% |
| 10_UI_UX.md | Not Started | 0% |
| 11_DESIGN_LANGUAGE.md | Not Started | 0% |
| 12_DECISIONS.md | Not Started | 0% |
| 13_TERMINOLOGY.md | Part 1 Completed | 25% |
| 14_CODING_STANDARD.md | Part 3 Prepared | 75% |
| 15_DIRECTORY_STRUCTURE.md | Not Started | 0% |
| 16_DEPLOYMENT.md | Not Started | 0% |
| 17_SECURITY.md | Not Started | 0% |
| 18_PROMPTING.md | Not Started | 0% |
| 19_DEVELOPMENT_GUIDE.md | Not Started | 0% |

# Active Task

**Current Task:** Sprint 0 — Affiliate Opportunity Platform Foundation

**Document:** `core/affiliate/rust/` (opportunity subsystem), `docs/implementation/AFFILIATE_OPPORTUNITY_*.md`

**Status:** Implemented on branch `arena/01a08d17-cat` (stacked on `feat/affiliate-opportunity-lifecycle-p0` @ `6791ace`), PR #40. Sprint 0 completes the discovery → source SPI → network adapter → ingestion → dedup → persistence → freshness → lifecycle → projection → revalidation → observability vertical slice: hardened lifecycle via a single canonical freshness engine, injectable clock, monotonic record revisions with optimistic-concurrency upserts, deterministic revalidation planner with durable request persistence (migration 0002, partial-unique dedup), opportunity status projection, persistence-neutral query/ranking/health contracts, source health tracking, neutral observability names + sink, typed opportunity event contracts, and expanded validation (URL/length/identity bounds, ASCII canonical-key limitation documented).

**Validation status: INCOMPLETE — VERIFICATION BLOCKED (external blockers). Sprint 0 is NOT closed.**

- **Branch / HEAD:** verification branch `arena/01a094aa-cat`, fast-forwarded to Sprint 0 head `fcdc654` (remote HEAD `fcdc6540c32feb45ca91506f0f7b960de8ee9e53`). Sprint 0 PR #40 is **MERGED** into `feat/affiliate-opportunity-lifecycle-p0` (merge commit `201c105`).
- **GitHub Actions will not start any job.** Every job on every branch since 2026-09-11T08:42Z fails with: *"The job was not started because recent account payments have failed or your spending limit needs to be increased. Please check the 'Billing & plans' section in your settings."* Confirmed on run `34660702949` (2026-09-12T00:10Z) and on the Sprint 0 HEAD run `34614021525`. No gate is observable as green or red.
- **Actions logs are unreachable** from the sandbox (`results-receiver.actions.githubusercontent.com` and `objects.githubusercontent.com` are TLS-blocked); check-run annotations carry only "Process completed with exit code 101", so even historical failures cannot be diagnosed from here.
- **No Rust toolchain, and none is installable.** `static.rust-lang.org`, `crates.io`, `index.crates.io`, `static.crates.io`, `deb.debian.org` and every Rust mirror are blocked; only `github.com`, `api.github.com`, `codeload.github.com`, `pypi.org`, `files.pythonhosted.org` and `registry.npmjs.org` are reachable. `cargo fmt`, `metadata`, `check`, `clippy` and `test` are unrunnable by any means.
- **No PostgreSQL instance** (nothing listening on `127.0.0.1:55432`; it cannot be installed — no apt, conda or docker). The database-backed Sprint 0 suite is unrunnable locally.
- **Three named local commits do not exist.** `aa5335b`, `053efdf`, `3913b79` are absent: all 818 reachable commits were scanned across every branch, tag and all 50 `refs/pull/*/head` refs. They were local-only commits of an earlier, discarded sandbox. Content-equivalents are present and pushed — `73816b2` (compile fix), `b4c57c3` + `fcdc654` (formatting), `0553a1b` (tokio::test convention + poison-safe velocity lock). **Unpushed commits: none.**
- **Last run where jobs actually executed:** `34533081445` (head `73816b2`) — 8 of 15 crates failed with exit code 101 (`cat-affiliate`, `cat-eventbus`, `cat-platform`, `cat-orchestrator`, `cat-planning`, `cat-decision`, `cat-eventstore-postgres`, `cat-reasoning`). Commits `1691686` … `fcdc654` were pushed afterwards and have **never been validated**.
- **CI hardening could NOT be committed — blocked by the `workflows` permission (proven this session).** The hardened workflow was committed locally and the push was rejected with `! [remote rejected] … (refusing to allow a GitHub App to create or update workflow .github/workflows/rust-workspace.yml without workflows permission)`; the branch was reset to `fcdc654`. The **committed** `.github/workflows/rust-workspace.yml` therefore gates only `cargo metadata`, the 15-package `cargo check` matrix and the 15-package `cargo test` matrix — **there is no `fmt` gate, no `clippy` gate and no PostgreSQL service in CI**, and this must not be claimed otherwise. The hardened file that adds all three is preserved at `docs/ci/rust-workspace-hardened.yml`: validated as parseable YAML, never executed. See `docs/implementation/CI_HARDENING.md`.
- **Static audits passed:** 0 `unsafe` in production code; `tracking.rs` poison-safe lock intact; both hand-written raw-waker executors remain replaced by `#[tokio::test]`; no secrets or hardcoded credentials; lifecycle is not a persisted column; CAS uses `SELECT … FOR UPDATE` and claiming uses `FOR UPDATE SKIP LOCKED`; checked/saturating conversions present; no floating-point scoring; both migrations additive and idempotent.
- **Gate to close Sprint 0:** grant the `workflows` scope and apply `docs/ci/rust-workspace-hardened.yml` → restore Actions billing → re-run the workflow on `arena/01a094aa-cat` → fix the `73816b2`-era compile failure → make fmt, metadata, clippy, check, test and the PostgreSQL suite green on the exact latest HEAD → record results in `docs/implementation/SPRINT_0_MANIFEST.md`.

# Next Task

Sprint 1 — Opportunity Lifecycle → Orchestrator → Durable Revalidation Execution: wire the revalidation request store into the Orchestrator execution boundary, persist attempts/results, and publish the Sprint 0 event contracts through the EventBus.

# Next Tasks

1. Sprint 0 — Affiliate Opportunity Platform Foundation: implementation complete on `arena/01a094aa-cat`, CI hardening applied, **closure BLOCKED on the GitHub Actions billing/payment block** (see validation status above)
2. Sprint 1 — revalidation execution through Orchestrator (durable attempts, reconciliation, event publication)
3. Post-merge — retarget stacked affiliate PRs (#34→#39 chain) so Sprint 0 lands on main
4. LLM / AI Core — authorized tool execution boundary completed
5. Memory Core — P0 foundation completed
6. Reasoning Core — P0 foundation completed
7. Decision Core — approval/human-gate foundation completed
8. Decision Core — durable PostgreSQL trace persistence adapter completed; next integration verification
9. Planning Core — P0 foundation completed

