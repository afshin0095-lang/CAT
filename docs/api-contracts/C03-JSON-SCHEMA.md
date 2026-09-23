# C03 — JSON Schema Conventions

CAT JSON contracts use explicit schemas with stable property names, documented required fields, semantic constraints, and version metadata where needed.

## Canonical scalar rules

| Type | Rule |
|---|---|
| ID | opaque string; clients must not infer structure |
| timestamp | RFC 3339 UTC |
| money | `{amount_minor, currency}` |
| percentage | integer basis points where precision matters |
| duration | integer milliseconds |
| boolean | JSON boolean, never string |
| enum | documented string values |

## Compatibility

Adding optional fields is normally backward-compatible. Removing or renaming fields, changing meaning, narrowing accepted values, or changing units is breaking and requires a versioning decision.

Schemas must distinguish `null` from omission whenever that distinction affects semantics.
