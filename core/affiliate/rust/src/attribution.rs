//! Attribution engine: given a conversion event, walk Identity -> Click
//! chain, apply one of 4 attribution models, and produce attribution records.
//!
//! Directly inspired by OpenPartner's `attribution.ts`. The core insight:
//! raw data (Click, Event) is immutable. Attribution is a derived view
//! that can be recomputed with a different model at any time.
//!
//! ## Models
//!
//! - `LastClick`  — 100% credit to the most recent click within the window
//! - `FirstClick` — 100% credit to the earliest click within the window
//! - `Linear`     — equal credit (1/N) to every click within the window
//! - `Position`   — U-shape: 40% first, 40% last, 20% split across middle

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tracking::{ClickId, ExternalUserId};
use crate::{AffiliateId, ProgramId};

// ---------------------------------------------------------------------------
// IDs
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AttributionId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ConversionEventId(pub Uuid);

// ---------------------------------------------------------------------------
// Attribution model enum
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum AttributionModel {
    LastClick,
    FirstClick,
    Linear,
    Position,
}

// ---------------------------------------------------------------------------
// Conversion event (immutable raw event from merchant)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConversionEventType {
    Signup,
    TrialStarted,
    SubscriptionCreated,
    InvoicePaid,
    Purchase,
    Custom(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionEvent {
    pub id: ConversionEventId,
    pub user_id: ExternalUserId,
    pub event_type: ConversionEventType,
    /// Revenue value in minor units (cents). `None` for non-revenue events.
    pub value_minor: Option<i64>,
    pub currency: Option<String>,
    /// External idempotency key (e.g. Stripe event ID) to prevent duplicates.
    pub external_event_id: Option<String>,
    pub timestamp_ms: i64,
}

// ---------------------------------------------------------------------------
// Attribution record (derived, recomputable)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attribution {
    pub id: AttributionId,
    pub event_id: ConversionEventId,
    pub affiliate_id: AffiliateId,
    pub program_id: ProgramId,
    pub click_id: ClickId,
    pub model: AttributionModel,
    /// Weight assigned to this touch (0.0 .. 1.0).
    pub weight: f64,
    pub computed_at_ms: i64,
}

// ---------------------------------------------------------------------------
// Attribution result
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AttributionResult {
    Attributed {
        touches: Vec<AttributionTouch>,
        model: AttributionModel,
    },
    NoIdentity,
    NoClick,
    OutsideWindow,
    AlreadyAttributed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributionTouch {
    pub click_id: ClickId,
    pub affiliate_id: AffiliateId,
    pub weight: f64,
    pub attribution_id: AttributionId,
}

// ---------------------------------------------------------------------------
// Weight distribution (pure function, no IO)
// ---------------------------------------------------------------------------

/// Distribute attribution weight across `n` touches using `model`.
/// Returns a `Vec<f64>` of length `n` where all weights sum to 1.0.
/// Clicks are assumed to be sorted oldest-first.
pub fn apply_model(model: AttributionModel, n: usize) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }

    match model {
        AttributionModel::LastClick => {
            let mut w = vec![0.0; n];
            w[n - 1] = 1.0;
            w
        }
        AttributionModel::FirstClick => {
            let mut w = vec![0.0; n];
            w[0] = 1.0;
            w
        }
        AttributionModel::Linear => {
            vec![1.0 / n as f64; n]
        }
        AttributionModel::Position => {
            if n == 1 {
                return vec![1.0];
            }
            if n == 2 {
                return vec![0.5, 0.5];
            }
            let mut w = vec![0.0; n];
            w[0] = 0.4;
            w[n - 1] = 0.4;
            let middle_weight = 0.2 / (n - 2) as f64;
            for weight in &mut w[1..n - 1] {
                *weight = middle_weight;
            }
            w
        }
    }
}

/// Check that a click is within the attribution window relative to
/// an event. Returns `true` if the click is eligible.
///
/// A grace period of 5 minutes absorbs clock drift between the system
/// stamping the click and the system stamping the event.
pub fn click_within_window(
    click_timestamp_ms: i64,
    event_timestamp_ms: i64,
    window_days: u32,
) -> bool {
    const GRACE_MS: i64 = 5 * 60 * 1000;
    let age_ms = event_timestamp_ms - click_timestamp_ms;
    let window_ms = window_days as i64 * 24 * 60 * 60 * 1000;
    age_ms >= -GRACE_MS && age_ms <= window_ms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_click_gives_all_weight_to_last() {
        let w = apply_model(AttributionModel::LastClick, 5);
        assert_eq!(w, vec![0.0, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn first_click_gives_all_weight_to_first() {
        let w = apply_model(AttributionModel::FirstClick, 3);
        assert_eq!(w, vec![1.0, 0.0, 0.0]);
    }

    #[test]
    fn linear_distributes_equally() {
        let w = apply_model(AttributionModel::Linear, 4);
        assert_eq!(w, vec![0.25, 0.25, 0.25, 0.25]);
    }

    #[test]
    fn position_model_u_shape() {
        let w = apply_model(AttributionModel::Position, 4);
        assert!((w[0] - 0.4).abs() < 1e-10);
        assert!((w[3] - 0.4).abs() < 1e-10);
        assert!((w[1] - 0.1).abs() < 1e-10);
        assert!((w[2] - 0.1).abs() < 1e-10);
    }

    #[test]
    fn position_model_two_touches() {
        let w = apply_model(AttributionModel::Position, 2);
        assert_eq!(w, vec![0.5, 0.5]);
    }

    #[test]
    fn position_model_single_touch() {
        let w = apply_model(AttributionModel::Position, 1);
        assert_eq!(w, vec![1.0]);
    }

    #[test]
    fn empty_produces_empty() {
        assert!(apply_model(AttributionModel::LastClick, 0).is_empty());
    }

    #[test]
    fn click_within_window_normal() {
        let click = 1000;
        let event = 1000 + 3_600_000;
        assert!(click_within_window(click, event, 30));
    }

    #[test]
    fn click_outside_window() {
        let click = 1000;
        let event = 1000 + 31 * 86_400_000;
        assert!(!click_within_window(click, event, 30));
    }

    #[test]
    fn click_slightly_after_event_within_grace() {
        let click = 1000 + 60_000;
        let event = 1000;
        assert!(click_within_window(click, event, 30));
    }

    #[test]
    fn click_way_after_event_rejected() {
        let click = 1000 + 600_000;
        let event = 1000;
        assert!(!click_within_window(click, event, 30));
    }
}
