use crate::{ContentDomainResult, PublicationReceipt};

pub trait PublicationReceiptRepository: Send + Sync {
    fn append(&mut self, receipt: PublicationReceipt) -> ContentDomainResult<()>;
    fn list(&self, content_id: crate::ContentId) -> Vec<PublicationReceipt>;
}
