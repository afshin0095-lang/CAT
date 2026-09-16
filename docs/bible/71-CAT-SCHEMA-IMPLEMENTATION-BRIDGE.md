# CAT Schema Implementation Bridge

**Status:** L2 — Architecture specified

## 1. Purpose

This bridge defines the handoff from the documentation contract layer to the existing Rust implementation. It is deliberately incremental: no parallel runtime is introduced.

## 2. Existing implementation anchors

The first mapping pass should inspect and reuse the existing packages for:

| Contract concern | Existing CAT anchor |
|---|---|
| Core identity/invariants | `kernel` |
| Events | `eventbus` |
| Durable event persistence | `eventstore-postgres` |
| Runtime | `runtime` |
| Knowledge | `knowledge` |
| Memory | `memory` |
| Models | `llm` |
| Reasoning | `reasoning` |
| Decision | `decision` |
| Planning | `planning` |
| Durable execution | `orchestrator` |
| Platform services | `platform` |
| Retrieval | `rag` |
| Affiliate | `affiliate` |
| Content | `content` |

The exact mapping must be verified against source code before implementation changes are made.

## 3. Implementation order

```text
Shared contract primitives
        ↓
Agent / Capability types
        ↓
Invocation + Outcome
        ↓
Registry interfaces
        ↓
Policy integration
        ↓
Tool / Connector SPI
        ↓
Provider selection
        ↓
Persistence + events
        ↓
Runtime integration
```

## 4. Architectural constraint

Do not introduce a second identity model, second event envelope, second orchestration abstraction, or provider-specific business layer when an existing CAT abstraction already serves that purpose.

## 5. Verification

Each bridge step must compile and test against the existing workspace. Documentation maturity should advance only after executable evidence exists.

```text
DOCUMENTED → CONTRACTED → IMPLEMENTED → INTEGRATED → VERIFIED
```

## 6. AI coding rule

Before implementing any contract, inspect the actual current branch and locate the nearest existing abstraction. Prefer extension over duplication when the semantics match. If semantics differ, record the difference explicitly rather than forcing unrelated concepts into one type.
