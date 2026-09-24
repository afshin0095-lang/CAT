# O08 — Cost Telemetry

CAT treats AI/provider consumption as a first-class operational signal.

```text
Agent Run
  ├─ model calls
  ├─ input/output units
  ├─ provider charges
  ├─ tool execution
  └─ infrastructure allocation
          ↓
      Cost Ledger
          ↓
 Budget / anomaly / profitability analysis
```

Cost records should support attribution by tenant, agent, workflow, provider, model, and operation where cardinality and privacy rules permit. Monetary values must use exact decimal/integer representations rather than floating-point arithmetic for financial accounting.
