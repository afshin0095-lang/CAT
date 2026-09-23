# D15 — Agent Domain

An Agent is a governed execution identity, not an unrestricted autonomous process.

Core concepts: agent definition, version, capabilities, policy bindings, budget, trust level, run, step, decision, approval and evidence.

```mermaid
flowchart TD
A[Agent] --> P[Policy Binding]
A --> C[Capability Set]
A --> R[Agent Run]
R --> S[Execution Steps]
S --> G[Governance Gate]
G --> X[Capability Gateway]
X --> E[External Effect]
E --> EV[Evidence]
```

Every run has correlation identity, authorization context, bounded resources and an explicit terminal result. High-risk actions may require human approval. Agents cannot directly access secret material or bypass capability gateways.
