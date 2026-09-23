# D08 — Event Catalog

Events describe facts that happened; they are not commands.

| Event family | Examples |
|---|---|
| Opportunity | OpportunityCreated, OpportunityObserved, OpportunityUpdated |
| Revalidation | RevalidationRequested, RevalidationClaimed, RevalidationCompleted, RevalidationFailed |
| Campaign | CampaignCreated, CampaignPublished, OfferChanged |
| Tracking | ClickRecorded, ConversionRecorded, AttributionCreated |
| Revenue | CommissionRecorded, PayoutRecorded |
| Agent | AgentRunStarted, CapabilityExecuted, AgentRunCompleted |
| Governance | ActionApproved, ActionRejected, PolicyChanged |
| Configuration | ConfigurationPublished |

Every event carries an event id, schema version, occurred-at timestamp, aggregate identity, tenant scope where applicable, producer identity and correlation/causation metadata.

Consumers must tolerate duplicate delivery and preserve ordering only where the aggregate contract requires it.
