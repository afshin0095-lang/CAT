# O04 — Metrics Catalog

CAT metrics are stable contracts. Names should be low-cardinality and independent of arbitrary user input.

### Platform

- `cat.requests.total`
- `cat.requests.duration_ms`
- `cat.requests.errors_total`
- `cat.queue.depth`
- `cat.queue.wait_ms`
- `cat.worker.active`

### Agents

- `cat.agent.runs_total`
- `cat.agent.failures_total`
- `cat.agent.steps_total`
- `cat.agent.duration_ms`
- `cat.agent.cost_units`

### Providers

- `cat.provider.requests_total`
- `cat.provider.errors_total`
- `cat.provider.latency_ms`
- `cat.provider.rate_limited_total`

### Affiliate domain

- `cat.opportunity.discovered_total`
- `cat.opportunity.revalidated_total`
- `cat.opportunity.revalidation_failures_total`
- `cat.conversion.events_total`

Metrics must not encode raw IDs as labels unless bounded cardinality is proven.
