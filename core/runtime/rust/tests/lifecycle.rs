use std::sync::{Arc, Mutex};

use cat_eventbus::{EventBus, PublishOutcome};
use cat_runtime::{Runtime, RuntimeState};

#[test]
fn lifecycle_follows_created_running_draining_stopped() {
    let bus = Arc::new(EventBus::new());
    let mut runtime = Runtime::new("test-runtime", bus);

    assert_eq!(runtime.state(), RuntimeState::Created);
    assert!(matches!(runtime.start().unwrap(), PublishOutcome::Published { handlers_called: 0 }));
    assert_eq!(runtime.state(), RuntimeState::Running);
    assert!(matches!(runtime.drain().unwrap(), PublishOutcome::Published { handlers_called: 0 }));
    assert_eq!(runtime.state(), RuntimeState::Draining);
    assert!(matches!(runtime.stop().unwrap(), PublishOutcome::Published { handlers_called: 0 }));
    assert_eq!(runtime.state(), RuntimeState::Stopped);
}

#[test]
fn invalid_transition_is_rejected_without_mutating_state() {
    let bus = Arc::new(EventBus::new());
    let mut runtime = Runtime::new("test-runtime", bus);

    let error = runtime.stop().unwrap_err();
    assert!(error.to_string().contains("invalid runtime transition"));
    assert_eq!(runtime.state(), RuntimeState::Created);
}

#[test]
fn lifecycle_events_cross_the_eventbus_boundary() {
    let bus = Arc::new(EventBus::new());
    let started = Arc::new(Mutex::new(Vec::<String>::new()));
    let started_capture = Arc::clone(&started);

    let mut runtime = Runtime::new("test-runtime", Arc::clone(&bus));
    runtime
        .subscribe(
            "cat.runtime.started",
            Arc::new(move |event| {
                started_capture.lock().unwrap().push(event.event_type.clone());
                Ok(())
            }),
        )
        .unwrap();

    let outcome = runtime.start().unwrap();
    assert_eq!(outcome, PublishOutcome::Published { handlers_called: 1 });
    assert_eq!(started.lock().unwrap().as_slice(), &["cat.runtime.started"]);
}

#[test]
fn stopped_runtime_does_not_allow_restart() {
    let bus = Arc::new(EventBus::new());
    let mut runtime = Runtime::new("test-runtime", bus);

    runtime.start().unwrap();
    runtime.stop().unwrap();
    assert!(runtime.start().is_err());
    assert_eq!(runtime.state(), RuntimeState::Stopped);
}
