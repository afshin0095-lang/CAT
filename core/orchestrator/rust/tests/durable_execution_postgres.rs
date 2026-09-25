use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use cat_eventbus::{EventBus, PublishOutcome};
use cat_kernel::{
    AgentContract, AgentId, CapabilityContract, CapabilityId, CapabilityLifecycle,
    CapabilityRegistry, ContractVersion, CorrelationId, EntityId, ExecutionContext,
    IdempotencyKey, InvocationRequest, SideEffectClass, TenantId, TimestampMs,
};
use cat_orchestrator::{
    ApprovalContext, AsyncPostgresOutbox, AsyncWorkerExecutor, CapabilityAdmission,
    DurableExecutionCoordinator, ExecutionAuditEvidence, ExecutionAttemptStore, ExecutionAuditQuery,
    AuthorizedAuditService, ExecutionAuditStore, OperatorIdentityStore, ProviderCallback,
    ProviderCallbackCorrelationState, ProviderCallbackStore, ProviderCallbackReconciliationWorker,
    PostgresExecutionStore, ReconciliationAction, StepState, WorkflowDefinition,
    WorkflowExecutionReconciler, WorkflowInstance, WorkflowState, WorkflowStep,
    WorkerExecutionInput, WorkerExecutionResult, ProviderExecutionJournalStore, ProviderOutcomeState,
};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

fn database_url() -> Option<String> {
    std::env::var("CAT_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .ok()
}

struct RecordingWorker {
    calls: Arc<AtomicUsize>,
    observed_capability: Arc<std::sync::Mutex<Option<String>>>,
    observed_token: Arc<std::sync::Mutex<Option<u64>>>,
}

#[async_trait::async_trait]
impl AsyncWorkerExecutor for RecordingWorker {
    async fn execute(&self, input: WorkerExecutionInput) -> WorkerExecutionResult {
        self.calls.fetch_add(1, Ordering::SeqCst);
        *self.observed_capability.lock().unwrap() =
            Some(input.authorization().capability_id().to_string());
        *self.observed_token.lock().unwrap() = Some(input.fencing_token().value());
        WorkerExecutionResult::success(serde_json::json!({"integration": "ok"}))
    }
}

fn governed_fixture() -> (
    CapabilityRegistry,
    AgentContract,
    InvocationRequest,
    WorkflowInstance,
    CapabilityId,
) {
    let capability_id = CapabilityId::new("cat.capability.integration.execute.v1").unwrap();
    let mut capability =
        CapabilityContract::new(capability_id.clone(), "integration", "Execute integration test")
            .unwrap();
    capability.contract_version = ContractVersion::V1;
    capability.inputs.push("input".into());
    capability.outputs.push("output".into());
    capability.failure_model.push("typed".into());
    capability.observability.push("integration.execute".into());
    capability.evaluation.push("deterministic".into());
    capability.lifecycle = CapabilityLifecycle::Validating;

    let mut registry = CapabilityRegistry::new();
    registry.register(capability).unwrap();
    registry
        .transition(&capability_id, CapabilityLifecycle::Active)
        .unwrap();

    let agent_id = AgentId::new();
    let agent = AgentContract::new(agent_id, "integration.agent", "integration execution")
        .unwrap()
        .with_capability(capability_id.as_str())
        .unwrap()
        .enable();

    let project_id = EntityId::new();

    let invocation = InvocationRequest::new(
        agent_id,
        capability_id.as_str(),
        ExecutionContext::new(
            TenantId::new(),
            CorrelationId::new(),
            EntityId::new(),
            TimestampMs::new(1000),
        )
        .with_project(project_id),
        IdempotencyKey::new(format!("integration-{}", Uuid::now_v7())).unwrap(),
        serde_json::json!({"input": true}),
        SideEffectClass::S0,
        TimestampMs::new(1000),
    )
    .unwrap();

    let workflow_id = Uuid::now_v7();
    let workflow = WorkflowInstance {
        id: workflow_id,
        state: WorkflowState::Running,
        revision: 0,
        definition: WorkflowDefinition {
            workflow_type: "integration.durable_execution".into(),
            version: 1,
            steps: vec![WorkflowStep {
                id: "execute".into(),
                capability_id: capability_id.clone(),
                dependencies: Vec::new(),
                state: StepState::Ready,
                attempt: 0,
                max_attempts: 3,
                compensation_step: None,
            }],
        },
    };

    (registry, agent, invocation, workflow, capability_id)
}

async fn cleanup(pool: &sqlx::PgPool, workflow_id: Uuid) {
    sqlx::query("DELETE FROM cat_workflow_outbox WHERE workflow_id = $1")
        .bind(workflow_id)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query(
        "DELETE FROM cat_provider_execution_results WHERE execution_id IN
         (SELECT execution_id FROM cat_execution_attempts WHERE workflow_id = $1)",
    )
    .bind(workflow_id)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query("DELETE FROM cat_execution_attempts WHERE workflow_id = $1")
        .bind(workflow_id)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM cat_provider_execution_callbacks WHERE provider LIKE $1")
        .bind(format!("integration-provider-{workflow_id}"))
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM cat_execution_leases WHERE resource LIKE $1")
        .bind(format!("workflow/{workflow_id}/step/%"))
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM cat_workflows WHERE id = $1")
        .bind(workflow_id)
        .execute(pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn durable_execution_persists_governance_and_publishes_outbox_event() {
    let Some(url) = database_url() else {
        eprintln!("skipping: CAT_TEST_DATABASE_URL/DATABASE_URL is not configured");
        return;
    };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();

    let store = PostgresExecutionStore::new(pool.clone());
    store.ensure_schema().await.unwrap();

    let (registry, agent, invocation, workflow, capability_id) = governed_fixture();
    let workflow_id = workflow.id;

    sqlx::query(
        "INSERT INTO cat_workflows
         (id, workflow_type, workflow_version, state, revision)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(workflow.id)
    .bind(&workflow.definition.workflow_type)
    .bind(workflow.definition.version as i32)
    .bind(serde_json::to_value(&workflow).unwrap())
    .bind(0_i64)
    .execute(&pool)
    .await
    .unwrap();

    let calls = Arc::new(AtomicUsize::new(0));
    let observed_capability = Arc::new(std::sync::Mutex::new(None));
    let observed_token = Arc::new(std::sync::Mutex::new(None));

    let worker = RecordingWorker {
        calls: Arc::clone(&calls),
        observed_capability: Arc::clone(&observed_capability),
        observed_token: Arc::clone(&observed_token),
    };

    let coordinator = DurableExecutionCoordinator::new(
        &store,
        &worker,
        "integration-worker",
        30_000,
        Default::default(),
    );
    let admission = CapabilityAdmission::new(&registry);

    let dispatch = coordinator
        .execute_step(
            workflow.id,
            "execute",
            2_000,
            &admission,
            &agent,
            &invocation,
            ApprovalContext::none(),
        )
        .await
        .unwrap();

    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        observed_capability.lock().unwrap().as_deref(),
        Some(capability_id.as_str())
    );
    assert!(observed_token.lock().unwrap().is_some());
    assert!(matches!(
        dispatch.action,
        cat_orchestrator::DispatchAction::Complete
    ));

    let execution_id: Uuid = sqlx::query_scalar(
        "SELECT execution_id FROM cat_execution_attempts
         WHERE workflow_id = $1 AND step_id = 'execute' AND attempt = 1",
    )
    .bind(workflow.id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let attempt = store
        .load_execution(execution_id)
        .await
        .unwrap()
        .expect("durable attempt must exist");
    assert_eq!(
        attempt.status,
        cat_orchestrator::ExecutionAttemptStatus::Succeeded
    );

    let authorization = store
        .load_execution_authorization(execution_id)
        .await
        .unwrap()
        .expect("authorization evidence must exist");
    assert_eq!(authorization.capability_id.as_str(), capability_id.as_str());
    assert_eq!(
        authorization.idempotency_key.as_str(),
        invocation.idempotency_key.as_str()
    );

    let reconciler = WorkflowExecutionReconciler::new(&store);

    let provider = format!("integration-provider-{workflow_id}");
    let callback_id = Uuid::new_v4();
    let callback = ProviderCallback {
        callback_id,
        provider: provider.clone(),
        provider_execution_id: "remote-integration-1".into(),
        request_hash: Some("sha256:integration".into()),
        outcome: ProviderOutcomeState::Succeeded,
        result: Some(serde_json::json!({"accepted": true})),
        error: None,
        received_at_ms: 2_050,
    };

    let early = store.ingest_callback(callback.clone()).await.unwrap();
    assert_eq!(early.correlation_state, ProviderCallbackCorrelationState::Unmatched);

    let callback_worker =
        ProviderCallbackReconciliationWorker::new(&store, provider.clone(), 10).unwrap();
    let early_report = callback_worker.run_once(2_075).await.unwrap();
    assert_eq!(early_report.scanned, 1);
    assert_eq!(early_report.still_unmatched, 1);

    store
        .record_provider_submission(
            execution_id,
            &provider,
            "remote-integration-1",
            "sha256:integration",
            2_080,
        )
        .await
        .unwrap();

    let replay_report = callback_worker.run_once(2_100).await.unwrap();
    assert_eq!(replay_report.scanned, 1);
    assert_eq!(replay_report.correlated, 1);

    let correlated = store.ingest_callback(callback.clone()).await.unwrap();
    assert_eq!(correlated.execution_id, Some(execution_id));
    assert_eq!(
        correlated.correlation_state,
        ProviderCallbackCorrelationState::Correlated
    );
    let unmatched = store
        .ingest_callback(ProviderCallback {
            callback_id: Uuid::new_v4(),
            provider: provider.clone(),
            provider_execution_id: "remote-unmatched-1".into(),
            request_hash: None,
            outcome: ProviderOutcomeState::Unknown,
            result: None,
            error: Some("waiting for submission".into()),
            received_at_ms: 2_250,
        })
        .await
        .unwrap();
    assert_eq!(
        unmatched.correlation_state,
        ProviderCallbackCorrelationState::Unmatched
    );
    assert_eq!(unmatched.execution_id, None);

    let unmatched_rows = store
        .list_unmatched_callbacks(&provider, 10)
        .await
        .unwrap();
    assert_eq!(unmatched_rows.len(), 1);
    assert_eq!(
        unmatched_rows[0].callback.provider_execution_id,
        "remote-unmatched-1"
    );

    let journal = store
        .list_execution_journal(execution_id)
        .await
        .unwrap();
    assert_eq!(journal.len(), 2);
    assert_eq!(
        journal[0].event,
        cat_orchestrator::ProviderExecutionJournalEvent::Submitted
    );
    assert_eq!(
        journal[1].event,
        cat_orchestrator::ProviderExecutionJournalEvent::Observed
    );

    let report = reconciler.reconcile(execution_id).await.unwrap();
    assert_eq!(report.action, ReconciliationAction::ConfirmSuccess);
    assert_eq!(
        report
            .authorization
            .as_ref()
            .expect("reconciliation must consume authorization evidence")
            .capability_id
            .as_str(),
        capability_id.as_str()
    );

    let audit = report
        .audit_evidence(&attempt)
        .expect("audit projection should be reconstructible from durable evidence");
    let encoded = serde_json::to_string(&audit).unwrap();
    let decoded: ExecutionAuditEvidence = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, audit);

    let audit_event = report
        .persist_audit(
            &attempt,
            &store,
            "reconciliation:integration:1",
            2_300,
        )
        .await
        .unwrap();
    assert_eq!(audit_event.execution_id, execution_id);

    let latest = store
        .load_latest_audit(execution_id)
        .await
        .unwrap()
        .expect("audit read model must contain the latest execution");
    assert_eq!(latest.event_key, "reconciliation:integration:1");
    assert_eq!(latest.action, ReconciliationAction::ConfirmSuccess);

    let queried = store
        .query_audit(ExecutionAuditQuery {
            capability_id: Some(capability_id.to_string()),
            limit: 10,
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(queried.len(), 1);
    assert_eq!(queried[0].execution_id, execution_id);

    let rebuilt = store.rebuild_audit_read_model().await.unwrap();
    assert_eq!(rebuilt, 1);
    let rebuilt_latest = store
        .load_latest_audit(execution_id)
        .await
        .unwrap()
        .expect("audit read model must be rebuildable");
    assert_eq!(rebuilt_latest.event_key, "reconciliation:integration:1");

    let operator_id = Uuid::new_v4();
    let session_id = Uuid::new_v4();
    let tenant_id = authorization.tenant_id.as_uuid();
    sqlx::query(
        "INSERT INTO cat_operator_identities
         (principal_id, external_subject, role, tenant_id, project_id, resource_scopes, enabled)
         VALUES ($1,$2,'operator',$3,$4,$5,TRUE)"
    )
    .bind(operator_id)
    .bind(format!("integration-subject-{operator_id}"))
    .bind(tenant_id)
    .bind(authorization.project_id.map(|value| value.as_uuid()))
    .bind(serde_json::json!(["audit/read"]))
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO cat_operator_sessions
         (session_id, principal_id, auth_method, issued_at, expires_at)
         VALUES ($1,$2,'integration-test',TO_TIMESTAMP($3 / 1000.0),TO_TIMESTAMP($4 / 1000.0))"
    )
    .bind(session_id)
    .bind(operator_id)
    .bind(3_000_f64)
    .bind(10_000_f64)
    .execute(&pool)
    .await
    .unwrap();

    let audit_service = AuthorizedAuditService::new(store.clone(), Default::default());
    let scoped = audit_service
        .query_for_session(
            &store,
            session_id,
            4_000,
            ExecutionAuditQuery {
                capability_id: Some(capability_id.to_string()),
                limit: 10,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(scoped.len(), 1);
    assert_eq!(scoped[0].execution_id, execution_id);

    let scoped_latest = audit_service
        .load_latest_for_session(&store, session_id, 4_000, execution_id)
        .await
        .unwrap();
    assert_eq!(scoped_latest.execution_id, execution_id);

    let wrong_project = Uuid::new_v4();
    assert!(audit_service
        .query_for_session(
            &store,
            session_id,
            4_000,
            ExecutionAuditQuery {
                tenant_id: Some(tenant_id),
                project_id: Some(wrong_project),
                limit: 10,
                ..Default::default()
            },
        )
        .await
        .is_err());

    store.revoke_session(session_id, 4_500).await.unwrap();
    assert!(audit_service
        .query_for_session(
            &store,
            session_id,
            4_500,
            ExecutionAuditQuery {
                limit: 10,
                ..Default::default()
            },
        )
        .await
        .is_err());

    sqlx::query("DELETE FROM cat_operator_sessions WHERE session_id = $1")
        .bind(session_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM cat_operator_identities WHERE principal_id = $1")
        .bind(operator_id)
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("DELETE FROM cat_execution_authorizations WHERE execution_id = $1")
        .bind(execution_id)
        .execute(&pool)
        .await
        .unwrap();

    let missing_auth_report = reconciler.reconcile(execution_id).await.unwrap();
    assert_eq!(
        missing_auth_report.action,
        ReconciliationAction::ManualReview
    );

    let mut outbox = store.clone();
    let claimed = outbox
        .claim_next("eventbus-integration", 2_000, 30_000)
        .await
        .unwrap()
        .expect("workflow state event must be in PostgreSQL outbox");

    let seen = Arc::new(AtomicUsize::new(0));
    let seen_handler = Arc::clone(&seen);
    let bus = EventBus::new();
    bus.subscribe(
        &claimed.event.event_type,
        Arc::new(move |_| {
            seen_handler.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }),
    )
    .unwrap();

    assert_eq!(
        bus.publish(claimed.event.clone()).unwrap(),
        PublishOutcome::Published { handlers_called: 1 }
    );

    outbox
        .acknowledge(claimed.event.event_id, "eventbus-integration")
        .await
        .unwrap();

    assert_eq!(seen.load(Ordering::SeqCst), 1);

    let remaining: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM cat_workflow_outbox WHERE event_id = $1")
            .bind(claimed.event.event_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(remaining, 0);

    cleanup(&pool, workflow_id).await;
}
