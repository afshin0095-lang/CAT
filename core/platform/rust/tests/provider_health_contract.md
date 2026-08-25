# Provider Health Contract Fixture

This marker file documents the deterministic integration fixture used by `provider_health_contract.rs`.

The executable contract verifies that provider health is observable through the platform boundary, that resilient adapters remain registerable through the provider-neutral registry, and that circuit state is reflected as Ready/Degraded/Unavailable without moving provider semantics into domain cores.
