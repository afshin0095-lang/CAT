use cat_content::{ContentDomain, ContentDomainError, ContentKind, ContentStatus, InMemoryContentRepository};

#[test]
fn content_lifecycle_is_guarded() {
    let mut repo = InMemoryContentRepository::default();
    let record = ContentDomain::create(&mut repo, ContentKind::Article, "CAT", "canonical content", 1).unwrap();
    assert_eq!(record.status, ContentStatus::Draft);
    assert!(matches!(ContentDomain::publish(&mut repo, record.id), Err(ContentDomainError::InvalidState(_))));
    assert_eq!(ContentDomain::submit_for_review(&mut repo, record.id).unwrap().status, ContentStatus::Review);
    assert_eq!(ContentDomain::approve(&mut repo, record.id).unwrap().status, ContentStatus::Approved);
    assert_eq!(ContentDomain::publish(&mut repo, record.id).unwrap().status, ContentStatus::Published);
}

#[test]
fn canonical_content_requires_real_fields() {
    let mut repo = InMemoryContentRepository::default();
    assert_eq!(ContentDomain::create(&mut repo, ContentKind::Article, "", "body", 1), Err(ContentDomainError::EmptyTitle));
    assert_eq!(ContentDomain::create(&mut repo, ContentKind::Article, "title", "", 1), Err(ContentDomainError::EmptyBody));
}
