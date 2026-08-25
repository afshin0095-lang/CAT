use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod error;
pub mod events;
pub mod memory_repository;
pub mod model;
pub mod repository;
pub mod service;
pub mod versioning;

pub use error::{ContentDomainError, ContentDomainResult};
pub use events::{ContentPublished, ContentVersionCreated};
pub use memory_repository::InMemoryContentRepository;
pub use model::{ContentId, ContentKind, ContentRecord, ContentStatus, ContentVersion};
pub use repository::ContentRepository;
pub use service::ContentDomain;
pub use versioning::{build_revision, RevisionPlan};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentEventMetadata {
    pub event_id: Uuid,
    pub producer: String,
}
