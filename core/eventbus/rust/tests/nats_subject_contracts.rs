use cat_eventbus::{EventBusError, subject_for_prefix};

#[test]
fn subject_prefix_normalization_is_deterministic() {
    assert_eq!(
        subject_for_prefix("cat.events", "order.created").unwrap(),
        "cat.events.order.created"
    );
    assert_eq!(
        subject_for_prefix("cat.events...", "order.created").unwrap(),
        "cat.events.order.created"
    );
    assert_eq!(
        subject_for_prefix("", "order.created").unwrap(),
        "order.created"
    );
}

#[test]
fn wildcard_and_malformed_fragments_are_rejected() {
    for fragment in [
        "",
        ".order",
        "order.",
        "order created",
        "order.*",
        "order.>",
    ] {
        assert!(matches!(
            subject_for_prefix("cat.events", fragment),
            Err(EventBusError::InvalidConfiguration(_))
        ));
    }
}
