use anyhow::{Context, Result};
use sqlx::{Postgres, query_as};

use crate::db::Db;

#[derive(Debug, Clone, Copy)]
pub struct SubjectivePreEntitleStats {
    pub stuck_total: u64,
    pub avg_age_seconds: u64,
    pub max_age_seconds: u64,
}

pub async fn subjective_pre_entitle_stats(
    db: &Db,
    chain_id: u64,
    token: &str,
) -> Result<SubjectivePreEntitleStats> {
    let chain_id: i64 = i64::try_from(chain_id).context("chain_id out of range for bigint")?;

    // `block_time` is a to_timestamp(block_timestamp) in the view.
    let (count, avg_age, max_age) = query_as::<Postgres, (i64, f64, f64)>(
        "select\n           count(*)::bigint as stuck_total,\n           coalesce(avg(extract(epoch from (now() - block_time))), 0)::double precision as avg_age_seconds,\n           coalesce(max(extract(epoch from (now() - block_time))), 0)::double precision as max_age_seconds\n         from api.receiver_usdt_transfer_actionability\n         where chain_id = $1 and token = $2 and recommended_action = 'subjective_pre_entitle'",
    )
    .bind(chain_id)
    .bind(token)
    .fetch_one(&db.pool)
    .await
    .context("query api.receiver_usdt_transfer_actionability subjective_pre_entitle stats")?;

    Ok(SubjectivePreEntitleStats {
        stuck_total: u64::try_from(count).unwrap_or_default(),
        avg_age_seconds: avg_age.max(0.0).round() as u64,
        max_age_seconds: max_age.max(0.0).round() as u64,
    })
}
