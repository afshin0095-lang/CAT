use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod error;
pub mod events;
pub mod memory_repository;
pub mod model;
pub mod policy;
pub mod provenance;
pub mod repository;
pub mod service;
pub mod versioning;
pub mod content;
pub mod ftc_compliance;
pub mod optimization;

pub use error::{ContentDomainError, ContentDomainResult};
pub use events::{ContentPublished, ContentVersionCreated};
pub use memory_repository::InMemoryContentRepository;
pub use model::{ContentId, ContentKind, ContentRecord, ContentStatus, ContentVersion};
pub use policy::PublicationPolicy;
pub use provenance::{ContentProvenance, ProvenanceLink};
pub use repository::ContentRepository;
pub use service::ContentDomain;
pub use versioning::{build_revision, RevisionPlan};
pub use content::{ContentTemplate, TemplateLibrary, TemplateMetrics, TemplateType};
pub use ftc_compliance::{ComplianceCheck, ComplianceIssue, CompliancePolicy};
pub use optimization::{ContentOptimizer, ContentVariant};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentEventMetadata { pub event_id: Uuid, pub producer: String }
