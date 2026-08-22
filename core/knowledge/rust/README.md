# CAT Knowledge Core

`cat-knowledge` is the first executable knowledge-graph layer for CAT OMNISYSTEM.

## Boundary

The crate stores **derived knowledge**, not canonical business truth. Knowledge nodes and edges carry explicit evidence references and confidence, while canonical entities remain owned by their domain engines.

## Current capabilities

- Stable typed node and edge identifiers.
- Canonical node-key de-duplication.
- Directed typed relations.
- Explicit evidence references.
- Confidence normalization to `[0.0, 1.0]`.
- Deterministic in-memory graph queries.
- Relation-type discovery.
- Endpoint validation before edge insertion.

## Architectural rule

AI and retrieval systems may propose or enrich knowledge, but this crate does not grant them mutation authority over canonical affiliate, treasury, decision, content, or other domain truth.

## Next expansion

The next iterations should add persistent graph adapters, temporal validity, provenance chains, graph snapshots, bounded traversal, and integration contracts with the EventBus and Memory/Knowledge engines without coupling the graph model to a specific database.
