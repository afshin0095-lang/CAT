use cat_affiliate::{
    AffiliateDomain, AffiliateKind, AffiliateRepository, InMemoryAffiliateRepository,
};

#[test]
fn round_trip_preserves_canonical_entities() {
    let mut repo = InMemoryAffiliateRepository::new();
    let merchant = AffiliateDomain::merchant("Acme", 1).unwrap();
    repo.save_merchant(merchant.clone()).unwrap();

    let program = AffiliateDomain::program(&merchant, "Partners", 1).unwrap();
    repo.save_program(program.clone()).unwrap();
    let product = AffiliateDomain::product(&merchant, "acme/widget", "Widget", 1).unwrap();
    repo.save_product(product.clone()).unwrap();

    assert_eq!(repo.merchant(merchant.id).unwrap(), merchant);
    assert_eq!(repo.program(program.id).unwrap(), program);
    assert_eq!(repo.product(product.id).unwrap(), product);
}

#[test]
fn duplicate_and_missing_records_are_rejected() {
    let mut repo = InMemoryAffiliateRepository::new();
    let merchant = AffiliateDomain::merchant("Acme", 1).unwrap();
    repo.save_merchant(merchant.clone()).unwrap();
    assert_eq!(
        repo.save_merchant(merchant),
        Err(cat_affiliate::AffiliateDomainError::RepositoryConflict)
    );

    assert_eq!(
        repo.merchant(cat_affiliate::MerchantId::nil()),
        Err(cat_affiliate::AffiliateDomainError::RepositoryNotFound(
            "merchant"
        ))
    );
}

#[test]
fn full_affiliate_flow_can_be_persisted_without_mutating_truth() {
    let mut repo = InMemoryAffiliateRepository::new();
    let merchant = AffiliateDomain::merchant("Acme", 1).unwrap();
    repo.save_merchant(merchant.clone()).unwrap();
    let mut program = AffiliateDomain::program(&merchant, "Partners", 1).unwrap();
    AffiliateDomain::activate_program(&mut program);
    repo.save_program(program.clone()).unwrap();
    let product = AffiliateDomain::product(&merchant, "acme/widget", "Widget", 1).unwrap();
    repo.save_product(product.clone()).unwrap();
    let mut offer = AffiliateDomain::offer(&program, Some(product.id), 1).unwrap();
    AffiliateDomain::activate_offer(&mut offer);
    repo.save_offer(offer.clone()).unwrap();
    let affiliate = AffiliateDomain::affiliate(AffiliateKind::Agent, "CAT Agent", 1).unwrap();
    repo.save_affiliate(affiliate.clone()).unwrap();
    let referral = AffiliateDomain::referral(&affiliate, &offer, 86_400).unwrap();
    repo.save_referral(referral.clone()).unwrap();

    assert_eq!(repo.referral(referral.id).unwrap(), referral);
    assert_eq!(
        repo.offer(offer.id).unwrap().status,
        cat_affiliate::OfferStatus::Active
    );
}
