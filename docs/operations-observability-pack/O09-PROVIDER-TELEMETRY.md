# O09 — Provider Telemetry

Every external provider adapter reports normalized operational outcomes.

| Signal | Purpose |
|---|---|
| availability | provider reachable/usable |
| latency | response distribution |
| status class | success/client/server/rate-limit |
| retries | retry pressure |
| quota state | remaining/unknown |
| request outcome | domain-level result |

Provider-specific details remain inside the adapter boundary. The rest of CAT consumes normalized telemetry and typed errors, preventing provider quirks from leaking through the domain.