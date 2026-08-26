use cat_content::{
    ContentDomain, ContentKind, InMemoryContentRepository, InMemoryPublicationReceiptRepository,
    PublicationOutcome, PublicationReceipt,
};
use serde_json::json;

#[test]
fn publication_receipt_is_observed_and_queryable() {
    let mut content = InMemoryContentRepository::default();
    let record = ContentDomain::create(
        &mut content,
        ContentKind::Article,
        "CAT publication",
        "verified body",
        1,
    )
    .expect("content should be created");

    let mut receipts = InMemoryPublicationReceiptRepository::default();
    let receipt = PublicationReceipt::new(
        record.id,
        record.id,
        "web",
        PublicationOutcome::Published,
        "content-policy-v1",
        "sha256:abc123",
        "publisher-agent",
    )
    .with_observed_at_ms(42)
    .with_metadata(json!({"channel": "public-site"}));

    let stored = ContentDomain::record_publication_receipt(&content, &mut receipts, receipt)
        .expect("receipt should be accepted");

    assert_eq!(stored.outcome, PublicationOutcome::Published);
    assert_eq!(stored.observed_at_ms, 42);
    assert_eq!(receipts.list(record.id).len(), 1);
}

#[test]
fn publication_receipt_cannot_reference_another_revision() {
    let mut content = InMemoryContentRepository::default();
    let record = ContentDomain::create(
        &mut content,
        ContentKind::Article,
        "CAT publication",
        "verified body",
        1,
    )
    .expect("content should be created");
    let other = ContentDomain::create(
        &mut content,
        ContentKind::Article,
        "Other",
        "body",
        1,
    )
    .expect("other content should be created");

    let mut receipts = InMemoryPublicationReceiptRepository::default();
    let receipt = PublicationReceipt::new(
        record.id,
        other.id,
        "web",
        PublicationOutcome::Published,
        "content-policy-v1",
        "sha256:abc123",
        "publisher-agent",
    );

    let result = ContentDomain::record_publication_receipt(&content, &mut receipts, receipt);
    assert!(result.is_err());
    assert!(receipts.list(record.id).is_empty());
}

#[test]
fn duplicate_publication_receipt_id_is_rejected() {
    let mut content = InMemoryContentRepository::default();
    let record = ContentDomain::create(
        &mut content,
        ContentKind::Article,
        "CAT publication",
        "verified body",
        1,
    )
    .expect("content should be created");

    let mut receipts = InMemoryPublicationReceiptRepository::default();
    let receipt = PublicationReceipt::new(
        record.id,
        record.id,
        "web",
        PublicationOutcome::Failed,
        "content-policy-v1",
        "sha256:abc123",
        "publisher-agent",
    );
    let duplicate = receipt.clone();

    ContentDomain::record_publication_receipt(&content, &mut receipts, receipt)
        .expect("first receipt should be accepted");
    let result = ContentDomain::record_publication_receipt(&content, &mut receipts, duplicate);

    assert!(result.is_err());
    assert_eq!(receipts.list(record.id).len(), 1);
}
