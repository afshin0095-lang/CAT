# CAT Contract Language Guide

**Status:** L2 — Architecture specified

## 1. Normative words

CAT documentation uses:

- **MUST** — mandatory invariant.
- **MUST NOT** — prohibited behavior.
- **SHOULD** — default requirement; deviation requires justification.
- **SHOULD NOT** — default prohibition; deviation requires justification.
- **MAY** — permitted behavior.

## 2. State language

Use canonical lifecycle terms exactly as defined in the glossary. Do not invent synonyms for operational states when a canonical term exists.

## 3. Outcome language

Use `SUCCESS`, `RETRYABLE_FAILURE`, `UNKNOWN`, `DENIED`, `CANCELLED`, and other registered outcome classes consistently. Avoid using the generic word “error” when the exact semantic state is known.

## 4. Evidence language

Distinguish:

```text
Observed fact
Estimated value
Model inference
Policy decision
Human decision
External claim
```

A generated statement must not be represented as an observed fact without supporting evidence.

## 5. Economic language

Always distinguish:

```text
expected_cost  ≠ realized_cost
expected_revenue ≠ realized_revenue
estimated_commission ≠ settled_commission
```

## 6. AI interpretation rule

When documentation is ambiguous, AI agents should prefer the narrowest interpretation that preserves security, durability and existing contracts, then request/record an explicit architectural decision if the ambiguity affects implementation.
