# A15 — Agent Evaluation

Agents are evaluated on task outcomes, not prose quality alone.

## Dimensions

- correctness against authoritative facts;
- schema validity;
- policy compliance;
- tool-call efficiency;
- cost and latency;
- reproducibility where deterministic behavior is required;
- false-positive/false-negative rates;
- recovery behavior;
- human approval rate for escalated work.

Evaluation datasets are versioned. Production outcomes can feed evaluation only after privacy and data-quality controls. Model/provider changes require regression evaluation before promotion.
