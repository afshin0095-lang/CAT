# O03 — Logging Standard

Logs are structured events, not prose dumps.

Required baseline fields:

| Field | Purpose |
|---|---|
| timestamp | event time |
| level | severity |
| service | emitting component |
| environment | deployment context |
| version | software/config version |
| request_id | request correlation |
| trace_id | distributed trace correlation |
| event | stable event name |
| outcome | success/failure/result class |

Secrets, credentials, authorization tokens, raw provider keys, and unnecessary personal data are never logged. Errors expose stable machine-readable codes while avoiding sensitive internals.

Sampling may reduce volume, but security/audit events must follow retention and evidence requirements independently.