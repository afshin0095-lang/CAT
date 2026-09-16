# CAT Implementation-First Rule

**Status:** L2 — Architecture governance rule

## Rule

Once a subsystem has sufficient contracts, registries, security boundaries, lifecycle definitions, examples and implementation mapping to begin coding, CAT should prefer executable implementation and verification over adding speculative documentation.

## Applies to

- Agent contracts;
- Capability contracts;
- Tool boundaries;
- Connector SPI;
- Provider registry;
- Schema validation;
- Invocation/outcome primitives;
- Registry runtime.

## Exception

Additional architecture documentation is justified only when implementation reveals a real ambiguity, missing invariant, integration conflict, security gap or migration requirement.

## Engineering principle

```text
Enough architecture → code → test → evidence → refine architecture
```

Not:

```text
architecture → architecture → architecture → implementation
```

This rule prevents CAT from becoming documentation-complete but executable-incomplete.
