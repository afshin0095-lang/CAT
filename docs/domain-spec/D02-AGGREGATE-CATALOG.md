# D02 — Aggregate Catalog

Aggregates define consistency boundaries, not merely table groupings.

| Aggregate | Root | Owns | Consistency rule |
|---|---|---|---|
| Opportunity | Opportunity | observations, revision facts | revision CAS protects concurrent mutation |
| RevalidationRequest | Request | attempts | one active claim per request |
| Campaign | Campaign | offers | campaign policy governs offer changes |
| Attribution | Attribution | evidence links | attribution decisions retain provenance |
| Commission | Commission | settlement facts | monetary state changes are auditable |
| AgentRun | AgentRun | execution steps | run state follows explicit transitions |
| Configuration | Configuration | version | immutable published versions |
| Tenant | Tenant | policy references | isolation invariant |

## Rules

1. An aggregate owns its invariants.
2. Cross-aggregate consistency uses domain events, commands, or an explicit application transaction.
3. A repository must not expose partial mutation of an aggregate root.
4. Optimistic revision checks are preferred for concurrent business updates.
5. Read projections are not aggregates.
