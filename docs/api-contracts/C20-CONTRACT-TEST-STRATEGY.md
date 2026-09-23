# C20 — Contract Test Strategy

Every contract is validated at the boundary where it is consumed.

## Test layers

1. **Schema tests** — valid/invalid payloads and compatibility.
2. **Handler tests** — authentication, authorization, status mapping, and envelope shape.
3. **Domain contract tests** — commands preserve invariants.
4. **Event contract tests** — serialization, versioning, and required metadata.
5. **Provider contract tests** — normalized success/error/rate-limit/timeout behavior.
6. **Consumer compatibility tests** — older supported consumers continue to parse current contracts.
7. **Security tests** — secret redaction, authorization bypass attempts, replay and signature validation.
8. **Property tests** — idempotency, deterministic ordering, pagination boundaries, and checked numeric conversions.

A contract is not considered stable until its compatibility behavior is represented by automated tests. Tests must never weaken production validation merely to make a fixture pass.
