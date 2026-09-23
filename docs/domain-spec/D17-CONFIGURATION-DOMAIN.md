# D17 — Configuration Domain

Configuration is versioned behavior, not an untracked bag of environment variables.

A configuration has key, schema/version, scope, status, author, effective time and provenance. Published versions are immutable. Activation selects a known version; rollback selects another known version.

Secrets are referenced by opaque secret identifiers and resolved only at execution boundaries. Configuration validation occurs before publication.

Environment variables may supply deployment defaults, but business configuration should be represented through versioned configuration contracts so changes are auditable and reproducible.
