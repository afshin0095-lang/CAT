use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod error;
pub mod events;
pub mod memory_publication_receipts;
pub mod memory_repository;
pub mod model;
pub mod policy;
pub mod provenance;
pub mod publication;
pub mod publication_repository;
pub mod repository;
pub mod service;
pub mod versioning;

pub use error::{ContentDomainError, ContentDomainResult};
pub use events::{ContentPublished, ContentVersionCreated};
pub use memory_publication_receipts::InMemoryPublicationReceiptRepository;
pub use memory_repository::InMemoryContentRepository;
pub use model::{ContentId, ContentKind, ContentRecord, ContentStatus, ContentVersion};
pub use policy::PublicationPolicy;
pub use provenance::{ContentProvenance, ProvenanceLink};
pub use publication::{PublicationOutcome, PublicationReceipt};
pub use publication_repository::PublicationReceiptRepository;
pub use repository::ContentRepository;
pub use service::ContentDomain;
pub use versioning::{build_revision, RevisionPlan};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentEventMetadata {
    pub event_id: Uuid,
    pub producer: String,
}
