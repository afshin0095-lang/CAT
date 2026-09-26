# O08 — Provider Telemetry

Provider adapters expose normalized operational signals without leaking provider-specific payloads.

Tracked dimensions include provider, capability, operation, latency class, status class, retry count, rate-limit state, availability, and error category.

Provider-specific raw errors remain at the adapter boundary and are translated into CAT error classes. Provider identifiers must be controlled vocabulary values rather than arbitrary user input.

The telemetry model supports comparing providers operationally without coupling domain decisions to a single vendor.
