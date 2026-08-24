use cat_content::{ContentDomain, ContentDomainError, ContentKind, ContentStatus, InMemoryContentRepository};

fn draft() -> (InMemoryContentRepository, cat_content::ContentRecord) {
    let mut repo = InMemoryContentRepository::default();
    let record = ContentDomain::create(
        &mut repo,
        ContentKind::Article,
        "Lifecycle contract",
        "immutable domain payload",
        1,
    )
    .expect("draft creation should succeed");
    (repo, record)
}

#[test]
fn lifecycle_advances_only_through_the_declared_order() {
    let (mut repo, record) = draft();

    let review = ContentDomain::submit_for_review(&mut repo, record.id).unwrap();
    assert_eq!(review.status, ContentStatus::Review);

    let approved = ContentDomain::approve(&mut repo, record.id).unwrap();
    assert_eq!(approved.status, ContentStatus::Approved);

    let published = ContentDomain::publish(&mut repo, record.id).unwrap();
    assert_eq!(published.status, ContentStatus::Published);
}

#[test]
fn review_cannot_be_reopened_after_approval() {
    let (mut repo, record) = draft();
    ContentDomain::submit_for_review(&mut repo, record.id).unwrap();
    ContentDomain::approve(&mut repo, record.id).unwrap();

    assert_eq!(
        ContentDomain::submit_for_review(&mut repo, record.id),
        Err(ContentDomainError::InvalidState("review requires draft"))
    );
}

#[test]
fn publish_requires_approval() {
    let (mut repo, record) = draft();

    assert_eq!(
        ContentDomain::publish(&mut repo, record.id),
        Err(ContentDomainError::InvalidState("publication requires approval"))
    );

    ContentDomain::submit_for_review(&mut repo, record.id).unwrap();
    assert_eq!(
        ContentDomain::publish(&mut repo, record.id),
        Err(ContentDomainError::InvalidState("publication requires approval"))
    );
}
