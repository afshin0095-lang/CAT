use crate::{
    OrchestratorResult, ProviderCallbackReplayDisposition, ProviderCallbackReplayResult,
    ProviderCallbackStore,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProviderCallbackReconciliationReport {
    pub scanned: u32,
    pub correlated: u32,
    pub still_unmatched: u32,
    pub rejected: u32,
    pub already_handled: u32,
}

pub struct ProviderCallbackReconciliationWorker<'a, S> {
    pub store: &'a S,
    pub provider: String,
    pub batch_size: u32,
}

impl<'a, S> ProviderCallbackReconciliationWorker<'a, S>
where
    S: ProviderCallbackStore,
{
    pub fn new(store: &'a S, provider: impl Into<String>, batch_size: u32) -> OrchestratorResult<Self> {
        let provider = provider.into();
        if provider.trim().is_empty() {
            return Err(crate::OrchestratorError::Serialization(
                "provider callback reconciliation requires a provider".into(),
            ));
        }

        Ok(Self {
            store,
            provider,
            batch_size: normalize_batch_size(batch_size),
        })
    }

    pub async fn run_once(
        &self,
        reconciled_at_ms: u64,
    ) -> OrchestratorResult<ProviderCallbackReconciliationReport> {
        if reconciled_at_ms == 0 {
            return Err(crate::OrchestratorError::Serialization(
                "callback reconciliation timestamp must be greater than zero".into(),
            ));
        }

        let callbacks = self
            .store
            .list_unmatched_callbacks(&self.provider, self.batch_size)
            .await?;

        let mut report = ProviderCallbackReconciliationReport {
            scanned: callbacks.len() as u32,
            ..Default::default()
        };

        for callback in callbacks {
            let result = self
                .store
                .reconcile_unmatched_callback(callback.callback.callback_id, reconciled_at_ms)
                .await?;

            match result.disposition {
                ProviderCallbackReplayDisposition::Correlated => report.correlated += 1,
                ProviderCallbackReplayDisposition::StillUnmatched => report.still_unmatched += 1,
                ProviderCallbackReplayDisposition::Rejected => report.rejected += 1,
                ProviderCallbackReplayDisposition::AlreadyHandled => report.already_handled += 1,
            }
        }

        Ok(report)
    }
}

fn normalize_batch_size(value: u32) -> u32 {
    value.clamp(1, 500)
}

impl ProviderCallbackReconciliationReport {
    pub fn record(result: &ProviderCallbackReplayResult) -> Self {
        let mut report = Self { scanned: 1, ..Default::default() };
        match result.disposition {
            ProviderCallbackReplayDisposition::Correlated => report.correlated = 1,
            ProviderCallbackReplayDisposition::StillUnmatched => report.still_unmatched = 1,
            ProviderCallbackReplayDisposition::Rejected => report.rejected = 1,
            ProviderCallbackReplayDisposition::AlreadyHandled => report.already_handled = 1,
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_size_is_bounded() {
        assert_eq!(normalize_batch_size(0), 1);
        assert_eq!(normalize_batch_size(900), 500);
    }

}