# A09 — Core Goal-to-Outcome Sequence

**Status:** TARGET sequence.

## Sequence

```mermaid
sequenceDiagram
    actor User
    participant API as API / Control Plane
    participant Agent as Agent Runtime
    participant Know as Knowledge/RAG
    participant Decision as Decision
    participant Plan as Planner
    participant Orch as Durable Orchestrator
    participant Bus as Event Bus
    participant Worker as Capability Worker
    participant Provider as External Provider
    participant Store as Durable Store
    participant Audit as Observability/Audit

    User->>API: goal + constraints
    API->>Agent: authorized task
    Agent->>Know: retrieve evidence
    Know-->>Agent: evidence + provenance
    Agent->>Decision: proposal
    Decision->>Decision: policy/risk evaluation
    Decision->>Plan: approved intent
    Plan->>Orch: durable workflow
    Orch->>Bus: execution command
    Bus->>Worker: capability request
    Worker->>Provider: external operation
    Provider-->>Worker: result / unknown state
    Worker->>Orch: outcome
    Orch->>Store: commit state + event
    Store-->>Orch: durable commit
    Orch->>Bus: outcome event
    Orch->>Audit: execution evidence
    Bus-->>Agent: outcome signal
    Agent-->>API: result / approval-needed / failure
    API-->>User: evidence + status
```

## Important transitions

| Transition | Durable? | Authorization |
|---|---:|---|
| Goal accepted | yes | authenticated actor |
| Evidence retrieval | normally no business mutation | read policy |
| Decision | yes/auditable when material | decision policy |
| Plan created | yes | workflow policy |
| Execution claimed | yes | capability policy |
| Provider call | external side effect | scoped credential |
| Outcome | yes | execution contract |
| Final response | derived | presentation policy |

## Failure branches

- Validation failure → terminal rejected state.
- Policy failure → denied state, no provider call.
- Worker crash → lease recovery/retry.
- Provider timeout → reconcile before replay.
- Duplicate delivery → idempotent consumer.
- Human approval required → durable waiting state.
