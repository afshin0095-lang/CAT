//! Deterministic revalidation planning.
//!
//! Inputs: an `OpportunityRecord`, its freshness evaluation, the current
//! timestamp, the freshness policy, and known source availability.
//! Output: zero or more `RevalidationRequest`s.
//!
//! Rules (all deterministic, side-effect free):
//! - Active opportunities within the freshness target never get requests;
//! - Active opportunities past the freshness target get a `PeriodicRefresh`
//!   (Low priority) so observation pipelines stay ahead of staleness;
//! - Stale opportunities get a `Stale` request (High priority) against the
//!   current best source;
//! - Expired opportunities get `Expired` requests (Critical priority)
//!   against *every* known available source: expired data requires
//!   independent confirmation before it becomes commercially usable again;
//! - expired opportunities are never deleted and records are never mutated;
//! - unavailable sources are skipped explicitly, never silently;
//! - if nothing can be targeted, the decision records why it was blocked.

use std::collections::{BTreeMap, BTreeSet};

use crate::opportunity_freshness::{FreshnessEvaluation, FreshnessPolicy, FreshnessPolicyError, FreshnessState};
use crate::opportunity_revalidation::{
    RevalidationBlockReason, RevalidationDecision, RevalidationPriority, RevalidationReason, RevalidationRequest,
    RevalidationSkipReason, RevalidationTarget, REVALIDATION_DEDUP_WINDOW_MS,
};
use crate::OpportunityRecord;

#[derive(Clone, Debug)]
pub struct RevalidationPlanner {
    policy: FreshnessPolicy,
    dedup_window_ms: u64,
}

impl RevalidationPlanner {
    pub fn new(policy: FreshnessPolicy) -> Self {
        Self {
            policy,
            dedup_window_ms: REVALIDATION_DEDUP_WINDOW_MS,
        }
    }

    /// `dedup_window_ms == 0` disables window bucketing (each plan produces a
    /// distinct dedup key).
    pub fn with_dedup_window(policy: FreshnessPolicy, dedup_window_ms: u64) -> Self {
        Self {
            policy,
            dedup_window_ms,
        }
    }

    pub fn policy(&self) -> FreshnessPolicy {
        self.policy
    }

    /// Convenience wrapper that derives freshness from the record itself.
    pub fn plan_for_record(
        &self,
        record: &OpportunityRecord,
        now_ms: u64,
        available_sources: &[String],
    ) -> Result<RevalidationDecision, FreshnessPolicyError> {
        let evaluation = self.policy.evaluate(record, now_ms)?;
        Ok(self.plan(record, evaluation.evaluation, available_sources, now_ms))
    }

    /// Deterministic plan for one opportunity. `available_sources` is the set
    /// of sources currently reachable; sources missing from it are skipped
    /// explicitly (recorded in the decision, never silently ignored).
    pub fn plan(
        &self,
        record: &OpportunityRecord,
        freshness: FreshnessEvaluation,
        available_sources: &[String],
        now_ms: u64,
    ) -> RevalidationDecision {
        let available: BTreeSet<&str> = available_sources.iter().map(|source| source.as_str()).collect();
        let known: BTreeSet<&str> = record.observations.keys().map(|source| source.as_str()).collect();

        let mut requests = Vec::new();
        let mut skipped = Vec::new();
        let mut blocked = None;

        if known.is_empty() {
            blocked = Some(RevalidationBlockReason::NoKnownSources);
            return Self::decision(record, now_ms, requests, skipped, blocked);
        }

        match freshness.state {
            FreshnessState::Active if freshness.is_fresh => {
                // Within the freshness target: nothing to do.
            }
            FreshnessState::Active => {
                let candidate = record.best_source.as_str();
                if available.contains(candidate) {
                    requests.push(self.request(
                        record,
                        candidate,
                        RevalidationReason::PeriodicRefresh,
                        RevalidationPriority::Low,
                        now_ms,
                        record
                            .last_observed_at_ms
                            .saturating_add(self.policy.active_threshold_ms)
                            .max(now_ms),
                    ));
                } else {
                    skipped.push((candidate.to_owned(), RevalidationSkipReason::SourceUnavailable));
                }
            }
            FreshnessState::Stale => {
                let candidate = record.best_source.as_str();
                if available.contains(candidate) {
                    requests.push(self.request(
                        record,
                        candidate,
                        RevalidationReason::Stale,
                        RevalidationPriority::High,
                        now_ms,
                        now_ms,
                    ));
                } else {
                    skipped.push((candidate.to_owned(), RevalidationSkipReason::SourceUnavailable));
                }
                if known.iter().all(|source| !available.contains(source)) {
                    blocked = Some(RevalidationBlockReason::AllSourcesUnavailable);
                }
            }
            FreshnessState::Expired => {
                let mut any_available = false;
                for source in &known {
                    if available.contains(source) {
                        any_available = true;
                        requests.push(self.request(
                            record,
                            source,
                            RevalidationReason::Expired,
                            RevalidationPriority::Critical,
                            now_ms,
                            now_ms,
                        ));
                    } else {
                        skipped.push(((*source).to_owned(), RevalidationSkipReason::SourceUnavailable));
                    }
                }
                if !any_available {
                    blocked = Some(RevalidationBlockReason::AllSourcesUnavailable);
                }
            }
        }

        Self::decision(record, now_ms, requests, skipped, blocked)
    }

    fn request(
        &self,
        record: &OpportunityRecord,
        source: &str,
        reason: RevalidationReason,
        priority: RevalidationPriority,
        requested_at_ms: u64,
        scheduled_for_ms: u64,
    ) -> RevalidationRequest {
        RevalidationRequest::with_dedup_window(
            record.id,
            RevalidationTarget::new(record.identity.as_str(), source),
            reason,
            priority,
            requested_at_ms,
            scheduled_for_ms,
            self.dedup_window_ms,
        )
    }

    fn decision(
        record: &OpportunityRecord,
        now_ms: u64,
        mut requests: Vec<RevalidationRequest>,
        mut skipped: Vec<(String, RevalidationSkipReason)>,
        blocked: Option<RevalidationBlockReason>,
    ) -> RevalidationDecision {
        // Deterministic ordering that never depends on HashMap iteration.
        requests.sort_by(|left, right| {
            right
                .priority
                .cmp(&left.priority)
                .then_with(|| left.target.identity.cmp(&right.target.identity))
                .then_with(|| left.target.source.cmp(&right.target.source))
                .then_with(|| left.reason.as_str().cmp(right.reason.as_str()))
        });
        skipped.sort_by(|left, right| left.0.cmp(&right.0));
        RevalidationDecision {
            identity: record.identity.as_str().to_owned(),
            opportunity_id: record.id,
            evaluated_at_ms: now_ms,
            requests,
            skipped,
            blocked,
        }
    }
}

/// Priority-ordered view over planned decisions across many opportunities.
#[derive(Clone, Debug, Default)]
pub struct RevalidationPlanningOutcome {
    pub decisions: Vec<RevalidationDecision>,
    pub requested: u64,
    pub skipped: u64,
    pub blocked: u64,
}

impl RevalidationPlanningOutcome {
    /// Plans a batch deterministically. Records are processed in identity
    /// order so output ordering is stable regardless of input order.
    pub fn plan_batch(
        planner: &RevalidationPlanner,
        records: &[OpportunityRecord],
        now_ms: u64,
        source_availability: &BTreeMap<String, bool>,
    ) -> Result<RevalidationPlanningOutcome, FreshnessPolicyError> {
        let mut records_by_identity: Vec<&OpportunityRecord> = records.iter().collect();
        records_by_identity.sort_by(|left, right| left.identity.cmp(&right.identity));

        let mut outcome = RevalidationPlanningOutcome::default();
        let available: Vec<String> = source_availability
            .iter()
            .filter(|(_, reachable)| **reachable)
            .map(|(source, _)| source.clone())
            .collect();
        for record in records_by_identity {
            let decision = planner.plan_for_record(record, now_ms, &available)?;
            outcome.requested += decision.requests.len() as u64;
            outcome.skipped += decision.skipped.len() as u64;
            if decision.blocked.is_some() {
                outcome.blocked += 1;
            }
            outcome.decisions.push(decision);
        }
        Ok(outcome)
    }

    /// Every produced request, globally deduplicated by `dedup_key`
    /// (highest priority occurrence wins; keys are deterministic so ties are
    /// resolved by construction).
    pub fn unique_requests(&self) -> Vec<&RevalidationRequest> {
        let mut seen = BTreeSet::new();
        let mut unique = Vec::new();
        for decision in &self.decisions {
            for request in &decision.requests {
                if seen.insert(request.dedup_key.clone()) {
                    unique.push(request);
                }
            }
        }
        unique
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{canonical_key, DiscoveryCandidate, DiscoveryOpportunity, InMemoryOpportunityStore, OpportunityStore};
    use uuid::Uuid;

    fn record(observed_at_ms: u64, sources: &[&str]) -> OpportunityRecord {
        record_for_product(observed_at_ms, "Widget", sources)
    }

    fn record_for_product(observed_at_ms: u64, product: &str, sources: &[&str]) -> OpportunityRecord {
        let mut store = InMemoryOpportunityStore::new();
        for (index, source) in sources.iter().enumerate() {
            let candidate = DiscoveryCandidate {
                source: (*source).into(),
                external_id: format!("sku-{index}"),
                merchant_name: "Acme".into(),
                product_name: product.into(),
                canonical_key: canonical_key("Acme", product),
                category: Some("electronics".into()),
                destination_url: "https://example.test/widget".into(),
                currency: "EUR".into(),
                price_minor: Some(10_000),
                commission_bps: Some(500),
                demand_score: 8_000,
                competition_score: 2_000,
                freshness_score: 10_000,
                compliance_score: 10_000,
                observed_at_ms,
            };
            let opportunity = DiscoveryOpportunity { id: Uuid::now_v7(), candidate, score: 8_000, rank: 1 };
            store.upsert(opportunity).expect("valid opportunity");
        }
        store.list().into_iter().next().expect("record exists")
    }

    fn planner() -> RevalidationPlanner {
        RevalidationPlanner::new(FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy"))
    }

    fn evaluation(policy: &FreshnessPolicy, record: &OpportunityRecord, now_ms: u64) -> FreshnessEvaluation {
        policy.evaluate(record, now_ms).expect("valid evaluation").evaluation
    }

    #[test]
    fn active_and_fresh_records_need_no_revalidation() {
        let planner = planner();
        let stored = record(10_000, &["network-a"]);
        let now = 10_500;
        let decision = planner.plan(
            &stored,
            evaluation(&planner.policy(), &stored, now),
            &["network-a".into()],
            now,
        );
        assert!(!decision.requires_revalidation());
        assert!(decision.requests.is_empty());
        assert!(decision.blocked.is_none());
    }

    #[test]
    fn active_but_aging_records_get_a_low_priority_periodic_refresh() {
        let planner = planner();
        let stored = record(10_000, &["network-a"]);
        let now = 11_500; // past the 1_000ms active threshold, still Active
        let decision = planner.plan(
            &stored,
            evaluation(&planner.policy(), &stored, now),
            &["network-a".into()],
            now,
        );
        assert_eq!(decision.requests.len(), 1);
        let request = &decision.requests[0];
        assert_eq!(request.reason, RevalidationReason::PeriodicRefresh);
        assert_eq!(request.priority, RevalidationPriority::Low);
        assert_eq!(request.target.source, "network-a");
        assert_eq!(request.scheduled_for_ms, 11_500, "refresh already overdue: scheduled immediately");
    }

    #[test]
    fn stale_records_request_high_priority_refresh_of_best_source() {
        let planner = planner();
        let stored = record(10_000, &["network-a", "network-b"]);
        let now = 13_000;
        let decision = planner.plan(
            &stored,
            evaluation(&planner.policy(), &stored, now),
            &["network-a".into()],
            now,
        );
        assert_eq!(decision.requests.len(), 1);
        let request = &decision.requests[0];
        assert_eq!(request.reason, RevalidationReason::Stale);
        assert_eq!(request.priority, RevalidationPriority::High);
        assert_eq!(request.scheduled_for_ms, now);
    }

    #[test]
    fn stale_records_with_unavailable_best_source_skip_explicitly() {
        let planner = planner();
        let stored = record(10_000, &["network-a"]);
        let now = 13_000;
        let decision = planner.plan(&stored, evaluation(&planner.policy(), &stored, now), &[], now);
        assert!(decision.requests.is_empty());
        assert_eq!(
            decision.skipped,
            vec![("network-a".to_owned(), RevalidationSkipReason::SourceUnavailable)]
        );
        assert_eq!(decision.blocked, Some(RevalidationBlockReason::AllSourcesUnavailable));
    }

    #[test]
    fn expired_records_request_critical_refresh_from_every_available_source() {
        let planner = planner();
        let stored = record(10_000, &["network-b", "network-a", "network-c"]);
        let now = 20_000;
        let decision = planner.plan(
            &stored,
            evaluation(&planner.policy(), &stored, now),
            &["network-a".into(), "network-c".into()],
            now,
        );
        assert_eq!(decision.requests.len(), 2, "one request per available source");
        for request in &decision.requests {
            assert_eq!(request.reason, RevalidationReason::Expired);
            assert_eq!(request.priority, RevalidationPriority::Critical);
            assert_eq!(request.scheduled_for_ms, now);
        }
        let sources: Vec<&str> = decision
            .requests
            .iter()
            .map(|request| request.target.source.as_str())
            .collect();
        assert_eq!(sources, vec!["network-a", "network-c"], "deterministic source order");
        assert_eq!(
            decision.skipped,
            vec![("network-b".to_owned(), RevalidationSkipReason::SourceUnavailable)]
        );
        assert!(decision.blocked.is_none());
    }

    #[test]
    fn expired_records_with_every_source_down_are_blocked_explicitly() {
        let planner = planner();
        let stored = record(10_000, &["network-a"]);
        let now = 20_000;
        let decision = planner.plan(&stored, evaluation(&planner.policy(), &stored, now), &[], now);
        assert!(decision.requests.is_empty());
        assert_eq!(decision.blocked, Some(RevalidationBlockReason::AllSourcesUnavailable));
    }

    #[test]
    fn records_without_observations_are_blocked() {
        let planner = planner();
        let mut stored = record(10_000, &["network-a"]);
        stored.observations.clear();
        let decision = planner.plan(
            &stored,
            FreshnessEvaluation { state: FreshnessState::Active, age_ms: 0, is_fresh: true },
            &[],
            10_000,
        );
        assert!(decision.requests.is_empty());
        assert_eq!(decision.blocked, Some(RevalidationBlockReason::NoKnownSources));
    }

    #[test]
    fn planning_is_deterministic_and_never_mutates_the_record() {
        let planner = planner();
        let stored = record(10_000, &["network-a", "network-b"]);
        let before = stored.clone();
        let now = 20_000;
        let first = planner.plan_for_record(&stored, now, &["network-a".into(), "network-b".into()])
            .expect("valid evaluation");
        let second = planner.plan_for_record(&stored, now, &["network-a".into(), "network-b".into()])
            .expect("valid evaluation");
        assert_eq!(stored, before, "records are never mutated");
        assert_eq!(first.identity, second.identity);
        assert_eq!(first.requests.len(), second.requests.len());
        let first_keys: Vec<&str> = first.requests.iter().map(|r| r.dedup_key.as_str()).collect();
        let second_keys: Vec<&str> = second.requests.iter().map(|r| r.dedup_key.as_str()).collect();
        assert_eq!(first_keys, second_keys);
    }

    #[test]
    fn batch_outcome_is_identity_ordered_and_deduplicated() {
        let planner = planner();
        let mut availability = BTreeMap::new();
        availability.insert("network-a".to_string(), true);
        let zeta = record_for_product(10_000, "Zeta Gadget", &["network-a"]);
        let alpha = record_for_product(10_000, "Alpha Widget", &["network-a"]);

        let outcome = RevalidationPlanningOutcome::plan_batch(&planner, &[zeta, alpha], 20_000, &availability)
            .expect("valid batch");
        assert_eq!(outcome.decisions.len(), 2);
        assert!(outcome.decisions[0].identity < outcome.decisions[1].identity);
        // Distinct identities produce distinct dedup keys, so both survive.
        assert_eq!(outcome.unique_requests().len(), 2);
        assert_eq!(outcome.requested, 2);
    }
}
