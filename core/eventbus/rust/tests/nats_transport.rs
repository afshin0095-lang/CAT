use cat_eventbus::{EventBusError, subject_for_prefix};

#[test]
fn subject_mapping_is_stable() {
    assert_eq!(
        subject_for_prefix("cat.events", "affiliate.created").unwrap(),
        "cat.events.affiliate.created"
    );
    assert_eq!(
        subject_for_prefix("cat.events.", "affiliate.created").unwrap(),
        "cat.events.affiliate.created"
    );
}

#[test]
fn wildcard_subject_fragments_are_rejected() {
    assert!(matches!(
        subject_for_prefix("cat.events", "affiliate.*"),
        Err(EventBusError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        subject_for_prefix("cat.events", "affiliate.>"),
        Err(EventBusError::InvalidConfiguration(_))
    ));
}
