# C15 — Versioning Contract

CAT uses explicit contract versions rather than implicit framework behavior.

## Compatibility classes

- **Non-breaking:** add optional response fields, add new resources, improve internal implementation without semantic change.
- **Conditionally breaking:** tighten validation or alter defaults; requires migration review.
- **Breaking:** remove/rename fields, change units/meaning, remove enum values, alter authentication semantics, or invalidate previously accepted requests.

HTTP API major versions are represented in the path (`/api/v1`). Event and command schemas carry independent integer schema versions. Old consumers remain supported for the documented compatibility window.
