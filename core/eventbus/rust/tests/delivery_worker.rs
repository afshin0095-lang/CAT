use cat_eventbus::{DeliveryOutcome, DeliveryWorker, InMemoryDeadLetterStore, InMemoryOutbox, RecordingTransport, RetryPolicy};

#[test]
fn empty_worker_reports_idle_without_spinning() {
    let mut worker = DeliveryWorker::new(InMemoryOutbox::default(), RecordingTransport::default(), InMemoryDeadLetterStore::default(), RetryPolicy::default());
    let outcomes = worker.run_until_idle().expect("worker should drain cleanly");
    assert_eq!(outcomes, vec![DeliveryOutcome::Idle]);
}

#[test]
fn worker_config_can_bound_a_maintenance_tick() {
    let mut worker = DeliveryWorker::new(InMemoryOutbox::default(), RecordingTransport::default(), InMemoryDeadLetterStore::default(), RetryPolicy::default());
    worker.config.max_attempts_per_run = 1;
    let outcomes = worker.run_until_idle().expect("bounded run should succeed");
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0], DeliveryOutcome::Idle);
}
