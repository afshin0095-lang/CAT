# D01 — Entity Catalog

## Purpose
Canonical nouns used by CAT. Each entity has one owning domain and a stable identity.

| Entity | Owner | Identity | Nature |
|---|---|---|---|
| Opportunity | Affiliate Intelligence | opportunity_id | business fact |
| Source | Discovery | source_id | integration identity |
| Observation | Discovery | observation_id | immutable fact |
| AffiliateNetwork | Affiliate Network | network_id | provider identity |
| Merchant | Campaign | merchant_id | business entity |
| Campaign | Campaign | campaign_id | business entity |
| Offer | Campaign | offer_id | commercial entity |
| Click | Tracking | click_id | immutable fact |
| Conversion | Tracking | conversion_id | immutable fact |
| Attribution | Tracking | attribution_id | derived fact with provenance |
| Commission | Revenue | commission_id | monetary fact |
| Payout | Revenue | payout_id | settlement fact |
| Agent | Agent | agent_id | runtime identity |
| AgentRun | Agent | run_id | execution record |
| Capability | Agent | capability_id | controlled operation |
| User | Identity | user_id | principal |
| Tenant | Identity | tenant_id | isolation boundary |
| Configuration | Configuration | config_id + version | versioned fact |
| AuditRecord | Audit | audit_id | evidence |

## Identity rule
Identifiers are opaque externally and must not encode mutable business attributes. Human-readable keys may exist as unique secondary keys.

## Ownership rule
Cross-domain references point to identifiers; domain modules do not reach into another domain's persistence tables as an implementation shortcut.
