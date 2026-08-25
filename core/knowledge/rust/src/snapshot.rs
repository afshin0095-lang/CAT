use serde::{Deserialize, Serialize};

use crate::{KnowledgeEdge, KnowledgeGraph, KnowledgeNode, KnowledgeStoreResult};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KnowledgeSnapshot {
    pub nodes: Vec<KnowledgeNode>,
    pub edges: Vec<KnowledgeEdge>,
}

impl KnowledgeSnapshot {
    pub fn from_graph(graph: &KnowledgeGraph) -> Self {
        Self {
            nodes: graph.nodes_iter().cloned().collect(),
            edges: graph.edges_iter().cloned().collect(),
        }
    }

    pub fn restore(self) -> KnowledgeStoreResult<KnowledgeGraph> {
        let mut graph = KnowledgeGraph::default();
        for node in self.nodes {
            graph.insert_node(node)?;
        }
        for edge in self.edges {
            graph.insert_edge(edge)?;
        }
        Ok(graph)
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    pub fn from_json(value: &str) -> serde_json::Result<Self> {
        serde_json::from_str(value)
    }
}
