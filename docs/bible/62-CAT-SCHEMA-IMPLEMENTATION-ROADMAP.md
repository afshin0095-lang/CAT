# CAT Schema Implementation Roadmap

**Status:** L2 — Architecture specified

## 1. Purpose

This document turns the canonical-schema architecture into an implementation sequence without prematurely coupling the Bible to a particular serialization library or deployment topology.

## 2. Sequence

### Phase A — Contract foundation

- define canonical IDs and lifecycle enums;
- define shared metadata/provenance types;
- define error and outcome taxonomy;
- define side-effect classes;
- define versioning rules.

### Phase B — Core schemas

Implement executable representations for:

1. Agent
2. Capability
3. Tool
4. Connector
5. Provider
6. Invocation
7. Outcome
8. Evidence

### Phase C — Registry runtime

- registry repository interfaces;
- validation service;
- dependency resolver;
- lifecycle transitions;
- cache/read model;
- audit events.

### Phase D — Runtime integration

- bootstrap resolution;
- policy integration;
- capability authorization;
- tool execution boundary;
- connector selection;
- provider selection;
- outcome/evidence persistence.

### Phase E — Developer ergonomics

- schema validation CLI;
- contract-test helpers;
- deterministic fixtures;
- generated language bindings where justified;
- architecture linting.

## 3. Implementation boundary

The first implementation should integrate with existing CAT Rust crates rather than creating an unrelated parallel runtime. New contracts must map explicitly to the existing kernel, eventbus, runtime, knowledge, memory, reasoning, decision, planning, orchestrator, platform, RAG, affiliate and content boundaries.

## 4. Acceptance criteria

The schema foundation is ready for runtime implementation when:

- every core object has a stable ID/version;
- invalid definitions fail closed;
- contracts are serializable deterministically;
- compatibility changes are detectable;
- sensitive values are reference-based;
- material invocations are attributable;
- unknown external outcomes are modeled;
- tests can validate contract behavior independently of a provider.

## 5. Migration discipline

No large rewrite is implied. Existing implementations should be wrapped or migrated incrementally behind the canonical contracts. Each migration must preserve behavior, tests, durability and observability before the old path is retired.
