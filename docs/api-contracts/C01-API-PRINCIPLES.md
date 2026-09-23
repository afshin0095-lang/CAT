# C01 — API Contract Principles

## Objectives

CAT APIs expose capabilities without leaking internal implementation. Every contract must be deterministic, observable, secure, evolvable, and independently testable.

## Rules

- Resource names are nouns; commands express intent where mutation semantics are non-CRUD.
- Responses contain stable identifiers and explicit status.
- Timestamps use RFC 3339 UTC.
- Monetary values use integer minor units plus ISO currency.
- Enumerations document forward-compatibility behavior.
- Unknown fields may be ignored only where the contract explicitly permits it.
- Unknown enum values must not silently become a valid known state.
- Correlation and causation identifiers propagate through async work.
- Transport errors never replace domain error semantics.

## Contract layers

```text
Transport → API Contract → Application Command/Query → Domain Contract → Persistence
```

Each layer owns its translation and validation boundary.
