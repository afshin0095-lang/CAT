use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct MemoryId(pub Uuid);

impl MemoryId {
    pub fn new() -> Self { Self(Uuid::now_v7()) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKind {
    Working,
    Episodic,
    Semantic,
    Procedural,
    Decision,
    Preference,
    Lesson,
    Observation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Classification {
    Public,
    Internal,
    Confidential,
    Restricted,
    Secret,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleState {
    Proposed,
    Validated,
    Active,
    Superseded,
    Retained,
    Expired,
    Revoked,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Provenance {
    pub source: String,
    pub source_version: Option<String>,
    pub captured_at_ms: u64,
    pub captured_by: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Consent {
    pub required: bool,
    pub granted: bool,
    pub scope: String,
    pub policy_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Retention {
    pub expires_at_ms: Option<u64>,
    pub legal_hold: bool,
    pub policy_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MemoryObject {
    pub id: MemoryId,
    pub kind: MemoryKind,
    pub namespace: String,
    pub subject: Option<String>,
    pub classification: Classification,
    pub state: LifecycleState,
    pub version: u32,
    pub content: serde_json::Value,
    pub provenance: Provenance,
    pub consent: Consent,
    pub retention: Retention,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
}

impl MemoryObject {
    pub fn validate(&self, now_ms: u64) -> Result<(), MemoryValidationError> {
        if self.namespace.trim().is_empty() { return Err(MemoryValidationError::EmptyNamespace); }
        if self.version == 0 { return Err(MemoryValidationError::InvalidVersion); }
        if self.provenance.source.trim().is_empty() { return Err(MemoryValidationError::MissingProvenance); }
        if self.provenance.captured_by.trim().is_empty() { return Err(MemoryValidationError::MissingProvenance); }
        if self.consent.required && !self.consent.granted {
            return Err(MemoryValidationError::ConsentRequired);
        }
        if self.retention.legal_hold && self.state == LifecycleState::Expired {
            return Err(MemoryValidationError::LegalHoldExpired);
        }
        if let Some(expiry) = self.retention.expires_at_ms {
            if expiry <= now_ms && self.state == LifecycleState::Active && !self.retention.legal_hold {
                return Err(MemoryValidationError::ActiveAfterExpiry);
            }
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum MemoryValidationError {
    #[error("memory namespace must not be empty")]
    EmptyNamespace,
    #[error("memory version must be greater than zero")]
    InvalidVersion,
    #[error("memory provenance is incomplete")]
    MissingProvenance,
    #[error("required consent has not been granted")]
    ConsentRequired,
    #[error("a legal hold cannot contain an expired memory state")]
    LegalHoldExpired,
    #[error("active memory has passed its retention expiry")]
    ActiveAfterExpiry,
}
