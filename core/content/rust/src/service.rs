use crate::{ContentDomainError, ContentDomainResult, ContentKind, ContentRecord, ContentRepository, ContentStatus};

pub struct ContentDomain;

impl ContentDomain {
    pub fn create<R: ContentRepository>(repo: &mut R, kind: ContentKind, title: impl Into<String>, body: impl Into<String>, version: u32) -> ContentDomainResult<ContentRecord> {
        let title = title.into();
        let body = body.into();
        if title.trim().is_empty() { return Err(ContentDomainError::EmptyTitle); }
        if body.trim().is_empty() { return Err(ContentDomainError::EmptyBody); }
        if version == 0 { return Err(ContentDomainError::InvalidVersion); }
        let record = ContentRecord::new(kind, title, body, version);
        repo.create(record.clone())?;
        Ok(record)
    }

    pub fn submit_for_review<R: ContentRepository>(repo: &mut R, id: crate::ContentId) -> ContentDomainResult<ContentRecord> {
        let mut record = repo.get(id)?;
        if record.status != ContentStatus::Draft { return Err(ContentDomainError::InvalidState("review requires draft")); }
        record.status = ContentStatus::Review;
        repo.update(record.clone())?;
        Ok(record)
    }

    pub fn approve<R: ContentRepository>(repo: &mut R, id: crate::ContentId) -> ContentDomainResult<ContentRecord> {
        let mut record = repo.get(id)?;
        if record.status != ContentStatus::Review { return Err(ContentDomainError::InvalidState("approval requires review")); }
        record.status = ContentStatus::Approved;
        repo.update(record.clone())?;
        Ok(record)
    }

    pub fn publish<R: ContentRepository>(repo: &mut R, id: crate::ContentId) -> ContentDomainResult<ContentRecord> {
        let mut record = repo.get(id)?;
        if record.status != ContentStatus::Approved { return Err(ContentDomainError::InvalidState("publication requires approval")); }
        record.status = ContentStatus::Published;
        repo.update(record.clone())?;
        Ok(record)
    }
}
