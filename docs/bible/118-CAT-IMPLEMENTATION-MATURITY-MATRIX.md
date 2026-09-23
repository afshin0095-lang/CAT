# CAT OMNISYSTEM — Implementation Maturity Matrix

**Status:** Canonical reporting model
**Important:** maturity in this document is an assessment framework, not proof that every row is currently at the stated level.

## 1. Levels

| Level | Meaning |
|---:|---|
| L0 | concept / idea |
| L1 | documented or researched |
| L2 | architecture specified |
| L3 | contract specified |
| L4 | implementation exists |
| L5 | automated tests/integration exist |
| L6 | production observability exists |
| L7 | continuously optimized from evidence |

## 2. Domain assessment framework

| Domain | Primary evidence to inspect | Required gate for L5+ |
|---|---|---|
| Kernel | source + tests | deterministic contract tests |
| EventBus | source + delivery tests | retry/idempotency/integration tests |
| EventStore | migrations + adapter tests | Postgres integration |
| Runtime | lifecycle + task tests | recovery/failure tests |
| Knowledge | graph/query/provenance tests | persistence + rebuild tests |
| Memory | policy/store/query tests | retention/recovery tests |
| LLM | provider contracts/tests | adapter/evaluation matrix |
| Reasoning | engine/trace tests | evaluation fixtures |
| Decision | policy/approval/trace tests | approval and replay contracts |
| Planning | plan/schedule tests | deterministic plan fixtures |
| Orchestrator | execution/attempt/reconciliation tests | durable workflow integration |
| Platform | adapters/health tests | provider contract suite |
| RAG | retrieval tests | source/provenance evaluation |
| Affiliate | discovery/lifecycle/revalidation tests | domain + persistence integration |
| Content | publication/provenance tests | end-to-end publication contract |
| Advertising | campaign/action contracts | financial/risk integration |
| Treasury | ledger/settlement contracts | reconciliation and audit tests |
| Control Plane | API/UI contracts | authorization + operator acceptance |

## 3. Evidence rule

A file existing in a directory is weak evidence. Strong evidence is:

`implemented source + automated test + integration evidence + observable runtime behavior`.

## 4. Reporting template

Every milestone report should contain:

- exact Git commit;
- files changed;
- implementation maturity before/after;
- tests executed;
- tests blocked and why;
- unresolved risks;
- contracts changed;
- next milestone.

## 5. AI implementation gate

Before claiming a feature is complete, the implementing agent must answer:

1. Where is the source of truth?
2. What contract guarantees behavior?
3. What test proves it?
4. What happens after a crash?
5. What happens when the provider lies or disappears?
6. What evidence is retained?
7. Who can authorize the action?
8. How is the behavior observed?
9. How can the feature be disabled or rolled back?
10. Which documentation becomes stale if the implementation changes?

## 6. Maturity is domain-specific

CAT should not use one global percentage as a substitute for engineering evidence. A mature affiliate lifecycle module can coexist with an unimplemented advertising subsystem. Reports should therefore use domain-level maturity plus a clearly defined overall roadmap state.
