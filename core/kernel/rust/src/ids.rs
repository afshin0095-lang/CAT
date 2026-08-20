use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Stable identifier for a canonical CAT entity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EntityId(Uuid);

impl EntityId {
    pub fn new() -> Self { Self(Uuid::now_v7()) }
    pub const fn from_uuid(value: Uuid) -> Self { Self(value) }
    pub const fn as_uuid(&self) -> Uuid { self.0 }
}
impl Default for EntityId { fn default() -> Self { Self::new() } }

/// Stable identifier for an immutable event envelope.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EventId(Uuid);
impl EventId {
    pub fn new() -> Self { Self(Uuid::now_v7()) }
    pub const fn from_uuid(value: Uuid) -> Self { Self(value) }
    pub const fn as_uuid(&self) -> Uuid { self.0 }
}
impl Default for EventId { fn default() -> Self { Self::new() } }

/// Tenant boundary identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TenantId(Uuid);
impl TenantId {
    pub fn new() -> Self { Self(Uuid::now_v7()) }
    pub const fn from_uuid(value: Uuid) -> Self { Self(value) }
    pub const fn as_uuid(&self) -> Uuid { self.0 }
}
impl Default for TenantId { fn default() -> Self { Self::new() } }

/// Identifier linking an operation to the originating request chain.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CorrelationId(Uuid);
impl CorrelationId {
    pub fn new() -> Self { Self(Uuid::now_v7()) }
    pub const fn from_uuid(value: Uuid) -> Self { Self(value) }
    pub const fn as_uuid(&self) -> Uuid { self.0 }
}
impl Default for CorrelationId { fn default() -> Self { Self::new() } }

/// Identifier linking an event to the event that directly caused it.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CausationId(Uuid);
impl CausationId {
    pub fn new() -> Self { Self(Uuid::now_v7()) }
    pub const fn from_uuid(value: Uuid) -> Self { Self(value) }
    pub const fn as_uuid(&self) -> Uuid { self.0 }
}
impl Default for CausationId { fn default() -> Self { Self::new() } }
