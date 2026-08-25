#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod adapter_layer;
mod adapters;
mod concrete_core;
mod context;
mod core_adapters;
mod error;
mod health;
mod provider_adapters;
mod remaining_core;
mod runtime;
mod workflow_runtime;

pub use adapter_layer::PlatformAdapterLayer;
pub use adapters::{AdapterRequest, AdapterRegistry, AdapterResponse, PassthroughAdapter, PlatformAdapter};
pub use concrete_core::ConcreteCoreRuntime;
pub use context::{IntegrationCommand, IntegrationContext, IntegrationTarget};
pub use core_adapters::{
    default_core_adapter_registry, execute_typed, CoreAdapter, CoreCommand,
    TypedCoreCommand, TypedCoreResponse, DECISION_ADAPTER, EVENT_BUS_ADAPTER,
    KNOWLEDGE_ADAPTER, LLM_ADAPTER, MEMORY_ADAPTER, ORCHESTRATOR_ADAPTER,
    PLANNING_ADAPTER, REASONING_ADAPTER, RETRIEVAL_ADAPTER,
};
pub use error::{PlatformError, PlatformResult};
pub use health::{ComponentHealth, HealthState, PlatformHealthSnapshot};
pub use provider_adapters::{
    DeterministicProviderAdapter, ExternalProviderAdapter, ProviderAdapterRegistry,
    ProviderCapabilities, ProviderHealth, ProviderId,
};
pub use remaining_core::RemainingCoreRuntime;
pub use runtime::{PlatformRuntime, ReadyWork};
pub use workflow_runtime::{WorkflowExecutionReceipt, WorkflowRuntime};
