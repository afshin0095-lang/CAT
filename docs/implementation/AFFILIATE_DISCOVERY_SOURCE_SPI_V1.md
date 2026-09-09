# CAT Affiliate Discovery Source SPI V1

## Purpose

Define the stable boundary between external product/program discovery mechanisms and the CAT Affiliate opportunity engine.

The SPI allows CAT to add affiliate networks, merchant catalogs, product feeds, marketplaces, search indexes, and future agent connectors without coupling the discovery engine to transport, authentication, or vendor-specific schemas.

## Architecture

```text
External Source
      |
      v
DiscoverySource adapter
      |
      | DiscoverySourceBatch
      v
DiscoveryIngestion
      |
      | DiscoveryCandidate
      v
DiscoveryEngine
      |
      v
DiscoveryOpportunity
```

`DiscoverySource` owns source I/O. `DiscoveryIngestion` coordinates source collection. `DiscoveryEngine` owns candidate validation, scoring, ranking, and opportunity creation.

## Contract

Each source exposes:

- `DiscoverySourceInfo` — stable source identity, name, kind, and capabilities.
- `DiscoverySourceRequest` — query, pagination, category, geography, and currency constraints.
- `DiscoverySourceBatch` — normalized candidates plus pagination state.
- `DiscoverySourceError` — transport/auth/rate-limit/response failures without leaking vendor SDK types.

The source method is future-based so network-bound adapters can remain asynchronous without adding a runtime dependency to the domain crate.

## Capabilities

V1 capabilities are explicit rather than inferred:

- Pagination
- Incremental sync
- Category filtering
- Geographic filtering
- Currency filtering

Consumers must not assume a capability that is absent from `DiscoverySourceInfo`.

## Determinism and isolation

- Registration order is preserved by `DiscoverySourceRegistry`.
- Source IDs are explicit and stable.
- Requests are validated before a source is invoked.
- Source errors are returned per source; one source failure does not erase successful results from other sources.
- The SPI does not rank or mutate candidates.
- Source adapters must normalize external records into `DiscoveryCandidate` before returning a batch.

## Pagination

A source may return `has_more=true` with `next_page=Some(page)`. Pagination policy remains outside the SPI so a future scheduler can implement rate limits, checkpoints, backoff, and incremental synchronization without changing the adapter contract.

## Security

Adapters must keep credentials outside domain models and must not return secrets in `DiscoverySourceError` or candidate fields. Authentication, signing, rate limiting, and vendor-specific request validation remain adapter responsibilities.

## Non-goals

V1 does not define:

- HTTP clients or SDKs
- credential storage
- retry policy
- distributed scheduling
- persistence/checkpoint storage
- deduplication policy
- opportunity scoring
- affiliate-link generation

Those concerns belong to higher-level infrastructure or the dedicated domain components.

## Future compatibility

The SPI is intentionally source-neutral. New source kinds and capabilities can be added without changing the `DiscoveryCandidate` contract. Network-specific adapters should reuse `NetworkAdapter` where appropriate and translate network program/catalog records into discovery candidates at this boundary.
