# D16 — Identity Domain

Identity defines who or what may act and the scope in which it may act.

Hierarchy: `User → Tenant → Project/Workspace → Resource`. Service principals and agents are separate identities with explicit bindings.

Authorization evaluates principal, action, resource, tenant scope, policy version and context. Authentication proves identity; authorization decides permission.

Cross-tenant access is denied by default. Administrative access is explicit, auditable and time-bounded where practical. Credentials are never stored in domain entities; references resolve through a secret-management boundary.
