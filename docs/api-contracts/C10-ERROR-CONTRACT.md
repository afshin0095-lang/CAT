# C10 — Error Contract

All externally visible failures use a stable machine-readable shape.

```json
{
  "error": {
    "code": "OPPORTUNITY_NOT_FOUND",
    "category": "not_found",
    "message": "Opportunity was not found.",
    "retryable": false,
    "request_id": "req_01...",
    "details": {}
  }
}
```

## Categories

`validation`, `authentication`, `authorization`, `not_found`, `conflict`, `rate_limit`, `dependency`, `timeout`, `internal`, `policy`.

Messages must be safe for external exposure. Internal stack traces, SQL, credentials, provider secrets, and infrastructure topology are never returned to clients.
