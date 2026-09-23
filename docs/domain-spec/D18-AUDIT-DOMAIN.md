# D18 — Audit Domain

Audit evidence explains what happened, who/what initiated it, under which policy, and with what outcome.

An audit record contains event/effect identity, actor identity, tenant scope, action, target reference, policy version, correlation/causation ids, timestamp, result classification and safe evidence references.

Audit records are append-oriented. Sensitive payloads are redacted or referenced rather than copied. Operational logs and audit evidence have different retention and access policies.

Auditability is required for external side effects, policy changes, authorization decisions, monetary corrections, configuration publication and human approvals.
