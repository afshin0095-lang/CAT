# A02 — C4 Container Model

**Level:** C4 Container
**Status:** TARGET model; implementation status must be checked against code.

## Container map

```mermaid
flowchart TB
    UI[Control Plane / UI]
    API[API & Integration Boundary]
    K[Kernel / Contract Core]
    AR[Agent Runtime]
    KN[Knowledge]
    MEM[Memory]
    LLM[LLM Gateway]
    RAG[RAG]
    REA[Reasoning]
    DEC[Decision]
    PLN[Planning]
    ORC[Durable Orchestrator]
    BUS[Event Bus]
    EST[Event Store]
    CAP[Business Capabilities]
    EXT[Provider Adapters]
    PG[(PostgreSQL)]
    CACHE[(Cache)]
    OBJ[(Object Storage)]
    OBS[Observability]

    UI --> API --> K
    K --> AR
    AR --> KN
    AR --> MEM
    AR --> LLM
    AR --> RAG
    AR --> REA
    REA --> DEC --> PLN --> ORC
    ORC --> BUS
    BUS --> CAP
    CAP --> EXT
    EST --> PG
    K --> EST
    KN --> PG
    MEM --> PG
    CAP --> PG
    AR --> CACHE
    KN --> OBJ
    ORC --> OBS
    AR --> OBS
    BUS --> OBS
```

## Container contracts

| Container | Owns | Consumes | Produces |
|---|---|---|---|
| Control Plane | configuration/approval UX | API state | commands/approval decisions |
| API Boundary | transport/auth validation | external requests | validated commands |
| Kernel | stable primitives | commands/contracts | domain-safe operations |
| Agent Runtime | agent execution context | goals, tools, memory | proposals/requests |
| Knowledge | canonical knowledge | documents/evidence | facts/retrieval context |
| Memory | agent/system memory | events/outcomes | context/experience |
| LLM Gateway | model abstraction | prompts/context | model responses |
| Reasoning | evidence synthesis | context/model output | reasoning artifacts |
| Decision | policy-aware choice | evidence/proposals | decisions |
| Planning | decomposition/scheduling | decisions | plans/workflows |
| Orchestrator | durable execution | workflows | commands/results/events |
| Event Bus | asynchronous delivery | domain events | consumer deliveries |
| Event Store | durable event history | events | replay/audit source |
| Capability layer | commerce functions | workflows/events | domain facts/commands |
| Provider adapters | external translation | CAT contracts | provider calls/results |
| Persistence | durable state | facts/commands | committed truth |
| Observability | telemetry | runtime signals | metrics/logs/traces |

## Dependency rule

Dependencies point inward toward stable contracts. Volatile providers, models and UI components remain at the edges. The existing ADR set explicitly governs architecture style, event-driven design, durable execution, provider independence and runtime boundaries. fileciteturn1525file0L2-L2

## Critical distinction

A container can propose an action without possessing authority to execute it. Side effects require the execution boundary and its policy, idempotency and reconciliation rules.
