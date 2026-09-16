# CAT Next Engineering Phase

**Status:** L2 — Ready for implementation planning

## Objective

Move from documentation-first architecture into executable canonical contract primitives while preserving the existing CAT runtime and Rust workspace.

## First implementation slice

1. inspect current `kernel` contracts;
2. define/reuse shared identity and version types;
3. define capability side-effect and lifecycle types;
4. define invocation/outcome contracts;
5. add serialization and validation tests;
6. expose the contracts through the existing workspace boundaries;
7. document the exact implementation mapping;
8. verify compilation and tests before expanding scope.

## Non-goals

The first slice must not:

- rewrite the orchestrator;
- replace the existing event system;
- introduce a second runtime;
- connect real external providers prematurely;
- add unrestricted agent autonomy;
- bypass current security boundaries.

## Completion criterion

The phase is complete when the canonical contract primitives exist as executable, tested code and can be consumed by the next registry implementation without duplicating identity, event or policy semantics.

```text
Bible
  ↓
Canonical Contract
  ↓
Rust Types + Validation
  ↓
Registry
  ↓
Runtime
```
