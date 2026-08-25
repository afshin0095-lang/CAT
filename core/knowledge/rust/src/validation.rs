use crate::{EvidenceRef, KnowledgeEdge, KnowledgeGraph, KnowledgeNode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KnowledgeViolation {
    EmptyEntityType,
    EmptyCanonicalKey,
    InvalidConfidence,
    MissingEvidence,
    InvalidEvidenceSource,
    InvalidEvidenceReference,
    EmptyRelation,
    SelfRelation,
    MissingEndpoint,
}

pub fn validate_evidence(evidence: &EvidenceRef) -> Vec<KnowledgeViolation> {
    let mut violations = Vec::new();
    if evidence.source.trim().is_empty() { violations.push(KnowledgeViolation::InvalidEvidenceSource); }
    if evidence.reference.trim().is_empty() { violations.push(KnowledgeViolation::InvalidEvidenceReference); }
    violations
}

pub fn validate_node(node: &KnowledgeNode) -> Vec<KnowledgeViolation> {
    let mut violations = Vec::new();
    if node.entity_type.trim().is_empty() { violations.push(KnowledgeViolation::EmptyEntityType); }
    if node.canonical_key.trim().is_empty() { violations.push(KnowledgeViolation::EmptyCanonicalKey); }
    if !(0.0..=1.0).contains(&node.confidence) { violations.push(KnowledgeViolation::InvalidConfidence); }
    if node.evidence.is_empty() { violations.push(KnowledgeViolation::MissingEvidence); }
    for evidence in &node.evidence { violations.extend(validate_evidence(evidence)); }
    violations
}

pub fn validate_edge(edge: &KnowledgeEdge) -> Vec<KnowledgeViolation> {
    let mut violations = Vec::new();
    if edge.relation.trim().is_empty() { violations.push(KnowledgeViolation::EmptyRelation); }
    if edge.from == edge.to { violations.push(KnowledgeViolation::SelfRelation); }
    if !(0.0..=1.0).contains(&edge.confidence) { violations.push(KnowledgeViolation::InvalidConfidence); }
    if edge.evidence.is_empty() { violations.push(KnowledgeViolation::MissingEvidence); }
    for evidence in &edge.evidence { violations.extend(validate_evidence(evidence)); }
    violations
}

impl KnowledgeGraph {
    pub fn validate_consistency(&self) -> Vec<KnowledgeViolation> {
        let mut violations = Vec::new();
        for node in self.nodes_iter() { violations.extend(validate_node(node)); }
        for edge in self.edges_iter() {
            if self.node(edge.from).is_none() || self.node(edge.to).is_none() { violations.push(KnowledgeViolation::MissingEndpoint); }
            violations.extend(validate_edge(edge));
        }
        violations
    }
}
