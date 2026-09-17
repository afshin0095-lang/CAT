# CAT Contract-to-Code Generation Architecture

**Status:** L2 — Architecture specified

## 1. Objective

CAT's canonical schemas should eventually become executable contracts without creating parallel sources of truth.

```text
Canonical Schema
      ↓
Contract Validation
      ↓
Language Representation
 ┌────┼─────────┐
 ▼    ▼         ▼
Rust  TypeScript  Python
      ↓
Adapters / API / Runtime
```

## 2. Source-of-truth rule

The semantic contract is canonical. Language-specific types are projections of that contract and may add implementation-specific concerns only when those concerns do not alter the public semantic meaning.

## 3. Rust priority

CAT's durable core uses Rust as the primary executable representation for critical domain contracts, state machines, event envelopes and reliability-sensitive infrastructure. Python and TypeScript representations serve their respective service/UI boundaries.

## 4. Generation safeguards

Generated code must never silently overwrite handwritten behavior. Generated artifacts should be clearly identifiable and regeneration should be deterministic.

Recommended pipeline:

```text
Schema Change
   ↓
Format + Parse
   ↓
Schema Validation
   ↓
Compatibility Diff
   ↓
Generate Types
   ↓
Compile
   ↓
Contract Tests
   ↓
Integration Tests
```

## 5. Contract-test principle

For every generated representation, tests should verify that serialization and deserialization preserve the canonical contract, including optional fields, enum semantics, versions and failure states.

## 6. API/event relationship

The same canonical semantic definitions should govern API request/response contracts and durable event payloads where their semantics overlap. API transport concerns must not leak into domain contracts.

## 7. Migration rule

A schema migration must identify:

- affected language types;
- API endpoints;
- event producers/consumers;
- database representations;
- agent contracts;
- tool/connectors;
- dashboards and evaluations;
- backward-compatibility window.

## 8. AI coding rule

An AI agent must search for an existing canonical schema and generated type before defining a new DTO, interface, enum or duplicate contract. Duplication is considered architectural debt unless explicitly justified.
