# ADR-0003: Durable Execution and Recovery

**Status:** Accepted
**Date:** 2026-09-17

## Context

CAT must execute workflows that may span provider calls, retries, approvals, worker restarts, network failures, and infrastructure changes. Process memory cannot be the source of truth for such work.

## Decision

CAT uses a **durable execution model** with persistent workflow state and distinct execution attempts.

Each durable execution MUST have:

- execution identity;
- workflow/state identity;
- attempt identity;
- lifecycle status;
- retry policy;
- correlation/causation lineage;
- durable timestamps;
- outcome/evidence state.

Recovery reconstructs execution from durable state and events. Workers are replaceable execution units rather than owners of workflow truth.

```text
Durable Workflow State
        ↓
   Claim / Fence
        ↓
   Execution Attempt
        ↓
 Provider / Capability
        ↓
 Outcome / Evidence
        ↓
 Persist + Event
        ↓
 Recovery-safe Next State
```

## Consequences

- worker crashes do not erase workflow state;
- retries can be bounded and policy-driven;
- fencing prevents stale workers from continuing ownership;
- recovery and failure-injection tests become mandatory for critical workflows;
- persistence becomes part of correctness, not merely storage.

## Rejected Alternative

In-memory workflow orchestration was rejected because it cannot provide reliable recovery after process or node failure.

## Invariants

- A retry MUST NOT create an ambiguous duplicate economic side effect.
- Unknown external results MUST be reconciled before irreversible retry where required.
- Recovery MUST be deterministic from durable information as far as practical.
- Execution state and execution-attempt state MUST remain distinguishable.
