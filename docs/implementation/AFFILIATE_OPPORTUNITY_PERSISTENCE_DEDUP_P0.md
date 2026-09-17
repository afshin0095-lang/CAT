# Affiliate Opportunity Persistence & Deduplication P0

## Purpose

Persist discovery opportunities behind a domain-level boundary while collapsing repeated observations of the same merchant/product into one aggregate record.

## Identity

`OpportunityIdentity` is derived from the existing `canonical_key(merchant_name, product_name)` function. The identity is source-neutral so the same commercial opportunity discovered through multiple affiliate networks is represented once.

The current canonical-key normalizer is intentionally reused rather than duplicated. It is ASCII-oriented; Unicode-aware normalization is a future hardening item for global catalogs.

## Provenance

Each source observation is retained in `OpportunityRecord.observations`, keyed by source. An observation stores the external ID, destination URL, currency, price, commission, score, and observation timestamp.

This prevents deduplication from discarding network-specific execution data needed later for provider selection and affiliate-link routing.

## Upsert semantics

- First observation creates a record and preserves its runtime UUIDv7 opportunity ID.
- A new source observation merges into the existing record.
- An identical observation is idempotent and reports `changed = false`.
- `first_observed_at_ms` is the minimum observed timestamp.
- `last_observed_at_ms` is the maximum observed timestamp.
- Best source is selected by highest score, then lexicographically smallest source name for deterministic ties.
- Existing category is preserved; a missing category may be filled by a later observation.
- Invalid candidates are rejected before persistence.

## Revisions (Sprint 0 hardening)

Every `OpportunityRecord` carries an `OpportunityRevision` (persisted as the `version` column, 1-based, monotonic):

- creation starts at revision 1;
- every persisted **change** performs a checked increment — overflow is an explicit error, never a silent wrap;
- unchanged re-observations leave the revision untouched;
- `OpportunityUpsertResult.revision` returns the post-write revision;
- `AsyncVersionedOpportunityStore::upsert_if_revision` provides optimistic concurrency: it locks the row, compares the stored version with the caller's expected revision, and fails with `RevisionConflict { identity, expected, actual }` on mismatch (reload and re-plan; never blindly retry);
- decoders reject out-of-domain stored values (negative scores/commissions, version `0`) with `CorruptRecord` instead of wrapping them silently.

All SQL is parameterized; the aggregate update and the observation upsert commit or roll back together in one transaction.

## Persistence boundary

`OpportunityStore` is storage-neutral. P0 provides an in-memory implementation for deterministic unit/integration coverage. PostgreSQL persistence is intentionally a later adapter so the affiliate domain does not become coupled to SQLx.

## Safety invariants

1. Deduplication never silently drops source provenance.
2. Stored identity is immutable for an aggregate.
3. Repeated identical observations are idempotent.
4. Invalid discovery candidates cannot enter the store.
5. Best-source selection is deterministic.
6. Persistence implementation details remain outside discovery logic.

## Next stage

Implement a PostgreSQL adapter with a unique identity constraint and transactional upsert semantics, then connect persistence to discovery ingestion without introducing duplicate network submissions.
