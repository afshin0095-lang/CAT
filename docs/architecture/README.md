# CAT Architecture Pack

**Status:** Canonical architecture documentation — target architecture unless explicitly marked `IMPLEMENTED`.

## Purpose

This pack turns CAT's architectural thesis into a machine-readable, reviewable architecture model. It complements `docs/bible/01-SYSTEM-ARCHITECTURE.md`, the accepted ADR set, contract architecture documents, and the executable workspace.

CAT is a modular, event-driven, contract-driven and provider-independent AI operating system. The system architecture separates intelligence, control, execution, truth, capabilities and operations; its target diagrams are not proof that every target capability is already implemented. fileciteturn1524file0L2-L2

## Pack map

| ID | Artifact | Primary question |
|---|---|---|
| A01 | C4 Context | Who and what surrounds CAT? |
| A02 | C4 Container | What are CAT's major runtime building blocks? |
| A03 | Domain Map | Which bounded domains own which concepts? |
| A04 | Event Flow | How does durable work propagate? |
| A05 | Data Flow | How does information become a decision and outcome? |
| A06 | Agent Mesh | How do agents, capabilities, tools and policy interact? |
| A07 | Deployment Topology | Where does each boundary run and scale? |
| A08 | Security Boundaries | Where are trust and authorization boundaries? |
| A09 | Core Sequence | What happens from goal to durable outcome? |
| A10 | Affiliate Sequence | How does an opportunity become measurable revenue? |
| A11 | Agent Execution Sequence | How is agent work governed and recovered? |
| A12 | Architecture Governance | How is architecture changed without drift? |

## Reading order

```text
Context → Containers → Domains → Events → Data → Agents → Deployment → Security → Sequences → Governance
```

## Notation

- `IMPLEMENTED` = supported by repository evidence.
- `TARGET` = designed direction; implementation must not be assumed.
- `CONTRACT` = stable semantic boundary implementations must satisfy.
- `EXTERNAL` = outside CAT's trust/control boundary.
- `DURABLE` = correctness depends on persistence.

## Core invariants

1. LLMs do not own durable business truth.
2. Decisions are separated from side-effecting execution.
3. External operations are idempotent or reconciliable.
4. Events are versioned contracts.
5. Providers remain replaceable.
6. Critical transitions are deterministic.
7. Observability and auditability are architectural concerns.
8. Scale comes from replication and composition, not accidental coupling.

## Existing architecture decisions

The current ADR index records accepted decisions for architecture style/dependency direction, event-driven core, durable execution, provider independence and polyglot runtime boundaries. fileciteturn1525file0L2-L2

## Authoritative-source rule

When this pack conflicts with executable code or an accepted ADR, implementation/ADR evidence wins and documentation must be corrected. Future capabilities remain explicitly `TARGET` until implementation evidence exists.
