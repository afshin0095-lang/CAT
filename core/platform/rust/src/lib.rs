#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod adapter_layer;
mod adapters;
mod concrete_core;
mod context;
mod core_adapters;
mod error;
mod health;
mod http_provider;
mod provider_adapters;
mod provider_error;
mod provider_telemetry;
mod remaining_core;
mod resilience;
mod resilient_adapter;
mod runtime;
mod workflow_runtime;

pub use adapter_layer::PlatformAdapterLayer;
pub use adapters::{
    AdapterRegistry, AdapterRequest, AdapterResponse, PassthroughAdapter, PlatformAdapter,
};
pub use concrete_core::ConcreteCoreRuntime;
pub use context::{IntegrationCommand, IntegrationContext, IntegrationTarget};
pub use core_adapters::{
    CoreAdapter, CoreCommand, DECISION_ADAPTER, EVENT_BUS_ADAPTER, KNOWLEDGE_ADAPTER, LLM_ADAPTER,
    MEMORY_ADAPTER, ORCHESTRATOR_ADAPTER, PLANNING_ADAPTER, REASONING_ADAPTER, RETRIEVAL_ADAPTER,
    TypedCoreCommand, TypedCoreResponse, default_core_adapter_registry, execute_typed,
};
pub use error::{PlatformError, PlatformResult};
pub use health::{ComponentHealth, HealthState, PlatformHealthSnapshot};
pub use http_provider::HttpJsonProviderAdapter;
pub use provider_adapters::{
    DeterministicProviderAdapter, ExternalProviderAdapter, ProviderAdapterRegistry,
    ProviderCapabilities, ProviderHealth, ProviderHealthProbe, ProviderId,
};
pub use provider_error::{ProviderFailure, ProviderFailureClass};
pub use provider_telemetry::ProviderTelemetry;
pub use remaining_core::RemainingCoreRuntime;
pub use resilience::{ProviderCircuitBreaker, ProviderCircuitConfig, ProviderCircuitState};
pub use resilient_adapter::{ProviderRetryConfig, ResilientProviderAdapter};
pub use runtime::{PlatformRuntime, ReadyWork};
pub use workflow_runtime::{WorkflowExecutionReceipt, WorkflowRuntime};
