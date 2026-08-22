#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod model;
mod store;
mod traversal;
mod validation;

pub use model::{EvidenceRef, KnowledgeEdge, KnowledgeEdgeId, KnowledgeNode, KnowledgeNodeId};
pub use store::{KnowledgeGraph, KnowledgeStoreError, KnowledgeStoreResult};
pub use traversal::TraversalResult;
pub use validation::{validate_edge, validate_node, KnowledgeViolation};
