# ADR-0004: Provider Independence

**Status:** Accepted
**Date:** 2026-09-17

## Context

CAT depends on external AI models, affiliate networks, data sources, advertising platforms, storage systems, and other providers. Any single provider can change pricing, availability, API semantics, limits, or policy.

## Decision

CAT separates business capabilities from concrete providers through stable provider and connector ports.

```text
Agent
  ↓
Capability
  ↓
Domain Service
  ↓
Provider / Connector Port
  ↓
Adapter
  ↓
External System
```

Provider selection is a runtime/control-plane concern based on declared capability, policy, reliability, economics, latency, geography, compliance, and resilience requirements.

Provider-specific response models MUST be normalized into canonical CAT contracts before entering domain logic.

## Consequences

- providers can be replaced without rewriting business logic;
- multiple providers can serve the same capability;
- failover and routing become explicit;
- adapter certification and contract tests become necessary;
- provider metadata becomes part of operational decision-making.

## Rejected Alternative

Embedding provider SDK calls directly in domain services was rejected because it creates vendor lock-in and couples business semantics to external API details.

## Invariants

- No provider is the canonical definition of a CAT capability.
- Provider credentials MUST remain outside domain models and frontend code.
- Provider-specific failures MUST be normalized into CAT failure semantics.
- Provider changes MUST NOT silently change canonical contract meaning.
