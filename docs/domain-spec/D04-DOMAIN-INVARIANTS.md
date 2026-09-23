# D04 — Domain Invariants

The following are non-negotiable invariants.

1. Identity is immutable after creation.
2. Revisions never decrease or wrap.
3. Derived lifecycle state is computed from authoritative facts and policy.
4. Clock regression fails closed where temporal ordering matters.
5. Duplicate observations are idempotent.
6. Claiming work is exclusive and concurrency-safe.
7. Domain scoring is deterministic and reproducible.
8. Monetary arithmetic never uses binary floating point.
9. Secrets never enter domain events, logs or prompts.
10. Every external side effect has an attributable actor, capability and evidence record.
11. Agents cannot bypass policy gates to call privileged capabilities.
12. Cross-tenant data access is denied unless an explicit trusted boundary permits it.
13. Unknown event versions are handled according to compatibility policy rather than destructively guessed.
14. Failed validation never silently downgrades to an executable action.
15. Replay must not duplicate irreversible side effects.

These invariants outrank convenience, performance shortcuts and individual agent instructions.
