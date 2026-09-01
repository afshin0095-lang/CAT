use std::collections::BTreeSet;

use crate::{KnowledgeEdge, KnowledgeGraph, KnowledgeNode, KnowledgeNodeId};

#[derive(Clone, Debug, Default)]
pub struct KnowledgeQuery {
    pub entity_type: Option<String>,
    pub relation: Option<String>,
    pub min_confidence: Option<f64>,
}

impl KnowledgeQuery {
    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn relation(mut self, value: impl Into<String>) -> Self {
        self.relation = Some(value.into());
        self
    }

    pub fn min_confidence(mut self, value: f64) -> Self {
        self.min_confidence = Some(value.clamp(0.0, 1.0));
        self
    }

    pub fn matches_node(&self, node: &KnowledgeNode) -> bool {
        self.entity_type.as_deref().is_none_or(|kind| node.entity_type == kind)
            && self.min_confidence.is_none_or(|min| node.confidence >= min)
    }

    pub fn matches_edge(&self, edge: &KnowledgeEdge) -> bool {
        self.relation.as_deref().is_none_or(|relation| edge.relation == relation)
            && self.min_confidence.is_none_or(|min| edge.confidence >= min)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct KnowledgeNeighborhood {
    pub nodes: BTreeSet<KnowledgeNodeId>,
    pub edges: BTreeSet<crate::KnowledgeEdgeId>,
}

impl KnowledgeGraph {
    pub fn query_nodes<'a>(&'a self, query: &KnowledgeQuery) -> Vec<&'a KnowledgeNode> {
        self.nodes_matching(query).collect()
    }

    pub fn query_edges<'a>(&'a self, query: &KnowledgeQuery) -> Vec<&'a KnowledgeEdge> {
        self.edges_matching(query).collect()
    }

    pub fn neighborhood(
        &self,
        root: KnowledgeNodeId,
        relation: Option<&str>,
    ) -> KnowledgeNeighborhood {
        let mut result = KnowledgeNeighborhood::default();
        result.nodes.insert(root);

        for edge in self.edges_from(root) {
            if relation.is_none_or(|wanted| edge.relation == wanted) {
                result.edges.insert(edge.id);
                result.nodes.insert(edge.to);
            }
        }
        for edge in self.edges_to(root) {
            if relation.is_none_or(|wanted| edge.relation == wanted) {
                result.edges.insert(edge.id);
                result.nodes.insert(edge.from);
            }
        }
        result
    }

    pub(crate) fn nodes_matching<'a>(&'a self, query: &KnowledgeQuery) -> impl Iterator<Item = &'a KnowledgeNode> {
        self.nodes_iter().filter(|node| query.matches_node(node))
    }

    pub(crate) fn edges_matching<'a>(&'a self, query: &KnowledgeQuery) -> impl Iterator<Item = &'a KnowledgeEdge> {
        self.edges_iter().filter(|edge| query.matches_edge(edge))
    }
}
