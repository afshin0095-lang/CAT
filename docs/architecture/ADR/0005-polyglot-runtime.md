# ADR-0005: Polyglot Runtime Boundaries

**Status:** Accepted
**Date:** 2026-09-17

## Context

CAT requires different runtime characteristics: Rust for deterministic and performance-sensitive core logic, Python for AI/data workflows, and Go for high-throughput infrastructure services. A single-language mandate would either constrain specialized components or force inappropriate responsibilities into one runtime.

## Decision

CAT adopts a **polyglot runtime architecture** with explicit service and contract boundaries.

### Rust

Primary language for the durable kernel, correctness-critical state machines, contract primitives, high-performance core algorithms, and other components where deterministic behavior and memory safety are central.

### Python

Primary language for AI/ML orchestration, research/data workflows, model-facing services, experimentation, and components whose ecosystem materially benefits from Python.

### Go

Primary language for infrastructure-oriented services, high-concurrency networking, workers, gateways, and operational components where Go provides a strong fit.

The languages MUST communicate through stable contracts rather than shared implementation assumptions.

## Consequences

- each runtime can be optimized for its role;
- contract testing becomes essential;
- deployment and observability conventions must be consistent across languages;
- duplicate domain logic across languages must be avoided;
- serialization formats and compatibility rules require central governance.

## Rejected Alternative

A single-language architecture was rejected because CAT's core and AI/infrastructure workloads have materially different requirements.

## Invariants

- Canonical domain semantics MUST be language-independent.
- A business rule MUST have one authoritative implementation owner.
- Cross-language boundaries MUST use versioned contracts.
- Language choice MUST follow responsibility and operational characteristics, not preference alone.
