# C09 — Provider Contract

Providers are replaceable external integrations. CAT depends on capabilities, not provider-specific SDK semantics.

```text
Provider Adapter
  ├── credentials/config
  ├── capability discovery
  ├── request translation
  ├── response normalization
  ├── retry classification
  ├── rate-limit classification
  └── error translation
```

Provider-specific errors must map into the CAT error taxonomy. Raw provider payloads may be retained only under explicit retention and privacy rules. Provider adapters must not mutate domain state directly outside their assigned application boundary.
