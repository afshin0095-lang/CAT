use async_trait::async_trait;
use cat_orchestrator::ExecutionAuthorization;

use crate::{
    AsyncVersionedOpportunityStore, DiscoveryOpportunity, DiscoverySourceId, DiscoverySourceRequest,
    DiscoverySourceRegistry, GovernedRevalidationExecutor, OpportunityIdentity, PostgresOpportunityStore,
    RevalidationExecutionResult, RevalidationRequestRecord,
};

/// Concrete source-backed implementation of the governed revalidation contract.
///
/// Provider/network authentication, rate limiting and HTTP details stay inside
/// the registered `DiscoverySource` implementation.
pub struct DiscoveryBackedRevalidationExecutor<'a> {
    pub sources: &'a DiscoverySourceRegistry,
    pub opportunities: &'a PostgresOpportunityStore,
    pub per_page: u32,
}

impl<'a> DiscoveryBackedRevalidationExecutor<'a> {
    pub fn new(
        sources: &'a DiscoverySourceRegistry,
        opportunities: &'a PostgresOpportunityStore,
        per_page: u32,
    ) -> Self {
        Self {
            sources,
            opportunities,
            per_page: per_page.clamp(1, 100),
        }
    }
}

#[async_trait]
impl<'a> GovernedRevalidationExecutor for DiscoveryBackedRevalidationExecutor<'a> {
    async fn revalidate(
        &self,
        request: &RevalidationRequestRecord,
        authorization: &ExecutionAuthorization,
    ) -> Result<RevalidationExecutionResult, String> {
        if authorization.capability_id().as_str() != crate::REVALIDATION_CAPABILITY_ID {
            return Err("revalidation executor received an unauthorized capability".into());
        }
        if authorization.tenant_id().as_uuid().is_nil() {
            return Err("revalidation authorization has no tenant scope".into());
        }

        let source_id = DiscoverySourceId(request.request.target.source.clone());
        let source = self
            .sources
            .get(&source_id)
            .ok_or_else(|| format!("revalidation source is not registered: {}", source_id.0))?;

        let batch = source
            .discover(DiscoverySourceRequest {
                query: Some(request.request.target.identity.clone()),
                page: 1,
                per_page: self.per_page,
                category: None,
                geographic_market: None,
                currency: None,
            })
            .await
            .map_err(|error| error.to_string())?;

        let candidate = find_matching_candidate(
            &batch.candidates,
            &request.request.target.identity,
            &request.request.target.source,
        )
        .ok_or_else(|| "revalidation source returned no matching opportunity".to_owned())?;

        candidate
            .validate()
            .map_err(|error| format!("revalidation candidate is invalid: {error}"))?;

        let identity = OpportunityIdentity::new(candidate);
        if identity.as_str() != request.request.target.identity {
            return Err("revalidation candidate identity does not match request identity".into());
        }

        let current = self
            .opportunities
            .get(&identity)
            .await
            .map_err(|error| error.to_string())?;

        let opportunity = DiscoveryOpportunity {
            id: current.id,
            candidate: candidate.clone(),
            score: candidate.opportunity_score(),
            rank: 1,
        };

        let upsert = self
            .opportunities
            .upsert_if_revision(&opportunity, current.revision)
            .await
            .map_err(|error| error.to_string())?;

        Ok(RevalidationExecutionResult {
            observed_at_ms: candidate.observed_at_ms,
            revision: upsert.revision.get(),
        })
    }
}

fn find_matching_candidate<'a>(
    candidates: &'a [crate::DiscoveryCandidate],
    identity: &str,
    source: &str,
) -> Option<&'a crate::DiscoveryCandidate> {
    candidates.iter().find(|candidate| {
        candidate.source == source
            && candidate.canonical_key == identity
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(source: &str, identity: &str) -> crate::DiscoveryCandidate {
        crate::DiscoveryCandidate {
            source: source.into(),
            external_id: "external-1".into(),
            merchant_name: "Acme".into(),
            product_name: "Widget".into(),
            canonical_key: identity.into(),
            category: None,
            destination_url: "https://example.com/widget".into(),
            currency: "USD".into(),
            price_minor: Some(100),
            commission_bps: Some(500),
            demand_score: 5000,
            competition_score: 3000,
            freshness_score: 9000,
            compliance_score: 9000,
            observed_at_ms: 2_000,
        }
    }

    #[test]
    fn matching_candidate_is_source_and_identity_scoped() {
        let candidates = vec![
            candidate("network-a", "acme:widget"),
            candidate("network-b", "acme:widget"),
        ];
        assert_eq!(
            find_matching_candidate(&candidates, "acme:widget", "network-b")
                .expect("network-b candidate")
                .source,
            "network-b"
        );
    }

    #[test]
    fn missing_match_returns_none() {
        let candidates = vec![candidate("network-a", "acme:widget")];
        assert!(find_matching_candidate(&candidates, "acme:other", "network-a").is_none());
        assert!(find_matching_candidate(&candidates, "acme:widget", "network-b").is_none());
    }
}