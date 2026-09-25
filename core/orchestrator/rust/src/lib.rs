#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod audit;
mod audit_store;
mod compensation;
mod dispatch_result;
mod durable;
mod durable_execution;
mod error;
mod events;
mod execution;
mod execution_admission;
mod execution_authorization_record;
mod execution_attempt;
mod execution_attempt_store;
mod execution_coordinator;
mod execution_cursor;
mod execution_dispatch;
mod execution_event;
mod execution_request;
mod execution_state;
mod fencing;
pub mod flywheel;
mod idempotency;
mod lease;
mod model;
mod monitoring_workflow;
mod operator_access;
mod outbox;
mod outbox_dispatcher;
mod postgres;
mod postgres_contract;
mod postgres_outbox;
mod provider_adapter;
mod provider_registry;
mod provider_execution_journal;
mod provider_result;
mod provider_result_store;
mod provider_selection;
mod reconciliation;
mod reconciliation_worker;
mod recovery;
mod replay;
mod retry;
mod retry_decision;
mod scheduler;
mod validation;
pub mod validation_gates;
mod worker;

pub use audit::ExecutionAuditEvidence;
pub use audit_store::{ExecutionAuditEvent, ExecutionAuditQuery, ExecutionAuditStore};
pub use compensation::{begin_compensation, compensation_order};
pub use dispatch_result::{DispatchAction, DispatchResult};
pub use durable::{
    DurableWorkflowStore, EventBusExecutionEventSink, ExecutionEventSink,
    InMemoryDurableWorkflowStore, RecordingExecutionEventSink,
};
pub use error::{OrchestratorError, OrchestratorResult};
pub use events::{
    WorkflowCompleted, WorkflowEventFactory, WorkflowStarted, WorkflowStepStateChanged,
};
pub use execution::ExecutionEngine;
pub use execution_admission::{
    CapabilityAdmission, CapabilityAdmissionResult, ExecutionAuthorization,
};
pub use execution_authorization_record::ExecutionAuthorizationRecord;
pub use execution_attempt::{
    ExecutionAttempt, ExecutionAttemptHealth, ExecutionAttemptKey, ExecutionAttemptStatus,
};
pub use execution_attempt_store::ExecutionAttemptStore;
pub use execution_coordinator::ExecutionCoordinator;
pub use execution_cursor::{ExecutionCursor, cursor};
pub use execution_dispatch::{claim_step, ready_requests};
pub use execution_event::{ExecutionEvent, ExecutionEventKind};
pub use execution_state::{ExecutionState, ExecutionStepState};
pub use fencing::{FencedLease, FencedLeaseProvider, FencingToken, InMemoryFencedLeaseProvider};
pub use flywheel::{FlywheelNode, FlywheelPlan, FlywheelStage};
pub use idempotency::{IdempotencyRegistry, workflow_key};
pub use lease::Lease;
pub use model::{StepState, WorkflowDefinition, WorkflowInstance, WorkflowState, WorkflowStep};
pub use monitoring_workflow::{DailyMonitoringRun, MonitoringRunState};
pub use operator_access::{
    AuthenticationEvidence, AuthorizedAuditReader, AuthorizedAuditService, OperatorAccessPolicy,
    OperatorAuthorizationDecision, OperatorAuthorizationOutcome, OperatorPermission, OperatorPrincipal,
    OperatorRole,
};
pub use outbox::{DurableOutboxStore, InMemoryDurableOutbox, OutboxDisposition, OutboxRecord};
pub use outbox_dispatcher::{OutboxDispatchOutcome, OutboxDispatcher};
pub use postgres::{AsyncPostgresExecutionStore, PostgresExecutionStore};
pub use postgres_contract::{PostgresDurableExecutor, PostgresSchemaV1};
pub use postgres_outbox::{AsyncPostgresOutbox, PostgresOutboxDisposition, PostgresOutboxRecord};
pub use provider_adapter::{
    ProviderExecutionAdapter, ProviderExecutionRequest, ProviderExecutionSubmission,
    idempotency_key as provider_idempotency_key, normalize_provider_outcome,
};
pub use provider_registry::{ProviderAdapterRegistry, ProviderCapability, ProviderRegistration};
pub use provider_execution_journal::{
    ProviderExecutionJournalEntry, ProviderExecutionJournalEvent, ProviderExecutionJournalStore,
};
pub use provider_result::{ProviderExecutionRecord, ProviderOutcomeState, ReconciliationAction};
pub use provider_selection::{
    ProviderScore, ProviderSelection, ProviderSelectionEngine, ProviderSelectionRequest,
};
pub use reconciliation::{
    ExecutionReconciliationStore, ReconciliationReport, WorkflowExecutionReconciler,
};
pub use reconciliation_worker::ReconciliationWorker;
pub use recovery::{
    AsyncWorkflowRecovery, RecoveryAction, WorkflowRecoveryReport, WorkflowRecoveryStore,
};
pub use replay::{ReplaySnapshot, snapshot as replay_snapshot, verify_replay};
pub use retry::RetryPolicy;
pub use retry_decision::{RetryDecision, decide_retry};
pub use scheduler::{ScheduleRequest, Scheduler};
pub use validation::{
    new_validated_instance, ready_steps, topological_order, validate_definition, workflow_id,
};
pub use validation_gates::{ContentValidator, ValidationGate, ValidationLevel, ValidationResult};
pub use worker::{
    AsyncWorkerExecutor, WorkerExecutionInput, WorkerExecutionOutcome, WorkerExecutionResult,
    WorkerExecutor,
};
pub use durable_execution::DurableExecutionCoordinator;
