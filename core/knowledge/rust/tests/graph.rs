use cat_knowledge::{KnowledgeEdge, KnowledgeGraph, KnowledgeNode};

#[test]
fn canonical_node_insertion_is_idempotent() {
    let mut graph = KnowledgeGraph::default();
    let first = graph
        .insert_node(KnowledgeNode::new("merchant", "merchant:acme"))
        .unwrap();
    let second = graph
        .insert_node(KnowledgeNode::new("merchant", "merchant:acme"))
        .unwrap();

    assert_eq!(first, second);
    assert_eq!(graph.node_count(), 1);
}

#[test]
fn edges_require_existing_endpoints_and_support_relation_queries() {
    let mut graph = KnowledgeGraph::default();
    let merchant = graph
        .insert_node(KnowledgeNode::new("merchant", "merchant:acme"))
        .unwrap();
    let program = graph
        .insert_node(KnowledgeNode::new("program", "program:acme:summer"))
        .unwrap();

    graph
        .insert_edge(KnowledgeEdge::new(merchant, "owns_program", program))
        .unwrap();

    assert_eq!(graph.edge_count(), 1);
    assert_eq!(graph.related(merchant, "owns_program"), vec![program]);
    assert_eq!(
        graph.relation_types().into_iter().collect::<Vec<_>>(),
        vec!["owns_program"]
    );
}

#[test]
fn confidence_is_bounded() {
    let low = KnowledgeNode::new("x", "x:low").with_confidence(-2.0);
    let high = KnowledgeNode::new("x", "x:high").with_confidence(2.0);
    assert_eq!(low.confidence, 0.0);
    assert_eq!(high.confidence, 1.0);
}
