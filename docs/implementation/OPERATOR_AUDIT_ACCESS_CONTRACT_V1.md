# CAT OMNISYSTEM — Operator Audit Access Contract V1

**Status:** Implemented P0  
**Scope:** Control Plane authorization boundary for operator-facing execution audit data

## 1. Purpose

The audit storage layer is intentionally authorization-agnostic. This contract adds the application boundary that decides whether an already-authenticated principal may read or rebuild the execution audit surface.

The authentication system remains outside cat-orchestrator. CAT receives authentication evidence, never raw credentials.

## 2. Boundary

```text
Credential / OIDC / API gateway
        |
        v
Identity + Authentication
        |
        v
OperatorPrincipal + AuthenticationEvidence
        |
        v
OperatorAccessPolicy
        |
        +---- deny ----> request rejected
        |
        v
AuthorizedAuditService
        |
        v
ExecutionAuditStore
        |
        v
PostgreSQL audit read model / append-only audit events
```

A caller must not bypass AuthorizedAuditService for operator-facing audit access.

## 3. Principal contract

OperatorPrincipal contains:

- immutable principal_id;
- explicit OperatorRole;
- enabled/disabled state;
- non-secret AuthenticationEvidence.

AuthenticationEvidence records only:

- authentication method label;
- non-nil session identifier;
- authentication timestamp.

Raw passwords, API keys, refresh tokens and provider credentials are forbidden at this boundary.

## 4. P0 permissions

| Permission | Meaning |
|---|---|
| ReadAudit | permission to query the operator audit surface |
| ReadAuditEvidence | permission to retrieve execution evidence |
| RebuildAuditReadModel | permission to rebuild the derived audit read model |

P0 role grants:

| Role | Read | Evidence | Rebuild |
|---|---:|---:|---:|
| Owner | yes | yes | yes |
| Operator | yes | yes | no |
| Auditor | yes | yes | no |

Disabled principals are always denied.

Unknown runtime identities are not implicitly promoted to operator roles.

## 5. Security invariants

1. Authentication and authorization remain separate.
2. Authorization fails closed for disabled or malformed principals.
3. The audit store does not make authorization decisions.
4. The operator service is the intended application boundary before operator audit access.
5. Authorization policy is versioned as operator-audit-access.v1.
6. No raw credential material is present in operator authorization types.
7. Read-model rebuild requires explicit elevated permission.
8. Tenant/resource scope is intentionally deferred until the durable execution model carries canonical tenancy context; this P0 must not invent cross-tenant ownership data.

## 6. Extension path

The next access-control revision can add:

- tenant/project/resource scope;
- policy version resolution;
- session revocation checks;
- step-up authentication for sensitive evidence;
- time-bounded operator grants;
- explicit service-principal separation;
- audit of the operator authorization decision itself.

Those extensions must preserve the current deny-by-default boundary.