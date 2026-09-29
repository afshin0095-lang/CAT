# O20 — Operations Readiness Gates

A capability is production-ready only when its operational contract exists.

| Gate | Required evidence |
|---|---|
| Observability | logs, metrics, traces/correlation where applicable |
| Reliability | timeout, retry, backpressure and failure semantics |
| Security | identity, authorization, secrets and audit controls |
| Recovery | rollback/reconciliation procedure |
| Capacity | known limits and load behavior |
| Data | backup/retention/restore requirements |
| Cost | measurable resource/provider consumption |
| Testing | unit/integration/failure-path coverage |

Autonomous or financially consequential capabilities require all gates before unrestricted production operation.