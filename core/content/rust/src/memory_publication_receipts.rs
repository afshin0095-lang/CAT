use std::collections::HashMap;

use crate::{ContentDomainError, ContentDomainResult, ContentId, PublicationReceipt, PublicationReceiptRepository};

#[derive(Default)]
pub struct InMemoryPublicationReceiptRepository {
    receipts: HashMap<ContentId, Vec<PublicationReceipt>>,
}

impl PublicationReceiptRepository for InMemoryPublicationReceiptRepository {
    fn append(&mut self, receipt: PublicationReceipt) -> ContentDomainResult<()> {
        let bucket = self.receipts.entry(receipt.content_id).or_default();
        if bucket.iter().any(|existing| existing.receipt_id == receipt.receipt_id) {
            return Err(ContentDomainError::AlreadyExists);
        }
        bucket.push(receipt);
        Ok(())
    }

    fn list(&self, content_id: ContentId) -> Vec<PublicationReceipt> {
        self.receipts.get(&content_id).cloned().unwrap_or_default()
    }
}
