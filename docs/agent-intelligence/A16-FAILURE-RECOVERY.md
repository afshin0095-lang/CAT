# A16 — Failure & Recovery

Failure classes are explicit:

`validation`, `policy`, `authentication`, `authorization`, `rate_limit`, `timeout`, `provider`, `transient_infrastructure`, `dependency`, `budget_exhausted`, `conflict`, `unknown`.

Recovery policy maps classes to `retry`, `backoff`, `alternate_provider`, `replan`, `dead_letter`, `human_review`, or `stop`.

Retries use idempotency keys and bounded exponential backoff with jitter where appropriate. A failed model response is never treated as a successful domain result. Repeated failures create durable evidence for observability and future evaluation.
