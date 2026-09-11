use cat_content::{
    ContentDomain, ContentKind, ContentStatus, InMemoryContentRepository, PublicationPolicy,
};

fn record(status: ContentStatus) -> cat_content::ContentRecord {
    let mut repo = InMemoryContentRepository::default();
    let created = ContentDomain::create(
        &mut repo,
        ContentKind::Article,
        "Policy test",
        "publishable body",
        1,
    )
    .unwrap();
    let mut updated = created;
    updated.status = status;
    updated
}

#[test]
fn default_policy_requires_approval() {
    let policy = PublicationPolicy::default();
    assert!(policy.validate(&record(ContentStatus::Approved)).is_ok());
    assert!(policy.validate(&record(ContentStatus::Review)).is_err());
}

#[test]
fn policy_rejects_empty_body() {
    let mut value = record(ContentStatus::Approved);
    value.body = "  \n".into();
    assert_eq!(
        PublicationPolicy::default().validate(&value),
        Err(cat_content::ContentDomainError::EmptyBody)
    );
}
