# O19 — Operations Runbooks

Runbooks are executable knowledge for recurring incidents.

Every runbook contains:

- symptoms and detection signals;
- scope and severity;
- immediate safety actions;
- diagnostic commands/queries;
- decision tree;
- mitigation;
- recovery;
- verification;
- rollback;
- escalation owner;
- post-incident evidence.

Runbooks must avoid embedding secrets. Commands that can mutate production state require explicit warnings and should be safe to copy only after the operator confirms environment and scope.
