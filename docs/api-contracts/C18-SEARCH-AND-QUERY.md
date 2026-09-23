# C18 — Search & Query Contract

Queries are read-only and must not mutate domain state. Filters are explicit, typed, composable, and bounded.

## Determinism

Results require a documented ordering. Ranking scores use the domain ranking contract and integer-safe arithmetic where financial or deterministic precision matters.

## Query boundary

```text
HTTP / CLI / Agent
        ↓
Query DTO
        ↓
Application Query
        ↓
Persistence-neutral Query Boundary
        ↓
Repository / Read Model
```

The query layer may use denormalized projections for performance, but the projection is rebuildable from authoritative facts.
