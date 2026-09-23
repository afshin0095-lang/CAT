# A10 — Affiliate Economic Flow Sequence

**Status:** TARGET end-to-end economic sequence.

```mermaid
sequenceDiagram
    participant Source as Discovery Sources
    participant Disc as Discovery
    participant Opp as Opportunity Domain
    participant Rank as Ranking / Decision
    participant Plan as Planner
    participant Orch as Orchestrator
    participant Net as Affiliate Network
    participant Pub as Distribution
    participant Track as Attribution
    participant Rev as Revenue Intelligence
    participant Store as Durable Store

    Source->>Disc: observations
    Disc->>Opp: normalized candidate
    Opp->>Store: durable opportunity fact
    Opp->>Rank: eligible opportunities
    Rank->>Plan: selected action intent
    Plan->>Orch: durable workflow
    Orch->>Net: validate/obtain affiliate execution data
    Net-->>Orch: offer/link/commission data
    Orch->>Pub: publish/distribute
    Pub-->>Track: click/interaction evidence
    Track->>Net: conversion/commission reconciliation
    Net-->>Track: provider settlement evidence
    Track->>Rev: attributed outcome
    Rev->>Store: economic fact + learning signal
    Rev-->>Rank: optimization signal
```

## Economic chain

```text
Source Observation
 → Opportunity
 → Eligibility
 → Decision
 → Affiliate Execution
 → Distribution
 → Click
 → Conversion
 → Commission
 → Settlement
 → Profitability
 → Learning
```

## Correctness requirements

1. Discovery evidence retains provenance.
2. Opportunity identity is deterministic.
3. Ranking is explainable and reproducible.
4. Affiliate links/executions are idempotent or reconciliable.
5. Clicks and conversions are attributed using explicit identifiers and time windows.
6. Revenue facts are never inferred solely from model output.
7. Provider settlement is reconciled against internal attribution.
8. Optimization signals are derived from durable outcomes, not speculative reasoning.

## Economic safety boundary

The agent may recommend an opportunity or campaign. Budget allocation, publishing, paid advertising and other material side effects remain capability-controlled operations with policy and audit requirements.
