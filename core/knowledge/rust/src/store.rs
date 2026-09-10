use std::collections::{BTreeMap, BTreeSet};

use crate::{KnowledgeEdge, KnowledgeEdgeId, KnowledgeNode, KnowledgeNodeId};

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum KnowledgeStoreError {
    #[error("knowledge node not found: {0:?}")]
    NodeNotFound(KnowledgeNodeId),
    #[error("knowledge edge references a missing node")]
    MissingEndpoint,
    #[error("knowledge node id already exists: {0:?}")]
    DuplicateNodeId(KnowledgeNodeId),
    #[error("knowledge edge id already exists: {0:?}")]
    DuplicateEdgeId(KnowledgeEdgeId),
}

pub type KnowledgeStoreResult<T> = Result<T, KnowledgeStoreError>;

#[derive(Default)]
pub struct KnowledgeGraph {
    nodes: BTreeMap<KnowledgeNodeId, KnowledgeNode>,
    edges: BTreeMap<KnowledgeEdgeId, KnowledgeEdge>,
    canonical_index: BTreeMap<(String, String), KnowledgeNodeId>,
}

impl KnowledgeGraph {
    pub fn insert_node(&mut self, node: KnowledgeNode) -> KnowledgeStoreResult<KnowledgeNodeId> {
        let key = (node.entity_type.clone(), node.canonical_key.clone());
        if let Some(existing) = self.canonical_index.get(&key) { return Ok(*existing); }
        let id = node.id;
        if self.nodes.contains_key(&id) { return Err(KnowledgeStoreError::DuplicateNodeId(id)); }
        self.canonical_index.insert(key, id);
        self.nodes.insert(id, node);
        Ok(id)
    }
    pub fn insert_edge(&mut self, edge: KnowledgeEdge) -> KnowledgeStoreResult<KnowledgeEdgeId> {
        if !self.nodes.contains_key(&edge.from) { return Err(KnowledgeStoreError::NodeNotFound(edge.from)); }
        if !self.nodes.contains_key(&edge.to) { return Err(KnowledgeStoreError::NodeNotFound(edge.to)); }
        let id = edge.id;
        if self.edges.contains_key(&id) { return Err(KnowledgeStoreError::DuplicateEdgeId(id)); }
        self.edges.insert(id, edge);
        Ok(id)
    }
    pub fn node(&self, id: KnowledgeNodeId) -> Option<&KnowledgeNode> { self.nodes.get(&id) }
    pub fn find_node(&self, entity_type: &str, canonical_key: &str) -> Option<&KnowledgeNode> {
        self.canonical_index.get(&(entity_type.to_owned(), canonical_key.to_owned())).and_then(|id| self.nodes.get(id))
    }
    pub fn edges_from(&self, node: KnowledgeNodeId) -> Vec<&KnowledgeEdge> { self.edges.values().filter(|edge| edge.from == node).collect() }
    pub fn edges_to(&self, node: KnowledgeNodeId) -> Vec<&KnowledgeEdge> { self.edges.values().filter(|edge| edge.to == node).collect() }
    pub fn related(&self, node: KnowledgeNodeId, relation: &str) -> Vec<KnowledgeNodeId> {
        self.edges.values().filter(|edge| edge.from == node && edge.relation == relation).map(|edge| edge.to).collect()
    }
    pub fn relation_types(&self) -> BTreeSet<String> { self.edges.values().map(|edge| edge.relation.clone()).collect() }
    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }
    pub(crate) fn nodes_iter(&self) -> impl Iterator<Item = &KnowledgeNode> { self.nodes.values() }
    pub(crate) fn edges_iter(&self) -> impl Iterator<Item = &KnowledgeEdge> { self.edges.values() }
}
