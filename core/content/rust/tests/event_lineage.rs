use cat_content::{ContentId, ContentPublished, ContentVersionCreated};
use cat_eventbus::{CatEvent, EventEnvelope};
use uuid::Uuid;

#[test]
fn revision_event_can_carry_subject_and_causation_lineage() {
    let subject = ContentId(Uuid::now_v7());
    let cause = Uuid::now_v7();
    let event = ContentVersionCreated {
        content_id: subject,
        version: 2,
        source_version: Some(1),
    };

    let envelope = EventEnvelope::from_typed(event, "content-domain")
        .unwrap()
        .with_subject_id(subject)
        .with_causation_id(cause);

    assert_eq!(envelope.subject_id, Some(subject));
    assert_eq!(envelope.causation_id, Some(cause));
    assert_eq!(envelope.event_type, ContentVersionCreated::TYPE);
    assert_eq!(envelope.version, ContentVersionCreated::VERSION);
}

#[test]
fn publication_event_can_join_a_correlation_chain() {
    let content_id = ContentId(Uuid::now_v7());
    let correlation = Uuid::now_v7();
    let event = ContentPublished {
        content_id,
        version: 2,
        publication_id: Uuid::now_v7(),
    };

    let envelope = EventEnvelope::from_typed(event, "content-publisher")
        .unwrap()
        .with_subject_id(content_id)
        .with_correlation_id(correlation);

    assert_eq!(envelope.subject_id, Some(content_id));
    assert_eq!(envelope.correlation_id, Some(correlation));
    assert_eq!(envelope.producer, "content-publisher");
}
