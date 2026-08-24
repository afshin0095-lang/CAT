use cat_content::{ContentPublished, ContentVersionCreated};
use cat_eventbus::{CatEvent, EventEnvelope};
use uuid::Uuid;

#[test]
fn content_events_use_stable_names_and_versions() {
    assert_eq!(ContentVersionCreated::TYPE, "content.version.created");
    assert_eq!(ContentPublished::TYPE, "content.published");
    assert_eq!(ContentVersionCreated::VERSION, 2);
    assert_eq!(ContentPublished::VERSION, 1);
}

#[test]
fn version_event_preserves_source_version() {
    let event = ContentVersionCreated {
        content_id: cat_content::ContentId(Uuid::now_v7()),
        version: 4,
        source_version: Some(3),
    };
    let envelope = EventEnvelope::from_typed(event, "content-domain").unwrap();
    assert_eq!(envelope.event_type, ContentVersionCreated::TYPE);
    assert_eq!(envelope.version, 2);
    assert_eq!(envelope.payload["version"], 4);
    assert_eq!(envelope.payload["source_version"], 3);
}

#[test]
fn publication_event_round_trips_through_envelope() {
    let event = ContentPublished {
        content_id: cat_content::ContentId(Uuid::now_v7()),
        version: 3,
        publication_id: Uuid::now_v7(),
    };
    let envelope = EventEnvelope::from_typed(event, "content-domain").unwrap();
    assert_eq!(envelope.event_type, ContentPublished::TYPE);
    assert_eq!(envelope.version, ContentPublished::VERSION);
    assert_eq!(envelope.payload["version"], 3);
    assert_eq!(envelope.producer, "content-domain");
}
