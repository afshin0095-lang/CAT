use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AffiliateId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MerchantId(pub Uuid);

impl MerchantId {
    pub const fn nil() -> Self { Self(Uuid::nil()) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ProgramId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct OfferId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ProductId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ReferralId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ConversionId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CommissionObligationId(pub Uuid);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VersionedName {
    pub value: String,
    pub version: u32,
}

impl VersionedName {
    pub fn new(value: impl Into<String>, version: u32) -> Self {
        Self { value: value.into(), version }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AffiliateKind {
    Human,
    Agent,
    Organization,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProgramStatus {
    Draft,
    Active,
    Paused,
    Retired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OfferStatus {
    Draft,
    Active,
    Suspended,
    Expired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReferralState {
    Received,
    Qualified,
    Attributed,
    Rejected,
    Expired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ConversionState {
    Pending,
    Verified,
    Rejected,
    Corrected,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Merchant {
    pub id: MerchantId,
    pub name: VersionedName,
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub id: ProgramId,
    pub merchant_id: MerchantId,
    pub name: VersionedName,
    pub status: ProgramStatus,
    pub version: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: ProductId,
    pub merchant_id: MerchantId,
    pub canonical_key: String,
    pub name: VersionedName,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Offer {
    pub id: OfferId,
    pub program_id: ProgramId,
    pub product_id: Option<ProductId>,
    pub status: OfferStatus,
    pub version: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Affiliate {
    pub id: AffiliateId,
    pub kind: AffiliateKind,
    pub display_name: VersionedName,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Referral {
    pub id: ReferralId,
    pub affiliate_id: AffiliateId,
    pub offer_id: OfferId,
    pub state: ReferralState,
    pub attribution_window_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Conversion {
    pub id: ConversionId,
    pub referral_id: ReferralId,
    pub state: ConversionState,
    pub gross_amount_minor: i64,
    pub net_amount_minor: i64,
    pub commissionable_amount_minor: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommissionObligation {
    pub id: CommissionObligationId,
    pub conversion_id: ConversionId,
    pub affiliate_id: AffiliateId,
    pub amount_minor: i64,
    pub currency: String,
    pub idempotency_key: String,
}

impl CommissionObligation {
    pub fn is_valid(&self) -> bool {
        self.amount_minor >= 0 && !self.currency.is_empty() && !self.idempotency_key.is_empty()
    }
}
