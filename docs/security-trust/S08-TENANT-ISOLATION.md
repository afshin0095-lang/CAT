# S08 — Tenant Isolation

Tenant boundaries are enforced in the application and persistence layers.

```text
Request Context
   ↓ tenant_id
Authorization
   ↓ scoped principal
Query/Command
   ↓ mandatory tenant predicate
Repository
   ↓ constrained data access
Database
```

Tenant identifiers are derived from trusted authenticated context rather than ordinary request fields. Cross-tenant reads and writes are denied by default. Background jobs and agents carry tenant context explicitly; a missing tenant context is an error for tenant-scoped operations.

Administrative cross-tenant operations require an explicit elevated policy and audit record.
