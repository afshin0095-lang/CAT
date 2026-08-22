use crate::{KnowledgeEdge, KnowledgeNode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KnowledgeViolation {
    EmptyEntityType,
    EmptyCanonicalKey,
    InvalidConfidence,
    MissingEvidence,
    EmptyRelation,
    SelfRelation,
}

pub fn validate_node(node: &KnowledgeNode) -> Vec<KnowledgeViolation> {
    let mut violations = Vec::new();
    if node.entity_type.trim().is_empty() {
        violations.push(KnowledgeViolation::EmptyEntityType);
    }
    if node.canonical_key.trim().is_empty() {
        violations.push(KnowledgeViolation::EmptyCanonicalKey);
    }
    if !(0.0..=1.0).contains(&node.confidence) {
        violations.push(KnowledgeViolation::InvalidConfidence);
    }
    if node.evidence.is_empty() {
        violations.push(KnowledgeViolation::MissingEvidence);
    }
    violations
}

pub fn validate_edge(edge: &KnowledgeEdge) -> Vec<KnowledgeViolation> {
    let mut violations = Vec::new();
    if edge.relation.trim().is_empty() {
        violations.push(KnowledgeViolation::EmptyRelation);
    }
    if edge.from == edge.to {
        violations.push(KnowledgeViolation::SelfRelation);
    }
    if !(0.0..=1.0).contains(&edge.confidence) {
        violations.push(KnowledgeViolation::InvalidConfidence);
    }
    if edge.evidence.is_empty() {
        violations.push(KnowledgeViolation::MissingEvidence);
    }
    violations
}
