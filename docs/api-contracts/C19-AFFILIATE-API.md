# C19 — Affiliate API Contract

The affiliate surface exposes normalized CAT concepts without exposing provider-specific internals.

## Core resource families

```text
/affiliates
/networks
/offers
/campaigns
/opportunities
/tracking-links
/conversions
/commissions
/reports
```

Mutations that have non-trivial domain semantics use explicit commands rather than pretending every operation is a generic CRUD update. Financial resources expose currency and integer minor units. Tracking identifiers are opaque and should not encode private internal identifiers.

Provider credentials, raw access tokens, internal database keys, policy internals, and agent chain-of-thought are never exposed through this API.
