# CAT Contract Architecture Baseline

**Baseline:** Documentation handoff to executable implementation.

## Canonical model

```text
Agent → Capability → Domain Service → Tool → Connector → Provider → External System
```

## Control model

```text
Identity + Contract + Policy + Budget + Security + Execution + Evidence + Evaluation
```

## Engineering next step

Inspect the actual current Rust implementation and establish the first executable canonical contract primitives with tests. Runtime completion must be demonstrated through code and verification, not documentation labels.
