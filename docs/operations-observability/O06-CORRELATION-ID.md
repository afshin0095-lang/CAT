# O06 — Correlation ID Standard

CAT uses separate identifiers for separate concerns.

| Identifier | Scope |
|---|---|
| request_id | inbound request |
| trace_id | distributed operation |
| span_id | trace segment |
| command_id | durable command |
| event_id | immutable event |
| job_id | asynchronous execution |
| agent_run_id | one agent execution |
| provider_request_id | external provider operation |

Identifiers must not be silently reused across semantic scopes. When one operation creates another durable operation, the parent correlation is retained as metadata while the child receives its own stable ID.
