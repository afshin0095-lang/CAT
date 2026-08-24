use crate::{ContentDomainError, ContentDomainResult, ContentId, ContentRecord, ContentStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationTarget {
    Web,
    Mobile,
    Email,
    Social,
    AgentFeed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationState {
    Planned,
    Submitted,
    Confirmed,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicationRecord {
    pub publication_id: Uuid,
    pub content_id: ContentId,
    pub content_version: u32,
    pub target: PublicationTarget,
    pub state: PublicationState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PublicationPlan {
    pub publication_id: Uuid,
    pub content_id: ContentId,
    pub content_version: u32,
    pub target: PublicationTarget,
}

impl PublicationPlan {
    pub fn from_record(record: &ContentRecord, target: PublicationTarget) -> ContentDomainResult<Self> {
        match record.status {
            ContentStatus::Approved | ContentStatus::Published => Ok(Self {
                publication_id: Uuid::now_v7(),
                content_id: record.id,
                content_version: record.version.version,
                target,
            }),
            _ => Err(ContentDomainError::InvalidState(
                "publication planning requires approved or published content",
            )),
        }
    }

    pub fn materialize(self) -> PublicationRecord {
        PublicationRecord {
            publication_id: self.publication_id,
            content_id: self.content_id,
            content_version: self.content_version,
            target: self.target,
            state: PublicationState::Planned,
        }
    }
}

impl PublicationRecord {
    pub fn submit(&mut self) -> ContentDomainResult<()> {
        if self.state != PublicationState::Planned {
            return Err(ContentDomainError::InvalidState(
                "publication submission requires planned state",
            ));
        }
        self.state = PublicationState::Submitted;
        Ok(())
    }

    pub fn confirm(&mut self) -> ContentDomainResult<()> {
        if self.state != PublicationState::Submitted {
            return Err(ContentDomainError::InvalidState(
                "publication confirmation requires submitted state",
            ));
        }
        self.state = PublicationState::Confirmed;
        Ok(())
    }

    pub fn fail(&mut self) -> ContentDomainResult<()> {
        match self.state {
            PublicationState::Planned | PublicationState::Submitted => {
                self.state = PublicationState::Failed;
                Ok(())
            }
            _ => Err(ContentDomainError::InvalidState(
                "publication failure requires planned or submitted state",
            )),
        }
    }

    pub fn cancel(&mut self) -> ContentDomainResult<()> {
        match self.state {
            PublicationState::Planned | PublicationState::Submitted => {
                self.state = PublicationState::Cancelled;
                Ok(())
            }
            _ => Err(ContentDomainError::InvalidState(
                "publication cancellation requires planned or submitted state",
            )),
        }
    }
}
