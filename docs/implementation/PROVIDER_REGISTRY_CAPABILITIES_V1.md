# Provider Registry and Capability Contract V1

## Purpose

The registry allows CAT to connect many affiliate networks, stores, advertising platforms, and other external providers without hard-coding them into the workflow engine.

## Provider capabilities

- **Idempotency**: accepts CAT's stable idempotency key.
- **ExecutionLookup**: CAT can query the remote execution after a crash.
- **AsyncCompletion**: completion may happen after submission.
- **Cancellation**: a submitted operation can be cancelled.

## Selection

A workflow declares the capabilities required for a safe operation. CAT selects only providers that satisfy all required capabilities.

```text
Workflow Requirement
       ↓
Provider Registry
       ↓
Capability Filter
       ↓
Eligible Providers
       ↓
Selected Adapter
```

The registry deliberately does not choose a provider by price, revenue, or popularity yet. Those policies belong to a future provider-selection engine.
