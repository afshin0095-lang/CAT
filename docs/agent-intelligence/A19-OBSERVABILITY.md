# A19 — Agent Observability

Every agent run is traceable through `run_id`, `correlation_id`, `causation_id`, `agent_id`, version, capability, model/provider class, start/end timestamps, outcome, token/cost counters, tool counts, policy decisions, and failure classification.

Sensitive prompts, credentials, and private payloads are not logged by default. Observability uses structured events and stable metric names.

Core metrics: run count, success/failure, latency, cost, model calls, tool calls, budget exhaustion, policy denials, validation failures, retries, escalations, and provider health.
