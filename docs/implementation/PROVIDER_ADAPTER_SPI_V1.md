# Provider Adapter SPI V1

## Purpose

This contract isolates CAT durable execution from affiliate networks, ad platforms, commerce APIs, and future external providers.

## Rules

1. Every external submission uses a stable CAT idempotency key.
2. The provider adapter returns an immutable provider execution identity.
3. Reconciliation looks up an existing provider execution before any replay.
4. A missing or unknown remote result must not trigger blind resubmission.
5. Provider-specific SDK details remain outside the orchestrator domain.

## Flow

```text
Execution Attempt
  -> ProviderExecutionAdapter.submit
  -> provider_execution_id
  -> Provider Result Ledger
  -> ReconciliationWorker.lookup
  -> Confirm / Fail / Continue / Manual Review
```

## Idempotency

CAT derives a stable key from the execution identity and immutable request hash. Adapters must pass it to providers whenever the provider supports idempotency.
