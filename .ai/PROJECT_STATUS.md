# CAT Project Status

> Official Development Dashboard
> Project: CAT (Commerce AI Trinity)
> Company: Omni System
> Repository Status: Active Development

---

# Project Information

| Item | Value |
|------|-------|
| Project | CAT (Commerce AI Trinity) |
| Company | Omni System |
| Status | Active Development |
| Version | 0.1.0 |
| Phase | Implementation Phase — Active |
| Repository | GitHub |
| Main Branch | main |

# Current Phase

## Phase B

Core implementation · Rust foundations · Event Bus · Event Store · Knowledge Graph · Memory Core · LLM / AI Core · Reasoning Core · Decision Core · Planning Core · Orchestrator · Retrieval Core · Platform Integration · Platform Adapter Layer

**Status:** Active Development

# Active Task

**Current Task:** Sprint 1 hardening — governed affiliate execution + operator authorization evidence

**Primary branch:** `feat/capability-registry-p0` / PR #61

**Status:** Provider callback gateway/replay, durable operator identity/session authorization, authorization-decision evidence, and the first governed Affiliate Revalidation execution bridge are implemented as executable Rust code. CI remains the authoritative compilation/test validator.

## Completed in this implementation stage

- `ExecutionAuditEvidence` serializable derived evidence view;
- transactional append-only `cat_execution_audit_events`;
- rebuildable latest `cat_execution_audit_read_model`;
- bounded audit query contract with agent/capability/action filters;
- append-only `cat_provider_execution_journal`;
- transactional provider current-state + journal writes;
- deterministic provider journal deduplication and multi-provider identity isolation;
- reconciliation-to-audit persistence bridge;
- PostgreSQL/EventBus/reconciliation/provider-journal/audit integration coverage;
- `OperatorPrincipal` and non-secret `AuthenticationEvidence` contracts;
- explicit operator roles and permissions;
- deny-by-default `OperatorAccessPolicy`;
- `AuthorizedAuditService` application boundary before audit storage;
- explicit elevated permission for read-model rebuild;
- P0 security tests for disabled principals, incomplete authentication evidence, and unauthorized rebuild;
- durable `ProviderCallback` contract and PostgreSQL callback evidence table;
- provider callback correlation by provider + provider execution ID;
- optional callback request-hash verification against durable submission state;
- unmatched callback retention for later reconciliation;
- callback-to-provider-result + journal updates in one PostgreSQL transaction;
- PostgreSQL E2E coverage for correlated, duplicate, and unmatched callbacks;
- bounded `ProviderCallbackReconciliationWorker` for durable out-of-order callback replay;
- `rejected` callback state with durable `correlation_error` for non-retryable correlation conflicts;
- replay path updates callback state, current provider result, and provider journal atomically;
- correlation mutations require expected callback/result row updates and fail closed on partial mutation;
- `ExecutionContext` carries optional project/workspace scope with backward-compatible deserialization;
- execution authorization evidence persists tenant/project scope;
- audit read model/query supports tenant/project filtering;
- durable `cat_operator_identities` + `cat_operator_sessions` tables;
- session-based `AuthorizedAuditService` entry points;
- tenant/project isolation and cross-project rejection covered by PostgreSQL E2E;
- durable session expiry/revocation and scoped identity unit tests;
- `ProviderCallbackIngress` transport boundary;
- provider-specific `ProviderCallbackVerifier` contract;
- deterministic verifier registry and provider routing;
- callback gateway E2E path from raw ingress through verification to durable correlation;
- non-secret provider callback verification evidence persisted with each new callback;
- verifier evidence validation during callback replay;
- provider verifier lifecycle/version registration and controlled rotation;
- durable operator authorization decision ledger;
- `DurableOperatorAuditService` persists decisions before protected audit reads;
- durable workflow registration boundary for provider/domain-owned workflow creation;
- governed Affiliate Revalidation capability/plan/worker/coordinator;
- scope resolver contract for tenant/project/agent identity;
- PostgreSQL E2E for governed Affiliate Revalidation, including tenant/project propagation.

## Validation status

**INCOMPLETE — the latest code-bearing CI run is still pending; no completed green result has been observed yet.**

Local `cargo` execution remains unavailable in the working sandbox because rust-lang.org/crates.io access is TLS-blocked by the egress proxy. GitHub Actions remains the authoritative compilation/test gate.

# Next Task

Implement concrete provider-side Affiliate Revalidation executors/adapters behind the governed worker contract, then add retry-attempt identity rotation without bypassing durable workflow identity.
