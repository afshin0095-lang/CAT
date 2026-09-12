use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::{HealthReport, HealthStatus, LifecyclePhase, LifecycleState, TaskSpec};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub name: String,
    pub shutdown_timeout_ms: u64,
    pub max_concurrent_tasks: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            name: "cat-runtime".into(),
            shutdown_timeout_ms: 30_000,
            max_concurrent_tasks: 64,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("runtime is not accepting tasks in phase {0:?}")]
    NotRunning(LifecyclePhase),
    #[error("runtime task capacity exhausted")]
    CapacityExhausted,
    #[error("invalid lifecycle transition: {0}")]
    Lifecycle(#[from] crate::LifecycleError),
}

#[derive(Clone)]
pub struct Runtime {
    config: RuntimeConfig,
    lifecycle: LifecycleState,
    active_tasks: Arc<AtomicU64>,
}

impl Runtime {
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            lifecycle: LifecycleState::default(),
            active_tasks: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }
    pub fn lifecycle(&self) -> &LifecycleState {
        &self.lifecycle
    }

    pub fn start(&mut self) -> Result<(), RuntimeError> {
        self.lifecycle.transition(LifecyclePhase::Starting)?;
        self.lifecycle.transition(LifecyclePhase::Running)?;
        Ok(())
    }

    pub fn register_task(&self, _task: &TaskSpec) -> Result<u64, RuntimeError> {
        if self.lifecycle.phase() != LifecyclePhase::Running {
            return Err(RuntimeError::NotRunning(self.lifecycle.phase()));
        }
        let current = self.active_tasks.load(Ordering::Acquire);
        if current >= self.config.max_concurrent_tasks as u64 {
            return Err(RuntimeError::CapacityExhausted);
        }
        let next = self.active_tasks.fetch_add(1, Ordering::AcqRel) + 1;
        if next > self.config.max_concurrent_tasks as u64 {
            self.active_tasks.fetch_sub(1, Ordering::AcqRel);
            return Err(RuntimeError::CapacityExhausted);
        }
        Ok(next)
    }

    pub fn task_finished(&self) {
        let _ = self
            .active_tasks
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
                Some(n.saturating_sub(1))
            });
    }

    pub fn active_tasks(&self) -> u64 {
        self.active_tasks.load(Ordering::Acquire)
    }

    pub fn health(&self) -> HealthReport {
        let status = match self.lifecycle.phase() {
            LifecyclePhase::Running => HealthStatus::Ready,
            LifecyclePhase::Starting | LifecyclePhase::Draining => HealthStatus::Degraded,
            LifecyclePhase::Created => HealthStatus::Starting,
            LifecyclePhase::Stopped => HealthStatus::Stopped,
            LifecyclePhase::Failed => HealthStatus::Failed,
        };
        HealthReport {
            status,
            generation: self.lifecycle.generation(),
            active_tasks: self.active_tasks(),
        }
    }

    pub fn begin_shutdown(&mut self) -> Result<(), RuntimeError> {
        if self.lifecycle.phase() == LifecyclePhase::Running {
            self.lifecycle.transition(LifecyclePhase::Draining)?;
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), RuntimeError> {
        if self.lifecycle.phase() == LifecyclePhase::Draining {
            self.lifecycle.transition(LifecyclePhase::Stopped)?;
        }
        Ok(())
    }
}
