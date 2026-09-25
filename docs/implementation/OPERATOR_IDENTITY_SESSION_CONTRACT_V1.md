# CAT OMNISYSTEM — Operator Identity & Session Contract V1

**Status:** Implemented P0
**Scope:** Durable operator identity, session validity, tenant/project scope, and audit access

## Identity

`cat_operator_identities` is the durable authority for operator role and scope.

Each identity contains:

- stable principal ID;
- external subject reference;
- explicit operator role;
- required tenant scope;
- optional project/workspace scope;
- resource scope expressions;
- enabled/disabled state.

Identity records never contain passwords, access tokens, refresh tokens, or provider credentials.

## Session

`cat_operator_sessions` is the durable session state.

A session is accepted only when:

1. session and principal identifiers are non-nil;
2. authentication method is present;
3. issuance is not in the future;
4. expiry is strictly after the current time;
5. the session has not been revoked.

Session revocation is durable and immediately affects subsequent control-plane authorization.

## Scope

Operator audit reads are constrained by:

`tenant_id → project_id (optional) → requested audit resource surface`

The access service injects the durable tenant/project scope into audit queries and rejects caller-supplied scopes that cross the operator's assigned boundary.

Scoped operators cannot rebuild the global audit read model.

## Integration

`AuthorizedAuditService` now exposes session-based entry points:

- `query_for_session(session_id, now_ms, query)`;
- `load_latest_for_session(session_id, now_ms, execution_id)`.

These resolve the durable identity/session record first and only then invoke the authorization boundary.

## Backward compatibility

Execution context now carries an optional project/workspace identifier and defaults it to absent during deserialization so older serialized contexts remain readable.

New authorization records persist tenant/project scope. Historical records created before this scope existed are not reverse-engineered from unrelated data; missing canonical scope must fail closed where scope-sensitive authorization is required.

## Security invariants

- authentication is separate from authorization;
- credentials stay outside CAT domain and orchestrator types;
- disabled identities are rejected;
- expired sessions are rejected;
- revoked sessions are rejected;
- tenant/project scope is derived from durable identity, not request parameters;
- storage remains authorization-agnostic;
- global audit rebuild requires an explicitly unscoped administrative principal.