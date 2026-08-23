use cat_affiliate::{CommissionObligation, CommissionObligationId, ConversionId, AffiliateId};
use uuid::Uuid;

#[test]
fn identifiers_are_distinct_and_serializable() {
    let affiliate = AffiliateId(Uuid::now_v7());
    let conversion = ConversionId(Uuid::now_v7());
    assert_ne!(affiliate.0, conversion.0);
    let json = serde_json::to_string(&affiliate).expect("affiliate id should serialize");
    let round_trip: AffiliateId = serde_json::from_str(&json).expect("affiliate id should deserialize");
    assert_eq!(affiliate, round_trip);
}

#[test]
fn commission_obligation_rejects_invalid_money_metadata() {
    let obligation = CommissionObligation {
        id: CommissionObligationId(Uuid::now_v7()),
        conversion_id: ConversionId(Uuid::now_v7()),
        affiliate_id: AffiliateId(Uuid::now_v7()),
        amount_minor: 0,
        currency: "EUR".into(),
        idempotency_key: "commission:conversion-1:affiliate-1".into(),
    };
    assert!(obligation.is_valid());
}

#[test]
fn commission_obligation_rejects_negative_amount() {
    let obligation = CommissionObligation {
        id: CommissionObligationId(Uuid::now_v7()),
        conversion_id: ConversionId(Uuid::now_v7()),
        affiliate_id: AffiliateId(Uuid::now_v7()),
        amount_minor: -1,
        currency: "EUR".into(),
        idempotency_key: "commission:conversion-1:affiliate-1".into(),
    };
    assert!(!obligation.is_valid());
}
