# D09 — Error Taxonomy

Errors are classified so retry and human intervention are deterministic.

- **Validation** — input violates a contract; do not retry unchanged.
- **Authorization** — principal lacks permission; do not retry automatically.
- **Conflict** — revision/idempotency conflict; reload and reconcile.
- **NotFound** — authoritative entity absent.
- **Unavailable** — dependency temporarily unavailable; retry with bounded backoff.
- **RateLimited** — provider quota; respect retry-after and budget.
- **Timeout** — execution exceeded deadline; retry only when operation is safe.
- **ProviderRejected** — external provider rejected a valid request; classify by provider policy.
- **InvariantViolation** — internal correctness breach; quarantine and alert.
- **Serialization/Schema** — incompatible payload; quarantine rather than guess.
- **Security** — policy or trust boundary violation; fail closed.
- **Unknown** — preserve evidence and prevent unsafe execution.

Each error should expose stable machine classification, safe human message, correlation id and retryability. Secrets and raw credentials are never included.
