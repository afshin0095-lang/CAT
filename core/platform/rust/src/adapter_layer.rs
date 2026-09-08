use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    ConcreteCoreRuntime, IntegrationTarget, PlatformError,
    PlatformResult, ProviderAdapterRegistry, ProviderId, RemainingCoreRuntime,
    TypedCoreCommand, TypedCoreResponse, WorkflowRuntime,
};

/// Concrete platform adapter layer.
///
/// This is the composition root for provider/core adapters. It deliberately does not implement
/// business semantics: domain behavior remains in the owning core, while this layer selects
/// the owning runtime, validates the adapter boundary, and preserves request context.
