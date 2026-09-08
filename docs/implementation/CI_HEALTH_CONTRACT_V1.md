# CAT CI Health Contract V1

## Purpose

The Rust workspace CI is a correctness gate for CAT's canonical Rust cores. CI must make failures attributable to a concrete workspace package instead of stopping at the first workspace-wide compilation error.

## Required gates

1. Cargo workspace metadata must resolve successfully.
2. Every workspace package must pass `cargo check --all-targets --locked`.
3. Every workspace package must pass `cargo test --all-targets --locked`.
4. Package checks and tests use `fail-fast: false` so independent failures remain visible in the same run.
5. The final summary job fails if any package check or test fails.
6. GitHub Actions runs use a concurrency group so superseded runs do not consume unnecessary capacity.
7. The checkout action uses the current Node runtime-compatible major version.

## Diagnostic principle

A workspace-wide failure can hide the actual failing crate and prevent later tests from running. The package matrix therefore acts as a diagnostic boundary as well as a validation boundary.

## Lockfile policy

CI uses `--locked`. Dependency changes must update the repository lockfile intentionally; CI must never silently rewrite dependency resolution during validation.

## Failure handling

A failed package is not treated as an infrastructure-wide failure by default. The failing matrix entry identifies the package boundary that requires remediation. The summary job remains the single required aggregate gate.

## Scope

This contract covers the current Rust workspace. It does not yet claim production readiness for external services, databases, affiliate networks, or deployment infrastructure.
