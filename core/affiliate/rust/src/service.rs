use uuid::Uuid;

use crate::{
    Affiliate, AffiliateDomainError, AffiliateDomainResult, AffiliateId, AffiliateKind,
    CommissionObligation, CommissionObligationId, Conversion, ConversionId, ConversionState,
    Merchant, MerchantId, Offer, OfferId, OfferStatus, Product, ProductId, Program, ProgramId,
    ProgramStatus, Referral, ReferralId, ReferralState, VersionedName,
};

pub struct AffiliateDomain;

impl AffiliateDomain {
    pub fn merchant(name: impl Into<String>, version: u32) -> AffiliateDomainResult<Merchant> {
        Ok(Merchant {
            id: MerchantId(Uuid::now_v7()),
            name: versioned_name(name, version)?,
            active: true,
        })
    }

    pub fn program(
        merchant: &Merchant,
        name: impl Into<String>,
        version: u32,
    ) -> AffiliateDomainResult<Program> {
        Ok(Program {
            id: ProgramId(Uuid::now_v7()),
            merchant_id: merchant.id,
            name: versioned_name(name, version)?,
            status: ProgramStatus::Draft,
            version,
        })
    }

    pub fn product(
        merchant: &Merchant,
        canonical_key: impl Into<String>,
        name: impl Into<String>,
        version: u32,
    ) -> AffiliateDomainResult<Product> {
        let canonical_key = canonical_key.into();
        if canonical_key.trim().is_empty() {
            return Err(AffiliateDomainError::EmptyCanonicalKey);
        }

        Ok(Product {
            id: ProductId(Uuid::now_v7()),
            merchant_id: merchant.id,
            canonical_key,
            name: versioned_name(name, version)?,
        })
    }

    pub fn activate_program(program: &mut Program) {
        program.status = ProgramStatus::Active;
    }

    pub fn pause_program(program: &mut Program) {
        program.status = ProgramStatus::Paused;
    }

    pub fn retire_program(program: &mut Program) {
        program.status = ProgramStatus::Retired;
    }

    pub fn offer(
        program: &Program,
        product_id: Option<ProductId>,
        version: u32,
    ) -> AffiliateDomainResult<Offer> {
        if version == 0 {
            return Err(AffiliateDomainError::InvalidVersion);
        }
        Ok(Offer {
            id: OfferId(Uuid::now_v7()),
            program_id: program.id,
            product_id,
            status: OfferStatus::Draft,
            version,
        })
    }

    pub fn activate_offer(offer: &mut Offer) {
        offer.status = OfferStatus::Active;
    }

    pub fn affiliate(
        kind: AffiliateKind,
        display_name: impl Into<String>,
        version: u32,
    ) -> AffiliateDomainResult<Affiliate> {
        Ok(Affiliate {
            id: AffiliateId(Uuid::now_v7()),
            kind,
            display_name: versioned_name(display_name, version)?,
        })
    }

    pub fn referral(
        affiliate: &Affiliate,
        offer: &Offer,
        attribution_window_seconds: u64,
    ) -> AffiliateDomainResult<Referral> {
        if offer.status != OfferStatus::Active {
            return Err(AffiliateDomainError::InactiveOffer);
        }
        if attribution_window_seconds == 0 {
            return Err(AffiliateDomainError::InvalidAttributionWindow);
        }

        Ok(Referral {
            id: ReferralId(Uuid::now_v7()),
            affiliate_id: affiliate.id,
            offer_id: offer.id,
            state: ReferralState::Received,
            attribution_window_seconds,
        })
    }

    pub fn qualify_referral(referral: &mut Referral) {
        if referral.state == ReferralState::Received {
            referral.state = ReferralState::Qualified;
        }
    }

    pub fn attribute_referral(referral: &mut Referral) {
        if matches!(
            referral.state,
            ReferralState::Received | ReferralState::Qualified
        ) {
            referral.state = ReferralState::Attributed;
        }
    }

    pub fn conversion(
        referral: &Referral,
        gross_amount_minor: i64,
        net_amount_minor: i64,
        commissionable_amount_minor: i64,
    ) -> AffiliateDomainResult<Conversion> {
        if gross_amount_minor < 0 || net_amount_minor < 0 || commissionable_amount_minor < 0 {
            return Err(AffiliateDomainError::NegativeAmount);
        }
        if net_amount_minor > gross_amount_minor {
            return Err(AffiliateDomainError::NetExceedsGross);
        }
        if commissionable_amount_minor > net_amount_minor {
            return Err(AffiliateDomainError::CommissionableExceedsNet);
        }

        Ok(Conversion {
            id: ConversionId(Uuid::now_v7()),
            referral_id: referral.id,
            state: ConversionState::Pending,
            gross_amount_minor,
            net_amount_minor,
            commissionable_amount_minor,
        })
    }

    pub fn verify_conversion(conversion: &mut Conversion) {
        if conversion.state == ConversionState::Pending {
            conversion.state = ConversionState::Verified;
        }
    }

    pub fn commission_obligation(
        conversion: &Conversion,
        affiliate: &Affiliate,
        amount_minor: i64,
        currency: impl Into<String>,
    ) -> AffiliateDomainResult<CommissionObligation> {
        if conversion.state != ConversionState::Verified || amount_minor < 0 {
            return Err(AffiliateDomainError::InvalidCommissionObligation);
        }
        if amount_minor > conversion.commissionable_amount_minor {
            return Err(AffiliateDomainError::InvalidCommissionObligation);
        }

        let currency = currency.into();
        let idempotency_key = format!(
            "commission:{}:{}:{}",
            conversion.id.0, affiliate.id.0, amount_minor
        );
        let obligation = CommissionObligation {
            id: CommissionObligationId(Uuid::now_v7()),
            conversion_id: conversion.id,
            affiliate_id: affiliate.id,
            amount_minor,
            currency,
            idempotency_key,
        };

        if !obligation.is_valid() {
            return Err(AffiliateDomainError::InvalidCommissionObligation);
        }
        Ok(obligation)
    }
}

fn versioned_name(value: impl Into<String>, version: u32) -> AffiliateDomainResult<VersionedName> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(AffiliateDomainError::EmptyName);
    }
    if version == 0 {
        return Err(AffiliateDomainError::InvalidVersion);
    }
    Ok(VersionedName::new(value, version))
}

#[allow(dead_code, clippy::too_many_arguments)]
fn _type_markers(
    _: MerchantId,
    _: ProgramId,
    _: ProductId,
    _: OfferId,
    _: AffiliateId,
    _: ReferralId,
    _: ConversionId,
    _: CommissionObligationId,
) {
}
