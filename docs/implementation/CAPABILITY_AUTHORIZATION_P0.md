# Capability Authorization P0 — Executable Policy Boundary

Status: Implemented on feat/capability-registry-p0

## Source of truth

- docs/bible/33-AGENT-CAPABILITY-MATRIX.md
- docs/bible/17-GOVERNANCE-HUMAN-IN-THE-LOOP.md
- docs/bible/53-CAT-TOOL-SECURITY-AND-EXECUTION-BOUNDARY.md
- docs/bible/47-CAT-CAPABILITY-REGISTRY.md
- docs/bible/110-CAT-AGENT-EXECUTION-LIFECYCLE.md

## Implementation

The authorization boundary is implemented in the existing cat-kernel crate.

- core/kernel/rust/src/authorization.rs
  - AuthorizationRequest identifies the agent, capability and requested side-effect class;
  - AuthorizationOutcome distinguishes Allowed, Denied and ApprovalRequired;
  - ApprovalContext carries only the approval reference needed by the kernel boundary;
  - AuthorizationDecision preserves machine-readable reasons and required policy identifiers;
  - CapabilityAuthorizationEngine performs deterministic fail-closed evaluation against the registered capability contract.

- core/kernel/rust/src/lib.rs
  - exports the authorization primitives through the stable kernel boundary.

## Enforced dimensions

The P0 boundary evaluates authorization dimensions that already have executable kernel representations:

1. request agent identity matches the supplied AgentContract;
2. agent is enabled and its contract validates;
3. requested capability exists in the registry;
4. capability contract validates and is Active;
5. the agent explicitly declares the capability;
6. requested side-effect class cannot exceed either the capability limit or the agent ceiling;
7. every capability-required policy must be present in the agent policy scope;
8. S3/high-impact execution pauses for explicit approval;
9. an approved S3 execution must carry a non-empty approval reference.

Unknown capabilities, disabled agents, missing policy scope, inactive capability contracts and side-effect escalation are denied rather than best-effort executed.

## Deliberate boundary

This module does not:

- execute tools or providers;
- resolve secrets;
- replace Decision Core approval records;
- persist authorization decisions;
- implement tenant/resource/budget enforcement that is not yet represented in the kernel contract model;
- replace the Orchestrator or EventBus.

Tenant/resource scope, environment constraints, budget ceilings, durable approval records and live revocation remain higher-layer concerns. The kernel must not claim those controls are enforced until their executable contracts exist.

## Verification

Unit tests cover:

- unknown capability denial;
- undeclared capability denial;
- missing policy denial;
- disabled-agent denial;
- mismatched agent identity denial;
- side-effect escalation denial;
- S3 approval-required state;
- approval-reference validation;
- successful approved path.

CI remains the authoritative compilation and workspace-test gate.

## Security mapping

Agent → Capability → Authorization/Policy → Tool → Connector → Provider remains the required path. The authorization result is not permission to bypass later execution controls; downstream boundaries may apply stricter constraints.

## Next boundary

Connect the authorization result to the existing durable invocation and Orchestrator admission boundary. The next implementation should carry the authorized capability identity, policy context, approval reference, idempotency identity and execution correlation into durable worker dispatch without granting the agent direct tool/provider access.