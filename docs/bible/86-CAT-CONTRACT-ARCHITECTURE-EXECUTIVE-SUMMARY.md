# CAT Contract Architecture — Executive Summary

**Status:** L2 — Documentation phase complete

CAT now has a documented canonical contract layer connecting AI agents to real-world systems through explicit, governed boundaries.

```text
AGENT
 ↓
CAPABILITY
 ↓
DOMAIN SERVICE
 ↓
TOOL
 ↓
CONNECTOR
 ↓
PROVIDER
 ↓
EXTERNAL SYSTEM
 ↓
EVIDENCE
 ↓
MEASUREMENT
```

The layer defines identity, versioning, lifecycle, authorization, side effects, idempotency, retries, unknown outcomes, security, economics, observability, compatibility and migration.

The next phase should be executable Rust implementation and verification. Documentation must now support implementation rather than replace it.
