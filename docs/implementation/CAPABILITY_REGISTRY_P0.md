# Capability Registry P0 — Executable Contract Foundation

**Status:** Implemented on `feat/capability-registry-p0`

## Source of truth

- `docs/bible/47-CAT-CAPABILITY-REGISTRY.md`
- `docs/bible/74-CAT-NEXT-ENGINEERING-PHASE.md`
- `docs/bible/118-CAT-IMPLEMENTATION-MATURITY-MATRIX.md`

## Implementation

The first executable registry slice lives in the existing `cat-kernel` crate:

- `core/kernel/rust/src/capability.rs`
  - canonical `CapabilityId` using `cat.capability.<domain>.<name>.v<major>`;
  - provider-neutral `CapabilityContract`;
  - `CapabilityLifecycle` state vocabulary;
  - `IdempotencyPolicy`;
  - validation of identity, inputs/outputs, failure model and observability/evaluation requirements;
  - policy, evidence and idempotency requirements for side-effecting capabilities.
- `core/kernel/rust/src/capability_registry.rs`
  - deterministic `BTreeMap` registry;
  - duplicate identity rejection;
  - lifecycle transition enforcement;
  - validation before activation;
  - stable ordered iteration.
- `core/kernel/rust/src/lib.rs`
  - exports the canonical capability primitives through the existing kernel boundary.

## Design boundaries

The registry is deliberately in-memory and persistence-neutral. It does not:

- call external providers;
- execute tools;
- authorize actions by itself;
- replace EventBus/EventStore;
- introduce a second runtime;
- persist derived lifecycle state.

Authorization, durable execution, provider selection, persistence and distributed synchronization remain higher-layer concerns.

## Verification requirements

The implementation includes unit tests for:

1. capability identifier shape and version constraints;
2. pure capability validation;
3. side-effect policy/evidence/idempotency requirements;
4. JSON serialization round-trip;
5. duplicate registry identity rejection;
6. canonical lifecycle transitions;
7. invalid transition rejection;
8. deterministic registry ordering.

CI remains the authoritative compilation/test gate for the repository.

## Maturity mapping

This slice moves the capability contract/registry area from documented architecture toward executable implementation. It must not be reported as L5 until automated CI and integration evidence are green.

## Next boundary

Connect capability registration to the existing authorization/execution boundary without granting agents unrestricted infrastructure access. The next implementation must preserve capability identity, side-effect classification, policy requirements, idempotency and evidence semantics established here.
