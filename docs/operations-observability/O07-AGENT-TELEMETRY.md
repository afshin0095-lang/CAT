# O07 — Agent Telemetry

Agent telemetry makes autonomous execution inspectable without exposing prompts or secrets.

```mermaid
sequenceDiagram
participant A as Agent
participant G as Capability Gateway
participant T as Telemetry
participant P as Provider
A->>G: capability request
G->>T: policy decision
G->>P: bounded operation
P-->>G: result
G->>T: latency/outcome/cost
G-->>A: validated result
```

Record run ID, agent identity, capability, step count, latency, token/cost units, outcome class, retry count, and policy decisions. Store sensitive prompt/content payloads separately under explicit retention policy.
