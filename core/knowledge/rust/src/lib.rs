#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod model;
mod query;
mod snapshot;
mod store;
mod traversal;
mod validation;

pub use model::{EvidenceRef, KnowledgeEdge, KnowledgeEdgeId, KnowledgeNode, KnowledgeNodeId};
pub use query::{KnowledgeNeighborhood, KnowledgeQuery};
pub use snapshot::KnowledgeSnapshot;
pub use store::{KnowledgeGraph, KnowledgeStoreError, KnowledgeStoreResult};
pub use traversal::TraversalResult;
pub use validation::{validate_edge, validate_node, KnowledgeViolation};
