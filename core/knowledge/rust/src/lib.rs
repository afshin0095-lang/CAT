#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod model;
mod persistence;
mod provenance;
mod query;
mod snapshot;
mod store;
mod temporal;
mod traversal;
mod validation;

pub use model::{EvidenceRef, KnowledgeEdge, KnowledgeEdgeId, KnowledgeNode, KnowledgeNodeId};
pub use persistence::{KnowledgePersistenceError, KnowledgePersistenceResult, KnowledgeSnapshotStore, MemorySnapshotStore};
pub use provenance::ProvenanceChain;
pub use query::{KnowledgeNeighborhood, KnowledgeQuery};
pub use snapshot::KnowledgeSnapshot;
pub use store::{KnowledgeGraph, KnowledgeStoreError, KnowledgeStoreResult};
pub use temporal::ValidityWindow;
pub use traversal::TraversalResult;
pub use validation::{validate_edge, validate_node, KnowledgeViolation};
