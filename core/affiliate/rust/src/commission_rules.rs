//! Compound commission rules engine.
//!
//! Inspired by OpenPartner's `CommissionSubRule` system. A program can
//! define multiple commission sub-rules that trigger independently on
//! the same event. One event can produce multiple commission records.
//!
//! ## Triggers
//!
//! - `Every`       — fires on every matching event
//! - `First`       — fires only on the FIRST event of this type for (affiliate, user)
//! - `Subsequent`  — fires on every event EXCEPT the first for (affiliate, user)
//!
//! ## Types
//!
//! - `Percent`     — percentage of event value (e.g. 20 = 20%)
//! - `Fixed`       — flat amount in minor units (e.g. 5000 = $50.00)
//!
//! ## Caps
//!
//! `recurring_months` limits how long a recurring rule keeps firing,
//! measured from the first attributed event of that type.

use serde::{Deserialize, Serialize};

use crate::attribution::ConversionEventType;

// ---------------------------------------------------------------------------
// Commission rule types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CommissionTrigger {
    /// Fires on every matching event.
    Every,
    /// Fires only on the first event of this type for (affiliate, user).
    First,
    /// Fires on every event EXCEPT the first for (affiliate, user).
    Subsequent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CommissionType {
    /// Percentage of event value. `value` = 20 means 20%.
    Percent,
    /// Fixed amount in minor units. `value` = 5000 means $50.00.
    Fixed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommissionSubRule {
    pub trigger: CommissionTrigger,
    /// Event type filter. `None` = matches all event types.
    pub event_type: Option<ConversionEventType>,
    pub commission_type: CommissionType,
    /// For Percent: 20 means 20%. For Fixed: amount in minor units.
    pub value: i64,
    /// Currency for fixed rules (e.g. "USD").
    pub currency: Option<String>,
    /// If true, rule keeps firing on subsequent matching events (renewals).
    pub recurring: bool,
    /// Cap in months. `None` = no cap. Only meaningful when `recurring = true`.
    pub recurring_months: Option<u32>,
}

/// A program's commission configuration is an ordered list of sub-rules.
pub type CommissionRule = Vec<CommissionSubRule>;

// ---------------------------------------------------------------------------
// Commission computation (pure, no IO)
// ---------------------------------------------------------------------------

/// Compute the commission amount for a single sub-rule against an event value.
/// Returns the amount in minor units, or 0 if not applicable.
///
/// `weight` is the attribution weight (0.0 .. 1.0) from the attribution model.
pub fn compute_commission(
    rule: &CommissionSubRule,
    event_value_minor: Option<i64>,
    weight: f64,
) -> i64 {
    let base = match rule.commission_type {
        CommissionType::Fixed => rule.value,
        CommissionType::Percent => {
            let revenue = event_value_minor.unwrap_or(0);
            (revenue * rule.value) / 100
        }
    };

    (base as f64 * weight).round() as i64
}

/// Check whether a recurring rule has exceeded its month cap.
pub fn recurring_cap_exceeded(
    rule: &CommissionSubRule,
    first_event_ms: i64,
    current_event_ms: i64,
) -> bool {
    match rule.recurring_months {
        None => false,
        Some(months) => {
            let month_ms: i64 = (30.4375 * 24.0 * 60.0 * 60.0 * 1000.0) as i64;
            let cap_ms = months as i64 * month_ms;
            (current_event_ms - first_event_ms) > cap_ms
        }
    }
}

// ---------------------------------------------------------------------------
// Builder for easy rule construction
// ---------------------------------------------------------------------------

pub struct CommissionRuleBuilder {
    rules: Vec<CommissionSubRule>,
}

impl CommissionRuleBuilder {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    /// Add a percentage commission on every event.
    pub fn percent_every(mut self, percent: i64) -> Self {
        self.rules.push(CommissionSubRule {
            trigger: CommissionTrigger::Every,
            event_type: None,
            commission_type: CommissionType::Percent,
            value: percent,
            currency: None,
            recurring: false,
            recurring_months: None,
        });
        self
    }

    /// Add a recurring percentage commission.
    pub fn percent_recurring(mut self, percent: i64, months: Option<u32>) -> Self {
        self.rules.push(CommissionSubRule {
            trigger: CommissionTrigger::Every,
            event_type: None,
            commission_type: CommissionType::Percent,
            value: percent,
            currency: None,
            recurring: true,
            recurring_months: months,
        });
        self
    }

    /// Add a fixed first-sale bonus.
    pub fn fixed_first(mut self, amount_minor: i64, currency: &str) -> Self {
        self.rules.push(CommissionSubRule {
            trigger: CommissionTrigger::First,
            event_type: Some(ConversionEventType::Purchase),
            commission_type: CommissionType::Fixed,
            value: amount_minor,
            currency: Some(currency.into()),
            recurring: false,
            recurring_months: None,
        });
        self
    }

    /// Add a recurring fixed amount for subsequent events.
    pub fn fixed_subsequent_recurring(
        mut self,
        amount_minor: i64,
        currency: &str,
        months: Option<u32>,
    ) -> Self {
        self.rules.push(CommissionSubRule {
            trigger: CommissionTrigger::Subsequent,
            event_type: None,
            commission_type: CommissionType::Fixed,
            value: amount_minor,
            currency: Some(currency.into()),
            recurring: true,
            recurring_months: months,
        });
        self
    }

    pub fn build(self) -> CommissionRule {
        self.rules
    }
}

impl Default for CommissionRuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_commission_basic() {
        let rule = CommissionSubRule {
            trigger: CommissionTrigger::Every,
            event_type: None,
            commission_type: CommissionType::Percent,
            value: 20,
            currency: None,
            recurring: false,
            recurring_months: None,
        };
        assert_eq!(compute_commission(&rule, Some(10000), 1.0), 2000);
    }

    #[test]
    fn percent_commission_with_weight() {
        let rule = CommissionSubRule {
            trigger: CommissionTrigger::Every,
            event_type: None,
            commission_type: CommissionType::Percent,
            value: 30,
            currency: None,
            recurring: false,
            recurring_months: None,
        };
        assert_eq!(compute_commission(&rule, Some(5000), 0.4), 600);
    }

    #[test]
    fn fixed_commission() {
        let rule = CommissionSubRule {
            trigger: CommissionTrigger::First,
            event_type: None,
            commission_type: CommissionType::Fixed,
            value: 5000,
            currency: Some("USD".into()),
            recurring: false,
            recurring_months: None,
        };
        assert_eq!(compute_commission(&rule, None, 1.0), 5000);
    }

    #[test]
    fn recurring_cap_not_exceeded() {
        let rule = CommissionSubRule {
            trigger: CommissionTrigger::Every,
            event_type: None,
            commission_type: CommissionType::Percent,
            value: 20,
            currency: None,
            recurring: true,
            recurring_months: Some(12),
        };
        let first = 0i64;
        let current = 6 * 30 * 24 * 60 * 60 * 1000;
        assert!(!recurring_cap_exceeded(&rule, first, current));
    }

    #[test]
    fn recurring_cap_exceeded_after_13_months() {
        let rule = CommissionSubRule {
            trigger: CommissionTrigger::Every,
            event_type: None,
            commission_type: CommissionType::Percent,
            value: 20,
            currency: None,
            recurring: true,
            recurring_months: Some(12),
        };
        let first = 0i64;
        let current = 13 * 31 * 24 * 60 * 60 * 1000;
        assert!(recurring_cap_exceeded(&rule, first, current));
    }

    #[test]
    fn builder_creates_compound_rules() {
        let rules = CommissionRuleBuilder::new()
            .fixed_first(10000, "USD")
            .percent_recurring(20, Some(12))
            .build();

        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].trigger, CommissionTrigger::First);
        assert_eq!(rules[0].commission_type, CommissionType::Fixed);
        assert_eq!(rules[1].trigger, CommissionTrigger::Every);
        assert!(rules[1].recurring);
    }
}
