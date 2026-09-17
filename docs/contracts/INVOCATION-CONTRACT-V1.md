# CAT Invocation Contract v1

**Status:** Implemented in Rust kernel
**Canonical implementation:** `core/kernel/rust/src/invocation.rs`

## Purpose

The invocation contract is the canonical boundary between an executable CAT agent capability and the runtime responsible for executing it.

## Request

`InvocationRequest` contains:

- `contract_version`
- `invocation_id`
- `agent_id`
- `capability`
- `context`
- `idempotency_key`
- `input`
- `requested_side_effect`
- `requested_at`

The request MUST identify the agent, capability, execution context, and idempotency boundary.

## Outcome

`InvocationOutcome` contains:

- `contract_version`
- `invocation_id`
- `status`
- `lifecycle_status`
- `evidence`
- optional `output`
- optional `error_code`
- `completed_at`

`Succeeded`, `Failed`, and `Unknown` are intentionally distinct. `Unknown` means the runtime cannot safely assert success or failure and therefore requires reconciliation rather than blind retry.

## Evidence

`EvidenceRef` records an evidence type and reference, with an optional source. Evidence is attached to the outcome rather than hidden in logs.

## Validation Rules

1. Contract major version MUST be non-zero.
2. Capability MUST be non-empty.
3. Idempotency key MUST be present and non-empty.
4. Outcome lifecycle MUST be terminal.
5. Outcome status MUST agree with lifecycle status.
6. Unknown MUST remain distinguishable from Failed.

## Compatibility

The contract is versioned independently from implementation language. Future breaking changes MUST introduce an explicit contract version and migration path.

## Implementation Rule

The Rust implementation is authoritative for kernel behavior. Higher-level adapters MAY serialize or map the contract, but MUST NOT silently alter its semantics.
