# O06 — Agent Telemetry

Every agent execution emits bounded operational telemetry.

```mermaid
sequenceDiagram
  participant O as Orchestrator
  participant A as Agent
  participant M as Model
  participant T as Tool
  participant Q as Telemetry
  O->>A: execute(command)
  A->>Q: started
  A->>M: inference
  M-->>A: result
  A->>T: capability call
  T-->>A: result
  A->>Q: step metrics
  A-->>O: outcome
  A->>Q: completed
```

Minimum fields include agent type/version, execution ID, command ID, duration, step count, model/provider identifiers, tool count, outcome, retry count, and bounded cost information. Prompt and response bodies are excluded unless an explicit controlled diagnostic policy permits their capture.
