use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ContentId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentKind {
    Article,
    ProductDescription,
    LandingPage,
    Email,
    Social,
    AgentBrief,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentStatus {
    Draft,
    Review,
    Approved,
    Published,
    Archived,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContentVersion {
    pub version: u32,
    pub source_version: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContentRecord {
    pub id: ContentId,
    pub kind: ContentKind,
    pub title: String,
    pub body: String,
    pub status: ContentStatus,
    pub version: ContentVersion,
    pub canonical: bool,
}

impl ContentRecord {
    pub fn new(kind: ContentKind, title: String, body: String, version: u32) -> Self {
        Self {
            id: ContentId(Uuid::now_v7()),
            kind,
            title,
            body,
            status: ContentStatus::Draft,
            version: ContentVersion {
                version,
                source_version: None,
            },
            canonical: true,
        }
    }
}
