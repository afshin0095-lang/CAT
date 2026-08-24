pub mod error;
pub mod events;
pub mod memory_repository;
pub mod model;
pub mod repository;
pub mod service;

pub use error::{AffiliateDomainError, AffiliateDomainResult};
pub use events::{
    AffiliateRegistered, CommissionObligationCreated, ConversionStateChanged, MerchantCreated,
    OfferCreated, ProgramCreated, ProgramStatusChanged, ReferralStateChanged,
};
pub use memory_repository::InMemoryAffiliateRepository;
pub use model::{
    Affiliate, AffiliateId, AffiliateKind, CommissionObligation, CommissionObligationId,
    Conversion, ConversionId, ConversionState, Merchant, MerchantId, Offer, OfferId, OfferStatus,
    Product, ProductId, Program, ProgramId, ProgramStatus, Referral, ReferralId, ReferralState,
    VersionedName,
};
pub use repository::AffiliateRepository;
pub use service::AffiliateDomain;

pub const DOMAIN_NAME: &str = "affiliate";
pub const DOMAIN_VERSION: u16 = 1;
