# CAT Engineering Constitution

**Status:** Normative
**Version:** 1.0
**Scope:** Entire CAT OMNISYSTEM repository

## 1. Purpose

This document is the engineering constitution of CAT OMNISYSTEM. It defines the invariants that implementation, review, testing, operations, and future agents MUST preserve.

The CAT Bible explains what CAT is and why its domains exist. This constitution defines the rules that code MUST obey.

## 2. Core Principles

1. **Contracts before integration.** Cross-boundary behavior MUST be represented by explicit versioned contracts.
2. **Domain ownership is explicit.** Every piece of canonical state MUST have one owning domain.
3. **Durability before convenience.** Business-critical state MUST survive process, node, and provider failure.
4. **Evidence before assumption.** External outcomes MUST be represented with provenance and uncertainty where applicable.
5. **Least privilege by default.** Agents, tools, providers, and operators receive only the permissions required for their declared scope.
6. **Provider independence.** Business logic MUST NOT depend directly on a single model, vendor, connector, or external API.
7. **Determinism where possible.** Planning, policy evaluation, state transitions, identifiers, retries, and migrations SHOULD be reproducible from durable inputs.
8. **Observable execution.** Important work MUST be traceable from intent through decision, execution attempt, outcome, evidence, and resulting events.
9. **Human authority remains explicit.** Human approval boundaries MUST be represented as policy, not hidden in UI behavior.
10. **Implementation follows the contract.** Code MUST implement an accepted contract; code MUST NOT silently redefine the domain contract.

## 3. Architecture Invariants

- The dependency direction MUST point toward stable domain abstractions.
- The Rust kernel MUST remain free of infrastructure-specific business rules.
- Domain services MUST NOT import provider-specific implementations directly.
- Infrastructure adapters MUST implement stable interfaces rather than leaking their types inward.
- Cross-domain communication SHOULD use commands, queries, events, and typed artifacts rather than shared mutable state.
- A module MUST have one clear responsibility and one clear owner.
- Circular dependencies between domain modules are prohibited.

## 4. Agent Invariants

Every production agent MUST have:

- a stable identity;
- an explicit mission and non-goals;
- versioned capabilities;
- input and output contracts;
- policy scope;
- memory and knowledge scope;
- declared side-effect class;
- execution and retry semantics;
- observability metadata;
- evaluation criteria;
- resource and economic limits;
- lifecycle state;
- ownership and provenance.

An agent MUST NOT acquire undeclared capabilities merely because a tool is technically reachable.

**Tool access is not equivalent to authority.** Capability authorization MUST precede tool execution.

## 5. Contract Invariants

Contracts MUST be:

- versioned;
- machine-readable where practical;
- structurally validated;
- semantically validated;
- compatibility-checked before release;
- traceable to their implementation;
- covered by contract tests.

Breaking changes MUST use an explicit migration/versioning strategy. Silent reinterpretation of an existing contract is prohibited.

## 6. State, Events, and Persistence

CAT distinguishes:

- canonical durable state;
- immutable domain events;
- projections/read models;
- caches;
- search/vector indexes;
- telemetry;
- external observations.

A projection MUST NOT be treated as the canonical source of truth when durable canonical state or events exist.

Business-critical state transitions MUST be transactionally durable or recoverable through an explicitly documented protocol.

At-least-once delivery MUST be assumed at event boundaries unless a stronger guarantee is explicitly implemented and verified.

Consumers MUST be idempotent.

## 7. Execution and Reliability

Every durable execution MUST have a distinguishable execution identity and attempt identity.

Retries MUST be bounded by policy and MUST respect idempotency. Unknown external outcomes MUST NOT be blindly retried when doing so could duplicate an economic or irreversible side effect.

Recovery MUST reconstruct execution from durable state rather than process memory.

Cancellation, timeout, backpressure, fencing, and quarantine behavior MUST be explicit for long-running workflows.

## 8. Security and Trust

CAT MUST enforce defense in depth across:

- identity;
- authorization;
- capability boundaries;
- credential isolation;
- input validation;
- prompt-injection defenses;
- audit logging;
- economic authorization;
- supply-chain integrity;
- incident response.

Secrets MUST NOT be committed to source control or exposed to frontend code.

Untrusted external content MUST be treated as data, not executable policy.

## 9. Economic Safety

Because CAT can operate revenue-generating workflows, financial side effects require explicit authorization.

Every economically meaningful action SHOULD carry:

- budget context;
- expected value or business objective;
- risk classification;
- authorization context;
- attribution metadata;
- correlation/causation lineage.

Agents MUST have bounded resource consumption. Runaway loops, unbounded retries, uncontrolled provider spending, and recursive delegation MUST be prevented by policy.

## 10. Observability

Production execution MUST expose enough information to answer:

1. What happened?
2. Why did it happen?
3. Which policy allowed it?
4. Which agent/capability/provider executed it?
5. What did it cost?
6. What evidence supports the outcome?
7. What should happen next?

Sensitive data MUST be redacted according to policy before entering logs, traces, metrics, or audit records.

## 11. Testing Constitution

A feature is not complete because its code compiles.

Production changes SHOULD include the appropriate combination of:

- unit tests;
- integration tests;
- contract tests;
- persistence/recovery tests;
- event/idempotency tests;
- security tests;
- failure-injection tests;
- end-to-end tests where applicable.

Tests MUST verify invariants, not merely implementation details.

## 12. Compatibility and Migration

Backward compatibility MUST be considered for APIs, events, schemas, persisted data, and provider adapters.

Database migrations MUST be:

- ordered;
- reversible where practical;
- observable;
- tested against realistic data;
- safe for rolling deployment.

Destructive migrations require an explicit recovery plan.

## 13. Forbidden Patterns

The following patterns are prohibited unless an ADR explicitly establishes a narrowly scoped exception:

- provider-specific logic inside domain entities;
- direct database access from presentation/UI code;
- unbounded retry loops;
- hidden mutable global state;
- secrets in source control;
- implicit agent authority;
- treating model output as trusted policy;
- blind retry of an unknown irreversible external outcome;
- breaking contract changes without versioning;
- business logic encoded only in prompts;
- telemetry without correlation/causation context for durable workflows;
- claiming operational readiness without verification.

## 14. Definition of Done

A production change is complete only when:

- its contract and ownership are clear;
- implementation follows architecture boundaries;
- failure and recovery behavior are defined;
- security boundaries are verified;
- tests cover critical invariants;
- observability is present;
- migration/compatibility impact is understood;
- documentation is updated when behavior or contracts change;
- CI verification is available and actual status is known.

## 15. Change Authority

When a proposed implementation conflicts with this constitution, the implementation MUST NOT silently override it.

The correct sequence is:

`Identify conflict → document rationale → create/update ADR → obtain explicit approval → implement → test → record outcome.`

## 16. Canonical Hierarchy

When documents appear to conflict, use this order:

1. Security and legal constraints
2. This Engineering Constitution
3. Accepted ADRs
4. Canonical contracts and schemas
5. Domain specifications
6. Implementation details
7. Informal notes

Lower-level material MUST NOT silently contradict a higher-level rule.
