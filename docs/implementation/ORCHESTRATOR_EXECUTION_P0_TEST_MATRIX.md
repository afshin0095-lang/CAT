# Orchestrator Execution P0 Test Matrix

| Area | Required behavior |
|---|---|
| Cursor | Exposes only ready steps and current revision |
| Retry | Uses bounded exponential backoff and stops at max attempts |
| State | Invalid transitions are rejected |
| Dependency | Completing a prerequisite unlocks dependent work |
| Cancellation | Non-terminal workflows can be cancelled; terminal ones cannot |
| Recovery | Exhausted retries enter the failure/recovery path |

These contracts are intentionally independent of external workers, databases, or transports so the core state machine can be verified deterministically.
