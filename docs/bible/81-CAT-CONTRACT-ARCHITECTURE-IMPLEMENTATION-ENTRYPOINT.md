# CAT Executable Contract Implementation Entrypoint

**Status:** L2 — Implementation entrypoint specified

## Immediate engineering target

The next code change should begin from the existing Rust `kernel` boundary and introduce only the shared primitives required by the canonical contract model.

## Required first primitives

```text
CanonicalId
ContractVersion
LifecycleStatus
SideEffectClass
InvocationId
IdempotencyKey
OutcomeStatus
Provenance
```

## Required properties

- strongly typed;
- serializable;
- deterministic;
- validated;
- version-aware;
- free of provider-specific semantics;
- covered by unit and contract tests.

## Dependency rule

These primitives should become reusable by the existing workspace rather than creating a separate contracts crate without dependency analysis. The exact placement must be determined after inspecting the current `kernel` APIs and Cargo dependency graph.

## Verification rule

Before moving beyond this entrypoint:

1. inspect current kernel source;
2. inspect Cargo manifests;
3. identify existing equivalent types;
4. implement/reuse primitives;
5. add tests;
6. run the narrowest relevant test suite;
7. run workspace validation;
8. document the actual mapping.

## Status boundary

This file intentionally stops before implementation. It is the handoff from the completed documentation architecture to executable engineering.
