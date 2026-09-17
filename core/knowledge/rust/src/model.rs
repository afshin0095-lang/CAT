use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct KnowledgeNodeId(Uuid);

impl KnowledgeNodeId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for KnowledgeNodeId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct KnowledgeEdgeId(Uuid);

impl KnowledgeEdgeId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for KnowledgeEdgeId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub source: String,
    pub reference: String,
    pub observed_at_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: KnowledgeNodeId,
    pub entity_type: String,
    pub canonical_key: String,
    pub attributes: Value,
    pub evidence: Vec<EvidenceRef>,
    pub confidence: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    pub id: KnowledgeEdgeId,
    pub from: KnowledgeNodeId,
    pub relation: String,
    pub to: KnowledgeNodeId,
    pub attributes: Value,
    pub evidence: Vec<EvidenceRef>,
    pub confidence: f64,
}

impl KnowledgeNode {
    pub fn new(entity_type: impl Into<String>, canonical_key: impl Into<String>) -> Self {
        Self {
            id: KnowledgeNodeId::new(),
            entity_type: entity_type.into(),
            canonical_key: canonical_key.into(),
            attributes: Value::Object(Default::default()),
            evidence: Vec::new(),
            confidence: 1.0,
        }
    }

    pub fn with_attributes(mut self, attributes: Value) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn with_evidence(mut self, evidence: Vec<EvidenceRef>) -> Self {
        self.evidence = evidence;
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}

impl KnowledgeEdge {
    pub fn new(from: KnowledgeNodeId, relation: impl Into<String>, to: KnowledgeNodeId) -> Self {
        Self {
            id: KnowledgeEdgeId::new(),
            from,
            relation: relation.into(),
            to,
            attributes: Value::Object(Default::default()),
            evidence: Vec::new(),
            confidence: 1.0,
        }
    }

    pub fn with_attributes(mut self, attributes: Value) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn with_evidence(mut self, evidence: Vec<EvidenceRef>) -> Self {
        self.evidence = evidence;
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}
