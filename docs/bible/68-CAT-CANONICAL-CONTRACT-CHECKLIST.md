# CAT Canonical Contract Checklist

**Status:** L2 — Architecture specified

Use this checklist whenever CAT introduces or changes an Agent, Capability, Tool, Connector, Provider or Schema.

## Identity

- [ ] Stable canonical ID exists.
- [ ] Semantic version is explicit.
- [ ] Owner is defined.
- [ ] Lifecycle status is defined.

## Semantics

- [ ] Purpose is singular and unambiguous.
- [ ] Inputs are typed.
- [ ] Outputs are typed.
- [ ] Preconditions and postconditions are documented.
- [ ] Non-goals are documented.

## Runtime

- [ ] Side-effect class is explicit.
- [ ] Idempotency is defined.
- [ ] Timeout is bounded.
- [ ] Retry behavior is typed.
- [ ] Cancellation behavior is defined.
- [ ] Unknown external outcomes are handled.

## Security

- [ ] Authorization boundary is explicit.
- [ ] Data scope is defined.
- [ ] Secrets are references, not plaintext.
- [ ] Network access is bounded.
- [ ] Prompt-injection boundary is preserved.
- [ ] Economic authority is bounded when applicable.

## Operations

- [ ] Metrics exist.
- [ ] Tracing/correlation exists where required.
- [ ] Audit/evidence requirements exist.
- [ ] Health model exists.
- [ ] Cost attribution exists.
- [ ] Failure alerts are actionable.

## Compatibility

- [ ] Existing consumers were identified.
- [ ] Schema compatibility was checked.
- [ ] Migration strategy exists if needed.
- [ ] Historical records remain interpretable.

## Testing

- [ ] Structural validation.
- [ ] Contract tests.
- [ ] Security tests.
- [ ] Failure/retry tests.
- [ ] Idempotency tests.
- [ ] Integration tests for supported implementations.

## Activation rule

A contract is not `ACTIVE` until all mandatory gates are satisfied and the activation is auditable.
