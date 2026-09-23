# D13 — Revenue & Commission Domain

Revenue is modeled as monetary facts with provenance, currency and settlement status.

Lifecycle: `Expected → Confirmed → Reversed | Disputed → Settled` according to the provider contract. Transitions are evidence-backed and auditable.

Money uses integer minor units plus currency. No floating-point arithmetic is permitted. Commission records reference the originating conversion/attribution and provider reference. Payouts reference settled commission facts and an external settlement identifier.

Corrections are compensating facts; historical monetary records are not silently overwritten.
