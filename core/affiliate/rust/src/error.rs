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
    #[error("repository already contains this canonical entity")]
    RepositoryConflict,
    #[error("repository record not found: {0}")]
    RepositoryNotFound(&'static str),
}

pub type AffiliateDomainResult<T> = Result<T, AffiliateDomainError>;

/// Coarse error category shared by every Affiliate Core error type.
///
/// Categories are contracts for callers: `Transient` errors may succeed on
/// retry (ideally with backoff), `Conflict` errors require reloading state,
/// everything else is deterministic and will reproduce until inputs change.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ErrorCategory {
    /// The input violated a domain contract; retrying unchanged inputs fails again.
    Validation,
    /// The referenced entity does not exist.
    NotFound,
    /// A concurrent writer won; the caller must reload and re-plan.
    Conflict,
    /// Environmental failure (network, database, provider); retry is meaningful.
    Transient,
    /// Deterministic refusal (authorization, unsupported, corrupt data).
    Permanent,
}

/// Classification contract implemented by every Affiliate Core error type.
///
/// Error Display strings never contain credentials or sensitive provider
/// payloads; classification adds structured metadata without changing that.
pub trait ErrorClassification {
    fn category(&self) -> ErrorCategory;

    /// Convenience for retry policies. Only `Transient` is retryable;
    /// `Conflict` is *resolvable* but not blindly retryable.
    fn is_retryable(&self) -> bool {
        matches!(self.category(), ErrorCategory::Transient)
    }
}

impl ErrorClassification for AffiliateDomainError {
    fn category(&self) -> ErrorCategory {
        match self {
            Self::EmptyName
            | Self::EmptyCanonicalKey
            | Self::InvalidVersion
            | Self::ProgramMerchantMismatch
            | Self::OfferProgramMismatch
            | Self::InactiveOffer
            | Self::InvalidAttributionWindow
            | Self::NegativeAmount
            | Self::NetExceedsGross
            | Self::CommissionableExceedsNet
            | Self::InvalidCommissionObligation => ErrorCategory::Validation,
            Self::RepositoryConflict => ErrorCategory::Conflict,
            Self::RepositoryNotFound(_) => ErrorCategory::NotFound,
        }
    }
}

// ---------------------------------------------------------------------------
// Opportunity subsystem error classification
// ---------------------------------------------------------------------------

impl ErrorClassification for crate::DiscoveryError {
    fn category(&self) -> ErrorCategory {
        // Discovery validation is deterministic: never retryable.
        ErrorCategory::Validation
    }
}

impl ErrorClassification for crate::DiscoverySourceError {
    fn category(&self) -> ErrorCategory {
        match self {
            Self::InvalidRequest(_) => ErrorCategory::Validation,
            Self::Unavailable(_) => ErrorCategory::Transient,
            Self::RateLimited => ErrorCategory::Transient,
            Self::AuthenticationFailed => ErrorCategory::Permanent,
            Self::InvalidResponse(_) => ErrorCategory::Permanent,
        }
    }
}

impl ErrorClassification for crate::OpportunityStoreError {
    fn category(&self) -> ErrorCategory {
        match self {
            Self::InvalidCandidate => ErrorCategory::Validation,
            Self::IdentityConflict => ErrorCategory::Conflict,
            Self::NotFound => ErrorCategory::NotFound,
            Self::RevisionOverflow(_) => ErrorCategory::Permanent,
        }
    }
}

impl ErrorClassification for crate::opportunity_lifecycle::OpportunityLifecycleError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Validation
    }
}

impl ErrorClassification for crate::opportunity_freshness::FreshnessPolicyError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Validation
    }
}

impl ErrorClassification for crate::OpportunityRevisionError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Validation
    }
}

impl ErrorClassification for crate::opportunity_revalidation::RevalidationStoreError {
    fn category(&self) -> ErrorCategory {
        match self {
            crate::opportunity_revalidation::RevalidationStoreError::DuplicateRequest {
                ..
            } => ErrorCategory::Conflict,
            crate::opportunity_revalidation::RevalidationStoreError::NotFound => {
                ErrorCategory::NotFound
            }
            crate::opportunity_revalidation::RevalidationStoreError::InvalidTransition {
                ..
            } => ErrorCategory::Validation,
            crate::opportunity_revalidation::RevalidationStoreError::UnknownEnumValue {
                ..
            } => ErrorCategory::Permanent,
            crate::opportunity_revalidation::RevalidationStoreError::InvalidRequest(_) => {
                ErrorCategory::Validation
            }
        }
    }
}
