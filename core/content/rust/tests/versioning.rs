use cat_content::{ContentDomain, ContentDomainError, ContentKind, ContentRepository, ContentStatus, InMemoryContentRepository};

#[test]
fn revision_creates_new_identity_and_preserves_source_version() {
    let mut repo = InMemoryContentRepository::default();
    let original = ContentDomain::create(&mut repo, ContentKind::Article, "v1", "body v1", 1).unwrap();
    let revised = ContentDomain::revise(&mut repo, original.id, "v2", "body v2", 2).unwrap();
    assert_ne!(original.id, revised.id);
    assert_eq!(original.version.version, 1);
    assert_eq!(original.version.source_version, None);
    assert_eq!(revised.version.version, 2);
    assert_eq!(revised.version.source_version, Some(1));
    assert_eq!(revised.status, ContentStatus::Draft);
    assert!(original.canonical && revised.canonical);
    assert_eq!(repo.get(original.id).unwrap().body, "body v1");
}

#[test]
fn revision_rejects_non_monotonic_versions() {
    let mut repo = InMemoryContentRepository::default();
    let original = ContentDomain::create(&mut repo, ContentKind::Article, "v1", "body", 3).unwrap();
    assert_eq!(ContentDomain::revise(&mut repo, original.id, "bad", "body", 3), Err(ContentDomainError::InvalidVersion));
    assert_eq!(ContentDomain::revise(&mut repo, original.id, "bad", "body", 2), Err(ContentDomainError::InvalidVersion));
}

#[test]
fn revision_rejects_empty_payloads_without_mutating_source() {
    let mut repo = InMemoryContentRepository::default();
    let original = ContentDomain::create(&mut repo, ContentKind::Article, "stable", "body", 1).unwrap();
    assert_eq!(ContentDomain::revise(&mut repo, original.id, " ", "next", 2), Err(ContentDomainError::EmptyTitle));
    assert_eq!(ContentDomain::revise(&mut repo, original.id, "next", " ", 2), Err(ContentDomainError::EmptyBody));
    assert_eq!(repo.get(original.id).unwrap().version.version, 1);
}
