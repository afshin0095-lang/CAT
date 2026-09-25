# CAT OMNISYSTEM — Operator Authorization Decision Contract V1

**Status:** Implemented P0
**Scope:** Durable evidence of operator authorization decisions

## Purpose

Authentication proves that a principal presented a valid session. Authorization determines whether that principal may perform a specific Control Plane operation. CAT now persists the authorization decision separately from the audit data being accessed.

## Evidence

`cat_operator_authorization_decisions` stores:

- decision ID and sequence;
- principal and session IDs;
- explicit role and permission;
- allowed/denied outcome;
- policy version;
- tenant/project scope when present;
- authentication method label;
- policy reasons;
- recorded timestamp.

No passwords, access tokens, refresh tokens, webhook signatures, or provider secrets are stored.

## Ordering

For a session-based audit operation:

```text
Resolve session
    ↓
Resolve durable identity + scope
    ↓
Evaluate OperatorAccessPolicy
    ↓
Persist authorization decision evidence
    ↓
Only when Allowed: execute audit operation
```

A denied decision is still durable evidence.

## Security invariants

1. Authorization evidence is append-only.
2. The authorization record is created before the protected audit operation.
3. A denied decision never proceeds to the audit store.
4. The evidence includes the policy version that produced the decision.
5. Tenant/project scope is captured when available.
6. Authorization evidence does not itself grant future permission.
7. The audit data being protected remains separate from the authorization decision ledger.

## Future extension

The next revision can add resource identifiers, request fingerprints, policy input hashes, step-up authentication evidence, and a durable relationship between authorization decisions and audit/read operations.