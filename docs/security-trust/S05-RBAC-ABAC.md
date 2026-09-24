# S05 — RBAC + ABAC

CAT combines role-based permissions with contextual attributes.

```text
Allow(principal, action, resource, context)
       │
       ├── role policy
       ├── capability policy
       ├── tenant/resource scope
       ├── risk/policy state
       └── runtime constraints
```

RBAC supplies stable coarse-grained roles. ABAC supplies resource, tenant, environment, ownership, sensitivity, and contextual restrictions.

Rules are deny-by-default. Authorization must be evaluated server-side and cannot be delegated to a UI, prompt, or client-provided role claim without verification.
