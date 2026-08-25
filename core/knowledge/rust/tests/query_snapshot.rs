use cat_knowledge::{KnowledgeGraph, KnowledgeQuery, KnowledgeSnapshot};

#[test]
fn query_filters_nodes_by_type_and_confidence() {
    let mut graph = KnowledgeGraph::default();
    graph.insert_node(cat_knowledge::KnowledgeNode::new("merchant", "m-1").with_confidence(0.95)).unwrap();
    graph.insert_node(cat_knowledge::KnowledgeNode::new("merchant", "m-2").with_confidence(0.40)).unwrap();
    graph.insert_node(cat_knowledge::KnowledgeNode::new("offer", "o-1").with_confidence(0.99)).unwrap();

    let query = KnowledgeQuery::default().entity_type("merchant").min_confidence(0.90);
    let result = graph.query_nodes(&query);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].canonical_key, "m-1");
}

#[test]
fn neighborhood_contains_incoming_and_outgoing_edges() {
    let mut graph = KnowledgeGraph::default();
    let merchant = graph.insert_node(cat_knowledge::KnowledgeNode::new("merchant", "m-1")).unwrap();
    let offer = graph.insert_node(cat_knowledge::KnowledgeNode::new("offer", "o-1")).unwrap();
    let partner = graph.insert_node(cat_knowledge::KnowledgeNode::new("partner", "p-1")).unwrap();

    graph.insert_edge(cat_knowledge::KnowledgeEdge::new(merchant, "publishes", offer)).unwrap();
    graph.insert_edge(cat_knowledge::KnowledgeEdge::new(partner, "promotes", merchant)).unwrap();

    let neighborhood = graph.neighborhood(merchant, None);
    assert_eq!(neighborhood.nodes.len(), 3);
    assert_eq!(neighborhood.edges.len(), 2);
}

#[test]
fn snapshot_round_trip_preserves_graph_shape() {
    let mut graph = KnowledgeGraph::default();
    let a = graph.insert_node(cat_knowledge::KnowledgeNode::new("merchant", "m-1")).unwrap();
    let b = graph.insert_node(cat_knowledge::KnowledgeNode::new("offer", "o-1")).unwrap();
    graph.insert_edge(cat_knowledge::KnowledgeEdge::new(a, "publishes", b)).unwrap();

    let snapshot = KnowledgeSnapshot::from_graph(&graph);
    let json = snapshot.to_json().unwrap();
    let restored = KnowledgeSnapshot::from_json(&json).unwrap().restore().unwrap();

    assert_eq!(restored.node_count(), graph.node_count());
    assert_eq!(restored.edge_count(), graph.edge_count());
    assert!(restored.find_node("merchant", "m-1").is_some());
}
