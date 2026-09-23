# C14 — Rate Limit Contract

Rate limiting protects both CAT and external providers.

## Dimensions

Limits may be applied by tenant, principal, endpoint, provider, capability, IP/network origin, or resource class.

## Response

A rejected request returns `429` with a stable error code and, where known, `Retry-After`.

## Internal backpressure

Workers must also respect concurrency, queue depth, provider quotas, cost budgets, and deadline budgets. A rate limit is not permission to retry immediately; retry policy must classify the dependency response.
