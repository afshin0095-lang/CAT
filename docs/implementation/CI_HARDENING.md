# CI Hardening — Sprint 0 (prepared, NOT committed to `.github/`)

## Status

**NOT APPLIED.** The sandbox GitHub App (`arena-ai-coding-agent[bot]`) cannot
create or update files under `.github/workflows/`:

```
! [remote rejected] ... (refusing to allow a GitHub App to create or update
workflow `.github/workflows/rust-workspace.yml` without `workflows` permission)
```

(the same change through the Contents API fails with
`Resource not accessible by integration`, HTTP 403).

Additionally, at the time of writing the repository's GitHub Actions minutes
were exhausted (private repository), so even existing workflow runs fail at
startup; CI could not validate the applied gates either way.

## What to apply

Replace `.github/workflows/rust-workspace.yml` with the file in this
directory (`rust-workspace-hardened.yml`). It is additive on top of the
committed workflow:

- everything already in place is preserved: `pull_request`/`push` triggers,
  path filters, `permissions`, concurrency group, `fail-fast: false`,
  `timeout-minutes`, the 15-package check/test matrices, the metadata job,
  and the fail-closed workspace summary;
- new quality gates: `cargo fmt --all -- --check` and
  `cargo clippy -p cat-affiliate --all-targets -- -D warnings`;
- new PostgreSQL job: a real `postgres:16` service container running the
  env-gated integration suites (`tests/opportunity_postgres.rs`) via
  `CAT_TEST_DATABASE_URL`;
- the summary job additionally requires `fmt`, `clippy`, and `database`.

No `--locked` is used (no `Cargo.lock` is tracked at the repository root).

## Exact manual steps

```bash
cp docs/ci/rust-workspace-hardened.yml .github/workflows/rust-workspace.yml
git add .github/workflows/rust-workspace.yml
git commit -m "ci: harden affiliate workspace validation (fmt, clippy, postgres)"
git push
```

Then re-run CI on PR #40 and record the results in
`docs/implementation/SPRINT_0_MANIFEST.md` before marking Sprint 0 complete.
