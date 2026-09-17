use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::{KnowledgeGraph, KnowledgeNodeId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraversalResult {
    pub nodes: Vec<KnowledgeNodeId>,
    pub depth_by_node: BTreeMap<KnowledgeNodeId, usize>,
}

impl TraversalResult {
    pub fn contains(&self, id: KnowledgeNodeId) -> bool {
        self.depth_by_node.contains_key(&id)
    }

    pub fn depth(&self, id: KnowledgeNodeId) -> Option<usize> {
        self.depth_by_node.get(&id).copied()
    }
}

impl KnowledgeGraph {
    /// Breadth-first traversal over outgoing edges, optionally restricted to one relation.
    /// The graph remains canonical; traversal is a derived read-only operation.
    pub fn traverse(
        &self,
        start: KnowledgeNodeId,
        max_depth: usize,
        relation: Option<&str>,
    ) -> TraversalResult {
        let mut queue = VecDeque::from([(start, 0usize)]);
        let mut visited = BTreeSet::from([start]);
        let mut depth_by_node = BTreeMap::from([(start, 0usize)]);
        let mut nodes = vec![start];

        while let Some((current, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }

            for edge in self.edges_from(current) {
                if relation.is_some_and(|expected| edge.relation != expected) {
                    continue;
                }
                if visited.insert(edge.to) {
                    let next_depth = depth + 1;
                    depth_by_node.insert(edge.to, next_depth);
                    nodes.push(edge.to);
                    queue.push_back((edge.to, next_depth));
                }
            }
        }

        TraversalResult {
            nodes,
            depth_by_node,
        }
    }

    pub fn neighbors(&self, node: KnowledgeNodeId) -> BTreeSet<KnowledgeNodeId> {
        let mut result = BTreeSet::new();
        for edge in self.edges_from(node) {
            result.insert(edge.to);
        }
        for edge in self.edges_to(node) {
            result.insert(edge.from);
        }
        result
    }
}
