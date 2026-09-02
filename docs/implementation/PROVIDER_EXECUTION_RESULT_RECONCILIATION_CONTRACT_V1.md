# CAT Provider Execution Result & Reconciliation Contract V1

## Purpose

This contract defines how CAT reconciles a durable workflow execution with an external provider after worker interruption, lease turnover, timeout, or process restart.

## Core rule

The internal workflow state is not sufficient evidence that an external side effect happened. When a worker may have crossed an external provider boundary, CAT must prefer durable provider evidence keyed by `provider + provider_execution_id` and correlated to the local `execution_id`.

## Identity

Every external side effect must have:

- a local `execution_id` for the execution attempt;
- a deterministic request hash representing the intended provider operation;
- a provider name;
- a provider-issued `provider_execution_id` whenever the provider exposes one.

The provider execution identifier is unique within the provider and is persisted before CAT treats the submission as safely reconcilable.

## Reconciliation states

| State | Meaning | Allowed automatic action |
| --- | --- | --- |
| No provider result | Submission exists but outcome is not yet observed | Continue reconciliation; do not duplicate blindly |
| Succeeded | Provider confirms the operation | Confirm local success |
| Failed | Provider confirms failure | Confirm local failure/retry according to policy |
| Unknown | Provider cannot establish a definitive outcome | Manual review or provider-specific recovery flow |

## Idempotency

A retry must never create a new provider side effect merely because the local worker was restarted. The retry path must first query the provider using the durable provider execution identity, or use an equivalent provider-specific idempotency key.

If the provider supports idempotency keys, the request hash and CAT execution identity should be mapped into that provider key. The mapping must be stable for the lifetime of the execution attempt.

## Fencing

Provider result recording does not replace lease fencing. The worker that submits or records a provider result must still hold the valid execution lease/fencing token for the corresponding attempt. A stale worker must not overwrite a newer execution's durable orchestration state.

## Result authority

Provider evidence is stronger than an in-memory worker return value after a crash boundary. A local `Succeeded` result without durable provider evidence must therefore remain reconcilable until the external boundary is known to be settled.

## Conflict handling

A provider execution result is immutable in outcome semantics:

- repeated observation of the same outcome is idempotent;
- changing `Succeeded` to `Failed`, or `Failed` to `Succeeded`, is rejected;
- changing the provider execution identity for an existing local execution is rejected by the uniqueness and consistency constraints.

## Recovery sequence

```text
load workflow
  -> load execution attempt
  -> load provider execution record
  -> provider result available?
       yes -> classify outcome
       no  -> query provider / provider reconciliation API
                 -> persist observed result
                 -> classify outcome
  -> apply orchestration transition under current fencing token
```

## Safety invariant

`STALE RUNNING` must never imply `SAFE TO REPLAY`.

Only a confirmed provider failure, a provider-side idempotent rejection, or an explicit human/provider reconciliation decision can authorize a new external attempt.

## Operational observability

At minimum, emit metrics and structured logs for:

- reconciliation attempts;
- unresolved provider executions;
- provider outcome latency;
- duplicate/idempotency detections;
- conflicting provider outcomes;
- stale execution attempts;
- manual reconciliation decisions.

## Future extension

Provider-specific adapters may expose stronger evidence such as settlement receipts, transaction hashes, order identifiers, shipment identifiers, or signed provider attestations. Such evidence should enrich the result ledger rather than bypass it.
