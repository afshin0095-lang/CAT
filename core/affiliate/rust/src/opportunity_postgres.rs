use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::opportunity_revalidation::{
    RevalidationReason, RevalidationRequest, RevalidationRequestRecord, RevalidationStatus,
};
use crate::{DiscoveryOpportunity, OpportunityIdentity, OpportunityRecord, OpportunityRevision, OpportunityStoreError};
use crate::opportunity_store::{OpportunityObservation, OpportunityUpsertResult};

#[derive(Debug, thiserror::Error)]
pub enum PostgresOpportunityStoreError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("domain error: {0}")]
    Domain(#[from] OpportunityStoreError),
    #[error("revalidation store error: {0}")]
    Revalidation(#[from] crate::opportunity_revalidation::RevalidationStoreError),
    #[error("timestamp {field}={value} exceeds PostgreSQL BIGINT range")]
    TimestampOverflow { field: &'static str, value: u64 },
    #[error("numeric {field}={value} does not fit the PostgreSQL column range")]
    CheckedConversion { field: &'static str, value: i64 },
    #[error("stored {field} is outside the domain's valid range; refusing to decode")]
    CorruptRecord { field: &'static str },
    #[error("revision conflict on {identity}: expected v{expected}, stored v{actual}")]
    RevisionConflict {
        identity: String,
        expected: u64,
        actual: u64,
    },
}

impl PostgresOpportunityStoreError {
    /// Conflict errors are resolvable by reloading, never by blind retry.
    pub fn is_revision_conflict(&self) -> bool {
        matches!(self, Self::RevisionConflict { .. })
    }
}

fn timestamp_to_i64(field: &'static str, value: u64) -> Result<i64, PostgresOpportunityStoreError> {
    i64::try_from(value).map_err(|_| PostgresOpportunityStoreError::TimestampOverflow { field, value })
}

fn u32_to_i32(field: &'static str, value: u32) -> Result<i32, PostgresOpportunityStoreError> {
    i32::try_from(value).map_err(|_| PostgresOpportunityStoreError::CheckedConversion {
        field,
        value: i64::from(value),
    })
}

fn i64_to_u64(field: &'static str, value: i64) -> Result<u64, PostgresOpportunityStoreError> {
    u64::try_from(value).map_err(|_| PostgresOpportunityStoreError::CorruptRecord { field })
}

fn i32_to_u32(field: &'static str, value: i32) -> Result<u32, PostgresOpportunityStoreError> {
    u32::try_from(value).map_err(|_| PostgresOpportunityStoreError::CorruptRecord { field })
}

#[async_trait]
pub trait AsyncOpportunityStore {
    async fn upsert(&self, opportunity: &DiscoveryOpportunity) -> Result<OpportunityUpsertResult, PostgresOpportunityStoreError>;
    async fn get(&self, identity: &OpportunityIdentity) -> Result<OpportunityRecord, PostgresOpportunityStoreError>;
}

/// Optimistic-concurrency extension over [`AsyncOpportunityStore`].
///
/// The `version` column of `cat_affiliate_opportunities` is the persisted
/// revision (see [`OpportunityRevision`]). Writers that must not clobber a
/// concurrent update compare-and-set against their observed revision.
#[async_trait]
pub trait AsyncVersionedOpportunityStore: AsyncOpportunityStore {
    async fn upsert_if_revision(
        &self,
        opportunity: &DiscoveryOpportunity,
        expected_revision: OpportunityRevision,
    ) -> Result<OpportunityUpsertResult, PostgresOpportunityStoreError>;

    async fn revision(&self, identity: &OpportunityIdentity) -> Result<Option<OpportunityRevision>, PostgresOpportunityStoreError>;
}

/// Transaction boundaries:
///
/// - every upsert runs inside a single transaction (aggregate update +
///   observation upsert commit or roll back together — no partial writes);
/// - the aggregate row is locked with `SELECT ... FOR UPDATE`, making the
///   read-modify-write safe against concurrent upserts;
/// - `upsert_if_revision` additionally compares the locked row's version
///   with the caller's expected revision before writing.
pub struct PostgresOpportunityStore {
    pool: PgPool,
}

impl PostgresOpportunityStore {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &PgPool { &self.pool }

    /// Idempotent schema bootstrap, aligned with `migrations/`. Safe to call
    /// on every start; never drops or rewrites existing data.
    pub async fn ensure_schema(&self) -> Result<(), sqlx::Error> {
        // One statement per prepared query: PostgreSQL rejects multi-statement
        // prepared commands, so each DDL statement runs (idempotently) alone.
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS cat_affiliate_opportunities (
                identity TEXT PRIMARY KEY,
                opportunity_id UUID NOT NULL,
                merchant_name TEXT NOT NULL,
                product_name TEXT NOT NULL,
                category TEXT,
                best_source TEXT NOT NULL,
                best_score INTEGER NOT NULL CHECK (best_score BETWEEN 0 AND 10000),
                first_observed_at_ms BIGINT NOT NULL,
                last_observed_at_ms BIGINT NOT NULL,
                version BIGINT NOT NULL DEFAULT 1,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );"#,
        ).execute(&self.pool).await?;
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS cat_affiliate_opportunity_observations (
                identity TEXT NOT NULL REFERENCES cat_affiliate_opportunities(identity) ON DELETE CASCADE,
                source TEXT NOT NULL,
                external_id TEXT NOT NULL,
                destination_url TEXT NOT NULL,
                currency TEXT NOT NULL,
                price_minor BIGINT,
                commission_bps INTEGER,
                score INTEGER NOT NULL CHECK (score BETWEEN 0 AND 10000),
                observed_at_ms BIGINT NOT NULL,
                PRIMARY KEY (identity, source)
            );"#,
        ).execute(&self.pool).await?;
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_cat_affiliate_opportunities_score
                ON cat_affiliate_opportunities (best_score DESC, identity ASC);"#,
        ).execute(&self.pool).await?;
        Self::ensure_revalidation_schema(&self.pool).await
    }

    /// Revalidation request persistence bootstrap (see migration
    /// `0002_opportunity_revalidation.sql`).
    pub async fn ensure_revalidation_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
        // One statement per prepared query (see `ensure_schema`).
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS cat_affiliate_revalidation_requests (
                request_id UUID PRIMARY KEY,
                identity TEXT NOT NULL,
                source TEXT NOT NULL,
                reason TEXT NOT NULL,
                priority TEXT NOT NULL CHECK (priority IN ('low','normal','high','critical')),
                status TEXT NOT NULL CHECK (status IN ('pending','claimed','running','succeeded','failed','cancelled','dead_lettered')),
                dedup_key TEXT NOT NULL,
                created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
                scheduled_at_ms BIGINT NOT NULL CHECK (scheduled_at_ms >= 0),
                started_at_ms BIGINT,
                completed_at_ms BIGINT,
                attempt INTEGER NOT NULL DEFAULT 0 CHECK (attempt >= 0),
                last_error TEXT,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );"#,
        ).execute(pool).await?;
        sqlx::query(
            r#"
            CREATE UNIQUE INDEX IF NOT EXISTS uq_cat_affiliate_revalidation_active_dedup
                ON cat_affiliate_revalidation_requests (dedup_key)
                WHERE status IN ('pending', 'claimed', 'running');"#,
        ).execute(pool).await?;
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_cat_affiliate_revalidation_claim
                ON cat_affiliate_revalidation_requests (status, scheduled_at_ms ASC);"#,
        ).execute(pool).await?;
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_cat_affiliate_revalidation_identity
                ON cat_affiliate_revalidation_requests (identity, created_at_ms DESC);"#,
        ).execute(pool).await?;
        Ok(())
    }

    async fn upsert_in_transaction(
        connection: &mut sqlx::PgConnection,
        opportunity: &DiscoveryOpportunity,
    ) -> Result<OpportunityUpsertResult, PostgresOpportunityStoreError> {
        opportunity.candidate.validate().map_err(|_| OpportunityStoreError::InvalidCandidate)?;
        let record = OpportunityRecord::from_opportunity(opportunity);
        let observed_at_ms = timestamp_to_i64("observed_at_ms", opportunity.candidate.observed_at_ms)?;
        let observed_at_u64 = i64_to_u64("observed_at_ms", observed_at_ms)?;
        let identity = record.identity.as_str().to_owned();

        let existing = sqlx::query("SELECT opportunity_id, best_score, best_source, first_observed_at_ms, last_observed_at_ms, category, version FROM cat_affiliate_opportunities WHERE identity = $1 FOR UPDATE")
            .bind(identity.as_str()).fetch_optional(&mut *connection).await?;

        let created = existing.is_none();
        let (changed, revision) = match existing {
            None => {
                let first_observed_at_ms = timestamp_to_i64("first_observed_at_ms", record.first_observed_at_ms)?;
                let last_observed_at_ms = timestamp_to_i64("last_observed_at_ms", record.last_observed_at_ms)?;
                sqlx::query("INSERT INTO cat_affiliate_opportunities (identity, opportunity_id, merchant_name, product_name, category, best_source, best_score, first_observed_at_ms, last_observed_at_ms, version) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)")
                    .bind(identity.as_str()).bind(record.id).bind(&record.merchant_name).bind(&record.product_name).bind(&record.category).bind(&record.best_source).bind(u32_to_i32("best_score", record.best_score)?).bind(first_observed_at_ms).bind(last_observed_at_ms).bind(1_i64).execute(&mut *connection).await?;
                (true, OpportunityRevision::initial())
            }
            Some(row) => {
                let prior_version = i64_to_u64("version", row.get::<i64, _>("version"))?;
                let prior_revision = OpportunityRevision::from_raw(prior_version)
                    .map_err(|_| PostgresOpportunityStoreError::CorruptRecord { field: "version" })?;
                let observation = OpportunityObservation::from_opportunity(opportunity);
                let prior = sqlx::query("SELECT destination_url, currency, price_minor, commission_bps, score, observed_at_ms FROM cat_affiliate_opportunity_observations WHERE identity = $1 AND source = $2")
                    .bind(identity.as_str()).bind(&observation.source).fetch_optional(&mut *connection).await?;
                let prior_commission = match prior.as_ref() {
                    Some(row) => match row.get::<Option<i32>, _>("commission_bps") {
                        Some(value) => Some(i32_to_u32("commission_bps", value)?),
                        None => None,
                    },
                    None => None,
                };
                let prior_score = match prior.as_ref() {
                    Some(row) => i32_to_u32("score", row.get::<i32, _>("score"))?,
                    None => 0,
                };
                let prior_observed_at = match prior.as_ref() {
                    Some(row) => i64_to_u64("observed_at_ms", row.get::<i64, _>("observed_at_ms"))?,
                    None => 0,
                };
                let observation_changed = match prior.as_ref() {
                    Some(row) => {
                        row.get::<String, _>("destination_url") != observation.destination_url
                            || row.get::<String, _>("currency") != observation.currency
                            || row.get::<Option<i64>, _>("price_minor") != observation.price_minor
                            || prior_commission != observation.commission_bps
                            || prior_score != observation.score
                            || prior_observed_at != observed_at_u64
                    }
                    None => true,
                };
                let best_score = i32_to_u32("best_score", row.get::<i32, _>("best_score"))?;
                let best_source = row.get::<String, _>("best_source");
                let promote = opportunity.score > best_score || (opportunity.score == best_score && observation.source < best_source);
                if observation_changed || promote {
                    let new_revision = prior_revision.next().map_err(OpportunityStoreError::RevisionOverflow)?;
                    sqlx::query("UPDATE cat_affiliate_opportunities SET best_source = CASE WHEN $2 THEN $3 ELSE best_source END, best_score = CASE WHEN $2 THEN $4 ELSE best_score END, first_observed_at_ms = LEAST(first_observed_at_ms,$5), last_observed_at_ms = GREATEST(last_observed_at_ms,$6), category = COALESCE(category,$7), version = version + 1, updated_at = NOW() WHERE identity = $1")
                        .bind(identity.as_str()).bind(promote).bind(&observation.source).bind(u32_to_i32("score", observation.score)?).bind(observed_at_ms).bind(observed_at_ms).bind(&record.category).execute(&mut *connection).await?;
                    (true, new_revision)
                } else {
                    (false, prior_revision)
                }
            }
        };

        let observation = OpportunityObservation::from_opportunity(opportunity);
        sqlx::query("INSERT INTO cat_affiliate_opportunity_observations (identity, source, external_id, destination_url, currency, price_minor, commission_bps, score, observed_at_ms) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) ON CONFLICT (identity, source) DO UPDATE SET external_id=EXCLUDED.external_id, destination_url=EXCLUDED.destination_url, currency=EXCLUDED.currency, price_minor=EXCLUDED.price_minor, commission_bps=EXCLUDED.commission_bps, score=EXCLUDED.score, observed_at_ms=EXCLUDED.observed_at_ms")
            .bind(identity.as_str()).bind(&observation.source).bind(&observation.external_id).bind(&observation.destination_url).bind(&observation.currency).bind(observation.price_minor).bind(observation.commission_bps.map(|v| u32_to_i32("commission_bps", v)).transpose()?).bind(u32_to_i32("score", observation.score)?).bind(observed_at_ms).execute(&mut *connection).await?;
        let _ = changed;
        Ok(OpportunityUpsertResult { created, changed, revision })
    }
}

#[async_trait]
impl AsyncOpportunityStore for PostgresOpportunityStore {
    async fn upsert(&self, opportunity: &DiscoveryOpportunity) -> Result<OpportunityUpsertResult, PostgresOpportunityStoreError> {
        let mut tx = self.pool.begin().await?;
        let result = Self::upsert_in_transaction(&mut tx, opportunity).await;
        match result {
            Ok(value) => {
                tx.commit().await?;
                Ok(value)
            }
            Err(error) => {
                // Explicit rollback on failure; a rollback error must not mask
                // the original cause, so it is intentionally not propagated.
                let _ = tx.rollback().await;
                Err(error)
            }
        }
    }

    async fn get(&self, identity: &OpportunityIdentity) -> Result<OpportunityRecord, PostgresOpportunityStoreError> {
        let row = sqlx::query("SELECT opportunity_id, merchant_name, product_name, category, best_source, best_score, first_observed_at_ms, last_observed_at_ms, version FROM cat_affiliate_opportunities WHERE identity = $1")
            .bind(identity.as_str()).fetch_optional(&self.pool).await?.ok_or(OpportunityStoreError::NotFound)?;
        let observations = sqlx::query("SELECT source, external_id, destination_url, currency, price_minor, commission_bps, score, observed_at_ms FROM cat_affiliate_opportunity_observations WHERE identity = $1 ORDER BY source ASC")
            .bind(identity.as_str()).fetch_all(&self.pool).await?;
        let mut map = std::collections::BTreeMap::new();
        for item in observations {
            let source = item.get::<String, _>("source");
            let commission_bps = match item.get::<Option<i32>, _>("commission_bps") {
                Some(value) => Some(i32_to_u32("commission_bps", value)?),
                None => None,
            };
            let score = i32_to_u32("score", item.get::<i32, _>("score"))?;
            let observed_at_ms = i64_to_u64("observed_at_ms", item.get::<i64, _>("observed_at_ms"))?;
            map.insert(source.clone(), OpportunityObservation {
                source,
                external_id: item.get("external_id"),
                destination_url: item.get("destination_url"),
                currency: item.get("currency"),
                price_minor: item.get("price_minor"),
                commission_bps,
                score,
                observed_at_ms,
            });
        }
        let revision = OpportunityRevision::from_raw(i64_to_u64("version", row.get::<i64, _>("version"))?)
            .map_err(|_| PostgresOpportunityStoreError::CorruptRecord { field: "version" })?;
        Ok(OpportunityRecord {
            id: row.get("opportunity_id"),
            identity: identity.clone(),
            merchant_name: row.get("merchant_name"),
            product_name: row.get("product_name"),
            category: row.get("category"),
            observations: map,
            best_source: row.get("best_source"),
            best_score: i32_to_u32("best_score", row.get::<i32, _>("best_score"))?,
            first_observed_at_ms: i64_to_u64("first_observed_at_ms", row.get::<i64, _>("first_observed_at_ms"))?,
            last_observed_at_ms: i64_to_u64("last_observed_at_ms", row.get::<i64, _>("last_observed_at_ms"))?,
            revision,
        })
    }
}

#[async_trait]
impl AsyncVersionedOpportunityStore for PostgresOpportunityStore {
    async fn upsert_if_revision(
        &self,
        opportunity: &DiscoveryOpportunity,
        expected_revision: OpportunityRevision,
    ) -> Result<OpportunityUpsertResult, PostgresOpportunityStoreError> {
        opportunity.candidate.validate().map_err(|_| OpportunityStoreError::InvalidCandidate)?;
        let identity = OpportunityIdentity::new(&opportunity.candidate);
        let mut tx = self.pool.begin().await?;

        let stored = sqlx::query("SELECT version FROM cat_affiliate_opportunities WHERE identity = $1 FOR UPDATE")
            .bind(identity.as_str()).fetch_optional(&mut *tx).await?;
        let actual = match stored {
            Some(row) => OpportunityRevision::from_raw(i64_to_u64("version", row.get::<i64, _>("version"))?)
                .map_err(|_| PostgresOpportunityStoreError::CorruptRecord { field: "version" })?,
            None => {
                let _ = tx.rollback().await;
                return Err(OpportunityStoreError::NotFound.into());
            }
        };
        if actual != expected_revision {
            let conflict = PostgresOpportunityStoreError::RevisionConflict {
                identity: identity.as_str().to_owned(),
                expected: expected_revision.get(),
                actual: actual.get(),
            };
            let _ = tx.rollback().await;
            return Err(conflict);
        }

        let result = Self::upsert_in_transaction(&mut tx, opportunity).await;
        match result {
            Ok(value) => {
                tx.commit().await?;
                Ok(value)
            }
            Err(error) => {
                let _ = tx.rollback().await;
                Err(error)
            }
        }
    }

    async fn revision(&self, identity: &OpportunityIdentity) -> Result<Option<OpportunityRevision>, PostgresOpportunityStoreError> {
        let row = sqlx::query("SELECT version FROM cat_affiliate_opportunities WHERE identity = $1")
            .bind(identity.as_str()).fetch_optional(&self.pool).await?;
        match row {
            Some(row) => Ok(Some(
                OpportunityRevision::from_raw(i64_to_u64("version", row.get::<i64, _>("version"))?)
                    .map_err(|_| PostgresOpportunityStoreError::CorruptRecord { field: "version" })?,
            )),
            None => Ok(None),
        }
    }
}

/// Durable revalidation request persistence (REQUEST bookkeeping only;
/// attempts and results belong to the Sprint 1 execution boundary).
#[async_trait]
pub trait AsyncRevalidationRequestStore {
    /// Inserts a pending request. Active requests (pending/claimed/running)
    /// with the same `dedup_key` suppress the insert with
    /// [`RevalidationStoreError::DuplicateRequest`]; terminal requests never
    /// block re-issue (partial unique index).
    async fn insert(&self, request: RevalidationRequest) -> Result<RevalidationRequestRecord, PostgresOpportunityStoreError>;
    async fn get(&self, request_id: Uuid) -> Result<RevalidationRequestRecord, PostgresOpportunityStoreError>;
    /// Applies the domain transition matrix under a row lock.
    async fn transition(
        &self,
        request_id: Uuid,
        to: RevalidationStatus,
        at_ms: u64,
        error: Option<String>,
    ) -> Result<RevalidationRequestRecord, PostgresOpportunityStoreError>;
    /// Deterministically claims due pending requests, ordered by
    /// (priority desc, scheduled asc, identity asc). Concurrent workers
    /// never observe the same request (`FOR UPDATE SKIP LOCKED`).
    async fn claim_due(&self, now_ms: u64, limit: u32) -> Result<Vec<RevalidationRequestRecord>, PostgresOpportunityStoreError>;
}

pub struct PostgresRevalidationStore {
    pool: PgPool,
}

impl PostgresRevalidationStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    fn decode_row(row: &sqlx::postgres::PgRow) -> Result<RevalidationRequestRecord, PostgresOpportunityStoreError> {
        let status_text: String = row.get("status");
        let status = RevalidationStatus::parse_strict(&status_text).ok_or(
            crate::opportunity_revalidation::RevalidationStoreError::UnknownEnumValue {
                field: "status",
                value: status_text,
            },
        )?;
        let priority_text: String = row.get("priority");
        let priority = crate::RevalidationPriority::parse_strict(&priority_text).ok_or(
            crate::opportunity_revalidation::RevalidationStoreError::UnknownEnumValue {
                field: "priority",
                value: priority_text,
            },
        )?;
        // Forward-compatible read: reasons written by newer versions decode
        // into `Unknown` instead of failing (mirrors the serde contract).
        let reason = RevalidationReason::parse_strict(&row.get::<String, _>("reason"))
            .unwrap_or_else(|| RevalidationReason::Unknown(row.get::<String, _>("reason")));
        let request = RevalidationRequest {
            request_id: row.get("request_id"),
            opportunity_id: row.get("opportunity_id"),
            target: crate::RevalidationTarget::new(row.get::<String, _>("identity"), row.get::<String, _>("source")),
            reason,
            priority,
            requested_at_ms: i64_to_u64("created_at_ms", row.get::<i64, _>("created_at_ms"))?,
            scheduled_for_ms: i64_to_u64("scheduled_at_ms", row.get::<i64, _>("scheduled_at_ms"))?,
            dedup_key: row.get("dedup_key"),
        };
        Ok(RevalidationRequestRecord {
            request,
            status,
            attempt: i32_to_u32("attempt", row.get::<i32, _>("attempt"))?,
            created_at_ms: i64_to_u64("created_at_ms", row.get::<i64, _>("created_at_ms"))?,
            started_at_ms: match row.get::<Option<i64>, _>("started_at_ms") {
                Some(value) => Some(i64_to_u64("started_at_ms", value)?),
                None => None,
            },
            completed_at_ms: match row.get::<Option<i64>, _>("completed_at_ms") {
                Some(value) => Some(i64_to_u64("completed_at_ms", value)?),
                None => None,
            },
            last_error: row.get("last_error"),
        })
    }
}

#[async_trait]
impl AsyncRevalidationRequestStore for PostgresRevalidationStore {
    async fn insert(&self, request: RevalidationRequest) -> Result<RevalidationRequestRecord, PostgresOpportunityStoreError> {
        if request.target.identity.trim().is_empty() || request.target.source.trim().is_empty() {
            return Err(crate::opportunity_revalidation::RevalidationStoreError::InvalidRequest(
                "revalidation target requires an identity and a source".into(),
            )
            .into());
        }
        let requested_at_ms = request.requested_at_ms;
        let created_at_ms = timestamp_to_i64("requested_at_ms", requested_at_ms)?;
        let scheduled_at_ms = timestamp_to_i64("scheduled_for_ms", request.scheduled_for_ms)?;
        let result = sqlx::query(
            r#"
            INSERT INTO cat_affiliate_revalidation_requests
                (request_id, opportunity_id, identity, source, reason, priority, status, dedup_key, created_at_ms, scheduled_at_ms)
            VALUES ($1, $2, $3, $4, $5, $6, 'pending', $7, $8, $9)
            ON CONFLICT (dedup_key) WHERE status IN ('pending', 'claimed', 'running') DO NOTHING
            "#,
        )
        .bind(request.request_id)
        .bind(request.opportunity_id)
        .bind(request.target.identity.as_str())
        .bind(request.target.source.as_str())
        .bind(request.reason.as_str())
        .bind(request.priority.as_str())
        .bind(request.dedup_key.as_str())
        .bind(created_at_ms)
        .bind(scheduled_at_ms)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(crate::opportunity_revalidation::RevalidationStoreError::DuplicateRequest {
                dedup_key: request.dedup_key,
            }
            .into());
        }
        Ok(RevalidationRequestRecord {
            request,
            status: RevalidationStatus::Pending,
            attempt: 0,
            created_at_ms: requested_at_ms,
            started_at_ms: None,
            completed_at_ms: None,
            last_error: None,
        })
    }

    async fn get(&self, request_id: Uuid) -> Result<RevalidationRequestRecord, PostgresOpportunityStoreError> {
        let row = sqlx::query(
            "SELECT request_id, opportunity_id, identity, source, reason, priority, status, dedup_key, created_at_ms, scheduled_at_ms, started_at_ms, completed_at_ms, attempt, last_error FROM cat_affiliate_revalidation_requests WHERE request_id = $1",
        )
        .bind(request_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(crate::opportunity_revalidation::RevalidationStoreError::NotFound)?;
        Self::decode_row(&row)
    }

    async fn transition(
        &self,
        request_id: Uuid,
        to: RevalidationStatus,
        at_ms: u64,
        error: Option<String>,
    ) -> Result<RevalidationRequestRecord, PostgresOpportunityStoreError> {
        let at = timestamp_to_i64("at_ms", at_ms)?;
        let mut tx = self.pool.begin().await?;
        let current_text: String = sqlx::query("SELECT status FROM cat_affiliate_revalidation_requests WHERE request_id = $1 FOR UPDATE")
            .bind(request_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(crate::opportunity_revalidation::RevalidationStoreError::NotFound)?
            .get("status");
        let current = RevalidationStatus::parse_strict(&current_text).ok_or(
            crate::opportunity_revalidation::RevalidationStoreError::UnknownEnumValue {
                field: "status",
                value: current_text,
            },
        )?;
        if !current.can_transition_to(to) {
            return Err(crate::opportunity_revalidation::RevalidationStoreError::InvalidTransition {
                from: current,
                to,
            }
            .into());
        }
        let bounded_error = error.map(|text| {
            let sanitized: String = text
                .chars()
                .map(|character| if character.is_control() { ' ' } else { character })
                .take(512)
                .collect();
            sanitized
        });
        sqlx::query(
            r#"
            UPDATE cat_affiliate_revalidation_requests SET
                status = $2,
                attempt = CASE WHEN $2 = 'running' THEN attempt + 1 ELSE attempt END,
                started_at_ms = COALESCE(started_at_ms, CASE WHEN $2 = 'running' THEN $3 END),
                completed_at_ms = CASE WHEN $2 IN ('succeeded','failed','cancelled','dead_lettered') THEN $3 ELSE completed_at_ms END,
                last_error = COALESCE($4, last_error),
                updated_at = NOW()
            WHERE request_id = $1
            "#,
        )
        .bind(request_id)
        .bind(to.as_str())
        .bind(at)
        .bind(bounded_error)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        self.get(request_id).await
    }

    async fn claim_due(&self, now_ms: u64, limit: u32) -> Result<Vec<RevalidationRequestRecord>, PostgresOpportunityStoreError> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let now = timestamp_to_i64("now_ms", now_ms)?;
        let rows = sqlx::query(
            r#"
            WITH due AS (
                SELECT request_id FROM cat_affiliate_revalidation_requests
                WHERE status = 'pending' AND scheduled_at_ms <= $1
                ORDER BY CASE priority WHEN 'critical' THEN 4 WHEN 'high' THEN 3 WHEN 'normal' THEN 2 ELSE 1 END DESC,
                         scheduled_at_ms ASC,
                         identity ASC,
                         request_id ASC
                LIMIT $2
                FOR UPDATE SKIP LOCKED
            )
            UPDATE cat_affiliate_revalidation_requests r
            SET status = 'claimed', updated_at = NOW()
            FROM due
            WHERE r.request_id = due.request_id
            RETURNING r.request_id, r.opportunity_id, r.identity, r.source, r.reason, r.priority, r.status,
                      r.dedup_key, r.created_at_ms, r.scheduled_at_ms, r.started_at_ms, r.completed_at_ms,
                      r.attempt, r.last_error
            "#,
        )
        .bind(now)
        .bind(i32::try_from(limit).map_err(|_| PostgresOpportunityStoreError::CheckedConversion { field: "limit", value: i64::from(limit) })?)
        .fetch_all(&self.pool)
        .await?;

        let mut claimed = Vec::with_capacity(rows.len());
        for row in &rows {
            claimed.push(Self::decode_row(row)?);
        }
        // Deterministic output order regardless of physical update order.
        claimed.sort_by(|left, right| {
            right
                .request
                .priority
                .cmp(&left.request.priority)
                .then_with(|| left.request.scheduled_for_ms.cmp(&right.request.scheduled_for_ms))
                .then_with(|| left.request.target.identity.cmp(&right.request.target.identity))
                .then_with(|| left.request.request_id.cmp(&right.request.request_id))
        });
        Ok(claimed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_conversion_rejects_values_outside_postgres_bigint() {
        let result = timestamp_to_i64("observed_at_ms", u64::MAX);
        assert!(matches!(result, Err(PostgresOpportunityStoreError::TimestampOverflow { .. })));
        assert_eq!(timestamp_to_i64("observed_at_ms", 0).unwrap(), 0);
        assert_eq!(timestamp_to_i64("observed_at_ms", i64::MAX as u64).unwrap(), i64::MAX);
    }

    #[test]
    fn numeric_conversions_are_checked_in_both_directions() {
        assert!(u32_to_i32("score", u32::MAX).is_err());
        assert_eq!(u32_to_i32("score", 10_000).unwrap(), 10_000);
        assert!(i64_to_u64("version", -1).is_err());
        assert!(i32_to_u32("score", -1).is_err());
        assert_eq!(i32_to_u32("score", 500).unwrap(), 500);
    }

    #[test]
    fn revision_conflicts_are_detectable() {
        let error = PostgresOpportunityStoreError::RevisionConflict {
            identity: "acme:widget".into(),
            expected: 3,
            actual: 7,
        };
        assert!(error.is_revision_conflict());
        assert!(!PostgresOpportunityStoreError::Database(sqlx::Error::RowNotFound).is_revision_conflict());
        assert!(matches!(
            OpportunityRevision::from_raw(0),
            Err(crate::OpportunityRevisionError::Zero)
        ));
    }
}
