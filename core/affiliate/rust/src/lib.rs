pub mod error;
pub mod model;
pub mod service;

pub use error::{AffiliateDomainError, AffiliateDomainResult};
pub use model::{
    Affiliate, AffiliateId, AffiliateKind, CommissionObligation, CommissionObligationId,
    Conversion, ConversionId, ConversionState, Merchant, MerchantId, Offer, OfferId, OfferStatus,
    Product, ProductId, Program, ProgramId, ProgramStatus, Referral, ReferralId, ReferralState,
    VersionedName,
};
pub use service::AffiliateDomain;

pub const DOMAIN_NAME: &str = "affiliate";
pub const DOMAIN_VERSION: u16 = 1;
