# CAT Domain & Data Specification Pack

This pack defines the canonical domain vocabulary, aggregates, invariants, commands, queries, events, errors, and domain-to-storage boundaries of CAT.

## Authority model

- **Implemented**: verified against repository code.
- **Contract**: required interface or invariant; implementation may follow.
- **Target**: future architecture and must not be represented as implemented.

## Documents

| ID | Document | Purpose |
|---|---|---|
| D01 | Entity Catalog | Canonical entities and ownership |
| D02 | Aggregate Catalog | Transactional consistency boundaries |
| D03 | Value Objects | Immutable domain primitives |
| D04 | Domain Invariants | Rules that must never be violated |
| D05 | State Machines | Explicit lifecycle transitions |
| D06 | Command Catalog | Intent and write operations |
| D07 | Query Catalog | Read-side contracts |
| D08 | Event Catalog | Facts emitted at real boundaries |
| D09 | Error Taxonomy | Stable failure vocabulary |
| D10 | Opportunity Domain | Affiliate opportunity model |
| D11 | Affiliate Network Domain | Network/provider boundary |
| D12 | Campaign Domain | Campaign and offer boundary |
| D13 | Revenue & Commission Domain | Monetary facts and settlement |
| D14 | Tracking & Attribution Domain | Click/conversion attribution |
| D15 | Agent Domain | Agent identity, policy and execution |
| D16 | Identity Domain | Human, tenant and authorization concepts |
| D17 | Configuration Domain | Versioned configuration |
| D18 | Audit Domain | Evidence and audit trail |
| D19 | Entity Relationship Model | Cross-domain relationships |
| D20 | Domain-to-Database Mapping | Persistence boundary |

## Global rule

Domain facts are persisted; derived projections are recomputable. No projection state may become the sole source of truth for a fact that can be reconstructed from authoritative events or records.
