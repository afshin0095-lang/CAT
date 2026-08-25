use crate::{KnowledgeGraph, KnowledgeSnapshot, KnowledgeStoreResult};

#[derive(Debug, thiserror::Error)]
pub enum KnowledgePersistenceError {
    #[error("snapshot serialization failed: {0}")]
    Serialization(String),
    #[error("snapshot restore failed: {0}")]
    Restore(String),
}

pub type KnowledgePersistenceResult<T> = Result<T, KnowledgePersistenceError>;

/// Persistence boundary for the knowledge graph. Concrete databases stay outside the domain model.
pub trait KnowledgeSnapshotStore: Send + Sync {
    fn save(&mut self, snapshot: &KnowledgeSnapshot) -> KnowledgePersistenceResult<()>;
    fn load(&self) -> KnowledgePersistenceResult<Option<KnowledgeSnapshot>>;
}

#[derive(Clone, Debug, Default)]
pub struct MemorySnapshotStore {
    encoded: Option<String>,
}

impl KnowledgeSnapshotStore for MemorySnapshotStore {
    fn save(&mut self, snapshot: &KnowledgeSnapshot) -> KnowledgePersistenceResult<()> {
        self.encoded = Some(
            snapshot
                .to_json()
                .map_err(|error| KnowledgePersistenceError::Serialization(error.to_string()))?,
        );
        Ok(())
    }

    fn load(&self) -> KnowledgePersistenceResult<Option<KnowledgeSnapshot>> {
        self.encoded
            .as_deref()
            .map(KnowledgeSnapshot::from_json)
            .transpose()
            .map_err(|error| KnowledgePersistenceError::Serialization(error.to_string()))
    }
}

impl KnowledgeGraph {
    pub fn persist<S: KnowledgeSnapshotStore>(
        &self,
        store: &mut S,
    ) -> KnowledgePersistenceResult<()> {
        store.save(&KnowledgeSnapshot::from_graph(self))
    }

    pub fn restore_from<S: KnowledgeSnapshotStore>(
        store: &S,
    ) -> KnowledgePersistenceResult<Option<Self>> {
        let Some(snapshot) = store.load()? else { return Ok(None); };
        snapshot
            .restore()
            .map(Some)
            .map_err(|error| KnowledgePersistenceError::Restore(error.to_string()))
    }
}

#[allow(dead_code)]
fn _keep_result_alias_used(_: KnowledgeStoreResult<KnowledgeGraph>) {}

#[cfg(test)]
mod tests {
    use super::{KnowledgeSnapshotStore, MemorySnapshotStore};
    use crate::{EvidenceRef, KnowledgeGraph, KnowledgeNode};

    #[test]
    fn memory_snapshot_round_trip_preserves_graph_shape() {
        let mut graph = KnowledgeGraph::default();
        let node = KnowledgeNode::new("merchant", "merchant:1").with_evidence(vec![EvidenceRef {
            source: "fixture".into(), reference: "r1".into(), observed_at_ms: 10,
        }]);
        graph.insert_node(node).unwrap();

        let mut store = MemorySnapshotStore::default();
        graph.persist(&mut store).unwrap();
        let restored = KnowledgeGraph::restore_from(&store).unwrap().unwrap();
        assert_eq!(restored.node_count(), 1);
        assert_eq!(restored.edge_count(), 0);
        assert!(store.load().unwrap().is_some());
    }
}
