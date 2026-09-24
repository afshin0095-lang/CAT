# Capability Registry P0 — Executable Contract Foundation

Status: Implemented on feat/capability-registry-p0

## Source of truth

- docs/bible/47-CAT-CAPABILITY-REGISTRY.md
- docs/bible/74-CAT-NEXT-ENGINEERING-PHASE.md
- docs/bible/118-CAT-IMPLEMENTATION-MATURITY-MATRIX.md

## Implementation

The first executable registry slice lives in the existing cat-kernel crate:

- core/kernel/rust/src/capability.rs
  - canonical CapabilityId using cat.capability.<domain>.<name>.v<major>;
  - provider-neutral CapabilityContract;
  - CapabilityLifecycle state vocabulary;
  - IdempotencyPolicy;
  - validation of identity, inputs/outputs, failure model and observability/evaluation requirements;
  - policy, evidence and idempotency requirements for side-effecting capabilities.
- core/kernel/rust/src/capability_registry.rs
  - deterministic BTreeMap registry;
  - duplicate identity rejection;
  - lifecycle transition enforcement;
  - validation before activation;
  - stable ordered iteration.
- core/kernel/rust/src/authorization.rs
  - deterministic fail-closed capability authorization;
  - agent/capability identity and lifecycle checks;
  - side-effect ceiling enforcement;
  - policy-scope enforcement;
  - explicit S3 approval boundary.
- core/kernel/rust/src/lib.rs
  - exports the canonical capability and authorization primitives through the existing kernel boundary.

## Design boundaries

The registry and authorization slice is deliberately provider-neutral and execution-free. It does not:

- call external providers;
- execute tools;
- resolve secrets;
- replace EventBus/EventStore;
- introduce a second runtime;
- persist derived capability lifecycle state;
- replace Decision Core approval records.

Dropping to a lower-level implementation must not weaken a higher-level policy. Authorization and execution remain separate: an Allowed authorization result admits the request to the next execution boundary but does not itself execute work.

## Verification requirements

The implementation includes unit tests for:

1. capability identifier shape and version constraints;
2. pure capability validation;
3. side-effect policy/evidence/idempotency requirements;
4. JSON serialization round-trip;
5. duplicate registry identity rejection;
6. canonical lifecycle transitions;
7. invalid transition rejection;
8. deterministic registry ordering;
9. unknown and undeclared capability denial;
10. policy-scope denial;
11. disabled-agent denial;
12. side-effect escalation denial;
13. S3 approval-required and approved paths.

CI remains the authoritative compilation/test gate for the repository.

## Maturity mapping

This slice moves the capability contract, registry and authorization boundary toward executable implementation. It must not be reported as L5 until automated CI and integration evidence are green.

## Next boundary

Connect the authorization result to the existing durable invocation and Orchestrator admission boundary. Preserve capability identity, side-effect classification, policy requirements, approval references, idempotency and evidence semantics established here.