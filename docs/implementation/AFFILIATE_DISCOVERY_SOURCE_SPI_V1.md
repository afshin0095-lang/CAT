# CAT Affiliate Discovery Source SPI V1

## Purpose

Define the stable boundary between external product/program discovery mechanisms and the CAT Affiliate opportunity engine.

The SPI permits affiliate networks, merchant catalogs, product feeds, marketplaces, search indexes, and future agent connectors to be added without coupling the discovery engine to transport, authentication, or vendor-specific schemas.

## Flow

```text
External Source -> DiscoverySource adapter -> DiscoverySourceBatch
                                      -> DiscoveryCandidate
                                      -> DiscoveryEngine -> DiscoveryOpportunity
```

`DiscoverySource` owns source I/O. `DiscoveryIngestion` coordinates collection. `DiscoveryEngine` owns validation, scoring, ranking, and opportunity creation.

## Contract

- `DiscoverySourceInfo`: stable source identity, kind, and capabilities.
- `DiscoverySourceRequest`: query, pagination, category, geography, and currency constraints.
- `DiscoverySourceBatch`: normalized candidates plus pagination state.
- `DiscoverySourceError`: source-independent request, availability, authentication, rate-limit, and response failures.

The adapter method is future-based so network-bound implementations remain asynchronous without forcing a runtime dependency into the domain crate.

## Capabilities

V1 exposes explicit capabilities:

- Pagination
- Incremental sync
- Category filtering
- Geographic filtering
- Currency filtering

Consumers must not assume unsupported capabilities.

## Correctness invariants

- Registration order is preserved.
- Source IDs are explicit and stable.
- Invalid requests are rejected before source invocation.
- A source failure is isolated to that source result.
- The SPI never ranks or silently repairs candidates.
- Adapters normalize vendor records into `DiscoveryCandidate`.

## Pagination

A source may return `has_more=true` with `next_page=Some(page)`. Pagination policy intentionally remains outside the SPI so a future scheduler can add rate limits, checkpoints, backoff, and incremental synchronization without changing source adapters.

## Security

Credentials and secrets must remain outside domain models. Adapters are responsible for authentication, signing, rate limiting, and vendor-specific validation and must not leak secrets through errors or candidates.

## Non-goals

V1 does not define HTTP clients, SDKs, credential storage, retries, distributed scheduling, checkpoint persistence, deduplication, scoring, or affiliate-link generation.

## Future compatibility

New source kinds and capabilities can be added independently of transport. Network-specific implementations can reuse `NetworkAdapter` and translate network records into discovery candidates at this boundary.
