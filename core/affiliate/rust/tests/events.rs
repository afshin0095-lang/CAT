use cat_affiliate::{
    AffiliateRegistered, CommissionObligationCreated, ConversionStateChanged, MerchantCreated,
    OfferCreated, OpportunityBecameExpired, OpportunityBecameStale, OpportunityDiscovered,
    OpportunityRevalidated, OpportunityRevalidationFailed, OpportunityRevalidationRequested,
    OpportunityUpdated, ProgramCreated, ProgramStatusChanged, ReferralStateChanged,
    RevalidationPriority, RevalidationReason,
};
use cat_eventbus::CatEvent;
use std::collections::HashSet;
use uuid::Uuid;

#[test]
fn affiliate_event_types_are_globally_unique() {
    let types = [
        MerchantCreated::TYPE,
        ProgramCreated::TYPE,
        ProgramStatusChanged::TYPE,
        OfferCreated::TYPE,
        AffiliateRegistered::TYPE,
        ReferralStateChanged::TYPE,
        ConversionStateChanged::TYPE,
        CommissionObligationCreated::TYPE,
        OpportunityDiscovered::TYPE,
        OpportunityUpdated::TYPE,
        OpportunityBecameStale::TYPE,
        OpportunityBecameExpired::TYPE,
        OpportunityRevalidationRequested::TYPE,
        OpportunityRevalidated::TYPE,
        OpportunityRevalidationFailed::TYPE,
    ];

    let unique = types.iter().copied().collect::<HashSet<_>>();
    assert_eq!(unique.len(), types.len());
    assert!(types.iter().all(|event_type| event_type.starts_with("affiliate.")));
}

fn identity() -> String {
    "acme:widget".to_owned()
}

#[test]
fn opportunity_lifecycle_events_round_trip_with_identity_and_time() {
    let discovered = OpportunityDiscovered {
        opportunity_id: Uuid::now_v7(),
        identity: identity(),
        source: "network-a".into(),
        best_score: 8_000,
        observed_at_ms: 1_000,
        revision: 1,
    };
    let encoded = serde_json::to_string(&discovered).expect("serialize");
    let decoded: OpportunityDiscovered = serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded, discovered);
    assert_eq!(decoded.identity, "acme:widget");
    assert_eq!(decoded.observed_at_ms, 1_000);
    assert_eq!(decoded.revision, 1);

    let became_stale = OpportunityBecameStale {
        opportunity_id: Uuid::now_v7(),
        identity: identity(),
        last_observed_at_ms: 1_000,
        stale_threshold_ms: 5_000,
        detected_at_ms: 6_000,
    };
    let encoded = serde_json::to_string(&became_stale).expect("serialize");
    let decoded: OpportunityBecameStale = serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded, became_stale);

    let became_expired = OpportunityBecameExpired {
        opportunity_id: Uuid::now_v7(),
        identity: identity(),
        last_observed_at_ms: 1_000,
        expiration_threshold_ms: 9_000,
        detected_at_ms: 10_000,
    };
    let encoded = serde_json::to_string(&became_expired).expect("serialize");
    let decoded: OpportunityBecameExpired = serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded, became_expired);
}

#[test]
fn opportunity_revalidation_events_carry_request_identity() {
    let requested = OpportunityRevalidationRequested {
        request_id: Uuid::now_v7(),
        opportunity_id: Uuid::now_v7(),
        identity: identity(),
        source: "network-a".into(),
        reason: RevalidationReason::Stale,
        priority: RevalidationPriority::High,
        requested_at_ms: 1_000,
        scheduled_for_ms: 2_000,
    };
    let envelope = requested.clone().into_envelope("affiliate-domain").expect("envelope");
    assert_eq!(envelope.event_type, "affiliate.opportunity.revalidation_requested");
    assert_eq!(envelope.version, 1);
    assert_eq!(envelope.payload["reason"], "stale");
    assert_eq!(envelope.payload["priority"], "high");

    let encoded = serde_json::to_string(&requested).expect("serialize");
    let decoded: OpportunityRevalidationRequested = serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded.request_id, requested.request_id);
    assert_eq!(decoded.reason, RevalidationReason::Stale);

    let revalidated = OpportunityRevalidated {
        request_id: Uuid::now_v7(),
        opportunity_id: Uuid::now_v7(),
        identity: identity(),
        source: "network-a".into(),
        observed_at_ms: 5_000,
        revision: 3,
    };
    let encoded = serde_json::to_string(&revalidated).expect("serialize");
    let decoded: OpportunityRevalidated = serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded, revalidated);
    assert_eq!(decoded.revision, 3);
}

#[test]
fn revalidation_failure_details_are_bounded_and_sanitized() {
    let oversized = format!("boom\n{}", "x".repeat(2_000));
    let failed = OpportunityRevalidationFailed::new(
        Uuid::now_v7(),
        Uuid::now_v7(),
        identity(),
        "network-a".into(),
        RevalidationReason::ProviderUnavailable,
        9_000,
        oversized,
    );
    assert!(failed.detail.len() <= 512, "detail is bounded");
    assert!(!failed.detail.contains('\n'), "control characters are sanitized");
    assert_eq!(failed.reason, RevalidationReason::ProviderUnavailable);

    let envelope = failed.clone().into_envelope("affiliate-domain").expect("envelope");
    assert_eq!(envelope.event_type, "affiliate.opportunity.revalidation_failed");
    let encoded = serde_json::to_string(&failed).expect("serialize");
    let decoded: OpportunityRevalidationFailed = serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded, failed);
}

#[test]
fn malformed_opportunity_event_payloads_are_rejected() {
    // Missing required fields must fail deserialization (fail closed).
    assert!(serde_json::from_str::<OpportunityDiscovered>(r#"{"identity":"acme:widget"}"#).is_err());
    assert!(serde_json::from_str::<OpportunityRevalidated>(r#"{"request_id":"not-a-uuid"}"#).is_err());
    // Unknown enum values in closed enums are rejected.
    assert!(serde_json::from_str::<OpportunityRevalidationRequested>(
        r#"{"request_id":"01890a5d-ac96-774b-bcce-b302099a8057","opportunity_id":"01890a5d-ac96-774b-bcce-b302099a8057","identity":"i","source":"s","reason":"stale","priority":"mega","requested_at_ms":1,"scheduled_for_ms":2}"#
    )
    .is_err());
}

#[test]
fn discovered_and_updated_events_produce_typed_envelopes() {
    let updated = OpportunityUpdated {
        opportunity_id: Uuid::now_v7(),
        identity: identity(),
        source: "network-b".into(),
        best_source: "network-a".into(),
        best_score: 9_000,
        observed_at_ms: 2_000,
        revision: 2,
    };
    let envelope = updated.into_envelope("affiliate-opportunity").expect("envelope");
    assert_eq!(envelope.event_type, "affiliate.opportunity.updated");
    assert_eq!(envelope.payload["identity"], "acme:widget");
    assert_eq!(envelope.payload["best_score"], 9_000);
    assert_eq!(envelope.payload["revision"], 2);
}

#[test]
fn commission_event_preserves_idempotency_identity() {
    let event = CommissionObligationCreated {
        obligation_id: cat_affiliate::CommissionObligationId(Uuid::now_v7()),
        conversion_id: cat_affiliate::ConversionId(Uuid::now_v7()),
        affiliate_id: cat_affiliate::AffiliateId(Uuid::now_v7()),
        amount_minor: 1250,
        currency: "EUR".to_owned(),
        idempotency_key: "commission:conversion:affiliate:1250".to_owned(),
    };

    let envelope = event.into_envelope("affiliate-domain").expect("envelope");
    assert_eq!(envelope.event_type, CommissionObligationCreated::TYPE);
    assert_eq!(envelope.version, CommissionObligationCreated::VERSION);
    assert_eq!(envelope.payload["currency"], "EUR");
    assert_eq!(envelope.payload["idempotency_key"], "commission:conversion:affiliate:1250");
}

#[test]
fn referral_state_event_carries_domain_identity() {
    let event = ReferralStateChanged {
        referral_id: cat_affiliate::ReferralId(Uuid::now_v7()),
        affiliate_id: cat_affiliate::AffiliateId(Uuid::now_v7()),
        offer_id: cat_affiliate::OfferId(Uuid::now_v7()),
        state: cat_affiliate::ReferralState::Attributed,
    };

    let envelope = event.into_envelope("affiliate-domain").expect("envelope");
    assert_eq!(envelope.event_type, "affiliate.referral.state_changed");
    assert_eq!(envelope.version, 1);
    assert!(envelope.payload.get("referral_id").is_some());
    assert_eq!(envelope.payload["state"], "Attributed");
}
