# ADR-0001: Architecture Style and Dependency Direction

**Status:** Accepted
**Date:** 2026-09-17

## Context

CAT must evolve from a personal deployment to a large multi-agent platform without repeatedly rewriting its domain core. The system also combines Rust, Python, Go, PostgreSQL, event-driven workflows, external providers, and future infrastructure changes.

## Decision

CAT adopts a **Clean/Hexagonal architecture with explicit domain boundaries**, implemented as layered modules and stable ports/adapters.

The dependency direction is:

```text
Presentation / Control Plane
          ↓
Application / Orchestration
          ↓
Domain Services + Domain Models
          ↓
Stable Ports / Contracts
          ↓
Infrastructure Adapters / Providers
```

The core kernel MUST remain independent from concrete infrastructure. External systems enter CAT through adapters implementing stable interfaces.

## Consequences

### Positive

- provider and infrastructure replacement is localized;
- domain behavior remains testable without external services;
- scaling from one process to distributed workers is less disruptive;
- contracts become explicit integration boundaries.

### Trade-offs

- more interfaces and explicit boundaries;
- additional mapping between external and canonical models;
- architectural discipline is required to prevent shortcuts.

## Rejected Alternative

A monolithic application with direct database/provider access from business modules was rejected because it creates coupling and makes long-term replacement and scaling expensive.

## Invariants

- Domain code MUST NOT depend on concrete providers.
- Infrastructure adapters MUST NOT redefine domain semantics.
- Circular dependencies are prohibited.
- Exceptions require a documented ADR amendment.
