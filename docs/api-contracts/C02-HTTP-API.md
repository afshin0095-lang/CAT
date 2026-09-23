# C02 — HTTP API Contract

## Conventions

- Base path: `/api/v1` for the first stable public contract.
- `GET` is side-effect free.
- `POST` creates resources or executes explicit commands.
- `PATCH` performs partial resource updates where supported.
- `DELETE` is used only where deletion is a domain-supported operation.
- `202 Accepted` means durable asynchronous work was accepted, not completed.
- `201 Created` means the resource was durably created.
- `409 Conflict` represents a domain or optimistic-concurrency conflict.
- `422 Unprocessable Content` represents structurally valid but semantically invalid input.
- `429 Too Many Requests` represents rate limiting.

## Headers

Required where applicable: `Authorization`, `Content-Type`, `Accept`, `X-Request-Id`, `Idempotency-Key` for retryable mutations, and `If-Match` for conditional writes.

## Response envelope

```json
{
  "data": {},
  "meta": {"request_id": "req_01..."}
}
```

Errors use the canonical error contract in C10.
