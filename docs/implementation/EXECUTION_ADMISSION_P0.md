# Execution Admission P0 — Authorized Durable Dispatch Boundary

Status: Implemented on `feat/capability-registry-p0`

## Purpose

This boundary connects the kernel capability authorization decision to the Orchestrator's durable execution identity without executing a tool, connector, or provider.

The safe path is:

`InvocationRequest → CapabilityAuthorizationEngine → CapabilityAdmission → ExecutionAuthorization → Worker Dispatch`

Worker dispatch now consumes only an authorized request; the unauthenticated worker-input path has been removed.

## Contract

`ExecutionAuthorization` is an immutable, serializable authorization receipt created only from an `Allowed` kernel authorization decision.

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

`CapabilityAdmission::admit` validates:

1. workflow identity is non-nil;
2. step identity is non-empty;
3. attempt is greater than zero;
4. the canonical invocation validates;
5. invocation capability parses as a canonical `CapabilityId`;
6. kernel authorization is evaluated fail-closed.

Results are explicitly separated:

- `Admitted` — a receipt exists and may be carried toward worker dispatch;
- `ApprovalRequired` — no execution receipt is created;
- `Denied` — no execution receipt is created.

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
- decide workflow retry/approval policy;
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

## Completed integration

`ExecutionRequest` now contains `ExecutionAuthorization` as a mandatory field and has no public constructor. `ExecutionIntent` is the pre-admission planning type.

`WorkerExecutionInput` can only be created from an authorized `ExecutionRequest`. Its authorization receipt is carried through to the worker boundary.

`ExecutionCoordinator::execute_step` performs admission before lease acquisition, rejects denied/approval-required requests without claiming the step, and verifies that the invocation capability exactly matches the workflow step capability.

This removes the unauthenticated production worker path.

## Next integration

Persist the authorization identity with the durable execution-attempt ledger and expose it through reconciliation/audit. The durable ledger should retain enough governance metadata to prove which agent/capability/policy/approval/idempotency context produced each attempt.
