use alloy::primitives::{Address, B256, FixedBytes, keccak256};
use anyhow::{Context, Result};
use sqlx::types::BigDecimal;
use sqlx::{Postgres, QueryBuilder, Transaction, query_scalar};
use std::collections::HashMap;
use std::str::FromStr;

use crate::db::Db;

pub struct WatchlistSnapshot {
    pub updated_at_epoch: i64,
    pub receiver_map: HashMap<alloy::primitives::Address, String>,
    pub to_addrs: Vec<alloy::primitives::Address>,
}

#[derive(Debug, Clone)]
pub struct WatchReceiverRow {
    pub receiver_salt: String,
    pub receiver_evm: String,
}

pub async fn list_hub_receiver_salts(db: &Db) -> Result<Vec<String>> {
    // "Ever mentioned" salts: the full history of hub.lease_versions.
    // (We intentionally do not filter to current leases.)
    let rows = sqlx::query_scalar::<Postgres, String>(
        "select distinct receiver_salt::text from hub.lease_versions",
    )
    .fetch_all(&db.pool)
    .await
    .context("list hub receiver salts")?;
    Ok(rows)
}

pub async fn upsert_watchlist(
    db: &Db,
    deployment_block: u64,
    rows: &[(String, String, String, String)],
) -> Result<()> {
    if rows.is_empty() {
        return Ok(());
    }

    let mut tx = db.pool.begin().await.context("begin upsert_watchlist tx")?;
    upsert_watchlist_tx(&mut tx, deployment_block, rows).await?;
    tx.commit().await.context("commit upsert_watchlist tx")?;
    Ok(())
}

async fn upsert_watchlist_tx(
    tx: &mut Transaction<'_, Postgres>,
    deployment_block: u64,
    rows: &[(String, String, String, String)],
) -> Result<()> {
    let deployment_block: i64 =
        i64::try_from(deployment_block).context("deployment_block out of range for bigint")?;

    let mut qb = QueryBuilder::new(
        "insert into ctl.receiver_watchlist (receiver_salt, receiver_evm, receiver, source, backfill_next_block) ",
    );

    qb.push_values(
        rows,
        |mut b, (salt, receiver_evm, receiver_tron, source)| {
            b.push_bind(salt);
            b.push_bind(receiver_evm);
            b.push_bind(receiver_tron);
            b.push_bind(source);
            b.push_bind(deployment_block);
        },
    );

    qb.push(
        " on conflict (receiver_salt) do update set \
          receiver_evm = excluded.receiver_evm, \
          receiver = excluded.receiver, \
          source = case when ctl.receiver_watchlist.source = 'env' then 'env' else excluded.source end, \
          updated_at = now()",
    );

    qb.build()
        .execute(&mut **tx)
        .await
        .context("upsert ctl.receiver_watchlist")?;

    Ok(())
}

pub fn compute_create2_address(
    create2_prefix: u8,
    deployer: Address,
    salt: FixedBytes<32>,
    init_code_hash: B256,
) -> Address {
    let mut data = [0u8; 1 + 20 + 32 + 32];
    data[0] = create2_prefix;
    data[1..21].copy_from_slice(deployer.as_slice());
    data[21..53].copy_from_slice(salt.as_slice());
    data[53..85].copy_from_slice(init_code_hash.as_slice());
    let hash = keccak256(data);
    Address::from_slice(&hash.as_slice()[12..])
}

pub async fn upsert_watchlist_from_sources(
    db: &Db,
    deployment_block: u64,
    preknown_receiver_salts: &[String],
    controller_create2_prefix: u8,
    controller_address_evm: alloy::primitives::Address,
    init_code_hash: alloy::primitives::B256,
) -> Result<()> {
    let mut salts: HashMap<String, &'static str> = HashMap::new();
    for s in preknown_receiver_salts {
        salts.insert(s.clone(), "env");
    }

    for s in list_hub_receiver_salts(db).await? {
        salts.entry(s).or_insert("hub");
    }

    let mut rows: Vec<(String, String, String, String)> = Vec::with_capacity(salts.len());
    for (salt_str, source) in salts {
        let salt = FixedBytes::<32>::from_str(&salt_str)
            .with_context(|| format!("invalid receiver salt: {salt_str}"))?;
        let receiver_evm = compute_create2_address(
            controller_create2_prefix,
            controller_address_evm,
            salt,
            init_code_hash,
        );
        let receiver_evm_str = receiver_evm.to_checksum_buffer(None).to_string();
        let receiver_tron = crate::domain::TronAddress::from_evm(receiver_evm).to_string();
        rows.push((
            salt_str,
            receiver_evm_str,
            receiver_tron,
            source.to_string(),
        ));
    }

    upsert_watchlist(db, deployment_block, &rows).await?;
    Ok(())
}

pub async fn ensure_tail_cursor(db: &Db, deployment_block: u64) -> Result<u64> {
    let deployment_block: i64 =
        i64::try_from(deployment_block).context("deployment_block out of range for bigint")?;

    let mut tx = db
        .pool
        .begin()
        .await
        .context("begin ensure_tail_cursor tx")?;
    sqlx::query(
        "insert into ctl.receiver_usdt_tail_cursor (stream, next_block) \
         values ('controller', $1) \
         on conflict (stream) do nothing",
    )
    .bind(deployment_block)
    .execute(&mut *tx)
    .await
    .context("insert receiver_usdt_tail_cursor")?;

    let next: i64 = query_scalar(
        "select next_block from ctl.receiver_usdt_tail_cursor where stream = 'controller'",
    )
    .fetch_one(&mut *tx)
    .await
    .context("read receiver_usdt_tail_cursor")?;

    tx.commit().await.context("commit ensure_tail_cursor tx")?;

    u64::try_from(next).context("tail cursor next_block out of range")
}

pub async fn clear_all_backfill(db: &Db) -> Result<()> {
    sqlx::query(
        "update ctl.receiver_watchlist set backfill_next_block = null, updated_at = now() where backfill_next_block is not null",
    )
    .execute(&db.pool)
    .await
    .context("clear receiver watchlist backfill cursors")?;
    Ok(())
}

pub async fn update_tail_cursor(db: &Db, next_block: u64) -> Result<()> {
    let next_block: i64 =
        i64::try_from(next_block).context("next_block out of range for bigint")?;
    sqlx::query(
        "update ctl.receiver_usdt_tail_cursor set next_block = $1, updated_at = now() where stream = 'controller'",
    )
    .bind(next_block)
    .execute(&db.pool)
    .await
    .context("update receiver_usdt_tail_cursor")?;
    Ok(())
}

pub async fn list_watch_receivers(db: &Db) -> Result<Vec<WatchReceiverRow>> {
    let rows = sqlx::query_as::<Postgres, (String, String)>(
        "select receiver_salt::text, receiver_evm::text from ctl.receiver_watchlist",
    )
    .fetch_all(&db.pool)
    .await
    .context("list receiver_watchlist")?;
    Ok(rows
        .into_iter()
        .map(|(receiver_salt, receiver_evm)| WatchReceiverRow {
            receiver_salt,
            receiver_evm,
        })
        .collect())
}

pub async fn watchlist_last_updated_at_epoch(db: &Db) -> Result<Option<i64>> {
    let ts: Option<i64> = query_scalar(
        "select extract(epoch from max(updated_at))::bigint from ctl.receiver_watchlist",
    )
    .fetch_one(&db.pool)
    .await
    .context("read receiver_watchlist max(updated_at)")?;
    Ok(ts)
}

pub async fn load_watchlist_snapshot(db: &Db) -> Result<WatchlistSnapshot> {
    let updated_at_epoch = watchlist_last_updated_at_epoch(db).await?.unwrap_or(0);
    let receivers = list_watch_receivers(db).await?;
    let receiver_map = build_receiver_map(&receivers)?;
    let to_addrs = receiver_map.keys().copied().collect::<Vec<_>>();
    Ok(WatchlistSnapshot {
        updated_at_epoch,
        receiver_map,
        to_addrs,
    })
}

pub async fn list_backfill_batch(
    db: &Db,
    limit: usize,
) -> Result<Option<(u64, Vec<WatchReceiverRow>)>> {
    let start: Option<i64> = query_scalar(
        "select min(backfill_next_block) from ctl.receiver_watchlist where backfill_next_block is not null",
    )
    .fetch_one(&db.pool)
    .await
    .context("read min backfill_next_block")?;

    let Some(start) = start else {
        return Ok(None);
    };

    let limit: i64 = i64::try_from(limit).context("limit out of range for bigint")?;
    let rows = sqlx::query_as::<Postgres, (String, String)>(
        "select receiver_salt::text, receiver_evm::text \
         from ctl.receiver_watchlist \
         where backfill_next_block = $1 \
         order by receiver_salt asc \
         limit $2",
    )
    .bind(start)
    .bind(limit)
    .fetch_all(&db.pool)
    .await
    .context("list backfill receiver batch")?;

    let receivers = rows
        .into_iter()
        .map(|(receiver_salt, receiver_evm)| WatchReceiverRow {
            receiver_salt,
            receiver_evm,
        })
        .collect::<Vec<_>>();

    if receivers.is_empty() {
        return Ok(None);
    }

    Ok(Some((
        u64::try_from(start).context("backfill_next_block out of range")?,
        receivers,
    )))
}

pub struct BackfillWork {
    pub start_block: u64,
    pub stop_at_or_above: u64,
    pub receiver_map: HashMap<alloy::primitives::Address, String>,
    pub to_addrs: Vec<alloy::primitives::Address>,
}

pub async fn next_backfill_work(
    db: &Db,
    deployment_block: u64,
    limit: usize,
) -> Result<Option<BackfillWork>> {
    let stop_at_or_above = ensure_tail_cursor(db, deployment_block).await?;
    let Some((start_block, batch)) = list_backfill_batch(db, limit).await? else {
        return Ok(None);
    };
    let receiver_map = build_receiver_map(&batch)?;
    let to_addrs = receiver_map.keys().copied().collect::<Vec<_>>();
    Ok(Some(BackfillWork {
        start_block,
        stop_at_or_above,
        receiver_map,
        to_addrs,
    }))
}

pub async fn advance_backfill_batch(
    db: &Db,
    start_block: u64,
    next_block: u64,
    stop_at_or_above: u64,
) -> Result<()> {
    let start_block: i64 =
        i64::try_from(start_block).context("start_block out of range for bigint")?;
    let next_block: i64 =
        i64::try_from(next_block).context("next_block out of range for bigint")?;
    let stop_at_or_above: i64 =
        i64::try_from(stop_at_or_above).context("stop_at_or_above out of range for bigint")?;

    sqlx::query(
        "update ctl.receiver_watchlist \
         set backfill_next_block = case \
             when $2 >= $3 then null \
             else $2 \
         end, \
         updated_at = now() \
         where backfill_next_block = $1",
    )
    .bind(start_block)
    .bind(next_block)
    .bind(stop_at_or_above)
    .execute(&db.pool)
    .await
    .context("advance backfill batch")?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct UsdtSetPoint {
    pub block_number: u64,
    pub usdt_tron: String,
    pub usdt_evm: String,
}

pub async fn usdt_set_points_up_to(
    db: &Db,
    from_block: u64,
    to_block: u64,
) -> Result<Vec<UsdtSetPoint>> {
    let from_block: i64 =
        i64::try_from(from_block).context("from_block out of range for bigint")?;
    let to_block: i64 = i64::try_from(to_block).context("to_block out of range for bigint")?;

    let rows = sqlx::query_as::<Postgres, (i64, String, String)>(
        r#"
        with prior as (
          select block_number, log_index, args->>'new_usdt' as usdt_evm
          from chain.event_appended
          where stream='controller'
            and canonical
            and event_type='UsdtSet'
            and block_number < $1
          order by block_number desc, log_index desc
          limit 1
        ),
        within as (
          select block_number, log_index, args->>'new_usdt' as usdt_evm
          from chain.event_appended
          where stream='controller'
            and canonical
            and event_type='UsdtSet'
            and block_number between $1 and $2
          order by block_number asc, log_index asc
        )
        select
          q.block_number,
          chain.tron_address_from_text(q.usdt_evm)::text as usdt_tron,
          q.usdt_evm
        from (
          select * from prior
          union all
          select * from within
        ) q
        order by q.block_number asc, q.log_index asc
        "#,
    )
    .bind(from_block)
    .bind(to_block)
    .fetch_all(&db.pool)
    .await
    .context("read controller UsdtSet points")?;

    rows.into_iter()
        .map(|(block_number, usdt_tron, usdt_evm)| {
            Ok(UsdtSetPoint {
                block_number: u64::try_from(block_number).context("block_number out of range")?,
                usdt_tron,
                usdt_evm,
            })
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct TransferRow {
    pub chain_id: i64,
    pub token: String,
    pub receiver_salt: String,
    pub sender: String,
    pub recipient: String,
    pub amount: BigDecimal,
    pub block_number: i64,
    pub block_timestamp: i64,
    pub block_hash: String,
    pub tx_hash: String,
    pub log_index: i32,
}

pub async fn insert_transfers(db: &Db, rows: &[TransferRow]) -> Result<()> {
    if rows.is_empty() {
        return Ok(());
    }

    let mut tx = db.pool.begin().await.context("begin insert_transfers tx")?;
    insert_transfers_tx(&mut tx, rows).await?;
    tx.commit().await.context("commit insert_transfers tx")?;
    Ok(())
}

async fn insert_transfers_tx(
    tx: &mut Transaction<'_, Postgres>,
    rows: &[TransferRow],
) -> Result<()> {
    let mut qb = QueryBuilder::new(
        "insert into ctl.receiver_usdt_transfers (\
         stream, chain_id, token, receiver_salt, \
         sender, recipient, amount, \
         block_number, block_timestamp, block_hash, \
         tx_hash, log_index, canonical\
         ) ",
    );

    qb.push_values(rows, |mut b, row| {
        // `stream` is a Postgres enum (`chain.stream`); binding a Rust `&str` makes it `text`,
        // which does not implicitly coerce to the enum type in prepared statements.
        b.push("'controller'::chain.stream");
        b.push_bind(row.chain_id);
        b.push_bind(&row.token);
        b.push_bind(&row.receiver_salt);
        b.push_bind(&row.sender);
        b.push_bind(&row.recipient);
        b.push_bind(&row.amount);
        b.push_bind(row.block_number);
        b.push_bind(row.block_timestamp);
        b.push_bind(&row.block_hash);
        b.push_bind(&row.tx_hash);
        b.push_bind(row.log_index);
        b.push_bind(true);
    });

    qb.push(
        " on conflict (chain_id, tx_hash, log_index) do update set \
          token = excluded.token, \
          receiver_salt = excluded.receiver_salt, \
          sender = excluded.sender, \
          recipient = excluded.recipient, \
          amount = excluded.amount, \
          block_number = excluded.block_number, \
          block_timestamp = excluded.block_timestamp, \
          block_hash = excluded.block_hash, \
          canonical = excluded.canonical \
          where \
            ctl.receiver_usdt_transfers.token is distinct from excluded.token \
            or ctl.receiver_usdt_transfers.receiver_salt is distinct from excluded.receiver_salt \
            or ctl.receiver_usdt_transfers.sender is distinct from excluded.sender \
            or ctl.receiver_usdt_transfers.recipient is distinct from excluded.recipient \
            or ctl.receiver_usdt_transfers.amount is distinct from excluded.amount \
            or ctl.receiver_usdt_transfers.block_number is distinct from excluded.block_number \
            or ctl.receiver_usdt_transfers.block_timestamp is distinct from excluded.block_timestamp \
            or ctl.receiver_usdt_transfers.block_hash is distinct from excluded.block_hash \
            or ctl.receiver_usdt_transfers.canonical is distinct from excluded.canonical",
    );

    qb.build()
        .execute(&mut **tx)
        .await
        .context("insert ctl.receiver_usdt_transfers")?;

    Ok(())
}

pub fn build_receiver_map(
    rows: &[WatchReceiverRow],
) -> Result<HashMap<alloy::primitives::Address, String>> {
    let mut map = HashMap::with_capacity(rows.len());
    for r in rows {
        let addr = r
            .receiver_evm
            .parse::<alloy::primitives::Address>()
            .with_context(|| format!("invalid receiver_evm address in DB: {}", r.receiver_evm))?;
        map.insert(addr, r.receiver_salt.clone());
    }
    Ok(map)
}
