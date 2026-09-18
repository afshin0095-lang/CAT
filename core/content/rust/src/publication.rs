use crate::ContentId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationOutcome {
    Published,
    Rejected,
    Failed,
    RolledBack,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicationReceipt {
    pub receipt_id: Uuid,
    pub content_id: ContentId,
    pub revision_id: ContentId,
    pub destination: String,
    pub outcome: PublicationOutcome,
    pub policy_version: String,
    pub content_hash: String,
    pub observed_at_ms: u64,
    pub actor_id: String,
    pub metadata: Value,
}

impl PublicationReceipt {
    pub fn new(
        content_id: ContentId,
        revision_id: ContentId,
        destination: impl Into<String>,
        outcome: PublicationOutcome,
        policy_version: impl Into<String>,
        content_hash: impl Into<String>,
        actor_id: impl Into<String>,
    ) -> Self {
        Self {
            receipt_id: Uuid::now_v7(),
            content_id,
            revision_id,
            destination: destination.into(),
            outcome,
            policy_version: policy_version.into(),
            content_hash: content_hash.into(),
            observed_at_ms: 0,
            actor_id: actor_id.into(),
            metadata: Value::Object(Default::default()),
        }
    }

    pub fn with_observed_at_ms(mut self, observed_at_ms: u64) -> Self {
        self.observed_at_ms = observed_at_ms;
        self
    }

    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = metadata;
        self
    }
}
