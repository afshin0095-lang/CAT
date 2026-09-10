# Affiliate Opportunity Events P0

## Purpose

Typed, versioned event contracts for the opportunity platform. Sprint 0 ships the contracts and their serialization guarantees; the event-producing boundaries are wired in Sprint 1.

## Scope

- `core/affiliate/rust/src/events.rs` — new `CatEvent` implementations.
- `core/affiliate/rust/tests/events.rs` — contract tests.

## Architecture

Events cross boundaries via `cat_eventbus::CatEvent` (typed payload → `EventEnvelope` with `TYPE`/`VERSION`). Commands and requests remain distinct from events:

```text
RevalidationRequested (command boundary input)  ≠  affiliate.opportunity.revalidation_requested (fact)
```

## Data model

| Event | TYPE | Payload facts |
|---|---|---|
| `OpportunityDiscovered` | `affiliate.opportunity.discovered` v1 | opportunity_id, identity, source, best_score, observed_at_ms, revision |
| `OpportunityUpdated` | `affiliate.opportunity.updated` v1 | + best_source, revision |
| `OpportunityBecameStale` | `affiliate.opportunity.became_stale` v1 | last_observed_at_ms, stale_threshold_ms, detected_at_ms |
| `OpportunityBecameExpired` | `affiliate.opportunity.became_expired` v1 | last_observed_at_ms, expiration_threshold_ms, detected_at_ms |
| `OpportunityRevalidationRequested` | `affiliate.opportunity.revalidation_requested` v1 | request_id, source, reason, priority, requested/scheduled |
| `OpportunityRevalidated` | `affiliate.opportunity.revalidated` v1 | request_id, source, observed_at_ms, revision |
| `OpportunityRevalidationFailed` | `affiliate.opportunity.revalidation_failed` v1 | request_id, source, reason, failed_at_ms, bounded detail |

All payloads are immutable typed structures (no ad-hoc JSON strings).

## API contracts

- Every event implements `CatEvent { TYPE, VERSION }` and serializes losslessly.
- `OpportunityRevalidationFailed::new(...)` bounds `detail` to 512 characters and strips control characters.

## State transitions

These contracts **represent facts**, not commands. Emission rules:

- `OpportunityDiscovered` / `OpportunityUpdated`: emitted by the ingestion publication boundary when the store reports `created` / `changed`.
- `OpportunityBecameStale` / `OpportunityBecameExpired`: only ever emitted by a detection boundary that records the transition as a fact. **Read queries that merely derive `Stale`/`Expired` must never emit them.**
- `OpportunityRevalidationRequested` / `OpportunityRevalidated` / `OpportunityRevalidationFailed`: emitted by the request store / execution boundary (Sprint 1).

## Invariants

1. Event types are globally unique across the affiliate domain.
2. `VERSION = 1` for all Sprint 0 opportunity events; additive payload evolution bumps the version.
3. Identity and timestamps survive serialization round trips exactly.
4. Malformed payloads (missing fields, bad UUIDs, unknown closed-enum values) are rejected at deserialization (fail closed).
5. Forward-compatible enums (`RevalidationReason`) decode unknown values into `Unknown(String)` on read paths.
6. No secrets, credentials, headers, or raw provider payloads in any payload; failure details are bounded and sanitized.

## Error handling

Serialization failures surface through `EventBusError::Serialization` at envelope construction; contract tests pin malformed-payload rejection.

## Persistence behavior

Events persist through the EventBus outbox/event-store boundaries, not through the affiliate domain. Sprint 0 defines contracts only.

## Concurrency behavior

Payloads are immutable `Send + Sync` values; envelopes receive UUIDv7 identifiers and kernel timestamps at construction.

## Idempotency

Consumers deduplicate on `request_id` / `opportunity_id` + `revision` carried in payloads; `OpportunityUpdated.revision` correlates with the store's version column.

## Testing strategy

Global type-uniqueness, typed envelope construction, JSON round trips, identity/timestamp preservation, forward-compatible reason decoding, malformed-payload rejection, and detail-bounding tests.

## Security considerations

Payload fields are domain facts only; the single free-text field (`OpportunityRevalidationFailed.detail`) is bounded and sanitized at construction.

## Future extension points

- Sprint 1 wires ingestion publication and durable revalidation execution to these contracts.
- A `subject_id` mapping (opportunity UUID) on envelopes for router-level filtering.

## Known limitations

- No emitting boundary exists in Sprint 0 by design; declaring these types does not imply emission.
