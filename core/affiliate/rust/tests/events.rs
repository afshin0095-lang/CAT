use cat_affiliate::{
    AffiliateRegistered, CommissionObligationCreated, ConversionStateChanged, MerchantCreated,
    OfferCreated, ProgramCreated, ProgramStatusChanged, ReferralStateChanged,
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
    ];

    let unique = types.iter().copied().collect::<HashSet<_>>();
    assert_eq!(unique.len(), types.len());
    assert!(types.iter().all(|event_type| event_type.starts_with("affiliate.")));
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
