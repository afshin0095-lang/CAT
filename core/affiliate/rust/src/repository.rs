use crate::{
    Affiliate, AffiliateDomainResult, AffiliateId, CommissionObligation, CommissionObligationId,
    Conversion, ConversionId, Merchant, MerchantId, Offer, OfferId, Product, ProductId, Program,
    ProgramId, Referral, ReferralId,
};

pub trait AffiliateRepository {
    fn save_merchant(&mut self, merchant: Merchant) -> AffiliateDomainResult<()>;
    fn merchant(&self, id: MerchantId) -> AffiliateDomainResult<Merchant>;
    fn save_program(&mut self, program: Program) -> AffiliateDomainResult<()>;
    fn program(&self, id: ProgramId) -> AffiliateDomainResult<Program>;
    fn save_product(&mut self, product: Product) -> AffiliateDomainResult<()>;
    fn product(&self, id: ProductId) -> AffiliateDomainResult<Product>;
    fn save_offer(&mut self, offer: Offer) -> AffiliateDomainResult<()>;
    fn offer(&self, id: OfferId) -> AffiliateDomainResult<Offer>;
    fn save_affiliate(&mut self, affiliate: Affiliate) -> AffiliateDomainResult<()>;
    fn affiliate(&self, id: AffiliateId) -> AffiliateDomainResult<Affiliate>;
    fn save_referral(&mut self, referral: Referral) -> AffiliateDomainResult<()>;
    fn referral(&self, id: ReferralId) -> AffiliateDomainResult<Referral>;
    fn save_conversion(&mut self, conversion: Conversion) -> AffiliateDomainResult<()>;
    fn conversion(&self, id: ConversionId) -> AffiliateDomainResult<Conversion>;
    fn save_commission_obligation(
        &mut self,
        obligation: CommissionObligation,
    ) -> AffiliateDomainResult<()>;
    fn commission_obligation(
        &self,
        id: CommissionObligationId,
    ) -> AffiliateDomainResult<CommissionObligation>;
}
