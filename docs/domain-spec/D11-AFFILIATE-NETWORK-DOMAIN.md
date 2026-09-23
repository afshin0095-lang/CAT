# D11 — Affiliate Network Domain

Represents networks and provider integrations without leaking provider-specific behavior into core domains.

Core concepts: network identity, account/connection reference, provider capability, rate/quota policy, credential reference, provider offer reference and provider health.

Adapters translate provider payloads into CAT contracts. Provider credentials are references to secret storage, never domain values. Provider failures map to the stable error taxonomy.

Network integration must support idempotency, bounded retries, rate limits, observability and explicit capability declarations. A provider adapter cannot bypass governance or write arbitrary domain state.
