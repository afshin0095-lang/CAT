# O03 — Metrics Contract

Metrics are designed for aggregation and alerting. They must remain bounded in cardinality.

Core categories:

- request rate, errors, latency;
- queue depth, age, throughput, retry rate;
- agent executions, tool calls, failures, duration;
- model calls, tokens, latency, failures, estimated cost;
- provider availability, rate limits, error classes;
- database latency, pool saturation, transaction failures;
- opportunity discovery, ingestion, ranking, revalidation and conversion-domain counters.

Stable metric names should use the `cat.*` namespace and avoid embedding IDs, URLs, user input, or arbitrary provider values in labels.
