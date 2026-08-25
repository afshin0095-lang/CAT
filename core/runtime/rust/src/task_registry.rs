use std::collections::BTreeMap;
use std::fmt;

/// Stable runtime task identifier.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct TaskId(String);

impl TaskId {
    pub fn new(value: impl Into<String>) -> Result<Self, TaskIdError> {
        let value = value.into();
        if value.is_empty() || value.len() > 128 {
            return Err(TaskIdError::InvalidLength);
        }
        if !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        {
            return Err(TaskIdError::InvalidCharacters);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskState {
    Registered,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl TaskState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Succeeded | Self::Failed | Self::Cancelled)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskSpec {
    pub id: TaskId,
    pub name: String,
    pub max_attempts: u32,
}

impl TaskSpec {
    pub fn new(id: TaskId, name: impl Into<String>, max_attempts: u32) -> Result<Self, TaskSpecError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(TaskSpecError::EmptyName);
        }
        if max_attempts == 0 {
            return Err(TaskSpecError::ZeroAttempts);
        }
        Ok(Self { id, name, max_attempts })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TaskIdError {
    InvalidLength,
    InvalidCharacters,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TaskSpecError {
    EmptyName,
    ZeroAttempts,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskRecord {
    pub spec: TaskSpec,
    pub state: TaskState,
    pub attempt: u32,
}

#[derive(Clone, Debug, Default)]
pub struct TaskRegistry {
    tasks: BTreeMap<TaskId, TaskRecord>,
}

impl TaskRegistry {
    pub fn register(&mut self, spec: TaskSpec) -> bool {
        let id = spec.id.clone();
        self.tasks
            .insert(
                id,
                TaskRecord {
                    spec,
                    state: TaskState::Registered,
                    attempt: 0,
                },
            )
            .is_none()
    }

    pub fn get(&self, id: &TaskId) -> Option<&TaskRecord> {
        self.tasks.get(id)
    }

    pub fn start(&mut self, id: &TaskId) -> Result<u32, TaskTransitionError> {
        let record = self.tasks.get_mut(id).ok_or(TaskTransitionError::UnknownTask)?;
        if record.state.is_terminal() {
            return Err(TaskTransitionError::TerminalState);
        }
        if record.attempt >= record.spec.max_attempts {
            return Err(TaskTransitionError::RetryLimitReached);
        }
        record.attempt += 1;
        record.state = TaskState::Running;
        Ok(record.attempt)
    }

    pub fn succeed(&mut self, id: &TaskId) -> Result<(), TaskTransitionError> {
        let record = self.tasks.get_mut(id).ok_or(TaskTransitionError::UnknownTask)?;
        if record.state != TaskState::Running {
            return Err(TaskTransitionError::InvalidTransition);
        }
        record.state = TaskState::Succeeded;
        Ok(())
    }

    pub fn fail(&mut self, id: &TaskId) -> Result<bool, TaskTransitionError> {
        let record = self.tasks.get_mut(id).ok_or(TaskTransitionError::UnknownTask)?;
        if record.state != TaskState::Running {
            return Err(TaskTransitionError::InvalidTransition);
        }
        record.state = if record.attempt >= record.spec.max_attempts {
            TaskState::Failed
        } else {
            TaskState::Registered
        };
        Ok(record.state == TaskState::Registered)
    }

    pub fn cancel(&mut self, id: &TaskId) -> Result<(), TaskTransitionError> {
        let record = self.tasks.get_mut(id).ok_or(TaskTransitionError::UnknownTask)?;
        if record.state.is_terminal() {
            return Err(TaskTransitionError::TerminalState);
        }
        record.state = TaskState::Cancelled;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TaskTransitionError {
    UnknownTask,
    InvalidTransition,
    RetryLimitReached,
    TerminalState,
}
