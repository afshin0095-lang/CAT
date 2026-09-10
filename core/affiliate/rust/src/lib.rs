pub mod attribution;
pub mod clock;
pub mod commission_rules;
pub mod discovery;
pub mod discovery_source;
pub mod discovery_source_health;
pub mod error;
pub mod events;
pub mod memory_repository;
pub mod model;
pub mod network_adapter;
pub mod network_discovery_adapter;
pub mod observability;
pub mod opportunity_freshness;
pub mod opportunity_health;
pub mod opportunity_ingestion;
pub mod opportunity_lifecycle;
pub mod opportunity_postgres;
pub mod opportunity_projection;
pub mod opportunity_query;
pub mod opportunity_ranking;
pub mod opportunity_revalidation;
pub mod opportunity_store;
pub mod opportunity_version;
pub mod repository;
pub mod revalidation_planner;
pub mod scoring;
pub mod service;
pub mod tracking;
pub mod smart_router;
pub mod monitoring;

pub use error::{AffiliateDomainError, AffiliateDomainResult, ErrorCategory, ErrorClassification};
pub use events::{AffiliateRegistered, CommissionObligationCreated, ConversionStateChanged, MerchantCreated, OfferCreated, OpportunityBecameExpired, OpportunityBecameStale, OpportunityDiscovered, OpportunityRevalidated, OpportunityRevalidationFailed, OpportunityRevalidationRequested, OpportunityUpdated, ProgramCreated, ProgramStatusChanged, ReferralStateChanged};
pub use memory_repository::InMemoryAffiliateRepository;
pub use model::{Affiliate, AffiliateId, AffiliateKind, CommissionObligation, CommissionObligationId, Conversion, ConversionId, ConversionState, Merchant, MerchantId, Offer, OfferId, OfferStatus, Product, ProductId, Program, ProgramId, ProgramStatus, Referral, ReferralId, ReferralState, VersionedName};
pub use repository::AffiliateRepository;
pub use service::AffiliateDomain;
pub use attribution::{apply_model, click_within_window, Attribution, AttributionId, AttributionModel, AttributionResult, AttributionTouch, ConversionEvent, ConversionEventId, ConversionEventType};
pub use commission_rules::{compute_commission, recurring_cap_exceeded, CommissionRule, CommissionRuleBuilder, CommissionSubRule, CommissionTrigger, CommissionType};
pub use clock::{Clock, FixedClock, FnClock, SystemClock};
pub use discovery::{canonical_key, DiscoveryCandidate, DiscoveryEngine, DiscoveryError, DiscoveryOpportunity, DiscoveryRequest, DiscoveryResult};
pub use discovery_source::{DiscoveryIngestion, DiscoverySource, DiscoverySourceBatch, DiscoverySourceCapability, DiscoverySourceError, DiscoverySourceFuture, DiscoverySourceId, DiscoverySourceInfo, DiscoverySourceKind, DiscoverySourceRegistry, DiscoverySourceRequest};
pub use discovery_source_health::{
    InMemorySourceHealthStore, SourceHealthSnapshot, SourceHealthState, SourceHealthStore,
    DEGRADED_AFTER_CONSECUTIVE_FAILURES, UNAVAILABLE_AFTER_CONSECUTIVE_FAILURES,
};
pub use network_adapter::{NetworkAdapter, NetworkConversion, NetworkId, NetworkInfo, NetworkProgram, NetworkRegistry};
pub use network_discovery_adapter::NetworkDiscoveryAdapter;
pub use observability::{ingestion_report_metrics, lifecycle_batch_metrics, metric_names, revalidation_status_metrics, InMemoryMetricSink, MetricSample, MetricSink};
pub use opportunity_freshness::{FreshnessEvaluation, FreshnessPolicy, FreshnessPolicyError, FreshnessState, RecordFreshnessEvaluation};
pub use opportunity_health::{HealthConcern, OpportunityHealth, OpportunityHealthAssessment, OpportunityHealthAssessor};
pub use opportunity_ingestion::{OpportunityIngestion, OpportunityIngestionError, OpportunityIngestionReport};
pub use opportunity_store::{InMemoryOpportunityStore, OpportunityIdentity, OpportunityObservation, OpportunityRecord, OpportunityStore, OpportunityStoreError, OpportunityUpsertResult};
pub use opportunity_lifecycle::{evaluate_records, evaluate_records_batch, LifecycleEvaluation, LifecycleEvaluationBatch, LifecycleStateCounts, OpportunityLifecycleError, OpportunityLifecycleEvaluator, OpportunityLifecyclePolicy, OpportunityLifecycleSnapshot, OpportunityLifecycleState};
pub use opportunity_postgres::{AsyncOpportunityStore, AsyncRevalidationRequestStore, AsyncVersionedOpportunityStore, PostgresOpportunityStore, PostgresOpportunityStoreError, PostgresRevalidationStore};
pub use opportunity_projection::{BestObservationView, OpportunityProjectionError, OpportunityStatusProjector, OpportunityStatusView};
pub use opportunity_query::{OpportunityFilter, OpportunityQuery, OpportunityQueryError, OpportunityQueryResult, OpportunityQueryService, OpportunitySort, OpportunitySortField, SortDirection};
pub use opportunity_ranking::{
    OpportunityRanker, OpportunityRankingError, OpportunityRankingProfile, RankedOpportunity, RankingFactors,
    WEIGHT_SCALE, DEFAULT_COMPOSITE_SCORE_WEIGHT_BPS, DEFAULT_FRESHNESS_WEIGHT_BPS, DEFAULT_ECONOMIC_VALUE_WEIGHT_BPS,
    DEFAULT_SOURCE_RELIABILITY_WEIGHT_BPS, UNKNOWN_SOURCE_RELIABILITY_BPS,
};
pub use opportunity_revalidation::{
    InMemoryRevalidationRequestStore, RevalidationBlockReason, RevalidationDecision, RevalidationPriority,
    RevalidationReason, RevalidationRequest, RevalidationRequestRecord, RevalidationRequestStore,
    RevalidationSkipReason, RevalidationStatus, RevalidationStoreError, RevalidationTarget,
    REVALIDATION_DEDUP_WINDOW_MS,
};
pub use opportunity_version::{OpportunityRevision, OpportunityRevisionError};
pub use revalidation_planner::{RevalidationPlanner, RevalidationPlanningOutcome};
pub use scoring::{rank_programs, score_content_potential, score_earning_potential, ProgramScore, ProgramVerdict};
pub use tracking::{build_utm_link, Click, ClickFraudFlag, ClickId, ExternalUserId, Identity, IdentityId, Link, LinkId, UtmParams, VelocityChecker};
pub use smart_router::{LinkHealth, RouteCandidate, RouteRequest, SmartRouter};
pub use monitoring::{build_daily_report, DailyMonitoringReport, FindingSeverity, MonitoringFinding, MonitoringInput};

pub const DOMAIN_NAME: &str = "affiliate";
pub const DOMAIN_VERSION: u16 = 3;
