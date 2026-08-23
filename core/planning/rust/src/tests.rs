#[cfg(test)]
mod tests {
    use crate::{validate_plan, PlanBuilder, PlanStatus, PlanTrace, PlanTraceKind, StepKind};
    use serde_json::json;

    #[test]
    fn valid_plan_is_deterministically_accepted() {
        let mut builder = PlanBuilder::new("publish a campaign");
        let ingest = builder.step("ingest", StepKind::Action);
        let approve = builder.step("approve", StepKind::Approval);
        let builder = builder.depends_on(approve, ingest, true);
        let plan = builder.build();
        let report = validate_plan(&plan);
        assert!(report.is_valid(), "unexpected validation errors: {:?}", report.errors);
    }

    #[test]
    fn dependency_cycle_is_rejected() {
        let mut builder = PlanBuilder::new("cycle");
        let a = builder.step("a", StepKind::Action);
        let b = builder.step("b", StepKind::Action);
        builder = builder.depends_on(a, b, true).depends_on(b, a, true);
        let report = validate_plan(&builder.build());
        assert!(!report.is_valid());
    }

    #[test]
    fn trace_is_append_only_and_preserves_plan_identity() {
        let plan = PlanBuilder::new("observe").metadata("source", json!("decision-engine")).build();
        let mut trace = PlanTrace::default();
        trace.append(&plan, PlanTraceKind::Created, None, json!({"reason":"test"}));
        assert_eq!(trace.entries().len(), 1);
        assert_eq!(trace.last().unwrap().plan_id, plan.id);
        assert_eq!(trace.last().unwrap().status, PlanStatus::Draft);
    }
}
