use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MonitoringRunState { Pending, Collecting, Evaluating, ReportReady, Failed }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DailyMonitoringRun {
    pub run_id: Uuid,
    pub report_date: String,
    pub idempotency_key: String,
    pub state: MonitoringRunState,
    pub revision: u64,
    pub report_event_id: Option<Uuid>,
    pub failure_reason: Option<String>,
}

impl DailyMonitoringRun {
    pub fn start(report_date: impl Into<String>) -> Self {
        let report_date = report_date.into();
        Self { run_id: Uuid::now_v7(), idempotency_key: format!("revenue-radar:{report_date}"), report_date, state: MonitoringRunState::Pending, revision: 0, report_event_id: None, failure_reason: None }
    }

    pub fn begin_collection(&mut self) -> Result<(), String> {
        self.transition(MonitoringRunState::Pending, MonitoringRunState::Collecting)
    }

    pub fn begin_evaluation(&mut self) -> Result<(), String> {
        self.transition(MonitoringRunState::Collecting, MonitoringRunState::Evaluating)
    }

    pub fn mark_report_ready(&mut self, event_id: Uuid) -> Result<(), String> {
        if self.state != MonitoringRunState::Evaluating { return Err("monitoring run is not evaluating".into()); }
        self.report_event_id = Some(event_id);
        self.state = MonitoringRunState::ReportReady;
        self.revision += 1;
        Ok(())
    }

    pub fn fail(&mut self, reason: impl Into<String>) -> Result<(), String> {
        if self.state == MonitoringRunState::ReportReady || self.state == MonitoringRunState::Failed { return Err("monitoring run is already terminal".into()); }
        self.failure_reason = Some(reason.into());
        self.state = MonitoringRunState::Failed;
        self.revision += 1;
        Ok(())
    }

    pub fn is_terminal(&self) -> bool { matches!(self.state, MonitoringRunState::ReportReady | MonitoringRunState::Failed) }

    fn transition(&mut self, from: MonitoringRunState, to: MonitoringRunState) -> Result<(), String> {
        if self.state != from { return Err(format!("invalid monitoring transition: {:?} -> {:?}", self.state, to)); }
        self.state = to;
        self.revision += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daily_run_has_stable_idempotency_key() {
        let a = DailyMonitoringRun::start("2026-09-08");
        let b = DailyMonitoringRun::start("2026-09-08");
        assert_eq!(a.idempotency_key, b.idempotency_key);
        assert_ne!(a.run_id, b.run_id);
    }

    #[test]
    fn report_lifecycle_is_ordered() {
        let mut run = DailyMonitoringRun::start("2026-09-08");
        run.begin_collection().unwrap();
        run.begin_evaluation().unwrap();
        run.mark_report_ready(Uuid::now_v7()).unwrap();
        assert!(run.is_terminal());
        assert!(run.begin_collection().is_err());
    }

    #[test]
    fn failure_is_terminal_and_preserves_reason() {
        let mut run = DailyMonitoringRun::start("2026-09-08");
        run.begin_collection().unwrap();
        run.fail("event store unavailable").unwrap();
        assert_eq!(run.state, MonitoringRunState::Failed);
        assert_eq!(run.failure_reason.as_deref(), Some("event store unavailable"));
        assert!(run.fail("again").is_err());
    }
}
