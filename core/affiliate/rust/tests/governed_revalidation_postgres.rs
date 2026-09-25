use cat_affiliate::{
    AsyncOpportunityStore, AsyncRevalidationRequestStore, DiscoveryBackedRevalidationExecutor,
    DiscoveryCandidate, DiscoveryOpportunity, DiscoverySource, DiscoverySourceBatch,
    DiscoverySourceId, DiscoverySourceInfo, DiscoverySourceKind, DiscoverySourceRegistry,
    DiscoverySourceRequest, FixedRevalidationScopeResolver, GovernedRevalidationCoordinator,
    PostgresOpportunityStore, PostgresRevalidationStore, RevalidationPriority, RevalidationReason,
    RevalidationRequest, RevalidationStatus, RevalidationTarget, register_revalidation_capability,
    canonical_key, OpportunityIdentity, OpportunityRevision,
};
use cat_kernel::{AgentId, CapabilityRegistry, EntityId, TenantId};
use cat_orchestrator::{
    ExecutionAttemptStatus, ExecutionAttemptStore, PostgresExecutionStore, RetryPolicy,
};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

fn database_url() -> Option<String> {
    std::env::var("CAT_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .ok()
}

struct StaticDiscoverySource {
    info: DiscoverySourceInfo,
    candidate: DiscoveryCandidate,
}
    
impl DiscoverySource for StaticDiscoverySource {
    fn info(&self) -> &DiscoverySourceInfo {
        &self.info
    }

    fn discover<'a>(
        &'a self,
        _request: DiscoverySourceRequest,
    ) -> cat_affiliate::DiscoverySourceFuture<'a, DiscoverySourceBatch> {
        let candidate = self.candidate.clone();
        Box::pin(async move {
            Ok(DiscoverySourceBatch {
                candidates: vec![candidate],
                has_more: false,
                next_page: None,
            })
        })
    }
}

#[tokio::test]
async fn governed_revalidation_executes_through_discovery_source_and_updates_revision() {
    let Some(url) = database_url() else {
        eprintln!("skipping: CAT_TEST_DATABASE_URL/DATABASE_URL is not configured");
        return;
    };

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&url)
        .await
        .unwrap();

    let execution_store = PostgresExecutionStore::new(pool.clone());
    execution_store.ensure_schema().await.unwrap();

    let opportunity_store = PostgresOpportunityStore::new(pool.clone());
    opportunity_store.ensure_schema().await.unwrap();

    let tag = Uuid::now_v7().simple().to_string();
    let source_name = format!("source-{tag}");
    let identity = canonical_key("Acme", "Widget");

    let initial_candidate = DiscoveryCandidate {
        source: source_name.clone(),
        external_id: "external-1".into(),
        merchant_name: "Acme".into(),
        product_name: "Widget".into(),
        canonical_key: identity.clone(),
        category: Some("electronics".into()),
        destination_url: "https://example.com/widget".into(),
        currency: "USD".into(),
        price_minor: Some(100),
        commission_bps: Some(500),
        demand_score: 5_000,
        competition_score: 3_000,
        freshness_score: 6_000,
        compliance_score: 9_000,
        observed_at_ms: 2_000,
    };

    let opportunity_id = Uuid::now_v7();
    opportunity_store
        .upsert(&DiscoveryOpportunity {
            id: opportunity_id,
            candidate: initial_candidate.clone(),
            score: initial_candidate.opportunity_score(),
            rank: 1,
        })
        .await
        .unwrap();

    let refreshed_candidate = DiscoveryCandidate {
        price_minor: Some(125),
        observed_at_ms: 5_000,
        ..initial_candidate.clone()
    };

    let mut sources = DiscoverySourceRegistry::new();
    sources.register(Box::new(StaticDiscoverySource {
        info: DiscoverySourceInfo {
            id: DiscoverySourceId(source_name.clone()),
            name: "Static Integration Source".into(),
            kind: DiscoverySourceKind::ProductFeed,
            capabilities: Vec::new(),
        },
        candidate: refreshed_candidate.clone(),
    }));

    let request = RevalidationRequest::new(
        opportunity_id,
        RevalidationTarget::new(identity.clone(), source_name.clone()),
        RevalidationReason::PriceChanged,
        RevalidationPriority::High,
        1_000,
        2_000,
    );
    let request_id = request.request_id;
    let request_store = PostgresRevalidationStore::new(pool.clone());
    request_store.insert(request).await.unwrap();

    let tenant_id = TenantId::new();
    let project_id = EntityId::new();
    let scope = FixedRevalidationScopeResolver {
        tenant_id,
        project_id: Some(project_id),
        agent_id: AgentId::new(),
    };

    let mut registry = cat_kernel::CapabilityRegistry::new();
    register_revalidation_capability(&mut registry).unwrap();

    let executor = DiscoveryBackedRevalidationExecutor::new(
        &sources,
        &opportunity_store,
        10,
    );

    let runner = GovernedRevalidationCoordinator::new(
        &request_store,
        &execution_store,
        &executor,
        &scope,
        &registry,
        "affiliate-source-backed-test",
        30_000,
        RetryPolicy::default(),
    );

    let reports = runner.run_due(3_000, 10).await.unwrap();
    assert_eq!(reports.len(), 1);
    assert!(matches!(
        reports[0],
        cat_affiliate::GovernedRevalidationRunReport::Succeeded { revision: 2, .. }
    ));

    let persisted_request = request_store.get(request_id).await.unwrap();
    assert_eq!(persisted_request.status, RevalidationStatus::Succeeded);
    assert_eq!(persisted_request.attempt, 1);

    let persisted_identity = OpportunityIdentity::new(&refreshed_candidate);
    let persisted_opportunity = opportunity_store
        .get(&persisted_identity)
        .await
        .unwrap();
    assert_eq!(persisted_opportunity.revision, OpportunityRevision::from_raw(2).unwrap());
    assert_eq!(
        persisted_opportunity.best_observation().unwrap().price_minor,
        Some(125)
    );
    assert_eq!(
        persisted_opportunity.best_observation().unwrap().observed_at_ms,
        5_000
    );

    sqlx::query("DELETE FROM cat_workflow_outbox WHERE workflow_id = $1")
        .bind(request_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM cat_execution_attempts WHERE workflow_id = $1")
        .bind(request_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM cat_execution_leases WHERE resource LIKE $1")
        .bind(format!("workflow/{request_id}/step/%"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM cat_workflows WHERE id = $1")
        .bind(request_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM cat_affiliate_revalidation_requests WHERE request_id = $1")
        .bind(request_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM cat_affiliate_opportunities WHERE identity = $1")
        .bind(identity)
        .execute(&pool)
        .await
        .unwrap();
}

#[allow(dead_code)]
fn _compile_boundary_types<S: WorkflowRegistrationStore + AsyncPostgresExecutionStore + ExecutionAttemptStore>() {}