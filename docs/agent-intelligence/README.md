# CAT Agent & Intelligence Specification Pack

This pack defines CAT's AI-native execution model. It specifies what an agent is, what it may do, how it receives authority, how it uses tools, how memory and planning work, and how multi-agent execution remains deterministic, auditable, bounded, and recoverable.

## Core principle

> Agents propose and execute bounded work; the domain remains the authority on truth.

```mermaid
flowchart LR
  I[Intent] --> O[Orchestrator]
  O --> P[Policy]
  P --> A[Agent Runtime]
  A --> T[Capability Gateway]
  T --> X[External Tools/Providers]
  X --> V[Validated Result]
  V --> D[Domain]
  D --> E[Events]
  E --> M[Memory/Projections]
```

## Documents

A01 Principles · A02 Taxonomy · A03 Identity · A04 Lifecycle · A05 Registry · A06 Runtime · A07 Planning · A08 Memory · A09 Tools & Capabilities · A10 Permissions · A11 Budgets · A12 Communication · A13 Supervisor · A14 Specialized Affiliate Agents · A15 Evaluation · A16 Failure Recovery · A17 Governance · A18 Human-in-the-loop · A19 Observability · A20 Security Model.
