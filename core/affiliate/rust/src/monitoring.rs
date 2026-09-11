use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitoringInput {
    pub report_date: String,
    pub links_checked: u32,
    pub links_failed: u32,
    pub clicks: Option<u64>,
    pub conversions: Option<u64>,
    pub commissions: Option<f64>,
    pub revenue: Option<f64>,
    pub previous_revenue: Option<f64>,
    pub duplicate_commissions: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FindingSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitoringFinding {
    pub code: String,
    pub severity: FindingSeverity,
    pub message: String,
    pub evidence: String,
    pub recommended_action: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DailyMonitoringReport {
    pub report_date: String,
    pub status: String,
    pub findings: Vec<MonitoringFinding>,
    pub data_quality: String,
}

pub fn build_daily_report(input: &MonitoringInput) -> DailyMonitoringReport {
    let mut findings = Vec::new();
    if input.links_checked > 0 && input.links_failed * 20 >= input.links_checked {
        findings.push(MonitoringFinding {
            code: "link_failure_rate".into(),
            severity: FindingSeverity::Critical,
            message: "At least 5% of checked links failed".into(),
            evidence: format!(
                "{}/{} links failed",
                input.links_failed, input.links_checked
            ),
            recommended_action: "Quarantine unhealthy routes and run reconciliation before replay"
                .into(),
        });
    } else if input.links_failed > 0 {
        findings.push(MonitoringFinding {
            code: "link_failures".into(),
            severity: FindingSeverity::Warning,
            message: "Some affiliate links failed health checks".into(),
            evidence: format!(
                "{}/{} links failed",
                input.links_failed, input.links_checked
            ),
            recommended_action: "Inspect failed destinations and provider status".into(),
        });
    }
    if input.duplicate_commissions > 0 {
        findings.push(MonitoringFinding {
            code: "duplicate_commission".into(),
            severity: FindingSeverity::Critical,
            message: "Duplicate commission obligations detected".into(),
            evidence: format!("{} duplicates", input.duplicate_commissions),
            recommended_action: "Freeze payout effects and reconcile source events".into(),
        });
    }
    let data_quality = if input.clicks.is_none() || input.revenue.is_none() {
        "not_measured"
    } else {
        "measured"
    };
    if data_quality == "not_measured" {
        findings.push(MonitoringFinding {
            code: "missing_telemetry".into(),
            severity: FindingSeverity::Warning,
            message: "Live click or revenue telemetry is missing".into(),
            evidence: "One or more required metrics were absent".into(),
            recommended_action: "Connect event-store projections before interpreting revenue"
                .into(),
        });
    }
    if let (Some(previous), Some(current)) = (input.previous_revenue, input.revenue) {
        if previous > 0.0 && current < previous * 0.7 {
            findings.push(MonitoringFinding { code: "revenue_drop".into(), severity: FindingSeverity::Critical, message: "Revenue dropped more than 30% versus the previous snapshot".into(), evidence: format!("previous={previous}, current={current}"), recommended_action: "Check ingestion, attribution, provider health, and routing before changing strategy".into() });
        }
    }
    let status = if findings
        .iter()
        .any(|f| f.severity == FindingSeverity::Critical)
    {
        "critical"
    } else if findings
        .iter()
        .any(|f| f.severity == FindingSeverity::Warning)
    {
        "warning"
    } else {
        "healthy"
    };
    DailyMonitoringReport {
        report_date: input.report_date.clone(),
        status: status.into(),
        findings,
        data_quality: data_quality.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn base() -> MonitoringInput {
        MonitoringInput {
            report_date: "2026-09-08".into(),
            links_checked: 100,
            links_failed: 0,
            clicks: Some(1000),
            conversions: Some(40),
            commissions: Some(120.0),
            revenue: Some(400.0),
            previous_revenue: Some(420.0),
            duplicate_commissions: 0,
        }
    }
    #[test]
    fn detects_link_outage() {
        let mut i = base();
        i.links_failed = 10;
        assert_eq!(build_daily_report(&i).status, "critical");
    }
    #[test]
    fn missing_revenue_is_not_zero() {
        let mut i = base();
        i.revenue = None;
        let r = build_daily_report(&i);
        assert_eq!(r.data_quality, "not_measured");
        assert!(r.findings.iter().any(|f| f.code == "missing_telemetry"));
    }
    #[test]
    fn detects_revenue_drop() {
        let mut i = base();
        i.revenue = Some(200.0);
        assert!(
            build_daily_report(&i)
                .findings
                .iter()
                .any(|f| f.code == "revenue_drop")
        );
    }
}
