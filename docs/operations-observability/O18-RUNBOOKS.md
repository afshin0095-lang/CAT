# O18 — Production Runbook Standard

Every operationally important CAT subsystem gets a runbook.

Runbook structure:

1. Purpose and scope
2. Symptoms
3. Impact assessment
4. Preconditions and safety warnings
5. Diagnostic commands/queries
6. Relevant dashboards and metrics
7. Containment steps
8. Recovery steps
9. Verification
10. Rollback/escalation
11. Evidence to preserve
12. Post-incident follow-up

Runbooks must avoid embedding secrets. Commands should be safe to copy, explicit about environment, and distinguish read-only diagnostics from state-changing operations.
