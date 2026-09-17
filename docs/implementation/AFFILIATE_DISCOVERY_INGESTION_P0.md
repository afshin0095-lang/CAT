# Affiliate Discovery → Opportunity Ingestion P0

## Purpose

This boundary connects the existing discovery-source SPI to the opportunity persistence model without coupling external providers to storage.

The pipeline is deliberately split into three responsibilities:

1. **DiscoverySource / DiscoveryIngestion** collects normalized `DiscoveryCandidate` batches.
2. **DiscoveryEngine** validates candidates, computes the deterministic opportunity score, filters by minimum score, and applies the per-source limit.
3. **OpportunityStore** persists the resulting `DiscoveryOpportunity` values and performs cross-source deduplication while retaining provenance.

## Contract

`OpportunityIngestion::ingest(request, min_score, limit_per_source)` performs one ingestion pass across all registered discovery sources.

- A failing source does not discard successful sources.
- Invalid candidates are rejected by the discovery engine and counted.
- One source cannot exceed `limit_per_source` opportunities in a pass.
- Deduplication is performed by the existing merchant/product canonical identity.
- Source observations remain attached to the aggregate opportunity.
- Repeated identical observations are idempotent at the store boundary.
- Persistence failures are counted rather than silently treated as successful writes.

## Report semantics

`OpportunityIngestionReport` exposes:

- `sources_succeeded`
- `sources_failed`
- `opportunities_discovered`
- `candidates_rejected`
- `opportunities_created`
- `opportunities_changed`
- `persistence_failures`

The report is intentionally operational rather than provider-specific. Detailed provider diagnostics remain the responsibility of the source layer.

## Safety invariants

- No external provider transport or authentication logic exists in the ingestion coordinator.
- No SQL or storage-specific code exists in the source boundary.
- Discovery ranking remains deterministic.
- Opportunity identity remains independent of source ordering.
- Cross-source provenance is never discarded merely because two providers describe the same merchant/product.
- PostgreSQL timestamp conversion is explicitly checked before converting the domain `u64` timestamp to `BIGINT`.

## Next boundary

The next affiliate-domain increment should add opportunity lifecycle semantics: freshness windows, expiration state, revalidation scheduling, and controlled re-discovery. Lifecycle decisions should consume persisted opportunity observations rather than reaching directly into discovery providers.
