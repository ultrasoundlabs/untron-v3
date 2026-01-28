use anyhow::{Context, Result};
use sqlx::{Postgres, QueryBuilder};

use crate::db::Db;

pub async fn list_unprocessed_subjective_pre_entitle_txids(
    db: &Db,
    limit: usize,
    recheck_after_secs: u64,
) -> Result<Vec<String>> {
    let limit: i64 = i64::try_from(limit.max(1)).context("limit out of range for bigint")?;
    let recheck_after_secs: i64 =
        i64::try_from(recheck_after_secs).context("recheck_after_secs out of range for bigint")?;

    // Candidates are txIds referenced by current subjective claims (origin=0) that are not yet
    // confirmed as objectively processed onchain (depositProcessed[txId] == true).
    //
    // We still re-check `processed=false` rows periodically to handle late proofs.
    let rows = sqlx::query_scalar::<Postgres, String>(
        r#"
        select c.origin_id::text
        from hub.claim_versions c
        left join hub.deposit_processed_cache dp
            on dp.tx_hash::text = c.origin_id::text
        where
            c.valid_to_seq is null
            and c.origin = 0
            and (
                dp.tx_hash is null
                or dp.processed = false
            )
            and (
                dp.last_checked_at is null
                or dp.last_checked_at < now() - ($1::bigint * interval '1 second')
            )
        order by c.valid_from_seq desc
        limit $2
        "#,
    )
    .bind(recheck_after_secs)
    .bind(limit)
    .fetch_all(&db.pool)
    .await
    .context("list_unprocessed_subjective_pre_entitle_txids")?;

    Ok(rows)
}

pub async fn upsert_deposit_processed_cache(db: &Db, rows: &[(String, bool)]) -> Result<()> {
    if rows.is_empty() {
        return Ok(());
    }

    let mut qb = QueryBuilder::new(
        "insert into hub.deposit_processed_cache (tx_hash, processed, last_checked_at, checked_count) ",
    );
    qb.push_values(rows, |mut b, (tx_hash, processed)| {
        b.push_bind(tx_hash);
        b.push_bind(processed);
        b.push("now()");
        b.push_bind(1_i64);
    });
    qb.push(
        " on conflict (tx_hash) do update set \
            processed = excluded.processed, \
            last_checked_at = excluded.last_checked_at, \
            checked_count = hub.deposit_processed_cache.checked_count + 1",
    );

    qb.build()
        .execute(&db.pool)
        .await
        .context("upsert hub.deposit_processed_cache")?;

    Ok(())
}
