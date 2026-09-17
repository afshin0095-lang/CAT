# 23 — Affiliate Network & Merchant Operating Model

**Status:** Architecture/target, with existing affiliate implementation authoritative for current behavior.

## 1. Mission

CAT's affiliate subsystem converts market observations into governed commercial opportunities and measurable outcomes. It is not merely a link generator.

## 2. Commercial lifecycle

```mermaid
flowchart LR
    Discover --> Normalize
    Normalize --> Deduplicate
    Deduplicate --> Evaluate
    Evaluate --> Select
    Select --> Activate
    Activate --> Promote
    Promote --> Click
    Click --> Conversion
    Conversion --> Commission
    Commission --> Reconcile
    Reconcile --> Learn
    Learn --> Evaluate
```

## 3. Core entities

| Entity | Meaning |
|---|---|
| Network | Affiliate intermediary/platform |
| Merchant | Seller/brand |
| Product | Commercial item |
| Offer | Merchant/product commercial proposition |
| Opportunity | CAT's normalized economic opportunity |
| Tracking link | Attributed destination |
| Click | Traffic event |
| Conversion | Commercial outcome |
| Commission | Economic return |
| Campaign | Coordinated promotion |

## 4. Opportunity identity

The same product may appear through multiple networks. CAT should maintain a stable canonical opportunity identity while preserving source-specific observations and provenance.

```text
Canonical Opportunity
├── Network A observation
├── Network B observation
└── Network C observation
```

This allows source comparison without losing provenance.

## 5. Discovery dimensions

Discovery should consider:

- expected commission;
- product price and economics;
- demand;
- competition;
- freshness;
- compliance;
- geography;
- currency;
- network quality;
- merchant quality;
- conversion evidence.

## 6. Network adapter boundary

```mermaid
flowchart TB
    Network API --> Adapter
    Adapter --> Normalized[CAT normalized opportunity]
    Normalized --> Discovery
    Discovery --> Ranking
    Ranking --> Store
```

Network-specific response formats must not leak through the domain boundary.

## 7. Merchant and network health

CAT should continuously evaluate:

| Signal | Example |
|---|---|
| Availability | offer still active |
| Feed quality | malformed records |
| Conversion | historical conversion rate |
| Commission | effective commission |
| Reliability | API success rate |
| Compliance | policy status |
| Attribution | tracking confidence |

## 8. Revalidation

Opportunities are observations, not permanent truth. A revalidation scheduler should prioritize stale or high-value records and refresh them through source adapters.

## 9. Commercial safety

CAT must distinguish:

- discovery from activation;
- recommendation from publication;
- click attribution from conversion confirmation;
- reported commission from reconciled commission;
- expected revenue from realized revenue.

## 10. Optimization

The economic objective should eventually use expected value rather than raw commission percentage:

```text
ExpectedValue
= traffic potential
× conversion probability
× effective commission
× attribution confidence
− acquisition cost
− content/compute cost
− risk adjustment
```

This is a target analytical model, not a claim about current implementation.

## 11. Compliance

Affiliate network terms, merchant rules, advertising requirements, disclosure obligations, geographic restrictions, and platform policies are constraints on action selection. They are not optional post-processing.

## 12. Learning

Every measurable commercial outcome becomes potential evidence for future ranking, provider selection, audience selection, content strategy, and budget allocation—subject to governance and data quality.