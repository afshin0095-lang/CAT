use cat_knowledge::{validate_edge, validate_node, EvidenceRef, KnowledgeEdge, KnowledgeGraph, KnowledgeNode, KnowledgeStoreError, KnowledgeViolation};
use serde_json::json;

fn evidence() -> Vec<EvidenceRef> {
    vec![EvidenceRef {
        source: "test".into(),
        reference: "fixture://knowledge/1".into(),
        observed_at_ms: 1,
    }]
}

#[test]
fn canonical_identity_is_deduplicated() {
    let mut graph = KnowledgeGraph::default();
    let a = graph.insert_node(KnowledgeNode::new("merchant", "acme").with_evidence(evidence())).unwrap();
    let b = graph.insert_node(KnowledgeNode::new("merchant", "acme").with_evidence(evidence())).unwrap();

    assert_eq!(a, b);
    assert_eq!(graph.node_count(), 1);
}

#[test]
fn edges_require_existing_endpoints() {
    let mut graph = KnowledgeGraph::default();
    let a = graph.insert_node(KnowledgeNode::new("merchant", "acme").with_evidence(evidence())).unwrap();
    let missing = KnowledgeNode::new("product", "missing").id;
    let edge = KnowledgeEdge::new(a, "offers", missing).with_evidence(evidence());

    assert!(graph.insert_edge(edge).is_err());
}

#[test]
fn traversal_is_derived_and_relation_scoped() {
    let mut graph = KnowledgeGraph::default();
    let a = graph.insert_node(KnowledgeNode::new("merchant", "acme").with_evidence(evidence())).unwrap();
    let b = graph.insert_node(KnowledgeNode::new("product", "phone").with_attributes(json!({"sku":"P1"})).with_evidence(evidence())).unwrap();
    let c = graph.insert_node(KnowledgeNode::new("category", "electronics").with_evidence(evidence())).unwrap();
    graph.insert_edge(KnowledgeEdge::new(a, "offers", b).with_evidence(evidence())).unwrap();
    graph.insert_edge(KnowledgeEdge::new(b, "member_of", c).with_evidence(evidence())).unwrap();

    let result = graph.traverse(a, 2, None);
    assert_eq!(result.depth(c), Some(2));
    assert!(result.contains(b));

    let scoped = graph.traverse(a, 2, Some("offers"));
    assert!(scoped.contains(b));
    assert!(!scoped.contains(c));
}

#[test]
fn validation_requires_evidence_and_valid_identity() {
    let node = KnowledgeNode::new("", "").with_confidence(0.5);
    let violations = validate_node(&node);
    assert!(violations.contains(&KnowledgeViolation::EmptyEntityType));
    assert!(violations.contains(&KnowledgeViolation::EmptyCanonicalKey));
    assert!(violations.contains(&KnowledgeViolation::MissingEvidence));
}

#[test]
fn validation_rejects_self_relation() {
    let node = KnowledgeNode::new("merchant", "acme").with_evidence(evidence());
    let edge = KnowledgeEdge::new(node.id, "related_to", node.id).with_evidence(evidence());
    assert!(validate_edge(&edge).contains(&KnowledgeViolation::SelfRelation));
}

#[test]
fn node_id_collision_is_rejected_without_overwriting_existing_state() {
    let mut graph = KnowledgeGraph::default();
    let original = KnowledgeNode::new("merchant", "acme").with_evidence(evidence());
    let id = original.id;
    graph.insert_node(original).unwrap();

    let mut conflicting = KnowledgeNode::new("merchant", "other").with_evidence(evidence());
    conflicting.id = id;
    let error = graph.insert_node(conflicting).unwrap_err();

    assert_eq!(error, KnowledgeStoreError::DuplicateNodeId(id));
    assert_eq!(graph.node_count(), 1);
    assert!(graph.find_node("merchant", "acme").is_some());
    assert!(graph.find_node("merchant", "other").is_none());
}

#[test]
fn edge_id_collision_is_rejected_without_replacing_existing_edge() {
    let mut graph = KnowledgeGraph::default();
    let a = graph.insert_node(KnowledgeNode::new("merchant", "acme").with_evidence(evidence())).unwrap();
    let b = graph.insert_node(KnowledgeNode::new("product", "phone").with_evidence(evidence())).unwrap();
    let first = KnowledgeEdge::new(a, "offers", b).with_evidence(evidence());
    let edge_id = first.id;
    graph.insert_edge(first).unwrap();

    let mut conflicting = KnowledgeEdge::new(a, "recommends", b).with_evidence(evidence());
    conflicting.id = edge_id;
    let error = graph.insert_edge(conflicting).unwrap_err();

    assert_eq!(error, KnowledgeStoreError::DuplicateEdgeId(edge_id));
    assert_eq!(graph.edge_count(), 1);
    assert_eq!(graph.edges_from(a)[0].relation, "offers");
}
