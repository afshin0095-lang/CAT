# S19 — Security Testing Strategy

Security tests emphasize denied paths and boundary failures.

## Required classes

- authentication bypass tests;
- authorization matrix tests;
- tenant-isolation tests;
- malformed input and oversized payload tests;
- replay/signature tests for webhooks;
- idempotency and duplicate-delivery tests;
- prompt-injection and indirect-injection tests;
- tool/capability escalation tests;
- secret-redaction tests;
- dependency/container scanning;
- race/concurrency tests around security-sensitive state;
- audit completeness tests.

A green happy-path suite is insufficient. Every important permission must have at least one explicit negative test.
