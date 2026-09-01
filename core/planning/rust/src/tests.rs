#[cfg(test)]
mod tests {
    use crate::{schedule_plan, validate_plan, PlanBuilder, PlanStatus, PlanTrace, PlanTraceKind, StepKind};
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
    fn schedule_creates_parallel_levels() {
        let mut builder = PlanBuilder::new("publish campaign");
        let discover = builder.step("discover", StepKind::Action);
        let enrich = builder.step("enrich", StepKind::Action);
        let review = builder.step("review", StepKind::Approval);
        builder = builder.depends_on(review, discover, true).depends_on(review, enrich, true);

        let plan = builder.build();
        let schedule = schedule_plan(&plan).expect("plan should schedule");

        assert_eq!(schedule.levels().len(), 2);
        assert_eq!(schedule.levels()[0], vec![discover, enrich]);
        assert_eq!(schedule.levels()[1], vec![review]);
        assert_eq!(schedule.level_for(review), Some(1));
    }

    #[test]
    fn schedule_preserves_plan_identity() {
        let plan = PlanBuilder::new("observe").step("observe", StepKind::Observation);
        let plan = plan.build();
        let schedule = schedule_plan(&plan).expect("plan should schedule");
        assert_eq!(schedule.plan_id(), plan.id);
        assert_eq!(schedule.execution_order().collect::<Vec<_>>(), vec![plan.steps[0].id]);
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
