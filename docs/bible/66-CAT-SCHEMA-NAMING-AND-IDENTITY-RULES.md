# CAT Schema Naming and Identity Rules

**Status:** L2 — Architecture specified

## 1. Stable identity

CAT identifiers must remain stable across deployments and implementations. Human-readable names may change; canonical IDs should not.

## 2. Naming patterns

```text
cat.agent.<domain>.<name>.v<major>
cat.capability.<domain>.<name>.v<major>
cat.tool.<domain>.<name>.v<major>
cat.connector.<domain>.<provider>.<service>.v<major>
cat.provider.<domain>.<name>
cat.schema.<object>.v<major>
```

## 3. Version semantics

The `v<major>` portion identifies the semantic compatibility family. Detailed `MAJOR.MINOR.PATCH` metadata identifies the exact contract revision.

## 4. Naming rules

- lowercase identifiers;
- stable domain vocabulary;
- no vendor name in provider-neutral capability IDs;
- no model name in business capability IDs;
- avoid abbreviations unless canonicalized in the glossary;
- one semantic responsibility per identifier;
- never recycle a retired identifier for a different meaning.

## 5. Identity vs display name

```yaml
id: cat.capability.affiliate.discover.v1
name: Affiliate Opportunity Discovery
```

The display name may be localized or refined without changing identity.

## 6. Provider rule

Provider names belong in provider and connector identifiers, not in provider-neutral capability contracts. This allows multiple implementations to satisfy one CAT capability.

## 7. AI rule

Before creating an identifier, search the repository and Bible glossary for existing terminology. If an equivalent concept exists, reuse it rather than creating a synonym.
