use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AffiliateDomainError {
    #[error("name must not be empty")]
    EmptyName,
    #[error("canonical product key must not be empty")]
    EmptyCanonicalKey,
    #[error("version must be greater than zero")]
    InvalidVersion,
    #[error("program belongs to a different merchant")]
    ProgramMerchantMismatch,
    #[error("offer belongs to a different program")]
    OfferProgramMismatch,
    #[error("referral must reference an active offer")]
    InactiveOffer,
    #[error("referral attribution window must be greater than zero")]
    InvalidAttributionWindow,
    #[error("conversion amounts must be non-negative")]
    NegativeAmount,
    #[error("net amount cannot exceed gross amount")]
    NetExceedsGross,
    #[error("commissionable amount cannot exceed net amount")]
    CommissionableExceedsNet,
    #[error("commission obligation is invalid")]
    InvalidCommissionObligation,
}

pub type AffiliateDomainResult<T> = Result<T, AffiliateDomainError>;
