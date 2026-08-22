#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod model;
mod store;

pub use model::{EvidenceRef, KnowledgeEdge, KnowledgeEdgeId, KnowledgeNode, KnowledgeNodeId};
pub use store::{KnowledgeGraph, KnowledgeStoreError, KnowledgeStoreResult};
