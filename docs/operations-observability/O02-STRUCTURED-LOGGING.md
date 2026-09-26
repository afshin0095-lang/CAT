# O02 — Structured Logging

CAT logs are structured events, not arbitrary strings.

Required common fields:

| Field | Purpose |
|---|---|
| timestamp | event time |
| level | severity |
| service | emitting component |
| environment | deployment context |
| request_id | request correlation |
| correlation_id | cross-component workflow correlation |
| trace_id | distributed trace linkage |
| tenant_id | scoped tenant context when applicable |
| operation | stable operation name |
| outcome | success/failure/partial |
| duration_ms | latency |
| error_code | stable machine-readable error |

Secrets, authentication tokens, raw provider credentials, and unnecessary personal data are prohibited from logs. High-cardinality or large payloads must not be emitted as labels or unbounded fields.
