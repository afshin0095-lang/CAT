# D06 — Command Catalog

Commands express intent and may cause state change.

| Command | Owner | Expected effect |
|---|---|---|
| CreateOpportunity | Opportunity | create authoritative opportunity |
| RecordObservation | Discovery | append idempotent observation |
| RequestRevalidation | Revalidation | enqueue durable work |
| ClaimRevalidation | Revalidation | exclusively claim work |
| CompleteRevalidation | Revalidation | persist result and facts |
| PublishCampaign | Campaign | publish validated commercial configuration |
| RecordClick | Tracking | record click fact |
| RecordConversion | Tracking | record conversion fact |
| AttributeConversion | Attribution | create attributable decision |
| RecordCommission | Revenue | record monetary obligation |
| StartAgentRun | Agent | authorize and create execution |
| ExecuteCapability | Agent | request controlled side effect |
| ApproveAction | Governance | release gated action |
| UpdateConfiguration | Configuration | create a new version |

Commands must be authenticated, authorized, validated and idempotency-aware before side effects.
