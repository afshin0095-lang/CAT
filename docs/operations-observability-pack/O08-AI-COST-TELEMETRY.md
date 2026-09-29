# O08 — AI Model & Cost Telemetry

AI operations are tracked as measurable resource consumption.

Minimum dimensions include model/provider identity, operation type, token or unit counts when available, latency, retry count, cache status, and estimated cost.

```text
Agent/Service
     ↓
Model Gateway
     ↓
Provider Adapter
     ↓
Usage Record
     ├─ input units
     ├─ output units
     ├─ latency
     ├─ provider
     └─ cost estimate
```

Cost estimates are explicitly marked as estimates until reconciled with provider billing data. Financial decisions must not silently treat estimates as settled invoices.