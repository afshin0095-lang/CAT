pub mod model;

pub use model::{
    Affiliate, AffiliateId, AffiliateKind, CommissionObligation, CommissionObligationId,
    Conversion, ConversionId, ConversionState, Merchant, MerchantId, Offer, OfferId, OfferStatus,
    Product, ProductId, Program, ProgramId, ProgramStatus, Referral, ReferralId, ReferralState,
    VersionedName,
};

pub const DOMAIN_NAME: &str = "affiliate";
pub const DOMAIN_VERSION: u16 = 1;
