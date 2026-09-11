use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthState {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub state: HealthState,
    pub detail: String,
}

impl ComponentHealth {
    pub fn healthy(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            state: HealthState::Healthy,
            detail: detail.into(),
        }
    }

    pub fn degraded(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            state: HealthState::Degraded,
            detail: detail.into(),
        }
    }

    pub fn unhealthy(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            state: HealthState::Unhealthy,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlatformHealthSnapshot {
    pub state: HealthState,
    pub queue_depth: usize,
    pub components: Vec<ComponentHealth>,
}

impl PlatformHealthSnapshot {
    pub fn from_queue_depth(queue_depth: usize) -> Self {
        let state = if queue_depth == 0 {
            HealthState::Healthy
        } else {
            HealthState::Degraded
        };

        let detail = if queue_depth == 0 {
            "scheduler queue is empty"
        } else {
            "scheduler has queued work"
        };

        Self {
            state,
            queue_depth,
            components: vec![ComponentHealth {
                name: "scheduler".into(),
                state,
                detail: detail.into(),
            }],
        }
    }

    pub fn is_ready(&self) -> bool {
        self.state != HealthState::Unhealthy
            && self
                .components
                .iter()
                .all(|component| component.state != HealthState::Unhealthy)
    }
}
