# CAT Contract Architecture — Definition of Done

**Status:** L2 — Architecture specified

The contract architecture phase is considered structurally complete when CAT has a coherent, discoverable and enforceable path from semantic intent to runtime execution.

## Required chain

```text
Agent
 → Capability
 → Domain Service
 → Tool
 → Connector
 → Provider
 → External System
 → Evidence
 → Measurement
```

## Required control plane

```text
Schema Registry
Capability Registry
Tool Registry
Connector Registry
Provider Registry
Policy Registry
Agent Registry
```

## Required properties

- canonical identities;
- explicit versions;
- lifecycle states;
- dependency graph;
- security boundaries;
- economic limits;
- failure and unknown-state semantics;
- observability and evidence;
- compatibility/migration rules;
- contract tests;
- AI-readable documentation.

## Completion boundary

This documentation phase does **not** claim that every registry or schema is already implemented in Rust. It establishes the target contract architecture against which implementation can now be measured.

The next implementation phase should map these specifications onto the existing repository without creating a parallel architecture.
