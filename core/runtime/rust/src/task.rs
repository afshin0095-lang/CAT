use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TaskId(pub u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TaskKind { EventDispatch, Projection, Scheduled, Maintenance }

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TaskSpec {
    pub id: TaskId,
    pub kind: TaskKind,
    pub name: String,
    pub critical: bool,
}

impl TaskSpec {
    pub fn new(id: u64, kind: TaskKind, name: impl Into<String>, critical: bool) -> Self {
        Self { id: TaskId(id), kind, name: name.into(), critical }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_identity_is_stable() {
        let task = TaskSpec::new(7, TaskKind::EventDispatch, "events", true);
        assert_eq!(task.id, TaskId(7));
        assert!(task.critical);
        assert_eq!(task.name, "events");
    }
}
