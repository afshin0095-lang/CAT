use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{DiscoveryOpportunity, OpportunityIdentity, OpportunityRecord, OpportunityStoreError};
use crate::opportunity_store::{OpportunityObservation, OpportunityUpsertResult};

#[derive(Debug, thiserror::Error)]
pub enum PostgresOpportunityStoreError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("domain error: {0}")]
    Domain(#[from] OpportunityStoreError),
    #[error("timestamp {field}={value} exceeds PostgreSQL BIGINT range")]
    TimestampOverflow { field: &'static str, value: u64 },
}

fn timestamp_to_i64(field: &'static str, value: u64) -> Result<i64, PostgresOpportunityStoreError> {
    i64::try_from(value).map_err(|_| PostgresOpportunityStoreError::TimestampOverflow { field, value })
}

#[async_trait]
pub trait AsyncOpportunityStore {
    async fn upsert(&self, opportunity: &DiscoveryOpportunity) -> Result<OpportunityUpsertResult, PostgresOpportunityStoreError>;
    async fn get(&self, identity: &OpportunityIdentity) -> Result<OpportunityRecord, PostgresOpportunityStoreError>;
}

pub struct PostgresOpportunityStore {
    pool: PgPool,
}

impl PostgresOpportunityStore {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &PgPool { &self.pool }

    pub async fn ensure_schema(&self) -> Result<(), sqlx::Error> {
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
            );
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
            );
            CREATE INDEX IF NOT EXISTS idx_cat_affiliate_opportunities_score
                ON cat_affiliate_opportunities (best_score DESC, identity ASC);
            "#,
        ).execute(&self.pool).await?;
        Ok(())
    }
}

#[async_trait]
impl AsyncOpportunityStore for PostgresOpportunityStore {
    async fn upsert(&self, opportunity: &DiscoveryOpportunity) -> Result<OpportunityUpsertResult, PostgresOpportunityStoreError> {
        opportunity.candidate.validate().map_err(|_| OpportunityStoreError::InvalidCandidate)?;
        let record = OpportunityRecord::from_opportunity(opportunity);
        let observed_at_ms = timestamp_to_i64("observed_at_ms", opportunity.candidate.observed_at_ms)?;
        let mut tx = self.pool.begin().await?;
        let identity = record.identity.as_str();
        let existing = sqlx::query("SELECT opportunity_id, best_score, best_source, first_observed_at_ms, last_observed_at_ms, category FROM cat_affiliate_opportunities WHERE identity = $1 FOR UPDATE")
            .bind(identity).fetch_optional(&mut *tx).await?;

        let created = existing.is_none();
        let changed = match existing {
            None => {
                let first_observed_at_ms = timestamp_to_i64("first_observed_at_ms", record.first_observed_at_ms)?;
                let last_observed_at_ms = timestamp_to_i64("last_observed_at_ms", record.last_observed_at_ms)?;
                sqlx::query("INSERT INTO cat_affiliate_opportunities (identity, opportunity_id, merchant_name, product_name, category, best_source, best_score, first_observed_at_ms, last_observed_at_ms) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)")
                    .bind(identity).bind(record.id).bind(&record.merchant_name).bind(&record.product_name).bind(&record.category).bind(&record.best_source).bind(record.best_score as i32).bind(first_observed_at_ms).bind(last_observed_at_ms).execute(&mut *tx).await?;
                true
            }
            Some(row) => {
                let observation = OpportunityObservation::from_opportunity(opportunity);
                let prior = sqlx::query("SELECT destination_url, currency, price_minor, commission_bps, score, observed_at_ms FROM cat_affiliate_opportunity_observations WHERE identity = $1 AND source = $2")
                    .bind(identity).bind(&observation.source).fetch_optional(&mut *tx).await?;
                let observation_changed = prior.as_ref().map(|r| {
                    r.get::<String, _>("destination_url") != observation.destination_url ||
                    r.get::<String, _>("currency") != observation.currency ||
                    r.get::<Option<i64>, _>("price_minor") != observation.price_minor ||
                    r.get::<Option<i32>, _>("commission_bps") != observation.commission_bps.map(|v| v as i32) ||
                    r.get::<i32, _>("score") != observation.score as i32 ||
                    r.get::<i64, _>("observed_at_ms") != observed_at_ms
                }).unwrap_or(true);
                let best_score = row.get::<i32, _>("best_score") as u32;
                let best_source = row.get::<String, _>("best_source");
                let promote = opportunity.score > best_score || (opportunity.score == best_score && observation.source < best_source);
                if observation_changed || promote {
                    sqlx::query("UPDATE cat_affiliate_opportunities SET best_source = CASE WHEN $2 THEN $3 ELSE best_source END, best_score = CASE WHEN $2 THEN $4 ELSE best_score END, first_observed_at_ms = LEAST(first_observed_at_ms,$5), last_observed_at_ms = GREATEST(last_observed_at_ms,$6), category = COALESCE(category,$7), version = version + 1, updated_at = NOW() WHERE identity = $1")
                        .bind(identity).bind(promote).bind(&observation.source).bind(observation.score as i32).bind(observed_at_ms).bind(observed_at_ms).bind(&record.category).execute(&mut *tx).await?;
                }
                observation_changed || promote
            }
        };

        let observation = OpportunityObservation::from_opportunity(opportunity);
        sqlx::query("INSERT INTO cat_affiliate_opportunity_observations (identity, source, external_id, destination_url, currency, price_minor, commission_bps, score, observed_at_ms) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) ON CONFLICT (identity, source) DO UPDATE SET external_id=EXCLUDED.external_id, destination_url=EXCLUDED.destination_url, currency=EXCLUDED.currency, price_minor=EXCLUDED.price_minor, commission_bps=EXCLUDED.commission_bps, score=EXCLUDED.score, observed_at_ms=EXCLUDED.observed_at_ms")
            .bind(identity).bind(&observation.source).bind(&observation.external_id).bind(&observation.destination_url).bind(&observation.currency).bind(observation.price_minor).bind(observation.commission_bps.map(|v| v as i32)).bind(observation.score as i32).bind(observed_at_ms).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(OpportunityUpsertResult { created, changed })
    }

    async fn get(&self, identity: &OpportunityIdentity) -> Result<OpportunityRecord, PostgresOpportunityStoreError> {
        let row = sqlx::query("SELECT opportunity_id, merchant_name, product_name, category, best_source, best_score, first_observed_at_ms, last_observed_at_ms FROM cat_affiliate_opportunities WHERE identity = $1")
            .bind(identity.as_str()).fetch_optional(&self.pool).await?.ok_or(OpportunityStoreError::NotFound)?;
        let observations = sqlx::query("SELECT source, external_id, destination_url, currency, price_minor, commission_bps, score, observed_at_ms FROM cat_affiliate_opportunity_observations WHERE identity = $1 ORDER BY source ASC")
            .bind(identity.as_str()).fetch_all(&self.pool).await?;
        let mut map = std::collections::BTreeMap::new();
        for item in observations {
            let source = item.get::<String, _>("source");
            map.insert(source.clone(), OpportunityObservation {
                source,
                external_id: item.get("external_id"),
                destination_url: item.get("destination_url"),
                currency: item.get("currency"),
                price_minor: item.get("price_minor"),
                commission_bps: item.get::<Option<i32>, _>("commission_bps").map(|v| v as u32),
                score: item.get::<i32, _>("score") as u32,
                observed_at_ms: item.get::<i64, _>("observed_at_ms") as u64,
            });
        }
        Ok(OpportunityRecord {
            id: row.get("opportunity_id"),
            identity: identity.clone(),
            merchant_name: row.get("merchant_name"),
            product_name: row.get("product_name"),
            category: row.get("category"),
            observations: map,
            best_source: row.get("best_source"),
            best_score: row.get::<i32, _>("best_score") as u32,
            first_observed_at_ms: row.get::<i64, _>("first_observed_at_ms") as u64,
            last_observed_at_ms: row.get::<i64, _>("last_observed_at_ms") as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_conversion_rejects_values_outside_postgres_bigint() {
        let result = timestamp_to_i64("observed_at_ms", u64::MAX);
        assert!(matches!(result, Err(PostgresOpportunityStoreError::TimestampOverflow { .. })));
    }

    #[test]
    fn timestamp_conversion_accepts_bigint_max() {
        assert_eq!(timestamp_to_i64("observed_at_ms", i64::MAX as u64).unwrap(), i64::MAX);
    }
}
