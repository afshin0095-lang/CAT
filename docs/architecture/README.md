# CAT Architecture Documentation

This directory contains implementation-oriented architecture decisions and system-level engineering references.

## Architecture Decision Records

| ADR | Decision | Status |
|---|---|---|
| [0001](ADR/0001-architecture-style.md) | Architecture style and dependency direction | Accepted |
| [0002](ADR/0002-event-driven-core.md) | Event-driven core | Accepted |
| [0003](ADR/0003-durable-execution.md) | Durable execution and recovery | Accepted |
| [0004](ADR/0004-provider-independence.md) | Provider independence | Accepted |
| [0005](ADR/0005-polyglot-runtime.md) | Polyglot runtime boundaries | Accepted |

## Relationship to the CAT Bible

The **CAT Bible** defines the product, domain, agent, capability, security, data, operations, and governance model.

The **Engineering Constitution** defines mandatory engineering invariants.

The **ADR set** records architectural decisions that constrain implementation choices.

Implementation code MUST follow all three layers.

## Next Engineering Layer

The next implementation work should extend the canonical contract primitives in `core/kernel/rust`, followed by contract tests and integration boundaries. New architecture documents should be added only when a concrete architectural decision or implementation constraint requires one.
