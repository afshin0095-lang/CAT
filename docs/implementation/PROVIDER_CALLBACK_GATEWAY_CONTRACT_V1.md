# CAT OMNISYSTEM — Provider Callback Gateway Contract V1

**Status:** Implemented P0
**Scope:** Provider-neutral routing and provider-specific callback verification

## 1. Trust boundary

Raw HTTP/webhook/provider callback input is untrusted data. It must not enter `ProviderCallbackStore` directly.

```text
External Provider
      ↓
ProviderCallbackIngress
      ↓
ProviderCallbackVerifierRegistry
      ↓
Provider-specific verifier
      ↓
Normalized ProviderCallback
      ↓
ProviderCallbackDispatcher
      ↓
Durable ProviderCallbackStore
      ↓
Correlation / Provider Result / Journal
```

## 2. Ingress contract

`ProviderCallbackIngress` contains only transport-level callback material:

- callback identifier;
- provider routing name;
- received headers;
- raw JSON body;
- receipt timestamp.

Transport credentials and signatures are not persisted by the dispatcher as authorization evidence.

## 3. Verifier contract

`ProviderCallbackVerifier` is provider-specific and owns authenticity/normalization rules.

A verifier may use:

- signed webhook headers;
- provider-specific SDK verification;
- canonical payload validation;
- provider-specific replay protection.

The verifier returns the normalized CAT `ProviderCallback` only after verification succeeds.

## 4. Dispatcher invariants

1. Unknown providers fail closed.
2. A verifier must be registered under the same provider name as the ingress.
3. A verifier cannot change callback ID, provider identity, or receipt timestamp.
4. Normalized callback validation runs again before persistence.
5. Persistence remains responsible for durable idempotency and execution correlation.
6. The dispatcher does not create capability authorization or operator authority.

## 5. Multi-provider design

The registry is intentionally separate from `ProviderAdapterRegistry` because execution adapters and callback verifiers have different trust and lifecycle concerns.

A single provider may later expose separate execution, polling, callback-verification, and reconciliation implementations without coupling those concerns.

## 6. Security

No raw long-lived credentials are stored in provider callback domain objects. Provider-specific secret handling belongs to infrastructure/security components injected into verifier implementations.

Future revisions can persist cryptographic verification metadata as evidence without making that metadata an authorization primitive.