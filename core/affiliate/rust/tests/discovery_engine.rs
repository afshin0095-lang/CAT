use cat_affiliate::{DiscoveryCandidate, DiscoveryEngine, DiscoveryRequest, canonical_key};

fn candidate(external_id: &str, demand: u32, commission_bps: u32) -> DiscoveryCandidate {
    DiscoveryCandidate {
        source: "network-a".into(),
        external_id: external_id.into(),
        merchant_name: "Acme Store".into(),
        product_name: external_id.into(),
        canonical_key: canonical_key("Acme Store", external_id),
        category: Some("electronics".into()),
        destination_url: "https://example.test/product".into(),
        currency: "EUR".into(),
        price_minor: Some(12_999),
        commission_bps: Some(commission_bps),
        demand_score: demand,
        competition_score: 10_000 - demand,
        freshness_score: 9_000,
        compliance_score: 10_000,
        observed_at_ms: 1_750_000_000_000,
    }
}

#[test]
fn discovery_returns_ranked_opportunities_with_stable_tie_breaking() {
    let result = DiscoveryEngine
        .discover(DiscoveryRequest {
            candidates: vec![
                candidate("zeta", 8_000, 8_000),
                candidate("alpha", 8_000, 8_000),
                candidate("below-threshold", 1_000, 1_000),
            ],
            min_score: 7_000,
            limit: 2,
        })
        .expect("request is valid");

    assert_eq!(result.rejected, 0);
    assert_eq!(result.opportunities.len(), 2);
    assert_eq!(result.opportunities[0].candidate.external_id, "alpha");
    assert_eq!(result.opportunities[0].rank, 1);
    assert_eq!(result.opportunities[1].candidate.external_id, "zeta");
    assert_eq!(result.opportunities[1].rank, 2);
}

#[test]
fn malformed_candidates_do_not_poison_valid_candidates() {
    let mut malformed = candidate("malformed", 9_000, 9_000);
    malformed.currency.clear();

    let result = DiscoveryEngine
        .discover(DiscoveryRequest {
            candidates: vec![malformed, candidate("valid", 9_000, 9_000)],
            min_score: 0,
            limit: 10,
        })
        .expect("request is valid");

    assert_eq!(result.rejected, 1);
    assert_eq!(result.opportunities.len(), 1);
    assert_eq!(result.opportunities[0].candidate.external_id, "valid");
}
