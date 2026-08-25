use cat_content::{ContentId, PublicationContext, PublicationEventFactory};
use cat_eventbus::CatEvent;
use uuid::Uuid;

#[test]
fn publication_event_contains_subject_and_contract_identity() {
    let content_id = ContentId(Uuid::now_v7());
    let publication_id = Uuid::now_v7();

    let envelope = PublicationEventFactory::published(
        content_id,
        7,
        publication_id,
        PublicationContext::new("content-publisher"),
    )
    .unwrap();

    assert_eq!(envelope.subject_id, Some(content_id));
    assert_eq!(envelope.event_type, "content.published");
    assert_eq!(envelope.version, 1);
    assert_eq!(envelope.producer, "content-publisher");
    assert_eq!(envelope.payload["version"], 7);
    assert_eq!(envelope.payload["publication_id"], publication_id.to_string());
}

#[test]
fn publication_event_preserves_correlation_and_causation() {
    let content_id = ContentId(Uuid::now_v7());
    let correlation = Uuid::now_v7();
    let causation = Uuid::now_v7();

    let envelope = PublicationEventFactory::published(
        content_id,
        2,
        Uuid::now_v7(),
        PublicationContext::new("content-publisher")
            .with_correlation_id(correlation)
            .with_causation_id(causation),
    )
    .unwrap();

    assert_eq!(envelope.subject_id, Some(content_id));
    assert_eq!(envelope.correlation_id, Some(correlation));
    assert_eq!(envelope.causation_id, Some(causation));
}

#[test]
fn publication_factory_exposes_stable_contract() {
    assert_eq!(PublicationEventFactory::event_contract(), ("content.published", 1));
}
