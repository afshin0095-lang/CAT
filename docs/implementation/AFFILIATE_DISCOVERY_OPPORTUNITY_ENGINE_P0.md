# Affiliate Discovery & Opportunity Engine P0

## Purpose

P0 establishes a deterministic domain boundary for turning heterogeneous affiliate/product observations into ranked CAT opportunities. It is intentionally provider-agnostic: network adapters remain responsible for acquisition, while discovery owns validation, normalization, scoring, ranking, thresholding, and limiting.

## Contract

`DiscoveryCandidate` contains:

- `source` and `external_id` for source-level traceability.
- merchant/product identity and a normalized `canonical_key`.
- destination URL, currency, optional price, and optional commission rate.
- demand, competition, freshness, and compliance signals on a 0..10,000 scale.
- source observation timestamp.

Malformed candidates are rejected individually so one bad upstream record cannot poison a discovery batch.

## Determinism

Ranking is deterministic:

1. opportunity score descending;
2. canonical key ascending;
3. source ascending;
4. external ID ascending.

The score is an integer basis-point-style value on a 0..10,000 scale. P0 weights are:

| Signal | Weight |
| --- | ---: |
| Economic potential | 35% |
| Demand | 25% |
| Competition inverse | 15% |
| Freshness | 15% |
| Compliance | 10% |

Economic potential currently uses the commission rate when available, capped at 10,000 bps for the scoring signal. This is a deliberately conservative P0 heuristic; future versions can replace it with learned models without changing the discovery boundary.

## Output

`DiscoveryResult` contains ranked `DiscoveryOpportunity` records and a count of rejected candidates. Each opportunity receives a runtime UUIDv7 identifier, score, and one-based rank.

## Non-goals for P0

- live crawling or scraping;
- affiliate-network authentication;
- remote API retries;
- persistence;
- ML-based scoring;
- automatic publishing or monetization decisions.

Those capabilities belong behind explicit provider, orchestration, and policy boundaries.

## Safety / correctness invariants

- Scores cannot exceed 10,000.
- Negative prices are rejected.
- Commission rates above 100,000 bps are rejected.
- Empty identity, URL, or currency fields are rejected.
- Zero discovery limit is rejected.
- Ranking and filtering are deterministic for identical inputs.
- The engine never performs network I/O.
