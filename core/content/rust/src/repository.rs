use crate::{ContentDomainError, ContentDomainResult, ContentId, ContentRecord};

pub trait ContentRepository: Send + Sync {
    fn create(&mut self, record: ContentRecord) -> ContentDomainResult<()>;
    fn get(&self, id: ContentId) -> ContentDomainResult<ContentRecord>;
    fn update(&mut self, record: ContentRecord) -> ContentDomainResult<()>;
    fn list(&self) -> Vec<ContentRecord>;
}

pub fn require_existing<T>(value: Option<T>) -> ContentDomainResult<T> {
    value.ok_or(ContentDomainError::NotFound)
}
