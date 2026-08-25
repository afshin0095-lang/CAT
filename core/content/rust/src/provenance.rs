use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ContentId;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContentProvenance {
    pub source_id: String,
    pub source_version: Option<u32>,
    pub observed_at: String,
    pub actor: String,
    pub evidence_id: Uuid,
}

impl ContentProvenance {
    pub fn new(
        source_id: impl Into<String>,
        source_version: Option<u32>,
        observed_at: impl Into<String>,
        actor: impl Into<String>,
    ) -> Self {
        Self {
            source_id: source_id.into(),
            source_version,
            observed_at: observed_at.into(),
            actor: actor.into(),
            evidence_id: Uuid::now_v7(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProvenanceLink {
    pub content_id: ContentId,
    pub evidence_id: Uuid,
    pub relation: String,
}

impl ProvenanceLink {
    pub fn derives_from(content_id: ContentId, evidence_id: Uuid) -> Self {
        Self {
            content_id,
            evidence_id,
            relation: "derives_from".to_owned(),
        }
    }
}
