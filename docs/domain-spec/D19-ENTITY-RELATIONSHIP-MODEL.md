# D19 — Entity Relationship Model

```mermaid
erDiagram
    TENANT ||--o{ USER : contains
    TENANT ||--o{ OPPORTUNITY : scopes
    SOURCE ||--o{ OBSERVATION : produces
    OPPORTUNITY ||--o{ OBSERVATION : receives
    TENANT ||--o{ CAMPAIGN : owns
    CAMPAIGN ||--o{ OFFER : contains
    OFFER }o--|| AFFILIATE_NETWORK : uses
    OFFER ||--o{ CLICK : generates
    CLICK ||--o{ CONVERSION : precedes
    CONVERSION ||--o| ATTRIBUTION : receives
    ATTRIBUTION ||--o{ COMMISSION : creates
    COMMISSION ||--o{ PAYOUT : settles
    AGENT ||--o{ AGENT_RUN : executes
    AGENT_RUN ||--o{ AUDIT_RECORD : produces
    TENANT ||--o{ CONFIGURATION : owns
```

Relationships are logical domain relationships. Physical database foreign keys and partitioning are implementation decisions governed by D20 and the technical architecture pack.
