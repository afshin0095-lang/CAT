# CAT Reasoning Core

Deterministic evidence evaluation and immutable reasoning traces for CAT OMNISYSTEM.

## Guarantees

- Canonical reasoning inputs are represented as typed `ReasoningRequest` values.
- Evidence carries source, confidence, authority, and metadata.
- Policy validation rejects malformed objectives, empty evidence, invalid confidence, and unsafe step budgets.
- Deterministic hypothesis aggregation uses stable ordering for reproducible ranking when confidence ties occur.
- Results are explicitly marked `advisory_only`; the reasoning core does not mutate canonical business truth or execute decisions.
- The engine is intentionally synchronous and deterministic at this layer; LLM-assisted reasoning remains an adapter concern.

## Contract tests

`tests/contracts.rs` covers request validation, confidence validation, step-budget enforcement, authority weighting, stable tie-breaking, and advisory-only output semantics.
