use cat_eventstore_postgres::{OutboxPublisherConfig, PublicationOutcome};
use std::time::Duration;
use uuid::Uuid;

#[test]
fn publisher_defaults_are_conservative_for_recovery_and_polling() {
    let config = OutboxPublisherConfig::default();
    assert_eq!(config.stale_after, Duration::from_secs(300));
    assert_eq!(config.idle_delay, Duration::from_millis(250));
}

#[test]
fn publication_outcomes_keep_event_identity_and_attempt_count() {
    let event_id = Uuid::now_v7();
    let outcome = PublicationOutcome::RetryScheduled {
        event_id,
        attempts: 3,
        delay: Duration::from_secs(2),
    };

    assert_eq!(outcome, PublicationOutcome::RetryScheduled {
        event_id,
        attempts: 3,
        delay: Duration::from_secs(2),
    });
}
