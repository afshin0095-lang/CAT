# O04 — Metrics Standard

Metrics describe aggregate system behavior without requiring raw payload inspection.

Core metric families:

- request rate, latency, error rate;
- queue depth and age;
- worker utilization and throughput;
- provider success/error/latency;
- model token usage and cost;
- agent steps, failures, retries, and tool calls;
- affiliate opportunity discovery, ingestion, revalidation, and conversion-related operational events.

Metric names are stable contracts. High-cardinality values such as raw user IDs, URLs, prompts, or arbitrary provider payloads must not become metric labels.

Counters are monotonic where appropriate; gauges represent current state; histograms capture distributions.