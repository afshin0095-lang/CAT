use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LifecyclePhase {
    Created,
    Starting,
    Running,
    Draining,
    Stopped,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LifecycleState {
    phase: LifecyclePhase,
    generation: u64,
}

impl Default for LifecycleState {
    fn default() -> Self {
        Self {
            phase: LifecyclePhase::Created,
            generation: 0,
        }
    }
}

impl LifecycleState {
    pub fn phase(&self) -> LifecyclePhase {
        self.phase
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn transition(&mut self, next: LifecyclePhase) -> Result<(), LifecycleError> {
        let valid = matches!(
            (self.phase, next),
            (LifecyclePhase::Created, LifecyclePhase::Starting)
                | (LifecyclePhase::Starting, LifecyclePhase::Running)
                | (LifecyclePhase::Running, LifecyclePhase::Draining)
                | (LifecyclePhase::Draining, LifecyclePhase::Stopped)
                | (LifecyclePhase::Starting, LifecyclePhase::Failed)
                | (LifecyclePhase::Running, LifecyclePhase::Failed)
                | (LifecyclePhase::Draining, LifecyclePhase::Failed)
                | (LifecyclePhase::Failed, LifecyclePhase::Starting)
        );
        if !valid {
            return Err(LifecycleError::InvalidTransition {
                from: self.phase,
                to: next,
            });
        }
        if matches!(next, LifecyclePhase::Starting) {
            self.generation = self.generation.saturating_add(1);
        }
        self.phase = next;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LifecycleError {
    #[error("invalid runtime lifecycle transition: {from:?} -> {to:?}")]
    InvalidTransition {
        from: LifecyclePhase,
        to: LifecyclePhase,
    },
}

impl fmt::Display for LifecyclePhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_is_monotonic_except_restart() {
        let mut state = LifecycleState::default();
        state.transition(LifecyclePhase::Starting).unwrap();
        state.transition(LifecyclePhase::Running).unwrap();
        state.transition(LifecyclePhase::Draining).unwrap();
        state.transition(LifecyclePhase::Stopped).unwrap();
        assert_eq!(state.generation(), 1);
        assert!(state.transition(LifecyclePhase::Running).is_err());
    }

    #[test]
    fn failed_runtime_can_restart_as_new_generation() {
        let mut state = LifecycleState::default();
        state.transition(LifecyclePhase::Starting).unwrap();
        state.transition(LifecyclePhase::Failed).unwrap();
        state.transition(LifecyclePhase::Starting).unwrap();
        assert_eq!(state.generation(), 2);
    }
}
