# CAT Schema Registry and Validation

**Status:** L2 — Architecture specified

## 1. Purpose

The Schema Registry indexes every canonical contract and records its lifecycle, compatibility guarantees, ownership and validation status.

```text
Schema Author
     ↓
Schema Registry
     ↓
Structural Validator
     ↓
Contract Tests
     ↓
Compatibility Check
     ↓
Activation
```

## 2. Registry entry

```yaml
schema_id: cat.schema.capability.v1
kind: capability
version: 1.0.0
status: active
owner: architecture
compatibility:
  backward: true
  forward: false
supersedes: null
validation:
  structural: passed
  contract: passed
  security: passed
```

## 3. Validation layers

### Layer 1 — Structure

Checks required fields, types, enumerations, formats and nesting.

### Layer 2 — Semantic contract

Checks invariants such as idempotency requirements, side-effect declarations, capability bindings and lifecycle legality.

### Layer 3 — Security

Checks data classification, credential references, network permissions, trust boundaries and economic authority.

### Layer 4 — Compatibility

Checks consumers, producers, persisted representations, event versions and migration requirements.

### Layer 5 — Operational readiness

Checks observability, health checks, timeout/retry policy, test coverage and ownership.

## 4. Compatibility matrix

| Change | Classification | Default action |
|---|---|---|
| Add optional field | Compatible | Minor version |
| Clarify description | Compatible | Patch |
| Add enum value | Context-dependent | Compatibility review |
| Remove field | Breaking | Major version |
| Change field type | Breaking | Major version |
| Change side-effect class | Breaking/risk change | Explicit approval |
| Widen authorization | Security-sensitive | Explicit approval |
| Change provider adapter only | Usually compatible | Connector version |

## 5. Registry invariant

There must be exactly one canonical active definition for a given `(kind, id, version)` tuple. Historical definitions remain addressable so old events and executions can be interpreted.

## 6. AI validation rule

Before generating code from a schema, an AI agent must validate that the schema is active, identify its version, inspect the associated contract and check whether generated types already exist. It must not create a second competing representation without an explicit migration plan.

## 7. Failure behavior

Invalid definitions fail closed. A malformed or unauthorized schema cannot activate a runtime capability, tool, connector or provider.
