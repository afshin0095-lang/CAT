use crate::OpportunityRecord;

/// Lifecycle classification derived from the age of the most recent observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpportunityLifecycleState {
    Active,
    Stale,
    Expired,
}

/// Time policy for opportunity freshness.
///
/// Thresholds are expressed in milliseconds and must satisfy
/// `0 < stale_after_ms < expire_after_ms`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpportunityLifecyclePolicy {
    pub stale_after_ms: u64,
    pub expire_after_ms: u64,
}

impl OpportunityLifecyclePolicy {
    pub fn new(stale_after_ms: u64, expire_after_ms: u64) -> Result<Self, OpportunityLifecycleError> {
        if stale_after_ms == 0 || stale_after_ms >= expire_after_ms {
            return Err(OpportunityLifecycleError::InvalidPolicy);
        }
        Ok(Self {
            stale_after_ms,
            expire_after_ms,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpportunityLifecycleSnapshot {
    pub state: OpportunityLifecycleState,
    pub age_ms: u64,
    pub last_observed_at_ms: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpportunityLifecycleError {
    InvalidPolicy,
    ClockBeforeObservation,
}

impl std::fmt::Display for OpportunityLifecycleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPolicy => formatter.write_str("lifecycle policy must satisfy 0 < stale_after_ms < expire_after_ms"),
            Self::ClockBeforeObservation => formatter.write_str("evaluation time cannot precede the last observation"),
        }
    }
}

impl std::error::Error for OpportunityLifecycleError {}

/// Deterministically evaluates freshness without mutating the persisted opportunity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpportunityLifecycleEvaluator {
    policy: OpportunityLifecyclePolicy,
}

impl OpportunityLifecycleEvaluator {
    pub fn new(policy: OpportunityLifecyclePolicy) -> Self {
        Self { policy }
    }

    pub fn evaluate(&self, record: &OpportunityRecord, now_ms: u64) -> Result<OpportunityLifecycleSnapshot, OpportunityLifecycleError> {
        let last_observed_at_ms = record.last_observed_at_ms;
        let age_ms = now_ms
            .checked_sub(last_observed_at_ms)
            .ok_or(OpportunityLifecycleError::ClockBeforeObservation)?;

        let state = if age_ms >= self.policy.expire_after_ms {
            OpportunityLifecycleState::Expired
        } else if age_ms >= self.policy.stale_after_ms {
            OpportunityLifecycleState::Stale
        } else {
            OpportunityLifecycleState::Active
        };

        Ok(OpportunityLifecycleSnapshot {
            state,
            age_ms,
            last_observed_at_ms,
        })
    }

    pub fn policy(&self) -> OpportunityLifecyclePolicy {
        self.policy
    }
}

/// Produces a deterministic lifecycle view for a batch of opportunities.
pub fn evaluate_records(
    records: &[OpportunityRecord],
    evaluator: OpportunityLifecycleEvaluator,
    now_ms: u64,
) -> Result<Vec<(String, OpportunityLifecycleSnapshot)>, OpportunityLifecycleError> {
    let mut snapshots = records
        .iter()
        .map(|record| {
            evaluator
                .evaluate(record, now_ms)
                .map(|snapshot| (record.identity.as_str().to_owned(), snapshot))
        })
        .collect::<Result<Vec<_>, _>>()?;

    snapshots.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(snapshots)
}
