# CI Validation — Current State

## Status

The active Rust workflow is `.github/workflows/rust-workspace.yml`. It runs:

- `cargo fmt --all -- --check`;
- workspace metadata validation;
- `cargo clippy -p cat-affiliate --all-targets --all-features -- -D warnings`;
- `cargo check --all-targets` for the Rust package matrix;
- `cargo test --all-targets` for the Rust package matrix;
- a PostgreSQL 16 service for integration tests;
- a fail-closed workspace summary.

The current Rust matrices contain 15 packages and **omit `cat-prompt`**, even
though `cat-prompt` is a member of the 16-package Cargo workspace. The Go
Gateway currently has no committed CI workflow. These are the first CI coverage
gaps to correct when workflow-file write permission is available.

## Latest observed run

The latest run on commit `cbed531` is
[GitHub Actions run 35332655214](https://github.com/afshin0095-lang/CAT/actions/runs/35332655214).
It completed with **20 failed jobs and 14 successful jobs**. The metadata job
passed, but formatting, the affiliate clippy gate, and multiple package check
and test jobs failed. The detailed runner log is currently unavailable from the
sandbox because GitHub's log host is blocked by the egress proxy; the check
annotations expose only the process exit codes.

This means Sprint 0 remains **unverified and open**. A green workflow run is a
release gate, not an optional follow-up.

## Proposed coverage patch

The next CI change should:

1. add `cat-prompt` to both Rust check and test matrices;
2. add a Go Gateway workflow for `go test ./...` and `go vet ./...`;
3. keep all existing PostgreSQL and fail-closed summary behavior;
4. commit the workflow changes through a GitHub identity with permission to
   modify `.github/workflows/`.

The Arena GitHub App rejected the attempted workflow update with GitHub's
`workflows` permission restriction, so this patch has not been pushed.

## Local validation requirement

Before closing Sprint 0, run the following from an environment with Rust,
PostgreSQL 16, and Go installed:

```bash
cargo fmt --all -- --check
cargo metadata --no-deps --format-version 1
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy -p cat-affiliate --all-targets --all-features -- -D warnings

cd backend/gateway/go
go test ./...
go vet ./...
```

The current Arena sandbox does not contain `cargo`, `rustc`, or `go`, so no
local execution result is claimed here.
