# Execution Admission P0 — Authorized Durable Dispatch Boundary

Status: Implemented on \`feat/capability-registry-p0\`

## Purpose

This boundary connects the kernel capability authorization decision to the Orchestrator's durable execution identity without executing a tool, connector, or provider.

The safe path is:

\`InvocationRequest → CapabilityAuthorizationEngine → CapabilityAdmission → ExecutionAuthorization → Worker Dispatch\`

Only the first four stages are implemented by this P0 boundary. Worker dispatch consumption is the next integration step.

## Contract

\`ExecutionAuthorization\` is an immutable, serializable authorization receipt created only from an \`Allowed\` kernel authorization decision.

It is bound to:

- invocation identity;
- workflow identity;
- workflow step;
- execution attempt;
- agent identity;
- canonical capability identity;
- requested side-effect class;
- required policy identifiers;
- optional approval reference;
- idempotency key;
- correlation identity;
- admission timestamp.

The receipt exposes read-only accessors and cannot be fabricated through a public constructor.

## Admission semantics

\`CapabilityAdmission::admit\` validates:

1. workflow identity is non-nil;
2. step identity is non-empty;
3. attempt is greater than zero;
4. the canonical invocation validates;
5. invocation capability parses as a canonical \`CapabilityId\`;
6. kernel authorization is evaluated fail-closed.

Results are explicitly separated:

- \`Admitted\` — a receipt exists and may be carried toward worker dispatch;
- \`ApprovalRequired\` — no execution receipt is created;
- \`Denied\` — no execution receipt is created.

An approval requirement therefore cannot be accidentally treated as an authorization grant.

## Durable execution invariants

The admission layer is intentionally evaluated before a worker lease and before external work.

The resulting receipt binds authorization to the exact execution attempt, so a receipt for one workflow/step/attempt cannot be silently reused for another execution identity.

Idempotency and correlation metadata are carried rather than regenerated downstream.

## Deliberate boundary

This module does not:

- persist authorization receipts;
- acquire leases;
- claim workflow steps;
- dispatch workers;
- call tools, connectors, providers, networks, or databases;
- replace Decision Core approval records;
- implement tenant/resource/budget/environment policy dimensions that are not yet represented in the kernel contracts.

## Verification

Unit tests cover:

- successful receipt construction and identity binding;
- S3 approval-required behavior before worker admission;
- denied identity mismatch with no receipt;
- zero-attempt rejection.

CI remains the authoritative workspace compilation and test gate.

## Next integration

Make \`ExecutionRequest\` and \`WorkerExecutionInput\` consume \`ExecutionAuthorization\` as a mandatory field, then require Orchestrator admission before a lease is acquired and a worker is invoked.

The integration must remove, not duplicate, an unauthenticated production worker path.
