use cat_affiliate::{
    AffiliateDomain, AffiliateDomainError, AffiliateKind, OfferStatus, ProgramStatus, ReferralState,
};

#[test]
fn lifecycle_preserves_domain_boundaries() {
    let merchant = AffiliateDomain::merchant("Acme", 1).unwrap();
    let mut program = AffiliateDomain::program(&merchant, "Acme Partners", 1).unwrap();
    assert_eq!(program.status, ProgramStatus::Draft);

    AffiliateDomain::activate_program(&mut program);
    assert_eq!(program.status, ProgramStatus::Active);

    let product = AffiliateDomain::product(&merchant, "acme/widget", "Widget", 1).unwrap();
    let mut offer = AffiliateDomain::offer(&program, Some(product.id), 1).unwrap();
    assert_eq!(offer.status, OfferStatus::Draft);

    AffiliateDomain::activate_offer(&mut offer);
    let affiliate = AffiliateDomain::affiliate(AffiliateKind::Agent, "CAT Publisher", 1).unwrap();
    let mut referral = AffiliateDomain::referral(&affiliate, &offer, 86_400).unwrap();
    assert_eq!(referral.state, ReferralState::Received);

    AffiliateDomain::qualify_referral(&mut referral);
    AffiliateDomain::attribute_referral(&mut referral);
    assert_eq!(referral.state, ReferralState::Attributed);

    let mut conversion = AffiliateDomain::conversion(&referral, 10_000, 9_000, 8_500).unwrap();
    AffiliateDomain::verify_conversion(&mut conversion);
    let obligation =
        AffiliateDomain::commission_obligation(&conversion, &affiliate, 1_250, "EUR").unwrap();

    assert!(obligation.is_valid());
    assert_eq!(obligation.amount_minor, 1_250);
    assert!(obligation.idempotency_key.starts_with("commission:"));
}

#[test]
fn invalid_money_relationships_are_rejected() {
    let merchant = AffiliateDomain::merchant("Acme", 1).unwrap();
    let mut program = AffiliateDomain::program(&merchant, "Partners", 1).unwrap();
    AffiliateDomain::activate_program(&mut program);
    let mut offer = AffiliateDomain::offer(&program, None, 1).unwrap();
    AffiliateDomain::activate_offer(&mut offer);
    let affiliate = AffiliateDomain::affiliate(AffiliateKind::Human, "Alice", 1).unwrap();
    let referral = AffiliateDomain::referral(&affiliate, &offer, 60).unwrap();

    assert_eq!(
        AffiliateDomain::conversion(&referral, 100, 101, 50).unwrap_err(),
        AffiliateDomainError::NetExceedsGross
    );
    assert_eq!(
        AffiliateDomain::conversion(&referral, 100, 80, 81).unwrap_err(),
        AffiliateDomainError::CommissionableExceedsNet
    );
    assert_eq!(
        AffiliateDomain::conversion(&referral, -1, 0, 0).unwrap_err(),
        AffiliateDomainError::NegativeAmount
    );
}

#[test]
fn referral_requires_active_offer_and_positive_window() {
    let merchant = AffiliateDomain::merchant("Acme", 1).unwrap();
    let program = AffiliateDomain::program(&merchant, "Partners", 1).unwrap();
    let offer = AffiliateDomain::offer(&program, None, 1).unwrap();
    let affiliate = AffiliateDomain::affiliate(AffiliateKind::Organization, "Network", 1).unwrap();

    assert_eq!(
        AffiliateDomain::referral(&affiliate, &offer, 60).unwrap_err(),
        AffiliateDomainError::InactiveOffer
    );

    let mut active_offer = offer;
    AffiliateDomain::activate_offer(&mut active_offer);
    assert_eq!(
        AffiliateDomain::referral(&affiliate, &active_offer, 0).unwrap_err(),
        AffiliateDomainError::InvalidAttributionWindow
    );
}
