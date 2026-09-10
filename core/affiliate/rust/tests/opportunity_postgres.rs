//! PostgreSQL integration tests for the hardened opportunity store and the
//! revalidation request store.
//!
//! These tests require a real PostgreSQL database and are skipped (never
//! hard-failed) when neither `CAT_TEST_DATABASE_URL` nor `DATABASE_URL` is
//! configured. CI provides the database through a service container; local
//! runs without PostgreSQL stay green.

use cat_affiliate::{
    canonical_key, AsyncOpportunityStore, AsyncRevalidationRequestStore, AsyncVersionedOpportunityStore,
    DiscoveryCandidate, DiscoveryOpportunity, OpportunityIdentity, OpportunityRevision, PostgresOpportunityStore,
    PostgresOpportunityStoreError, PostgresRevalidationStore, RevalidationPriority, RevalidationReason,
    RevalidationRequest, RevalidationStatus, RevalidationStoreError, RevalidationTarget,
};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

async fn connect() -> Option<sqlx::PgPool> {
    let url = std::env::var("CAT_TEST_DATABASE_URL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            std::env::var("DATABASE_URL")
                .ok()
                .filter(|value| !value.trim().is_empty())
        })?;
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&url)
        .await
        .expect("connect to the configured test database");
    Some(pool)
}

fn opportunity(identity_tag: &str, score: u32, observed_at_ms: u64) -> DiscoveryOpportunity {
    let candidate = DiscoveryCandidate {
        source: format!("network-{identity_tag}"),
        external_id: format!("sku-{identity_tag}"),
        merchant_name: "Acme".into(),
        product_name: format!("Pg Widget {identity_tag}"),
        canonical_key: canonical_key("Acme", &format!("Pg Widget {identity_tag}")),
        category: Some("electronics".into()),
        destination_url: "https://example.test/widget".into(),
        currency: "EUR".into(),
        price_minor: Some(9_999),
        commission_bps: Some(500),
        demand_score: 8_000,
        competition_score: 2_000,
        freshness_score: 10_000,
        compliance_score: 10_000,
        observed_at_ms,
    };
    DiscoveryOpportunity { id: Uuid::now_v7(), candidate, score, rank: 1 }
}

#[tokio::test]
async fn postgres_store_upsert_get_and_revision_conflicts() {
    let Some(pool) = connect().await else {
        eprintln!("skipping: no CAT_TEST_DATABASE_URL/DATABASE_URL configured");
        return;
    };
    let store = PostgresOpportunityStore::new(pool.clone());
    store.ensure_schema().await.expect("schema bootstrap is idempotent");
    store.ensure_schema().await.expect("second bootstrap is a no-op");

    let tag = Uuid::now_v7().simple().to_string();
    let first = opportunity(&tag, 7_000, 1_000);

    // Create.
    let created = store.upsert(&first).await.expect("create");
    assert!(created.created);
    assert_eq!(created.revision, OpportunityRevision::initial());

    // Idempotent repeat: unchanged observation, same revision.
    let repeat = store.upsert(&first).await.expect("repeat");
    assert!(!repeat.created);
    assert!(!repeat.changed);
    assert_eq!(repeat.revision, OpportunityRevision::initial());

    // Read back with provenance and revision.
    let identity = OpportunityIdentity::new(&first.candidate);
    let record = store.get(&identity).await.expect("read back");
    assert_eq!(record.revision, OpportunityRevision::initial());
    assert_eq!(record.observations.len(), 1);
    assert_eq!(record.best_score, 7_000);

    // Changed observation bumps the revision.
    let mut improved = first.clone();
    improved.score = 9_000;
    improved.candidate.observed_at_ms = 2_000;
    let updated = store.upsert(&improved).await.expect("update");
    assert!(updated.changed);
    assert_eq!(updated.revision, OpportunityRevision::from_raw(2).expect("valid"));

    // CAS with the current revision succeeds without change.
    let cas = store
        .upsert_if_revision(&improved, OpportunityRevision::from_raw(2).expect("valid"))
        .await
        .expect("cas ok");
    assert!(!cas.changed, "identical re-observation under matching revision");
    assert_eq!(cas.revision, OpportunityRevision::from_raw(2).expect("valid"));

    // CAS with a stale revision is rejected with a conflict.
    let stale = opportunity(&tag, 9_500, 3_000);
    let error = store
        .upsert_if_revision(&stale, OpportunityRevision::from_raw(1).expect("valid"))
        .await
        .expect_err("stale revision must conflict");
    assert!(matches!(error, PostgresOpportunityStoreError::RevisionConflict { .. }));

    // Current revision is exposed for reload-and-retry flows.
    let current = store.revision(&identity).await.expect("revision").expect("exists");
    assert!(current.get() >= 2);

    // Cleanup.
    sqlx::query("DELETE FROM cat_affiliate_opportunities WHERE identity = $1")
        .bind(identity.as_str())
        .execute(&pool)
        .await
        .expect("cleanup");
}

#[tokio::test]
async fn postgres_revalidation_store_enforces_dedup_and_claim_order() {
    let Some(pool) = connect().await else {
        eprintln!("skipping: no CAT_TEST_DATABASE_URL/DATABASE_URL configured");
        return;
    };
    let store = PostgresRevalidationStore::new(pool.clone());
    PostgresOpportunityStore::ensure_revalidation_schema(&pool)
        .await
        .expect("revalidation schema bootstrap");

    let tag = Uuid::now_v7().simple().to_string();
    let identity = format!("pgtest-{tag}");

    // Scope-clean any rows left by previous runs of this test namespace so
    // the claim assertion below sees exactly this run's requests.
    sqlx::query("DELETE FROM cat_affiliate_revalidation_requests WHERE identity LIKE 'pgtest-%'")
        .execute(&pool)
        .await
        .expect("cleanup before run");

    let critical = RevalidationRequest::new(
        Uuid::now_v7(),
        RevalidationTarget::new(identity.clone(), "network-critical"),
        RevalidationReason::Expired,
        RevalidationPriority::Critical,
        1_000,
        1_500,
    );
    let inserted_critical = store.insert(critical).await.expect("insert critical");
    assert_eq!(inserted_critical.status, RevalidationStatus::Pending);

    // Active duplicate suppressed by the partial unique index.
    let duplicate = RevalidationRequest::new(
        Uuid::now_v7(),
        RevalidationTarget::new(identity.clone(), "network-critical"),
        RevalidationReason::Expired,
        RevalidationPriority::Critical,
        2_000,
        1_500,
    );
    let error = store
        .insert(duplicate)
        .await
        .expect_err("duplicate must be suppressed");
    assert!(matches!(
        error,
        PostgresOpportunityStoreError::Revalidation(RevalidationStoreError::DuplicateRequest { .. })
    ));

    let normal = RevalidationRequest::new(
        Uuid::now_v7(),
        RevalidationTarget::new(identity.clone(), "network-normal"),
        RevalidationReason::Stale,
        RevalidationPriority::Normal,
        1_000,
        1_000,
    );
    store.insert(normal).await.expect("insert normal");

    // Claim order: critical (higher priority) before normal.
    let claimed = store.claim_due(1_600, 10).await.expect("claim");
    assert_eq!(claimed.len(), 2);
    assert_eq!(claimed[0].request.priority, RevalidationPriority::Critical);
    assert_eq!(claimed[0].status, RevalidationStatus::Claimed);

    // Domain transition matrix holds in SQL: claimed -> running -> succeeded.
    let done = store
        .transition(claimed[0].request.request_id, RevalidationStatus::Running, 2_000, None)
        .await
        .expect("start");
    assert_eq!(done.attempt, 1);
    let finished = store
        .transition(done.request.request_id, RevalidationStatus::Succeeded, 2_500, None)
        .await
        .expect("finish");
    assert_eq!(finished.status, RevalidationStatus::Succeeded);
    assert_eq!(finished.completed_at_ms, Some(2_500));

    // Illegal transition fails closed in SQL too.
    let error = store
        .transition(finished.request.request_id, RevalidationStatus::Running, 3_000, None)
        .await
        .expect_err("terminal statuses reject transitions");
    assert!(matches!(
        error,
        PostgresOpportunityStoreError::Revalidation(RevalidationStoreError::InvalidTransition { .. })
    ));

    // Terminal records stop suppressing dedup: re-issue works.
    let reissued = RevalidationRequest::new(
        Uuid::now_v7(),
        RevalidationTarget::new(identity.clone(), "network-critical"),
        RevalidationReason::Expired,
        RevalidationPriority::Critical,
        3_000,
        1_500,
    );
    store.insert(reissued).await.expect("re-issue after terminal");

    // Cleanup by identity.
    sqlx::query("DELETE FROM cat_affiliate_revalidation_requests WHERE identity = $1")
        .bind(identity)
        .execute(&pool)
        .await
        .expect("cleanup");
}
