# O03 — Logging Standard

Logs are structured records, not prose dumps.

Required baseline fields:

| Field | Purpose |
|---|---|
| timestamp | event time |
| level | severity |
| service | emitting component |
| environment | runtime environment |
| request_id | request correlation |
| trace_id | distributed correlation |
| actor_id | principal where permitted |
| tenant_id | tenant context where permitted |
| event | stable machine-readable name |
| duration_ms | operation latency |
| outcome | success/failure classification |

Never log credentials, authorization headers, raw provider secrets, or unrestricted personal data. Error logs should contain actionable classification and a safe diagnostic reference.
