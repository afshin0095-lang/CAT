use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod content;
pub mod error;
pub mod events;
pub mod ftc_compliance;
pub mod memory_repository;
pub mod model;
pub mod optimization;
pub mod policy;
pub mod provenance;
pub mod repository;
pub mod service;
pub mod versioning;

pub use content::{ContentTemplate, TemplateLibrary, TemplateMetrics, TemplateType};
pub use error::{ContentDomainError, ContentDomainResult};
pub use events::{ContentPublished, ContentVersionCreated};
pub use ftc_compliance::{ComplianceCheck, ComplianceIssue, CompliancePolicy};
pub use memory_repository::InMemoryContentRepository;
pub use model::{ContentId, ContentKind, ContentRecord, ContentStatus, ContentVersion};
pub use optimization::{ContentOptimizer, ContentVariant};
pub use policy::PublicationPolicy;
pub use provenance::{ContentProvenance, ProvenanceLink};
pub use repository::ContentRepository;
pub use service::ContentDomain;
pub use versioning::{RevisionPlan, build_revision};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentEventMetadata {
    pub event_id: Uuid,
    pub producer: String,
}
