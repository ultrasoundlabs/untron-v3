use crate::{config::Stream, domain};
use alloy::primitives::B256;
use anyhow::{Context, Result};
use sqlx::{Postgres, QueryBuilder, Transaction, query_scalar, types::Json};
use std::str::FromStr;

use super::Db;

pub fn parse_b256_hex(value: &str) -> Result<B256> {
    B256::from_str(value).with_context(|| format!("invalid bytes32 hex: {value}"))
}

pub async fn resume_from_block(db: &Db, stream: Stream, deployment_block: u64) -> Result<u64> {
    let max_block: Option<i64> = query_scalar(
        "select max(block_number) from chain.event_appended where stream = $1::chain.stream and canonical",
    )
    .bind(stream.as_str())
    .fetch_one(&db.pool)
    .await
    .context("read max(block_number)")?;

    let next = match max_block {
        None => deployment_block,
        Some(b) => u64::try_from(b)
            .ok()
            .unwrap_or(deployment_block)
            .saturating_add(1)
            .max(deployment_block),
    };

    Ok(next)
}

#[derive(Debug, Clone)]
pub struct StoredBlockHash {
    pub block_number: u64,
    pub block_hash: domain::BlockHash,
}

pub async fn recent_canonical_block_hashes(
    db: &Db,
    stream: Stream,
    limit: u64,
) -> Result<Vec<StoredBlockHash>> {
    // Deduplicate per-block without scanning/aggregating the full table.
    // This matches `event_appended_canonical_block` index ordering.
    let rows = sqlx::query_as::<Postgres, (i64, String)>(
        r#"
        select distinct on (block_number)
          block_number,
          block_hash::text as block_hash
        from chain.event_appended
        where stream = $1::chain.stream and canonical
        order by block_number desc, log_index desc
        limit $2
        "#,
    )
    .bind(stream.as_str())
    .bind(i64::try_from(limit).context("limit out of range for bigint")?)
    .fetch_all(&db.pool)
    .await
    .context("read recent canonical block hashes")?;

    rows.into_iter()
        .map(|(n, h)| {
            Ok(StoredBlockHash {
                block_number: u64::try_from(n)
                    .with_context(|| format!("db block_number out of range: {n}"))?,
                block_hash: domain::BlockHash(parse_b256_hex(&h).context("db block_hash invalid")?),
            })
        })
        .collect()
}

pub async fn latest_canonical_block_hash(
    db: &Db,
    stream: Stream,
) -> Result<Option<StoredBlockHash>> {
    let row = sqlx::query_as::<Postgres, (i64, String)>(
        r#"
        select
          block_number,
          block_hash::text as block_hash
        from chain.event_appended
        where stream = $1::chain.stream and canonical
        order by block_number desc, log_index desc
        limit 1
        "#,
    )
    .bind(stream.as_str())
    .fetch_optional(&db.pool)
    .await
    .context("read latest canonical block hash")?;

    row.map(|(n, h)| {
        Ok(StoredBlockHash {
            block_number: u64::try_from(n)
                .with_context(|| format!("db block_number out of range: {n}"))?,
            block_hash: domain::BlockHash(parse_b256_hex(&h).context("db block_hash invalid")?),
        })
    })
    .transpose()
}

pub async fn invalidate_from_block(db: &Db, stream: Stream, from_block: u64) -> Result<()> {
    let mut tx = db.pool.begin().await.context("begin invalidate tx")?;
    invalidate_from_block_tx(&mut tx, stream, from_block).await?;
    tx.commit().await.context("commit invalidate tx")?;
    Ok(())
}

async fn invalidate_from_block_tx(
    tx: &mut Transaction<'_, Postgres>,
    stream: Stream,
    from_block: u64,
) -> Result<()> {
    let from_block = i64::try_from(from_block).context("from_block out of range for bigint")?;
    sqlx::query(
        "update chain.event_appended set canonical = false where stream = $1::chain.stream and canonical and block_number >= $2",
    )
    .bind(stream.as_str())
    .bind(from_block)
    .execute(&mut **tx)
    .await
    .context("invalidate chain.event_appended")?;

    if stream == Stream::Controller {
        sqlx::query(
            "update chain.controller_tip_proofs set canonical = false where canonical and block_number >= $1",
        )
        .bind(from_block)
        .execute(&mut **tx)
        .await
        .context("invalidate chain.controller_tip_proofs")?;
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct EventAppendedRow {
    pub stream: Stream,
    pub chain_id: i64,
    pub contract_address: domain::ContractAddressDb,

    pub block_number: i64,
    pub block_timestamp: i64,
    pub block_hash: domain::BlockHash,

    pub tx_hash: domain::TxHash,
    pub log_index: i32,

    pub event_seq: i64,
    pub prev_tip: domain::Tip,
    pub new_tip: domain::Tip,
    pub event_signature: domain::EventSignature,
    pub abi_encoded_event_data: domain::AbiEncodedEventData,

    pub event_type: String,
    pub args_json: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ControllerTipProofRow {
    pub chain_id: i64,
    pub contract_address: domain::ControllerContractAddressDb,

    pub block_number: i64,
    pub block_timestamp: i64,
    pub block_hash: domain::BlockHash,

    pub tx_hash: domain::TxHash,
    pub log_index: i32,

    pub caller: domain::TronAddress,
    pub proved_tip: domain::Tip,
}

async fn insert_event_appended_rows_tx(
    tx: &mut Transaction<'_, Postgres>,
    rows: &[EventAppendedRow],
) -> Result<()> {
    let mut qb = QueryBuilder::new(
        "insert into chain.event_appended (\
         stream, chain_id, contract_address, \
         block_number, block_timestamp, block_hash, \
         tx_hash, log_index, canonical, \
         event_seq, prev_tip, new_tip, event_signature, abi_encoded_event_data, \
         event_type, args\
         ) ",
    );

    qb.push_values(rows, |mut b, row| {
        b.push_bind(row.stream.as_str())
            .push_unseparated("::chain.stream");
        b.push_bind(row.chain_id);
        b.push_bind(row.contract_address.as_str());

        b.push_bind(row.block_number);
        b.push_bind(row.block_timestamp);
        b.push_bind(row.block_hash);

        b.push_bind(row.tx_hash);
        b.push_bind(row.log_index);
        b.push_bind(true);

        b.push_bind(row.event_seq);
        b.push_bind(row.prev_tip);
        b.push_bind(row.new_tip);
        b.push_bind(row.event_signature);
        b.push_bind(row.abi_encoded_event_data.clone());

        b.push_bind(&row.event_type);
        b.push_bind(Json(&row.args_json));
    });

    qb.push(
        " on conflict (chain_id, tx_hash, log_index) do update set \
          stream = excluded.stream, \
          contract_address = excluded.contract_address, \
          block_number = excluded.block_number, \
          block_timestamp = excluded.block_timestamp, \
          block_hash = excluded.block_hash, \
          canonical = excluded.canonical, \
          event_seq = excluded.event_seq, \
          prev_tip = excluded.prev_tip, \
          new_tip = excluded.new_tip, \
          event_signature = excluded.event_signature, \
          abi_encoded_event_data = excluded.abi_encoded_event_data, \
          event_type = excluded.event_type, \
          args = excluded.args \
          where \
            chain.event_appended.stream is distinct from excluded.stream \
            or chain.event_appended.contract_address is distinct from excluded.contract_address \
            or chain.event_appended.block_number is distinct from excluded.block_number \
            or chain.event_appended.block_timestamp is distinct from excluded.block_timestamp \
            or chain.event_appended.block_hash is distinct from excluded.block_hash \
            or chain.event_appended.canonical is distinct from excluded.canonical \
            or chain.event_appended.event_seq is distinct from excluded.event_seq \
            or chain.event_appended.prev_tip is distinct from excluded.prev_tip \
            or chain.event_appended.new_tip is distinct from excluded.new_tip \
            or chain.event_appended.event_signature is distinct from excluded.event_signature \
            or chain.event_appended.abi_encoded_event_data is distinct from excluded.abi_encoded_event_data \
            or chain.event_appended.event_type is distinct from excluded.event_type \
            or chain.event_appended.args is distinct from excluded.args",
    );

    qb.build()
        .execute(&mut **tx)
        .await
        .context("insert chain.event_appended")?;

    Ok(())
}

async fn insert_controller_tip_proofs_tx(
    tx: &mut Transaction<'_, Postgres>,
    rows: &[ControllerTipProofRow],
) -> Result<()> {
    let mut qb = QueryBuilder::new(
        "insert into chain.controller_tip_proofs (\
         stream, chain_id, contract_address, \
         block_number, block_timestamp, block_hash, \
         tx_hash, log_index, canonical, \
         caller, proved_tip\
         ) ",
    );

    qb.push_values(rows, |mut b, row| {
        b.push_bind(Stream::Controller.as_str())
            .push_unseparated("::chain.stream");
        b.push_bind(row.chain_id);
        b.push_bind(row.contract_address.as_str());

        b.push_bind(row.block_number);
        b.push_bind(row.block_timestamp);
        b.push_bind(row.block_hash);

        b.push_bind(row.tx_hash);
        b.push_bind(row.log_index);
        b.push_bind(true);

        b.push_bind(row.caller);
        b.push_bind(row.proved_tip);
    });

    qb.push(
        " on conflict (chain_id, tx_hash, log_index) do update set \
          block_number = excluded.block_number, \
          block_timestamp = excluded.block_timestamp, \
          block_hash = excluded.block_hash, \
          canonical = excluded.canonical, \
          caller = excluded.caller, \
          proved_tip = excluded.proved_tip \
          where \
            chain.controller_tip_proofs.block_number is distinct from excluded.block_number \
            or chain.controller_tip_proofs.block_timestamp is distinct from excluded.block_timestamp \
            or chain.controller_tip_proofs.block_hash is distinct from excluded.block_hash \
            or chain.controller_tip_proofs.canonical is distinct from excluded.canonical \
            or chain.controller_tip_proofs.caller is distinct from excluded.caller \
            or chain.controller_tip_proofs.proved_tip is distinct from excluded.proved_tip",
    );

    qb.build()
        .execute(&mut **tx)
        .await
        .context("insert chain.controller_tip_proofs")?;

    Ok(())
}

pub async fn insert_batch(
    db: &Db,
    event_rows: &[EventAppendedRow],
    proof_rows: &[ControllerTipProofRow],
) -> Result<()> {
    if event_rows.is_empty() && proof_rows.is_empty() {
        return Ok(());
    }

    let mut tx = db.pool.begin().await.context("begin insert batch tx")?;
    if !event_rows.is_empty() {
        insert_event_appended_rows_tx(&mut tx, event_rows).await?;
    }
    if !proof_rows.is_empty() {
        insert_controller_tip_proofs_tx(&mut tx, proof_rows).await?;
    }
    tx.commit().await.context("commit insert batch tx")?;
    Ok(())
}
