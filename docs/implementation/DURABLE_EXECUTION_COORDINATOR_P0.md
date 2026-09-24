# Durable Execution Coordinator P0 — Production Governed Path

Status: Implemented on `feat/capability-registry-p0`

## Purpose

This contract is the production async execution path between capability governance and durable worker execution.

## Required sequence

`Workflow READY → Step Capability Match → Capability Admission → Fenced Lease → Claim Step → Durable Attempt + Authorization Evidence → Worker → Workflow + Outbox Commit → Attempt Completion`

No worker dispatch occurs unless authorization and durable attempt registration both succeed.

## Governance binding

The worker receives:

- execution ID;
- workflow ID;
- step ID;
- attempt;
- `ExecutionAuthorization`;
- fencing token.

The execution request cannot be constructed without an authorization receipt, and the receipt must match the workflow/step/attempt identity.

The workflow step's `CapabilityId` must exactly match the invocation capability before admission.

## Approval

Capability approval is a pre-dispatch boundary. `ApprovalRequired` and `Denied` decisions stop execution before lease acquisition.

A worker returning `WaitingApproval` after governed admission is treated as a contract violation. The attempt is failed rather than silently turning a previously authorized worker execution back into an approval flow.

## Durable attempt registration

Before the worker runs, PostgreSQL persistence records:

- execution identity;
- workflow/step/attempt;
- owner and fencing token;
- running status;
- authorization evidence;
- invocation ID;
- agent ID;
- canonical capability ID;
- requested side-effect class;
- required policies;
- approval reference;
- idempotency key;
- correlation ID;
- admission timestamp.

Authorization evidence and attempt start are committed in one database transaction.

## Failure semantics

If attempt registration fails, the worker is not invoked.

If the worker completes but the workflow commit fails, the durable attempt remains available for reconciliation.

Workflow state and outbox events remain committed through the existing transactional outbox contract.

Attempt completion is fenced by execution ID, owner and fencing token.

## Worker boundary

The production async worker implements `AsyncWorkerExecutor`.

Workers own external I/O but must validate fencing before side effects. The coordinator does not resolve secrets or call external providers directly.

## Recovery

A running attempt is durable evidence that work may have started. Recovery must reconcile external execution before replaying the workflow step.

Authorization evidence is retained separately from the workflow snapshot so audit and reconciliation do not depend on in-memory coordinator state.

## Verification

Unit-level contracts are implemented. End-to-end PostgreSQL/EventBus validation remains the authoritative CI/integration gate.

The current GitHub Actions workflow has not yet produced a green result for the latest branch head.