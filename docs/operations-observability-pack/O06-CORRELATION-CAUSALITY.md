# O06 — Correlation & Causality

CAT distinguishes identifiers that answer different questions.

| Identifier | Meaning |
|---|---|
| request_id | one inbound request |
| trace_id | one distributed execution trace |
| command_id | one requested state-changing operation |
| event_id | one emitted domain/integration event |
| job_id | one asynchronous job execution |
| agent_run_id | one bounded agent run |
| provider_request_id | provider-side request correlation |

Identifiers are propagated only across semantically related work. Retries create attempts but retain the parent command/job relationship. This prevents a retry storm from appearing as a new unrelated operation.