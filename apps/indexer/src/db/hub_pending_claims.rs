use anyhow::{Context, Result};
use sqlx::{Postgres, query_as};

use crate::db::Db;

#[derive(Debug, Clone, Copy)]
pub struct PendingClaimsStats {
    pub pending_total: u64,
    pub avg_age_seconds: u64,
    pub max_age_seconds: u64,
}

pub async fn pending_claims_stats(db: &Db) -> Result<PendingClaimsStats> {
    // Age is computed from the ClaimCreated hub event timestamp for the current "created" version.
    // We join on valid_from_seq (hub event seq) to chain.event_appended to get block_timestamp.
    let (count, avg_age, max_age) = query_as::<Postgres, (i64, f64, f64)>(
        r#"
        select
            count(*)::bigint as pending_total,
            coalesce(avg(extract(epoch from (now() - to_timestamp(e.block_timestamp)))), 0)::double precision as avg_age_seconds,
            coalesce(max(extract(epoch from (now() - to_timestamp(e.block_timestamp)))), 0)::double precision as max_age_seconds
        from hub.claim_versions c
        join chain.event_appended e
            on e.stream = 'hub'
            and e.canonical
            and e.event_seq = c.valid_from_seq
        where
            c.valid_to_seq is null
            and c.status = 'created'
        "#,
    )
    .fetch_one(&db.pool)
    .await
    .context("query pending hub claims stats")?;

    Ok(PendingClaimsStats {
        pending_total: u64::try_from(count).unwrap_or_default(),
        avg_age_seconds: avg_age.max(0.0).round() as u64,
        max_age_seconds: max_age.max(0.0).round() as u64,
    })
}
