use std::sync::Arc;
use std::time::Duration;

use cat_platform::{
    AdapterRequest, DeterministicProviderAdapter, ExternalProviderAdapter, IntegrationCommand,
    IntegrationContext, IntegrationTarget, PlatformError, ProviderAdapterRegistry,
    ProviderCircuitConfig, ProviderCircuitState, ProviderHealth, ProviderHealthProbe, ProviderId,
    ProviderRetryConfig, ResilientProviderAdapter,
};
use serde_json::json;
use std::sync::atomic::{AtomicU32, Ordering};

struct FailingProvider {
    capabilities: cat_platform::ProviderCapabilities,
    remaining_failures: AtomicU32,
}

impl FailingProvider {
    fn new(failures: u32) -> Self {
        Self {
            capabilities: cat_platform::ProviderCapabilities {
                provider_id: ProviderId::new("probe-failing").unwrap(),
                target: IntegrationTarget::Llm,
                operations: vec!["generate".into()],
                health: ProviderHealth::Ready,
            },
            remaining_failures: AtomicU32::new(failures),
        }
    }
}

impl ExternalProviderAdapter for FailingProvider {
    fn capabilities(&self) -> cat_platform::ProviderCapabilities {
        self.capabilities.clone()
    }

    fn execute(
        &self,
        request: &AdapterRequest,
    ) -> cat_platform::PlatformResult<cat_platform::AdapterResponse> {
        if self.remaining_failures.load(Ordering::SeqCst) > 0 {
            self.remaining_failures.fetch_sub(1, Ordering::SeqCst);
            return Err(PlatformError::TransportUnavailable(
                "synthetic health failure".into(),
            ));
        }
        Ok(cat_platform::AdapterResponse {
            request_id: request.context.request_id,
            target: request.target,
            operation: request.operation.clone(),
            accepted: true,
            payload: json!({"ok": true}),
        })
    }
}

fn request() -> AdapterRequest {
    AdapterRequest::from_command(
        IntegrationCommand::new(
            IntegrationTarget::Llm,
            "generate",
            IntegrationContext::new("provider-health-contract"),
        ),
        json!({"prompt": "health"}),
    )
}

#[test]
fn registry_health_probe_is_deterministic_and_provider_neutral() {
    let mut registry = ProviderAdapterRegistry::default();
    registry
        .register(Arc::new(
            DeterministicProviderAdapter::new("provider-b", IntegrationTarget::Llm, ["generate"])
                .unwrap(),
        ))
        .unwrap();
    registry
        .register(Arc::new(
            DeterministicProviderAdapter::new("provider-a", IntegrationTarget::Llm, ["generate"])
                .unwrap(),
        ))
        .unwrap();

    assert_eq!(
        registry
            .probe_health(&ProviderId::new("provider-a").unwrap())
            .unwrap(),
        ProviderHealth::Ready
    );
    assert_eq!(
        registry.probe_all_health().unwrap(),
        vec![
            (
                ProviderId::new("provider-a").unwrap(),
                ProviderHealth::Ready
            ),
            (
                ProviderId::new("provider-b").unwrap(),
                ProviderHealth::Ready
            ),
        ]
    );
}

#[test]
fn resilient_adapter_can_be_registered_and_reports_circuit_health() {
    let resilient = Arc::new(
        ResilientProviderAdapter::new(
            Arc::new(FailingProvider::new(10)),
            ProviderRetryConfig {
                max_attempts: 2,
                retry_delay: Duration::ZERO,
                max_retry_delay: Duration::ZERO,
            },
            ProviderCircuitConfig {
                failure_threshold: 1,
                recovery_after: Duration::from_secs(60),
            },
        )
        .unwrap(),
    );
    let mut registry = ProviderAdapterRegistry::default();
    registry.register(resilient.clone()).unwrap();
    let provider = ProviderId::new("probe-failing").unwrap();

    assert_eq!(
        registry.probe_health(&provider).unwrap(),
        ProviderHealth::Ready
    );
    assert!(registry.execute(&provider, &request()).is_err());
    assert_eq!(resilient.circuit_state(), ProviderCircuitState::Open);
    assert_eq!(
        registry.probe_health(&provider).unwrap(),
        ProviderHealth::Unavailable
    );
    assert!(matches!(
        registry.execute(&provider, &request()),
        Err(PlatformError::TransportUnavailable(_))
    ));
}

#[test]
fn resilient_adapter_preserves_provider_contract_after_recovery() {
    let resilient = ResilientProviderAdapter::new(
        Arc::new(FailingProvider::new(1)),
        ProviderRetryConfig {
            max_attempts: 2,
            retry_delay: Duration::ZERO,
        },
        ProviderCircuitConfig {
            failure_threshold: 3,
            recovery_after: Duration::ZERO,
        },
    )
    .unwrap();

    assert!(resilient.execute(&request()).unwrap().accepted);
    assert_eq!(ProviderHealthProbe::probe_health(&resilient).unwrap(), ProviderHealth::Ready);
    assert_eq!(resilient.capabilities().target, IntegrationTarget::Llm);
}

#[test]
fn deterministic_probe_contract_is_available_without_network() {
    let adapter =
        DeterministicProviderAdapter::new("local", IntegrationTarget::Llm, ["generate"]).unwrap();
    assert_eq!(
        ProviderHealthProbe::probe_health(&adapter).unwrap(),
        ProviderHealth::Ready
    );
}
