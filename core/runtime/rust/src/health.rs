use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus { Starting, Ready, Degraded, Stopped, Failed }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HealthReport {
    pub status: HealthStatus,
    pub generation: u64,
    pub active_tasks: u64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RuntimeHealth;

impl RuntimeHealth {
    pub fn healthy(report: HealthReport) -> bool { matches!(report.status, HealthStatus::Ready) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_ready_is_healthy() {
        assert!(RuntimeHealth::healthy(HealthReport { status: HealthStatus::Ready, generation: 1, active_tasks: 0 }));
        assert!(!RuntimeHealth::healthy(HealthReport { status: HealthStatus::Degraded, generation: 1, active_tasks: 1 }));
    }
}
