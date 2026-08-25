use crate::{ContentDomainError, ContentDomainResult, ContentRecord, ContentStatus};

/// Publication policy evaluated at the content-domain boundary.
///
/// This policy is deliberately deterministic: generated recommendations may
/// propose a publication, but only a record that has passed the declared
/// lifecycle gates can be published by the domain service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicationPolicy {
    pub require_approval: bool,
    pub require_non_empty_body: bool,
}

impl Default for PublicationPolicy {
    fn default() -> Self {
        Self {
            require_approval: true,
            require_non_empty_body: true,
        }
    }
}

impl PublicationPolicy {
    pub fn validate(&self, record: &ContentRecord) -> ContentDomainResult<()> {
        if self.require_non_empty_body && record.body.trim().is_empty() {
            return Err(ContentDomainError::EmptyBody);
        }
        if self.require_approval && record.status != ContentStatus::Approved {
            return Err(ContentDomainError::InvalidState(
                "publication policy requires approval",
            ));
        }
        Ok(())
    }
}
