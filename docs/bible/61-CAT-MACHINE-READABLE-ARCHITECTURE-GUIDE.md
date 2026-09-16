# CAT Machine-Readable Architecture Guide

**Status:** L2 — Architecture specified

## 1. Canonical mental model

AI systems working on CAT should interpret the following hierarchy as foundational:

```text
SYSTEM
├── CONTROL PLANE
│   ├── Registries
│   ├── Policies
│   ├── Governance
│   └── Human Operations
│
├── DATA / KNOWLEDGE PLANE
│   ├── Canonical State
│   ├── Events
│   ├── Knowledge Graph
│   ├── RAG
│   └── Memory
│
├── INTELLIGENCE PLANE
│   ├── Research
│   ├── Reasoning
│   ├── Decision
│   ├── Planning
│   └── Evaluation
│
├── EXECUTION PLANE
│   ├── Orchestration
│   ├── Workers
│   ├── Capabilities
│   ├── Tools
│   └── Connectors
│
└── EXTERNAL ECONOMIC PLANE
    ├── Affiliate Networks
    ├── Merchants
    ├── Content Channels
    ├── Advertising Platforms
    └── Measurement Systems
```

## 2. Non-negotiable distinctions

```text
Agent       ≠ Model
Capability  ≠ Tool
Tool        ≠ Provider
Provider    ≠ Business Logic
Event       ≠ Current State
Projection  ≠ Canonical Truth
Reasoning   ≠ Decision
Decision    ≠ Plan
Plan        ≠ Execution
Execution   ≠ Outcome
Expected    ≠ Realized
Unknown     ≠ Failed
```

These distinctions exist to prevent architectural drift.

## 3. Universal execution chain

```text
GOAL
 ↓
CONTEXT
 ↓
REASONING
 ↓
DECISION
 ↓
PLAN
 ↓
POLICY
 ↓
CAPABILITY
 ↓
TOOL
 ↓
CONNECTOR
 ↓
PROVIDER
 ↓
EXTERNAL EFFECT
 ↓
EVIDENCE
 ↓
MEASUREMENT
 ↓
EVALUATION
 ↓
LEARNING
 ↓
RE-PLANNING
```

## 4. Universal identity chain

Every material action should be traceable through:

```text
principal_id
→ agent_id
→ workflow_id
→ task_id
→ invocation_id
→ execution_attempt_id
→ capability_id
→ tool_id
→ connector_id
→ provider_id
→ evidence/event IDs
```

Not every operation requires every identifier, but durable or side-effecting operations should retain sufficient correlation to reconstruct causality.

## 5. AI implementation heuristic

When an AI agent receives a feature request:

```text
1. Identify domain.
2. Identify capability.
3. Find existing contract.
4. Find registry entry.
5. Find existing service/SPI.
6. Find tool/connector/provider boundary.
7. Find state/events.
8. Find policy and security boundary.
9. Find tests.
10. Change the smallest correct layer.
11. Update contract/docs/tests.
```

## 6. Forbidden shortcuts

An implementation should not:

- call external providers directly from an agent;
- store secrets in source-controlled configuration;
- use an event as an implicit authorization mechanism;
- retry an uncertain economic side effect blindly;
- duplicate an existing canonical contract;
- treat a model response as authoritative without validation;
- bypass the registry for a material capability;
- silently widen autonomy or spending authority.

## 7. Architecture success condition

A future engineer or AI agent should be able to answer, for any material CAT action:

**Who authorized it? Why was it chosen? Which capability performed it? Which implementation executed it? Which provider was used? What did it cost? What evidence proves the outcome? What happened if the external system was uncertain?**

If the architecture cannot answer these questions, the relevant boundary is incomplete.
