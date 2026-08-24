use std::collections::HashMap;

use crate::{repository::require_existing, ContentDomainError, ContentDomainResult, ContentId, ContentRecord, ContentRepository};

#[derive(Default)]
pub struct InMemoryContentRepository { records: HashMap<ContentId, ContentRecord> }

impl ContentRepository for InMemoryContentRepository {
    fn create(&mut self, record: ContentRecord) -> ContentDomainResult<()> {
        if self.records.contains_key(&record.id) { return Err(ContentDomainError::AlreadyExists); }
        self.records.insert(record.id, record);
        Ok(())
    }

    fn get(&self, id: ContentId) -> ContentDomainResult<ContentRecord> {
        require_existing(self.records.get(&id).cloned())
    }

    fn update(&mut self, record: ContentRecord) -> ContentDomainResult<()> {
        if !self.records.contains_key(&record.id) { return Err(ContentDomainError::NotFound); }
        self.records.insert(record.id, record);
        Ok(())
    }

    fn list(&self) -> Vec<ContentRecord> { self.records.values().cloned().collect() }
}
