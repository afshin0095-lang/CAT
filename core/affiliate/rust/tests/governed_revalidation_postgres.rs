use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use async_trait::async_trait;
use cat_affiliate::{
    AsyncRevalidationRequestStore, GovernedRevalidationCoordinator, GovernedRevalidationExecutor,
    FixedRevalidationScopeResolver, PostgresRevalidationStore, RevalidationExecutionResult,
    RevalidationPriority, RevalidationReason, RevalidationRequest, RevalidationStatus,
    RevalidationTarget, register_revalidation_capability, RevalidationRequestRecord,
};
use cat_kernel::{AgentId, CapabilityRegistry, EntityId, TenantId};
use cat_orchestrator::{
    AsyncPostgresExecutionStore, ExecutionAttemptStore, PostgresExecutionStore, RetryPolicy,
    WorkflowRegistrationStore, ExecutionAttemptStatus,
};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

fn database_url() -> Option<String> {
    std::env::var("CAT_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .ok()
}

struct RecordingRevalidator {
    calls: Arc<AtomicUsize>,
    expected_tenant: TenantId,
    expected_project: Option<EntityId>,
}

#[async_trait]
impl GovernedRevalidationExecutor for RecordingRevalidator {
    async fn revalidate(
        &self,
        request: &RevalidationRequestRecord,
        authorization: &cat_orchestrator::ExecutionAuthorization,
    ) -> Result<RevalidationExecutionResult, String> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if authorization.tenant_id() != self.expected_tenant {
            return Err("tenant scope did not survive admission".into());
        }
        if authorization.project_id() != self.expected_project {
            return Err("project scope did not survive admission".into());
        }
        if authorization.workflow_id() != request.request.request_id {
            return Err("workflow identity did not bind to revalidation request".into());
        }
        Ok(RevalidationExecutionResult {
            observed_at_ms: 5_000,
            revision: 7,
        })
    }
}

#[tokio::test]
async fn governed_revalidation_executes_through_durable_orchestrator() {
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
    PostgresRevalidationStore::new(pool.clone()).pool();
    PostgresRevalidationStore::ensure_revalidation_schema(&pool)
        .await
        .unwrap();

    let tag = Uuid::now_v7().simple().to_string();
    let opportunity_id = Uuid::now_v7();
    let request = RevalidationRequest::new(
        opportunity_id,
        RevalidationTarget::new(format!("governed-{tag}"), format!("source-{tag}")),
        RevalidationReason::Stale,
        RevalidationPriority::High,
        1_000,
        2_000,
    );
    let request_id = request.request_id;
    let request_store = PostgresRevalidationStore::new(pool.clone());
    request_store.insert(request).await.unwrap();

    let tenant_id = TenantId::new();
    let project_id = EntityId::new();
    let agent_id = AgentId::new();
    let scope = FixedRevalidationScopeResolver {
        tenant_id,
        project_id: Some(project_id),
        agent_id,
    };

    let mut registry = CapabilityRegistry::new();
    register_revalidation_capability(&mut registry).unwrap();

    let calls = Arc::new(AtomicUsize::new(0));
    let executor = RecordingRevalidator {
        calls: Arc::clone(&calls),
        expected_tenant: tenant_id,
        expected_project: Some(project_id),
    };

    let runner = GovernedRevalidationCoordinator::new(
        &request_store,
        &execution_store,
        &executor,
        &scope,
        &registry,
        "affiliate-governed-test",
        30_000,
        RetryPolicy::default(),
    );

    let reports = runner.run_due(3_000, 10).await.unwrap();
    assert_eq!(reports.len(), 1);
    assert!(matches!(
        reports[0],
        cat_affiliate::GovernedRevalidationRunReport::Succeeded { .. }
    ));
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    let persisted = request_store.get(request_id).await.unwrap();
    assert_eq!(persisted.status, RevalidationStatus::Succeeded);
    assert_eq!(persisted.attempt, 1);

    let execution_id: Uuid = sqlx::query_scalar(
        "SELECT execution_id FROM cat_execution_attempts
         WHERE workflow_id = $1 AND step_id = 'revalidate' AND attempt = 1"
    )
    .bind(request_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let attempt = execution_store.load_execution(execution_id).await.unwrap().unwrap();
    assert_eq!(attempt.status, ExecutionAttemptStatus::Succeeded);

    let authorization = execution_store
        .load_execution_authorization(execution_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(authorization.tenant_id, tenant_id);
    assert_eq!(authorization.project_id, Some(project_id));
    assert_eq!(authorization.capability_id.as_str(), cat_affiliate::REVALIDATION_CAPABILITY_ID);

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
}

#[allow(dead_code)]
fn _compile_boundary_types<S: WorkflowRegistrationStore + AsyncPostgresExecutionStore + ExecutionAttemptStore>() {}