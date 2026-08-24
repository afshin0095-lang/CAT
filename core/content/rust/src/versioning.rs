use crate::{ContentDomainError, ContentDomainResult, ContentRecord};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevisionPlan {
    pub next_version: u32,
    pub source_version: u32,
}

impl RevisionPlan {
    pub fn from_current(current: &ContentRecord, next_version: u32) -> ContentDomainResult<Self> {
        if next_version == 0 || next_version <= current.version.version {
            return Err(ContentDomainError::InvalidVersion);
        }
        Ok(Self {
            next_version,
            source_version: current.version.version,
        })
    }
}

pub fn build_revision(current: &ContentRecord, title: String, body: String, plan: RevisionPlan) -> ContentDomainResult<ContentRecord> {
    if title.trim().is_empty() {
        return Err(ContentDomainError::EmptyTitle);
    }
    if body.trim().is_empty() {
        return Err(ContentDomainError::EmptyBody);
    }

    let mut revision = ContentRecord::new(current.kind, title, body, plan.next_version);
    revision.version.source_version = Some(plan.source_version);
    Ok(revision)
}
