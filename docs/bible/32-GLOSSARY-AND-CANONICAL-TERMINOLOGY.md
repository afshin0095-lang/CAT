# 32 — Canonical Glossary & Terminology

This glossary prevents semantic drift between humans, AI agents, source code, and documentation.

| Term | Canonical meaning |
|---|---|
| CAT | Commerce AI Trinity; the project/system identity |
| Agent | Governed autonomous software actor with bounded capabilities |
| Capability | Versioned operation an agent/system is authorized to invoke |
| Tool | Concrete mechanism used to perform work |
| Provider | External/internal implementation behind a stable CAT boundary |
| Connector | Integration package for an external system |
| Adapter | Translation boundary between CAT and provider semantics |
| Workflow | Durable multi-step execution definition/instance |
| Step | Bounded unit inside a workflow |
| Worker | Component that performs computational/external work at the execution boundary |
| Event | Immutable statement of something that happened |
| Command | Request for a desired action |
| Query | Request for information without intended state mutation |
| Outbox | Durable pending event publication record |
| Inbox | Consumer-side duplicate/idempotency boundary |
| Reconciliation | Process of resolving uncertain external execution state |
| Opportunity | Normalized commercial opportunity represented by CAT |
| Observation | Source-specific observation of an entity/opportunity |
| Provenance | Evidence describing where information originated |
| Knowledge | Structured/semantic information CAT uses about the world |
| Memory | Durable information retained from experience or operation |
| Learning | Evidence-driven change to future behavior or knowledge |
| Policy | Explicit rule constraining what CAT may do |
| Approval | Human authorization for a bounded high-impact action |
| Autonomy | Permission to act within a defined boundary |
| Projection | Derived read model optimized for queries |
| Canonical ID | CAT-owned stable identity |
| External ID | Identifier owned by an external provider |
| Revalidation | Refresh/check of previously observed state |
| Freshness | How current an observation is relative to policy |
| Evidence | Data supporting a claim or decision |
| Decision | Selected action/strategy after evaluating options and constraints |
| Plan | Explicit steps/dependencies intended to achieve an objective |
| Execution identity | Stable identity preventing duplicate external side effects |

## Terminology rules

1. Do not call a provider a capability.
2. Do not call generated text evidence unless independently grounded.
3. Do not call an expected outcome a realized outcome.
4. Do not call a projection canonical truth.
5. Do not call an uncertain external result a failure without reconciliation.
6. Do not use "autonomous" to imply unrestricted authority.
7. Use `implemented` only when repository evidence supports implementation.
8. Use `target` or `planned` for architecture not yet implemented.

## Status vocabulary

```text
VISION
RESEARCH
PLANNED
SPECIFIED
CONTRACTED
IMPLEMENTED
INTEGRATED
VERIFIED
OPERATIONAL
OPTIMIZING
DEPRECATED
```

Status words are evidence claims and should therefore be used carefully.