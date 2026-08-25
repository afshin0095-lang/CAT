use cat_knowledge::{EvidenceRef, KnowledgeGraph, KnowledgeNode, KnowledgeSnapshotStore, MemorySnapshotStore, ProvenanceChain, ValidityWindow};

#[test]
fn temporal_boundaries_are_deterministic() {
    let window = ValidityWindow::from(1_000).until(2_000);
    assert!(!window.is_valid_at(999));
    assert!(window.is_valid_at(1_000));
    assert!(window.is_valid_at(1_999));
    assert!(!window.is_valid_at(2_000));
}

#[test]
fn provenance_chain_is_deterministic_and_deduplicated() {
    let first = EvidenceRef { source: "source-b".into(), reference: "2".into(), observed_at_ms: 20 };
    let second = EvidenceRef { source: "source-a".into(), reference: "1".into(), observed_at_ms: 10 };
    let chain = ProvenanceChain::from_evidence([first.clone(), second.clone(), first]);
    assert_eq!(chain.entries(), &[second, EvidenceRef { source: "source-b".into(), reference: "2".into(), observed_at_ms: 20 }]);
    assert_eq!(chain.observed_at_bounds(), Some((10, 20)));
}

#[test]
fn snapshot_persistence_does_not_change_graph_identity() {
    let mut graph = KnowledgeGraph::default();
    let node = KnowledgeNode::new("product", "product:42").with_evidence(vec![EvidenceRef {
        source: "catalog".into(), reference: "sku-42".into(), observed_at_ms: 100,
    }]);
    let id = graph.insert_node(node).expect("node insertion");

    let mut store = MemorySnapshotStore::default();
    graph.persist(&mut store).expect("persist");
    let restored = KnowledgeGraph::restore_from(&store).expect("restore").expect("snapshot");

    assert!(restored.node(id).is_some());
    assert_eq!(restored.node_count(), 1);
    assert_eq!(restored.edge_count(), 0);
    assert!(store.load().expect("load").is_some());
}
