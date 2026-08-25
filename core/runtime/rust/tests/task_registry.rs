#[path = "../src/task_registry.rs"]
mod task_registry;

use task_registry::{TaskId, TaskRegistry, TaskSpec, TaskState};

fn task(id: &str, attempts: u32) -> TaskSpec {
    TaskSpec::new(TaskId::new(id).unwrap(), "example", attempts).unwrap()
}

#[test]
fn registration_is_idempotent_for_a_single_task_id() {
    let mut registry = TaskRegistry::default();
    assert!(registry.register(task("task-a", 2)));
    assert!(!registry.register(task("task-a", 3)));
    assert_eq!(registry.len(), 1);
}

#[test]
fn failed_tasks_can_retry_until_the_attempt_budget_is_exhausted() {
    let mut registry = TaskRegistry::default();
    let id = TaskId::new("task-a").unwrap();
    registry.register(task("task-a", 2));

    assert_eq!(registry.start(&id).unwrap(), 1);
    assert!(registry.fail(&id).unwrap());
    assert_eq!(registry.get(&id).unwrap().state, TaskState::Registered);

    assert_eq!(registry.start(&id).unwrap(), 2);
    assert!(!registry.fail(&id).unwrap());
    assert_eq!(registry.get(&id).unwrap().state, TaskState::Failed);
}

#[test]
fn successful_tasks_are_terminal_and_cannot_restart() {
    let mut registry = TaskRegistry::default();
    let id = TaskId::new("task-a").unwrap();
    registry.register(task("task-a", 1));

    assert_eq!(registry.start(&id).unwrap(), 1);
    registry.succeed(&id).unwrap();
    assert_eq!(registry.get(&id).unwrap().state, TaskState::Succeeded);
    assert!(registry.start(&id).is_err());
}

#[test]
fn cancellation_is_terminal() {
    let mut registry = TaskRegistry::default();
    let id = TaskId::new("task-a").unwrap();
    registry.register(task("task-a", 1));

    registry.cancel(&id).unwrap();
    assert_eq!(registry.get(&id).unwrap().state, TaskState::Cancelled);
    assert!(registry.start(&id).is_err());
}
