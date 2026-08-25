use cat_eventbus::CatEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ContentId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentVersionCreated {
    pub content_id: ContentId,
    pub version: u32,
    pub source_version: Option<u32>,
}

impl CatEvent for ContentVersionCreated {
    const TYPE: &'static str = "content.version.created";
    const VERSION: u16 = 2;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentPublished {
    pub content_id: ContentId,
    pub version: u32,
    pub publication_id: Uuid,
}

impl CatEvent for ContentPublished {
    const TYPE: &'static str = "content.published";
    const VERSION: u16 = 1;
}
