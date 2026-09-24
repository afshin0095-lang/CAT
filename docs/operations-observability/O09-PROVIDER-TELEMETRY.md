# O09 — Provider Telemetry

Every external provider adapter exposes normalized operational telemetry.

Required dimensions: provider, operation, outcome class, latency, retry count, rate-limit state, timeout count, and provider request ID where available.

Provider-specific error codes remain available as structured metadata but must not leak secrets. Adapter telemetry enables CAT to compare providers operationally without coupling domain code to vendor-specific SDKs.
