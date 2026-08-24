use cat_eventbus::CatEvent;
use serde::{Deserialize, Serialize};

use crate::{
    AffiliateId, AffiliateKind, CommissionObligationId, ConversionId, ConversionState, MerchantId,
    OfferId, OfferStatus, ProgramId, ProgramStatus, ReferralId, ReferralState,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MerchantCreated {
    pub merchant_id: MerchantId,
    pub name: String,
    pub version: u32,
}

impl CatEvent for MerchantCreated {
    const TYPE: &'static str = "affiliate.merchant.created";
    const VERSION: u16 = 1;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgramCreated {
    pub program_id: ProgramId,
    pub merchant_id: MerchantId,
    pub version: u32,
}

impl CatEvent for ProgramCreated {
    const TYPE: &'static str = "affiliate.program.created";
    const VERSION: u16 = 1;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgramStatusChanged {
    pub program_id: ProgramId,
    pub status: ProgramStatus,
}

impl CatEvent for ProgramStatusChanged {
    const TYPE: &'static str = "affiliate.program.status_changed";
    const VERSION: u16 = 1;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OfferCreated {
    pub offer_id: OfferId,
    pub program_id: ProgramId,
    pub product_id: Option<crate::ProductId>,
    pub status: OfferStatus,
    pub version: u32,
}

impl CatEvent for OfferCreated {
    const TYPE: &'static str = "affiliate.offer.created";
    const VERSION: u16 = 1;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AffiliateRegistered {
    pub affiliate_id: AffiliateId,
    pub kind: AffiliateKind,
}

impl CatEvent for AffiliateRegistered {
    const TYPE: &'static str = "affiliate.partner.registered";
    const VERSION: u16 = 1;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReferralStateChanged {
    pub referral_id: ReferralId,
    pub affiliate_id: AffiliateId,
    pub offer_id: OfferId,
    pub state: ReferralState,
}

impl CatEvent for ReferralStateChanged {
    const TYPE: &'static str = "affiliate.referral.state_changed";
    const VERSION: u16 = 1;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConversionStateChanged {
    pub conversion_id: ConversionId,
    pub referral_id: ReferralId,
    pub state: ConversionState,
}

impl CatEvent for ConversionStateChanged {
    const TYPE: &'static str = "affiliate.conversion.state_changed";
    const VERSION: u16 = 1;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommissionObligationCreated {
    pub obligation_id: CommissionObligationId,
    pub conversion_id: ConversionId,
    pub affiliate_id: AffiliateId,
    pub amount_minor: i64,
    pub currency: String,
    pub idempotency_key: String,
}

impl CatEvent for CommissionObligationCreated {
    const TYPE: &'static str = "affiliate.commission_obligation.created";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use cat_eventbus::EventEnvelope;
    use uuid::Uuid;

    #[test]
    fn event_contracts_produce_typed_envelopes() {
        let event = MerchantCreated {
            merchant_id: MerchantId(Uuid::now_v7()),
            name: "Acme".to_owned(),
            version: 1,
        };

        let envelope = event.into_envelope("affiliate-domain").expect("envelope");

        assert_eq!(envelope.event_type, "affiliate.merchant.created");
        assert_eq!(envelope.version, 1);
        assert_eq!(envelope.producer, "affiliate-domain");
        assert!(envelope.payload.get("merchant_id").is_some());
    }

    #[test]
    fn event_contracts_are_json_round_trip_safe() {
        let event = ProgramStatusChanged {
            program_id: ProgramId(Uuid::now_v7()),
            status: ProgramStatus::Active,
        };

        let encoded = serde_json::to_string(&event).expect("serialize");
        let decoded: ProgramStatusChanged = serde_json::from_str(&encoded).expect("deserialize");

        assert_eq!(decoded, event);
    }

    #[test]
    fn event_contracts_are_versioned_and_named() {
        assert_eq!(AffiliateRegistered::TYPE, "affiliate.partner.registered");
        assert_eq!(ReferralStateChanged::TYPE, "affiliate.referral.state_changed");
        assert_eq!(ConversionStateChanged::TYPE, "affiliate.conversion.state_changed");
        assert_eq!(CommissionObligationCreated::TYPE, "affiliate.commission_obligation.created");
        assert_eq!(OfferCreated::VERSION, 1);
    }
}
