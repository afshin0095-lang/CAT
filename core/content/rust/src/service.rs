use crate::{
    policy::PublicationPolicy,
    versioning::{build_revision, RevisionPlan},
    ContentDomainError, ContentDomainResult, ContentKind, ContentRecord, ContentRepository, ContentStatus,
    PublicationReceipt, PublicationReceiptRepository,
};

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

    /// Creates a new content identity rather than mutating the existing version.
    /// The returned revision carries the source version needed to reconstruct lineage.
    pub fn revise<R: ContentRepository>(repo: &mut R, id: crate::ContentId, title: impl Into<String>, body: impl Into<String>, next_version: u32) -> ContentDomainResult<ContentRecord> {
        let current = repo.get(id)?;
        let plan = RevisionPlan::from_current(&current, next_version)?;
        let revision = build_revision(&current, title.into(), body.into(), plan)?;
        repo.create(revision.clone())?;
        Ok(revision)
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
        Self::publish_with_policy(repo, id, &PublicationPolicy::default())
    }

    pub fn publish_with_policy<R: ContentRepository>(
        repo: &mut R,
        id: crate::ContentId,
        policy: &PublicationPolicy,
    ) -> ContentDomainResult<ContentRecord> {
        let mut record = repo.get(id)?;
        policy.validate(&record)?;
        record.status = ContentStatus::Published;
        repo.update(record.clone())?;
        Ok(record)
    }

    /// Records an observed publication outcome without changing authorization or canonical content truth.
    pub fn record_publication_receipt<R: ContentRepository, P: PublicationReceiptRepository>(
        repo: &R,
        receipts: &mut P,
        receipt: PublicationReceipt,
    ) -> ContentDomainResult<PublicationReceipt> {
        let content = repo.get(receipt.content_id)?;
        if receipt.revision_id != content.id {
            return Err(ContentDomainError::InvalidState("publication receipt revision mismatch"));
        }
        if receipt.destination.trim().is_empty() {
            return Err(ContentDomainError::InvalidState("publication receipt destination is empty"));
        }
        if receipt.policy_version.trim().is_empty() {
            return Err(ContentDomainError::InvalidState("publication receipt policy version is empty"));
        }
        if receipt.content_hash.trim().is_empty() {
            return Err(ContentDomainError::InvalidState("publication receipt content hash is empty"));
        }
        receipts.append(receipt.clone())?;
        Ok(receipt)
    }
}
