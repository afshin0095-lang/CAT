# D03 — Value Objects

Value objects are immutable, validated and compared by value.

Core vocabulary includes `OpportunityId`, `SourceId`, `TenantId`, `AgentId`, `Revision`, `CanonicalKey`, `Money`, `Currency`, `Percentage`, `Score`, `Timestamp`, `DateRange`, `Pagination`, `DedupKey`, `ProviderRef`, and `PolicyVersion`.

## Money
Never represent monetary values with floating point. Store integer minor units plus an explicit ISO currency code. Arithmetic must detect overflow.

## Score
Ranking scores use deterministic integer representations where precision and ordering are contractual.

## CanonicalKey
Normalization must be deterministic and locale-independent. Invalid or ambiguous normalization fails closed rather than silently producing a collision.

## Timestamp
UTC instants are preferred. Domain logic receives a Clock abstraction; business tests must not depend on wall-clock sleep.

## Revision
Monotonically increasing, checked arithmetic. Overflow is an explicit domain error.
