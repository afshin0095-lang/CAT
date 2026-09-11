//! Adapter from the existing `NetworkAdapter` SPI to the `DiscoverySource` SPI.
//!
//! Responsibilities: commission normalization, program normalization,
//! pagination signaling, capability reporting, and provider error mapping.
//! Fail-closed behavior is preserved: unsupported filters are rejected, never
//! silently ignored; malformed provider data fails the affected program, and
//! provider failures surface as `Unavailable` (retryable, transient).

use crate::clock::{Clock, SystemClock};
use crate::network_adapter::{NetworkAdapter, NetworkProgram};
use crate::{
    DiscoveryCandidate, DiscoverySource, DiscoverySourceBatch, DiscoverySourceCapability,
    DiscoverySourceError, DiscoverySourceFuture, DiscoverySourceId, DiscoverySourceInfo,
    DiscoverySourceKind, DiscoverySourceRequest, canonical_key,
};

/// Converts a provider commission string into basis points.
///
/// Accepted forms (after trimming): `"5%"`, `"5.5%"`, `"0.05"` (fraction of
/// 1 ⇒ treated as 5%), `"0.5"` (⇒ 50%), `"100%"`. Rejected: blank values,
/// non-numeric text, NaN/infinite values, and anything above 100%.
///
/// Values at or below `1.0` without a `%` suffix are interpreted as a
/// *fraction* of the sale (the dominant machine-readable convention); values
/// above `1.0` without a suffix are interpreted as a percentage. This
/// ambiguity is a property of provider feeds; both interpretations are
/// bounds-checked so no input can produce an out-of-range rate.
fn commission_to_bps(value: &str) -> Result<u32, DiscoverySourceError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(DiscoverySourceError::InvalidResponse(
            "commission_rate must not be blank".into(),
        ));
    }
    let percent = value.ends_with('%');
    let number = value
        .trim_end_matches('%')
        .trim()
        .parse::<f64>()
        .map_err(|_| {
            DiscoverySourceError::InvalidResponse(format!("invalid commission_rate: {value}"))
        })?;
    let percentage = if percent {
        number
    } else if number <= 1.0 {
        number * 100.0
    } else {
        number
    };
    if !percentage.is_finite() || !(0.0..=100.0).contains(&percentage) {
        return Err(DiscoverySourceError::InvalidResponse(format!(
            "commission_rate out of range: {value}"
        )));
    }
    Ok((percentage * 100.0).round() as u32)
}

fn normalize_program(
    program: NetworkProgram,
    observed_at_ms: u64,
) -> Result<DiscoveryCandidate, DiscoverySourceError> {
    if program.external_id.trim().is_empty()
        || program.name.trim().is_empty()
        || program.merchant_name.trim().is_empty()
        || program.url.trim().is_empty()
    {
        return Err(DiscoverySourceError::InvalidResponse(
            "network program contains a required blank field".into(),
        ));
    }
    if program.external_id.chars().count() > crate::discovery::MAX_EXTERNAL_ID_LEN {
        return Err(DiscoverySourceError::InvalidResponse(
            "network program external_id exceeds the supported length".into(),
        ));
    }
    // The adapter emits an unknown currency: networks do not report currency
    // in this contract. Downstream stores keep this as a fact; it is never
    // silently substituted with a guessed currency.
    let candidate = DiscoveryCandidate {
        source: program.network_id.0,
        external_id: program.external_id,
        merchant_name: program.merchant_name.clone(),
        product_name: program.name.clone(),
        canonical_key: canonical_key(&program.merchant_name, &program.name),
        category: program.categories.first().cloned(),
        destination_url: program.url,
        currency: "UNKNOWN".into(),
        price_minor: None,
        commission_bps: Some(commission_to_bps(&program.commission_rate)?),
        demand_score: 5_000,
        competition_score: 5_000,
        freshness_score: 10_000,
        compliance_score: if program.accepting_applications {
            10_000
        } else {
            5_000
        },
        observed_at_ms,
    };
    candidate.validate().map_err(|error| {
        DiscoverySourceError::InvalidResponse(format!(
            "normalized program failed validation: {error}"
        ))
    })?;
    Ok(candidate)
}

/// Adapts one existing `NetworkAdapter` without exposing network-specific APIs to discovery.
pub struct NetworkDiscoveryAdapter<'a> {
    network: &'a dyn NetworkAdapter,
    info: DiscoverySourceInfo,
    clock: Box<dyn Clock + 'a>,
}

impl<'a> NetworkDiscoveryAdapter<'a> {
    pub fn new(network: &'a dyn NetworkAdapter) -> Self {
        let network_info = network.info();
        Self {
            network,
            info: DiscoverySourceInfo {
                id: DiscoverySourceId(format!("network:{}", network_info.id.0)),
                name: network_info.name.clone(),
                kind: DiscoverySourceKind::AffiliateNetwork,
                capabilities: vec![DiscoverySourceCapability::Pagination],
            },
            clock: Box::new(SystemClock::new()),
        }
    }

    /// Injects a deterministic clock (tests, replay). Production code uses
    /// [`NetworkDiscoveryAdapter::new`].
    pub fn with_clock(network: &'a dyn NetworkAdapter, clock: impl Clock + 'a) -> Self {
        let mut adapter = Self::new(network);
        adapter.clock = Box::new(clock);
        adapter
    }
}

impl<'a> DiscoverySource for NetworkDiscoveryAdapter<'a> {
    fn info(&self) -> &DiscoverySourceInfo {
        &self.info
    }

    fn discover<'b>(
        &'b self,
        request: DiscoverySourceRequest,
    ) -> DiscoverySourceFuture<'b, DiscoverySourceBatch> {
        Box::pin(async move {
            request.validate()?;
            if request.category.is_some()
                || request.geographic_market.is_some()
                || request.currency.is_some()
            {
                return Err(DiscoverySourceError::InvalidRequest(
                    "network adapter supports only query and pagination".into(),
                ));
            }
            let programs = self
                .network
                .list_programs(request.query.as_deref(), request.page, request.per_page)
                .await
                .map_err(|e| DiscoverySourceError::Unavailable(e.to_string()))?;
            let observed_at_ms = self.clock.now_ms();
            let mut candidates = Vec::with_capacity(programs.len());
            for program in programs {
                candidates.push(normalize_program(program, observed_at_ms)?);
            }
            let has_more = candidates.len() == request.per_page as usize && request.per_page > 0;
            let next_page = has_more.then(|| request.page.saturating_add(1));
            Ok(DiscoverySourceBatch {
                candidates,
                has_more,
                next_page,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::FixedClock;
    use crate::{AffiliateDomainResult, NetworkConversion, NetworkId, NetworkInfo};
    use std::future::{Future, ready};
    use std::pin::Pin;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    struct Stub {
        info: NetworkInfo,
        programs: Vec<NetworkProgram>,
        fail: bool,
    }
    impl NetworkAdapter for Stub {
        fn info(&self) -> &NetworkInfo {
            &self.info
        }
        fn list_programs(
            &self,
            _: Option<&str>,
            _: u32,
            _: u32,
        ) -> Pin<Box<dyn Future<Output = AffiliateDomainResult<Vec<NetworkProgram>>> + Send + '_>>
        {
            if self.fail {
                Box::pin(ready(Err(crate::AffiliateDomainError::RepositoryNotFound(
                    "programs",
                ))))
            } else {
                Box::pin(ready(Ok(self.programs.clone())))
            }
        }
        fn generate_link(
            &self,
            _: &str,
            _: &str,
            _: &str,
        ) -> Pin<Box<dyn Future<Output = AffiliateDomainResult<String>> + Send + '_>> {
            Box::pin(ready(Ok("https://example.test/link".into())))
        }
        fn fetch_conversions(
            &self,
            _: i64,
            _: i64,
        ) -> Pin<Box<dyn Future<Output = AffiliateDomainResult<Vec<NetworkConversion>>> + Send + '_>>
        {
            Box::pin(ready(Ok(vec![])))
        }
        fn validate_credentials(
            &self,
        ) -> Pin<Box<dyn Future<Output = AffiliateDomainResult<bool>> + Send + '_>> {
            Box::pin(ready(Ok(true)))
        }
    }
    fn network(programs: Vec<NetworkProgram>) -> Stub {
        Stub {
            info: NetworkInfo {
                id: NetworkId("test".into()),
                name: "Test".into(),
                base_url: "https://test.invalid".into(),
                supports_real_time_reporting: false,
                supports_deep_linking: true,
                default_cookie_days: 30,
            },
            programs,
            fail: false,
        }
    }
    fn program(rate: &str) -> NetworkProgram {
        NetworkProgram {
            external_id: "p1".into(),
            network_id: NetworkId("test".into()),
            name: "Widget".into(),
            merchant_name: "Acme".into(),
            commission_rate: rate.into(),
            cookie_days: 30,
            categories: vec!["electronics".into()],
            url: "https://merchant.invalid/widget".into(),
            description: None,
            accepting_applications: true,
        }
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        fn clone(_: *const ()) -> RawWaker {
            raw_waker()
        }
        fn wake(_: *const ()) {}
        fn wake_by_ref(_: *const ()) {}
        fn drop(_: *const ()) {}
        fn raw_waker() -> RawWaker {
            RawWaker::new(
                std::ptr::null(),
                &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
            )
        }
        let waker = unsafe { Waker::from_raw(raw_waker()) };
        let mut context = Context::from_waker(&waker);
        let mut future = std::pin::pin!(future);
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(value) => return value,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    #[test]
    fn commission_rates_are_normalized() {
        assert_eq!(commission_to_bps("5%").unwrap(), 500);
        assert_eq!(commission_to_bps("5.5%").unwrap(), 550);
        assert_eq!(commission_to_bps("0.05").unwrap(), 500);
        assert_eq!(commission_to_bps("0.5").unwrap(), 5_000);
        assert_eq!(commission_to_bps("100%").unwrap(), 10_000);
        assert_eq!(commission_to_bps("7%").unwrap(), 700);
        assert_eq!(commission_to_bps(" 7 % ").unwrap(), 700);
        assert_eq!(
            commission_to_bps("1.0").unwrap(),
            10_000,
            "fraction 1.0 means 100%"
        );
    }

    #[test]
    fn invalid_commission_rates_fail_closed() {
        assert!(commission_to_bps("").is_err());
        assert!(commission_to_bps("   ").is_err());
        assert!(commission_to_bps("abc").is_err());
        assert!(commission_to_bps("101%").is_err());
        assert!(commission_to_bps("150").is_err());
        assert!(commission_to_bps("-5%").is_err());
        assert!(commission_to_bps("NaN%").is_err());
        assert!(commission_to_bps("inf").is_err());
        assert!(
            commission_to_bps("1e400%").is_err(),
            "parses as +inf, rejected"
        );
    }

    #[test]
    fn adapter_exposes_only_supported_capability() {
        let n = network(vec![]);
        let a = NetworkDiscoveryAdapter::with_clock(&n, FixedClock::new(42));
        assert_eq!(a.info().id, DiscoverySourceId("network:test".into()));
        assert!(a.info().supports(DiscoverySourceCapability::Pagination));
        assert!(!a.info().supports(DiscoverySourceCapability::CurrencyFilter));
    }

    #[test]
    fn normalization_preserves_identity_and_timestamp() {
        let result = normalize_program(program("7.5%"), 42).unwrap();
        assert_eq!(result.source, "test");
        assert_eq!(result.commission_bps, Some(750));
        assert_eq!(result.canonical_key, "acme:widget");
        assert_eq!(result.observed_at_ms, 42);
        assert_eq!(result.currency, "UNKNOWN");
        assert!(
            result.validate().is_ok(),
            "normalized candidates satisfy domain validation"
        );
    }

    #[test]
    fn blank_program_fields_are_rejected() {
        let mut blank = program("5%");
        blank.merchant_name = "  ".into();
        assert!(matches!(
            normalize_program(blank, 1),
            Err(DiscoverySourceError::InvalidResponse(_))
        ));
        let mut blank_url = program("5%");
        blank_url.url = "".into();
        assert!(normalize_program(blank_url, 1).is_err());
    }

    #[test]
    fn oversized_external_ids_are_rejected() {
        let mut oversized = program("5%");
        oversized.external_id = "p".repeat(crate::discovery::MAX_EXTERNAL_ID_LEN + 1);
        assert!(matches!(
            normalize_program(oversized, 1),
            Err(DiscoverySourceError::InvalidResponse(_))
        ));
    }

    #[test]
    fn unsupported_filters_are_rejected_never_ignored() {
        let n = network(vec![]);
        let a = NetworkDiscoveryAdapter::with_clock(&n, FixedClock::new(1));
        let request = DiscoverySourceRequest {
            per_page: 10,
            category: Some("electronics".into()),
            ..Default::default()
        };
        assert!(matches!(
            block_on(a.discover(request)),
            Err(DiscoverySourceError::InvalidRequest(_))
        ));
    }

    #[test]
    fn provider_failures_map_to_transient_unavailable() {
        let mut n = network(vec![]);
        n.fail = true;
        let a = NetworkDiscoveryAdapter::with_clock(&n, FixedClock::new(1));
        let result = block_on(a.discover(DiscoverySourceRequest {
            per_page: 10,
            ..Default::default()
        }));
        assert!(matches!(&result, Err(DiscoverySourceError::Unavailable(_))));
        // Error classification: retryable transient failure.
        let error = result.unwrap_err();
        assert!(crate::ErrorClassification::is_retryable(&error));
    }

    #[test]
    fn pagination_signals_a_saturating_next_page() {
        let programs: Vec<NetworkProgram> = (0..3).map(|_| program("5%")).collect();
        let n = network(programs);
        let a = NetworkDiscoveryAdapter::with_clock(&n, FixedClock::new(7));

        let batch = block_on(a.discover(DiscoverySourceRequest {
            page: 1,
            per_page: 3,
            ..Default::default()
        }))
        .expect("valid batch");
        assert!(batch.has_more);
        assert_eq!(batch.next_page, Some(2));
        assert_eq!(batch.candidates.len(), 3);
        assert!(
            batch
                .candidates
                .iter()
                .all(|candidate| candidate.observed_at_ms == 7)
        );

        // Last page: fewer candidates than per_page ends pagination.
        let batch = block_on(a.discover(DiscoverySourceRequest {
            page: 2,
            per_page: 5,
            ..Default::default()
        }))
        .expect("valid batch");
        assert!(!batch.has_more);
        assert_eq!(batch.next_page, None);
    }

    #[test]
    fn pagination_page_overflow_saturates_without_panicking() {
        let programs: Vec<NetworkProgram> = (0..2).map(|_| program("5%")).collect();
        let n = network(programs);
        let a = NetworkDiscoveryAdapter::with_clock(&n, FixedClock::new(7));
        let batch = block_on(a.discover(DiscoverySourceRequest {
            page: u32::MAX,
            per_page: 2,
            ..Default::default()
        }))
        .expect("valid batch");
        assert!(batch.has_more);
        assert_eq!(
            batch.next_page,
            Some(u32::MAX),
            "page counter saturates instead of wrapping"
        );
    }

    #[test]
    fn zero_per_page_is_rejected_before_the_provider_is_called() {
        let n = network(vec![]);
        let a = NetworkDiscoveryAdapter::with_clock(&n, FixedClock::new(1));
        assert!(
            block_on(a.discover(DiscoverySourceRequest {
                per_page: 0,
                ..Default::default()
            }))
            .is_err()
        );
    }
}
