# Affiliate Network Discovery Adapter P0

## Purpose

Bridge the existing `NetworkAdapter` contract into the source-neutral `DiscoverySource` SPI. Network-specific SDK, authentication, transport, and rate-limit behavior remain behind `NetworkAdapter`.

The existing network boundary exposes paginated program discovery through `list_programs(query, page, per_page)`. This adapter maps those programs into validated `DiscoveryCandidate` records. fileciteturn816file0L2-L2

## Contract

`NetworkDiscoveryAdapter` implements `DiscoverySource` and exposes:

- source identity: `network:<network_id>`
- source kind: `AffiliateNetwork`
- capability: `Pagination`
- query and pagination forwarding
- normalized commission rate in basis points
- stable merchant/product canonical identity
- neutral values where the network contract has no market signal

## Normalization Rules

- `5%` -> `500` bps
- `5.5%` -> `550` bps
- `0.05` -> `500` bps
- blank or malformed commission rates are rejected
- commission rates outside `0..=100%` are rejected
- blank required program identity/URL fields are rejected
- currency remains `UNKNOWN` because `NetworkProgram` does not provide currency
- demand and competition use neutral `5,000` values rather than fabricated network data
- freshness is `10,000` at ingestion time
- application eligibility maps to a conservative compliance signal

## Capability Boundary

The adapter currently supports only query + pagination because the current `NetworkAdapter::list_programs` contract does not expose category, geography, or currency filters. Unsupported filters fail before the network call instead of being silently ignored.

## Pagination

When the returned batch size equals `per_page`, the adapter reports `has_more = true` and provides the next page using saturating arithmetic. An undersized batch is treated as terminal.

## Determinism

Candidate identity and ranking inputs are deterministic for a given network response. `observed_at_ms` is supplied by the adapter clock and is therefore explicitly treated as observation metadata, not an identity field.

## Safety Invariants

1. No network-specific SDK type crosses the discovery boundary.
2. Invalid upstream records are rejected rather than partially normalized.
3. Unsupported filters fail closed.
4. No credentials or secrets are stored in discovery candidates.
5. Network failures are translated into `DiscoverySourceError::Unavailable`.
6. Discovery scoring remains outside this adapter.

## Non-Goals

P0 does not implement concrete vendor SDKs, authentication, retry policy, rate limiting, persistent ingestion, deduplication, or product-level market intelligence. Those belong to later adapter/infrastructure layers.
