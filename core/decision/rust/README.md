# CAT Decision Core

Deterministic decision boundary for CAT OMNISYSTEM.

## Invariants

- A recommendation is not an execution.
- Policy evaluation happens before approval.
- High-impact decisions require explicit human approval by default.
- Execution is impossible while a decision remains proposed or rejected.
- Policy identity is part of the decision contract.
- Confidence and risk thresholds are deterministic inputs, not hidden model behavior.
- The core emits a decision record; monetary truth remains outside this crate.

## Current surface

- `DecisionCandidate` — immutable-style input describing a proposed action.
- `DecisionPolicy` — deterministic policy gate.
- `DecisionEngine` — proposal, approval, and execution boundary.
- `DecisionRecord` — auditable state transition record.

This crate intentionally has no LLM, database, network, or broker dependency.
