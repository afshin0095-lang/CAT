# A12 — Architecture Governance and Change Control

**Status:** Canonical governance for the Architecture Pack.

## Change hierarchy

```text
Executable code / tests
        ↑
Accepted ADRs
        ↑
Architecture Pack
        ↑
CAT Bible / product intent
        ↑
Research / proposals
```

Lower layers may refine higher-level intent, but an implementation cannot silently redefine an accepted contract. If reality diverges, the discrepancy must be recorded and the canonical document updated deliberately.

## Required artifact for major changes

A major architectural change should include:

1. problem statement;
2. affected domains/containers;
3. context and dependency impact;
4. data/event contract impact;
5. security impact;
6. failure/recovery impact;
7. migration and rollback plan;
8. observability requirements;
9. implementation status;
10. test/verification evidence.

## Architecture fitness checklist

| Question | Required answer |
|---|---|
| Ownership | Which domain owns the new truth? |
| Boundary | Which contract separates it from neighbors? |
| Durability | What survives restart/failure? |
| Events | What facts are emitted and why? |
| Security | What authority is required? |
| Idempotency | How are duplicates handled? |
| Reconciliation | What if remote state is unknown? |
| Scale | What is the scaling dimension? |
| Observability | What proves the system behaves correctly? |
| Replacement | Can the provider/technology be swapped? |

## Diagram governance

Mermaid diagrams in this pack are canonical conceptual artifacts. Diagram nodes must map to a named container/domain/contract. A new diagram should not introduce a new subsystem name without adding its definition to the appropriate architecture or Bible document.

## Implementation-status rule

Documentation must never promote `TARGET` to `IMPLEMENTED` merely because a design exists. Evidence should be one or more of:

- executable implementation;
- automated test;
- migration/schema;
- accepted runtime contract;
- verified deployment configuration.

This rule is essential because CAT's existing system architecture explicitly distinguishes target architecture from implementation evidence. fileciteturn1524file0L2-L2
