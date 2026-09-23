# C16 — Audit Contract

Security-sensitive and financially meaningful actions produce audit evidence.

## Minimum fields

`audit_id`, `occurred_at`, `principal_id` or actor type, `tenant_id`, action, target type/id, outcome, correlation id, request id, policy decision reference, and reason/context where appropriate.

Audit records are append-oriented. They are not silently rewritten to change historical meaning. Sensitive values are redacted or represented by references/digests according to the data-classification policy.

Audit evidence must distinguish actor intent, system action, provider response, and resulting domain fact.
