# CAT Contract Architecture — Implementation Readiness

**Status:** L2 — Ready for executable implementation

## Ready state

The contract architecture is now sufficiently specified to begin implementation without inventing the semantic model during coding.

### Known boundaries

```text
Agent
Capability
Domain Service
Tool
Connector
Provider
Schema
Registry
Policy
Invocation
Outcome
Evidence
```

### Known controls

```text
Identity
Versioning
Lifecycle
Authorization
Idempotency
Retries
Unknown-state handling
Security
Economics
Observability
Compatibility
Migration
```

## First code target

Implement the smallest shared contract primitives in the existing Rust architecture, verify them with tests, then build registries on top of those primitives.

## Stop condition

If implementation discovers that an architectural assumption conflicts with the existing durable runtime, stop and record the conflict before introducing a second abstraction. The Bible may then be updated through an explicit architectural decision.
