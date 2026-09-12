use cat_eventbus::CatEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

// ---------------------------------------------------------------------------
// Opportunity platform events
//
// These are typed FACT contracts for event-producing boundaries. Sprint 0
// ships the contracts and their serialization guarantees; the emitting
// boundaries are wired in Sprint 1 (ingestion publication, durable
// revalidation execution). Read-only derivations must never emit lifecycle
// events: computing "Expired" in a query is not a domain fact.
// ---------------------------------------------------------------------------

/// Emitted when an opportunity aggregate is created by ingestion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityDiscovered {
    pub opportunity_id: Uuid,
    pub identity: String,
    pub source: String,
    pub best_score: u32,
    pub observed_at_ms: u64,
    pub revision: u64,
}

impl CatEvent for OpportunityDiscovered {
    const TYPE: &'static str = "affiliate.opportunity.discovered";
    const VERSION: u16 = 1;
}

/// Emitted when a persisted opportunity's facts change (new observation,
/// improved best source, category enrichment).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityUpdated {
    pub opportunity_id: Uuid,
    pub identity: String,
    pub source: String,
    pub best_source: String,
    pub best_score: u32,
    pub observed_at_ms: u64,
    pub revision: u64,
}

impl CatEvent for OpportunityUpdated {
    const TYPE: &'static str = "affiliate.opportunity.updated";
    const VERSION: u16 = 1;
}

/// Fact contract: an opportunity crossed its staleness threshold and a
/// detection boundary recorded that fact. Read queries deriving `Stale` MUST
/// NOT emit this event.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityBecameStale {
    pub opportunity_id: Uuid,
    pub identity: String,
    pub last_observed_at_ms: u64,
    pub stale_threshold_ms: u64,
    pub detected_at_ms: u64,
}

impl CatEvent for OpportunityBecameStale {
    const TYPE: &'static str = "affiliate.opportunity.became_stale";
    const VERSION: u16 = 1;
}

/// Fact contract: an opportunity crossed its expiration threshold. Expiration
/// never deletes provenance; this event signals commercial un-usability.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityBecameExpired {
    pub opportunity_id: Uuid,
    pub identity: String,
    pub last_observed_at_ms: u64,
    pub expiration_threshold_ms: u64,
    pub detected_at_ms: u64,
}

impl CatEvent for OpportunityBecameExpired {
    const TYPE: &'static str = "affiliate.opportunity.became_expired";
    const VERSION: u16 = 1;
}

/// Fact: a revalidation request was created for an opportunity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityRevalidationRequested {
    pub request_id: Uuid,
    pub opportunity_id: Uuid,
    pub identity: String,
    pub source: String,
    pub reason: crate::RevalidationReason,
    pub priority: crate::RevalidationPriority,
    pub requested_at_ms: u64,
    pub scheduled_for_ms: u64,
}

impl CatEvent for OpportunityRevalidationRequested {
    const TYPE: &'static str = "affiliate.opportunity.revalidation_requested";
    const VERSION: u16 = 1;
}

/// Fact: a revalidation attempt completed and the opportunity was updated
/// with fresh observations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityRevalidated {
    pub request_id: Uuid,
    pub opportunity_id: Uuid,
    pub identity: String,
    pub source: String,
    pub observed_at_ms: u64,
    pub revision: u64,
}

impl CatEvent for OpportunityRevalidated {
    const TYPE: &'static str = "affiliate.opportunity.revalidated";
    const VERSION: u16 = 1;
}

/// Fact: a revalidation attempt failed. `detail` is a bounded, sanitized
/// operator note — never a raw provider payload, credential, or header.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityRevalidationFailed {
    pub request_id: Uuid,
    pub opportunity_id: Uuid,
    pub identity: String,
    pub source: String,
    pub reason: crate::RevalidationReason,
    pub failed_at_ms: u64,
    pub detail: String,
}

impl OpportunityRevalidationFailed {
    /// Bounded constructor: `detail` is truncated to 512 characters on a
    /// char boundary so oversized diagnostics cannot bloat the event log.
    pub fn new(
        request_id: Uuid,
        opportunity_id: Uuid,
        identity: String,
        source: String,
        reason: crate::RevalidationReason,
        failed_at_ms: u64,
        detail: String,
    ) -> Self {
        const MAX_DETAIL_CHARS: usize = 512;
        let detail: String = detail
            .chars()
            .map(|character| {
                if character.is_control() {
                    ' '
                } else {
                    character
                }
            })
            .take(MAX_DETAIL_CHARS)
            .collect();
        Self {
            request_id,
            opportunity_id,
            identity,
            source,
            reason,
            failed_at_ms,
            detail,
        }
    }
}

impl CatEvent for OpportunityRevalidationFailed {
    const TYPE: &'static str = "affiliate.opportunity.revalidation_failed";
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
        assert_eq!(
            ReferralStateChanged::TYPE,
            "affiliate.referral.state_changed"
        );
        assert_eq!(
            ConversionStateChanged::TYPE,
            "affiliate.conversion.state_changed"
        );
        assert_eq!(
            CommissionObligationCreated::TYPE,
            "affiliate.commission_obligation.created"
        );
        assert_eq!(OfferCreated::VERSION, 1);
    }
}
