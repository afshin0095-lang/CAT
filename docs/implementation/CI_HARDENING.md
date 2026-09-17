# CI Hardening — Sprint 0 (prepared, NOT committable to `.github/`)

## Status

**NOT APPLIED — blocked by a credential permission, re-confirmed this session
with hard evidence.**

The hardened workflow was committed locally on branch `arena/01a094aa-cat`
(`ci(hardening): apply fmt, clippy and PostgreSQL gates…`) and the push was
**rejected by GitHub**:

```
! [remote rejected] arena/01a094aa-cat -> arena/01a094aa-cat (refusing to allow
a GitHub App to create or update workflow `.github/workflows/rust-workspace.yml`
without `workflows` permission)
```

The branch was reset to `fcdc654` and the file is preserved here instead.

The credential available to this integration has repository `contents: write`
(it pushes branches, source and documentation successfully) but **not** the
`workflows` scope, which GitHub requires for any commit that creates or updates
a file under `.github/workflows/`.

> **Why an earlier push looked successful.** The first push of this session
> created `arena/01a094aa-cat` at `fcdc654` and appeared to carry a modified
> workflow file: the remote ref does contain that blob. It was accepted only
> because the blob was already present on `arena/01a08d17-cat`, i.e. it was not
> *new or changed by this credential*. Any actual modification — verified this
> session — is rejected. The workflow file on the remote is unchanged from
> `fcdc654`.

**Additionally**, even if the file could be committed, GitHub Actions would not
execute it: every job on every branch since 2026-09-11T08:42Z fails at startup
with *"The job was not started because recent account payments have failed or
your spending limit needs to be increased."* Both blockers must clear before
these gates can be observed.

## What to apply

Replace `.github/workflows/rust-workspace.yml` with the file in this directory
(`rust-workspace-hardened.yml`). It is additive on top of the committed
workflow:

- everything already in place is preserved: `pull_request`/`push` triggers,
  path filters, `permissions`, concurrency group, `fail-fast: false`,
  `timeout-minutes`, the 15-package check/test matrices, the metadata job, and
  the fail-closed workspace summary;
- new quality gates: `cargo fmt --all -- --check` and
  `cargo clippy -p cat-affiliate --all-targets --all-features -- -D warnings`;
- a real PostgreSQL service: a `postgres:16` container on port `55432`
  running the env-gated integration suites
  (`core/affiliate/rust/tests/opportunity_postgres.rs`) via
  `CAT_TEST_DATABASE_URL=postgresql://postgres:postgres@127.0.0.1:55432/cat_test`;
- the summary job additionally requires `fmt`, `clippy` and `test` (which now
  includes the database service).

No `--locked` is used (no `Cargo.lock` is tracked at the repository root).

**Clippy scope note.** Clippy is gated for `cat-affiliate` only, not
`--workspace`. A workspace-wide `-D warnings` clippy job would be red on
pre-existing debt in crates outside the Sprint 0 scope, which would make the
Sprint 0 gate permanently unclosable. This is deliberate scoping, not a
weakening of the Sprint 0 gate.

The file was validated as parseable YAML and its job graph inspected:

```
jobs: ['fmt', 'metadata', 'clippy', 'check', 'test', 'workspace-summary']
fmt      -> cargo fmt --all -- --check
metadata -> cargo metadata --no-deps --format-version 1
clippy   -> cargo clippy -p cat-affiliate --all-targets --all-features -- -D warnings
check    -> cargo check -p ${{ matrix.package }} --all-targets   (15 packages)
test     -> cargo test  -p ${{ matrix.package }} --all-targets   (15 packages + postgres:16)
summary  -> needs [fmt, metadata, clippy, check, test], fail-closed
```

It has **never been executed**.

## Exact manual steps

```bash
cp docs/ci/rust-workspace-hardened.yml .github/workflows/rust-workspace.yml
git add .github/workflows/rust-workspace.yml
git commit -m "ci: harden affiliate workspace validation (fmt, clippy, postgres)"
git push
```

This must be run with a credential that has the `workflows` scope. Then re-run
CI on the Sprint 0 branch and record the results in
`docs/implementation/SPRINT_0_MANIFEST.md` before marking Sprint 0 complete.
