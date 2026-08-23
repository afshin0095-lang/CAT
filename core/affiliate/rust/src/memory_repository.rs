use std::collections::HashMap;

use crate::{Affiliate, AffiliateId, AffiliateRepository, CommissionObligation, CommissionObligationId, Conversion, ConversionId, Merchant, MerchantId, Offer, OfferId, Product, ProductId, Program, ProgramId, Referral, ReferralId, AffiliateDomainError, AffiliateDomainResult};

#[derive(Default)]
pub struct InMemoryAffiliateRepository {
    merchants: HashMap<MerchantId, Merchant>,
    programs: HashMap<ProgramId, Program>,
    products: HashMap<ProductId, Product>,
    offers: HashMap<OfferId, Offer>,
    affiliates: HashMap<AffiliateId, Affiliate>,
    referrals: HashMap<ReferralId, Referral>,
    conversions: HashMap<ConversionId, Conversion>,
    obligations: HashMap<CommissionObligationId, CommissionObligation>,
}

impl InMemoryAffiliateRepository {
    pub fn new() -> Self { Self::default() }

    fn insert<T, I>(map: &mut HashMap<I, T>, id: I, value: T) -> AffiliateDomainResult<()>
    where
        I: std::hash::Hash + Eq + Copy,
    {
        if map.contains_key(&id) {
            return Err(AffiliateDomainError::RepositoryConflict);
        }
        map.insert(id, value);
        Ok(())
    }
}

macro_rules! repository_impl {
    ($save:ident, $get:ident, $field:ident, $ty:ty, $id:ty, $entity:literal) => {
        fn $save(&mut self, value: $ty) -> AffiliateDomainResult<()> {
            Self::insert(&mut self.$field, value.id, value)
        }
        fn $get(&self, id: $id) -> AffiliateDomainResult<$ty> {
            self.$field.get(&id).cloned().ok_or(AffiliateDomainError::RepositoryNotFound($entity))
        }
    };
}

impl AffiliateRepository for InMemoryAffiliateRepository {
    repository_impl!(save_merchant, merchant, merchants, Merchant, MerchantId, "merchant");
    repository_impl!(save_program, program, programs, Program, ProgramId, "program");
    repository_impl!(save_product, product, products, Product, ProductId, "product");
    repository_impl!(save_offer, offer, offers, Offer, OfferId, "offer");
    repository_impl!(save_affiliate, affiliate, affiliates, Affiliate, AffiliateId, "affiliate");
    repository_impl!(save_referral, referral, referrals, Referral, ReferralId, "referral");
    repository_impl!(save_conversion, conversion, conversions, Conversion, ConversionId, "conversion");
    repository_impl!(save_commission_obligation, commission_obligation, obligations, CommissionObligation, CommissionObligationId, "commission_obligation");
}
