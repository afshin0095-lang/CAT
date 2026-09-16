# CAT Contract Architecture Metrics

**Status:** L2 — Architecture specified

## 1. Purpose

CAT should measure contract quality as an engineering system, not only runtime throughput.

## 2. Structural metrics

- active contracts by domain;
- deprecated contracts still consumed;
- duplicate/similar contract candidates;
- registry validation failures;
- compatibility violations;
- undocumented dependencies.

## 3. Runtime metrics

- invocation success rate;
- retry rate;
- unknown outcome rate;
- reconciliation completion rate;
- authorization denial rate;
- capability latency;
- tool/connector error rate.

## 4. Security metrics

- policy blocks;
- secret-access events;
- blocked egress;
- prompt-injection detections;
- quarantined integrations;
- anomalous economic actions.

## 5. Economic metrics

- cost per capability invocation;
- cost per successful outcome;
- provider cost variance;
- failed-spend exposure;
- expected-vs-realized economics.

## 6. Architecture health

```text
Contract Quality
    + Compatibility
    + Security
    + Reliability
    + Observability
    + Economics
    = Contract Architecture Health
```

These metrics are evidence for improvement; they are not a substitute for policy or architectural judgment.
