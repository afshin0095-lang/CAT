# CAT Memory — Verification Surface

The memory core is a governed state boundary, not a generic key-value cache. Every persisted memory object carries provenance, consent, classification, lifecycle, and retention metadata.

## Invariants

- canonical memory identity is immutable;
- provenance is mandatory;
- required consent is enforced before admission;
- lifecycle transitions are explicit and terminal states cannot reactivate;
- retention expiry cannot silently leave active memory alive;
- legal hold prevents an object from being treated as expired;
- query operations are read-only projections over stored objects;
- classification filters are monotonic and never downgrade visibility requirements.

## Verification

Run:

```text
cargo test --manifest-path core/memory/rust/Cargo.toml
```

The contract suite covers lifecycle transition safety, terminal-state behavior, namespace/kind/state filtering, and classification floors. The next persistence adapters should reuse these contracts rather than redefine memory semantics.
