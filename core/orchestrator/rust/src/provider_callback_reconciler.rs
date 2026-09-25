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
            return Err(crate::OrchestratorError::InvalidAuthorizationInput(
                "provider callback reconciliation requires a provider".into(),
            ));
        }

        Ok(Self {
            store,
            provider,
            batch_size: batch_size.clamp(1, 500),
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
        assert_eq!(ProviderCallbackReconciliationWorker::<DummyStore>::normalize_batch_size(0), 1);
        assert_eq!(ProviderCallbackReconciliationWorker::<DummyStore>::normalize_batch_size(900), 500);
    }

    struct DummyStore;

    #[async_trait::async_trait]
    impl ProviderCallbackStore for DummyStore {
        async fn ingest_callback(
            &self,
            _callback: crate::ProviderCallback,
        ) -> OrchestratorResult<crate::ProviderCallbackRecord> {
            unreachable!()
        }

        async fn list_unmatched_callbacks(
            &self,
            _provider: &str,
            _limit: u32,
        ) -> OrchestratorResult<Vec<crate::ProviderCallbackRecord>> {
            unreachable!()
        }

        async fn reconcile_unmatched_callback(
            &self,
            _callback_id: uuid::Uuid,
            _reconciled_at_ms: u64,
        ) -> OrchestratorResult<crate::ProviderCallbackReplayResult> {
            unreachable!()
        }
    }
}