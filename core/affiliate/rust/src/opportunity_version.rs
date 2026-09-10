//! Explicit revision (version) concept for persisted opportunity aggregates.
//!
//! Every persisted `OpportunityRecord` carries a monotonic revision. The
//! revision is a *fact about writes*, not derived lifecycle state:
//!
//! - created records start at [`OpportunityRevision::INITIAL`];
//! - every persisted change performs a checked increment;
//! - readers can supply an expected revision for optimistic concurrency;
//! - wraparound is impossible (overflow is an explicit error, never silent).

use serde::{Deserialize, Serialize};

/// Smallest valid revision; every persisted opportunity starts here.
pub const INITIAL_REVISION: u64 = 1;

/// Monotonic revision of an opportunity aggregate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct OpportunityRevision(u64);

impl OpportunityRevision {
    pub const INITIAL: Self = Self(INITIAL_REVISION);

    /// Revision of a newly created aggregate.
    pub const fn initial() -> Self {
        Self::INITIAL
    }

    /// Validates a revision read from an external store.
    ///
    /// Zero is rejected: revisions are 1-based by contract, so a zero value
    /// means corrupted or foreign data and must fail closed.
    pub const fn from_raw(value: u64) -> Result<Self, OpportunityRevisionError> {
        if value == 0 {
            return Err(OpportunityRevisionError::Zero);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    /// Checked successor revision.
    ///
    /// `u64::MAX` cannot be incremented; instead of wrapping silently, this
    /// returns [`OpportunityRevisionError::Overflow`]. Reaching it would
    /// require more writes than any storage system can persist.
    pub fn next(self) -> Result<Self, OpportunityRevisionError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(OpportunityRevisionError::Overflow)
    }
}

impl std::fmt::Display for OpportunityRevision {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "v{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpportunityRevisionError {
    /// External data carried the invalid revision `0`.
    Zero,
    /// Increment would exceed `u64::MAX`; refused instead of wrapping.
    Overflow,
}

impl std::fmt::Display for OpportunityRevisionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => formatter.write_str("revision must be at least 1"),
            Self::Overflow => formatter.write_str("revision increment would overflow"),
        }
    }
}

impl std::error::Error for OpportunityRevisionError {}

/// Outcome of an optimistic-concurrency check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RevisionCheck {
    /// Stored revision matches the expectation.
    Matched,
    /// Stored revision differs; the caller must reload and re-plan.
    Mismatch {
        expected: OpportunityRevision,
        actual: OpportunityRevision,
    },
}

impl RevisionCheck {
    pub fn evaluate(expected: OpportunityRevision, actual: u64) -> Result<Self, OpportunityRevisionError> {
        let actual = OpportunityRevision::from_raw(actual)?;
        if actual == expected {
            Ok(Self::Matched)
        } else {
            Ok(Self::Mismatch { expected, actual })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revisions_start_at_one_and_increment_monotonically() {
        let initial = OpportunityRevision::initial();
        assert_eq!(initial.get(), 1);
        assert_eq!(initial.next().expect("first increment").get(), 2);
        assert!(initial < initial.next().expect("first increment"));
    }

    #[test]
    fn zero_is_rejected_on_read() {
        assert_eq!(OpportunityRevision::from_raw(0), Err(OpportunityRevisionError::Zero));
        assert_eq!(
            OpportunityRevision::initial().get(),
            OpportunityRevision::from_raw(1).expect("valid revision").get()
        );
    }

    #[test]
    fn overflow_is_explicit_not_silent() {
        let max = OpportunityRevision::from_raw(u64::MAX).expect("valid revision");
        assert_eq!(max.next(), Err(OpportunityRevisionError::Overflow));
    }

    #[test]
    fn revision_check_reports_mismatch_with_actual_value() {
        assert_eq!(
            RevisionCheck::evaluate(OpportunityRevision::initial(), 1),
            Ok(RevisionCheck::Matched)
        );
        assert_eq!(
            RevisionCheck::evaluate(OpportunityRevision::initial(), 7),
            Ok(RevisionCheck::Mismatch {
                expected: OpportunityRevision::initial(),
                actual: OpportunityRevision::from_raw(7).expect("valid revision"),
            })
        );
        assert_eq!(RevisionCheck::evaluate(OpportunityRevision::initial(), 0), Err(OpportunityRevisionError::Zero));
    }

    #[test]
    fn revision_survives_serialization_round_trip() {
        let revision = OpportunityRevision::from_raw(42).expect("valid revision");
        let encoded = serde_json::to_string(&revision).expect("serialize");
        assert_eq!(encoded, "42");
        let decoded: OpportunityRevision = serde_json::from_str(&encoded).expect("deserialize");
        assert_eq!(decoded, revision);
    }
}
