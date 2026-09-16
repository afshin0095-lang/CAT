# CAT Contract Migration Playbook

**Status:** L2 — Architecture specified

## 1. Goal

Contract evolution must be incremental, observable and reversible whenever technically possible.

## 2. Migration sequence

```text
Inventory consumers
      ↓
Define new contract
      ↓
Compatibility analysis
      ↓
Implement dual-read/write if required
      ↓
Migrate consumers
      ↓
Observe
      ↓
Retire old path
```

## 3. Consumer inventory

Before a breaking change, identify:

- agents;
- workflows;
- APIs;
- event producers/consumers;
- database projections;
- tools/connectors;
- evaluations;
- dashboards;
- external integrations.

## 4. Dual-version operation

When required, CAT may run old and new versions concurrently. Each invocation must identify the contract version actually used.

```text
consumer A → capability v1
consumer B → capability v2
```

The coexistence period must have an explicit retirement condition.

## 5. Data migration

Persisted records must retain enough version/provenance metadata to interpret historical state. Migration scripts must be deterministic, idempotent and tested against representative data.

## 6. Rollback

Rollback plans must distinguish:

- code rollback;
- schema rollback;
- data rollback;
- provider rollback;
- policy rollback.

A successful code rollback does not automatically reverse an external side effect.

## 7. AI migration rule

AI agents must never perform a breaking contract migration by editing only the producer. All known consumers and persisted representations must be evaluated first.
