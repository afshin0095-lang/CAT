use cat_content::{ContentDomain, ContentDomainError, ContentKind, InMemoryContentRepository};

fn source() -> (InMemoryContentRepository, cat_content::ContentRecord) {
    let mut repo = InMemoryContentRepository::default();
    let record = ContentDomain::create(
        &mut repo,
        ContentKind::Article,
        "source",
        "stable source body",
        1,
    )
    .unwrap();
    (repo, record)
}

#[test]
fn revision_rejects_empty_title_without_mutating_source() {
    let (mut repo, source) = source();

    assert_eq!(
        ContentDomain::revise(&mut repo, source.id, "   ", "new body", 2),
        Err(ContentDomainError::EmptyTitle)
    );
    assert_eq!(repo.get(source.id).unwrap().title, "source");
    assert_eq!(repo.get(source.id).unwrap().version.version, 1);
}

#[test]
fn revision_rejects_empty_body_without_mutating_source() {
    let (mut repo, source) = source();

    assert_eq!(
        ContentDomain::revise(&mut repo, source.id, "new title", "\n\t", 2),
        Err(ContentDomainError::EmptyBody)
    );
    assert_eq!(repo.get(source.id).unwrap().body, "stable source body");
    assert_eq!(repo.get(source.id).unwrap().version.version, 1);
}

#[test]
fn creation_rejects_zero_version_before_persisting() {
    let mut repo = InMemoryContentRepository::default();

    assert_eq!(
        ContentDomain::create(&mut repo, ContentKind::Article, "title", "body", 0),
        Err(ContentDomainError::InvalidVersion)
    );
}
