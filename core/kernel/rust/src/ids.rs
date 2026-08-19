use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Stable identifier for a canonical CAT entity.
///
/// UUIDv7 is generated at the boundary where an entity is created. The kernel
/// does not infer domain meaning from the identifier; domain modules own the
/// meaning and lifecycle of the entity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EntityId(Uuid);

impl EntityId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    pub const fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}
