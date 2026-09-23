# C05 — Authorization Contract

Authorization evaluates a principal against policy, resource, action, tenant context, and relevant conditions.

## Policy decision

```json
{
  "allow": true,
  "principal_id": "usr_123",
  "action": "opportunity.read",
  "resource": "opportunity:opp_123",
  "policy_version": "2026-01"
}
```

The decision must be deterministic for a fixed policy snapshot and context. Denials fail closed. Application code must not bypass the authorization boundary by calling repositories directly from transport handlers.

## Scope hierarchy

```text
Global → Tenant → Workspace → Resource → Operation
```

A narrower scope may restrict a broader grant but must not silently expand it.
