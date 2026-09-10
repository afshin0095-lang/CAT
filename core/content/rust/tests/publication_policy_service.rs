use cat_content::{ContentDomain, ContentDomainError, ContentKind, ContentRepository, ContentStatus, InMemoryContentRepository, PublicationPolicy};

#[test]
fn publish_with_policy_requires_approval_and_persists_published_state() {
    let mut repo = InMemoryContentRepository::default();
    let record = ContentDomain::create(&mut repo, ContentKind::Article, "Policy publication", "verified body", 1).unwrap();

    assert_eq!(
        ContentDomain::publish_with_policy(&mut repo, record.id, &PublicationPolicy::default()),
        Err(ContentDomainError::InvalidState("publication policy requires approval"))
    );

    ContentDomain::submit_for_review(&mut repo, record.id).unwrap();
    ContentDomain::approve(&mut repo, record.id).unwrap();
    let published = ContentDomain::publish_with_policy(&mut repo, record.id, &PublicationPolicy::default()).unwrap();

    assert_eq!(published.status, ContentStatus::Published);
    assert_eq!(repo.get(record.id).unwrap().status, ContentStatus::Published);
}
