# D07 — Query Catalog

Queries are side-effect free and optimized independently from write models.

Core queries: `GetOpportunity`, `ListOpportunities`, `FindDueRevalidations`, `GetSourceHealth`, `GetCampaign`, `GetOffer`, `GetAttribution`, `GetRevenueSummary`, `GetAgent`, `GetAgentRun`, `GetAuditTrail`, and `GetConfigurationVersion`.

## Pagination
Limits are bounded and checked. Cursor pagination is the preferred long-term contract; offset pagination may exist as a compatibility adapter.

## Consistency
Each query documents whether it requires strongly consistent authoritative data or may use an eventually consistent projection.

## Security
Tenant scope and authorization filters are mandatory query inputs, never optional UI behavior.
