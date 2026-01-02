use crate::{config::Stream, domain, util};
use anyhow::{Context, Result};
use sqlx::{
    Encode, PgPool, Postgres, QueryBuilder, Transaction, Type, postgres::PgPoolOptions,
    query_builder::Separated, query_scalar, types::Json,
};

#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {
    pub async fn connect(database_url: &str, max_connections: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(database_url)
            .await
            .context("connect to database")?;
        Ok(Self { pool })
    }
}

#[derive(Debug, Clone)]
pub enum ResolvedStream {
    Hub {
        chain_id: u64,
        /// Canonical DB representation: "hub" = chain.instance.contract_address (case preserved).
        contract_address_db: domain::HubContractAddressDb,
    },
    Controller {
        chain_id: u64,
        /// Canonical DB representation: "controller" = base58check (T…).
        contract_address_db: domain::ControllerContractAddressDb,
    },
}

impl ResolvedStream {
    pub fn into_parts(self) -> (Stream, u64, domain::ContractAddressDb) {
        match self {
            ResolvedStream::Hub {
                chain_id,
                contract_address_db,
            } => (
                Stream::Hub,
                chain_id,
                domain::ContractAddressDb::Hub(contract_address_db),
            ),
            ResolvedStream::Controller {
                chain_id,
                contract_address_db,
            } => (
                Stream::Controller,
                chain_id,
                domain::ContractAddressDb::Controller(contract_address_db),
            ),
        }
    }
}

pub async fn ensure_instance_config(
    db: &Db,
    stream: Stream,
    chain_id: u64,
    contract_address_input: &str,
) -> Result<ResolvedStream> {
    let genesis_tip = util::compute_event_chain_genesis(stream.index_name());
    let genesis_tip_hex = util::b256_hex(genesis_tip);

    if stream == Stream::Hub {
        contract_address_input
            .parse::<alloy::primitives::Address>()
            .with_context(|| format!("invalid hub contract address: {contract_address_input}"))?;
    }

    // Canonicalize the controller address to base58 via the DB helper (accepts base58 or 0x…).
    let controller_contract_address_db: Option<String> = match stream {
        Stream::Hub => None,
        Stream::Controller => Some(
            query_scalar("select chain.tron_address_from_text($1)::text")
                .bind(contract_address_input)
                .fetch_one(&db.pool)
                .await
                .context("canonicalize controller contract address")?,
        ),
    };

    // Does the instance already exist?
    let existing = sqlx::query_as::<Postgres, (i64, String, String)>(
        "select chain_id, contract_address::text, genesis_tip::text from chain.instance where stream = $1::chain.stream",
    )
    .bind(stream.as_str())
    .fetch_optional(&db.pool)
    .await
    .context("read chain.instance")?;

    match &existing {
        None => {
            configure_instance(
                &db.pool,
                stream,
                chain_id,
                contract_address_input,
                &genesis_tip_hex,
            )
            .await?;
        }
        Some((db_chain_id, db_contract_address, db_genesis_tip)) => {
            let db_chain_id = u64::try_from(*db_chain_id).context("db chain_id out of range")?;
            if db_chain_id != chain_id {
                anyhow::bail!(
                    "chain.instance mismatch for {}: chain_id db={} env={}",
                    stream.as_str(),
                    db_chain_id,
                    chain_id
                );
            }

            match stream {
                Stream::Hub => {
                    let db_addr = db_contract_address
                        .parse::<alloy::primitives::Address>()
                        .with_context(|| {
                            format!(
                                "invalid hub contract_address in chain.instance: {}",
                                db_contract_address
                            )
                        })?;
                    let env_addr = contract_address_input
                        .parse::<alloy::primitives::Address>()
                        .with_context(|| {
                            format!("invalid hub contract address: {contract_address_input}")
                        })?;
                    if db_addr != env_addr {
                        anyhow::bail!(
                            "chain.instance mismatch for hub: contract_address db={} env={}",
                            db_contract_address,
                            contract_address_input
                        );
                    }
                }
                Stream::Controller => {
                    let Some(contract_address_db) = &controller_contract_address_db else {
                        anyhow::bail!(
                            "internal error: missing canonical controller contract address"
                        );
                    };
                    if db_contract_address != contract_address_db {
                        anyhow::bail!(
                            "chain.instance mismatch for controller: contract_address db={} env={}",
                            db_contract_address,
                            contract_address_input
                        );
                    }
                }
            }

            if db_genesis_tip != &genesis_tip_hex {
                anyhow::bail!(
                    "chain.instance mismatch for {}: genesis_tip db={} env={}",
                    stream.as_str(),
                    db_genesis_tip,
                    genesis_tip_hex
                );
            }

            // Ensure `chain.stream_cursor` exists (some manual DB setups may forget it).
            let cursor_exists: Option<i64> = query_scalar(
                "select applied_through_seq from chain.stream_cursor where stream = $1::chain.stream",
            )
            .bind(stream.as_str())
            .fetch_optional(&db.pool)
            .await
            .context("read chain.stream_cursor")?;

            if cursor_exists.is_none() {
                configure_instance(
                    &db.pool,
                    stream,
                    chain_id,
                    contract_address_input,
                    &genesis_tip_hex,
                )
                .await?;
            }
        }
    }

    let contract_address_db: String = match &existing {
        Some((_, addr, _)) => addr.clone(),
        None => match stream {
            Stream::Hub => contract_address_input.to_string(),
            Stream::Controller => controller_contract_address_db
                .clone()
                .expect("controller canonicalization should have run"),
        },
    };

    Ok(match stream {
        Stream::Hub => ResolvedStream::Hub {
            chain_id,
            contract_address_db: domain::HubContractAddressDb::new(contract_address_db),
        },
        Stream::Controller => ResolvedStream::Controller {
            chain_id,
            contract_address_db: domain::ControllerContractAddressDb::new(contract_address_db),
        },
    })
}

async fn configure_instance(
    pool: &PgPool,
    stream: Stream,
    chain_id: u64,
    contract_address_input: &str,
    genesis_tip: &str,
) -> Result<()> {
    let mut tx = pool.begin().await.context("begin configure_instance tx")?;
    let chain_id = i64::try_from(chain_id).context("chain_id out of range for bigint")?;

    match stream {
        Stream::Hub => {
            sqlx::query(
                "select chain.configure_instance($1::chain.stream, $2::bigint, $3::chain_address, $4::bytes32_hex)",
            )
            .bind(stream.as_str())
            .bind(chain_id)
            .bind(contract_address_input)
            .bind(genesis_tip)
            .execute(&mut *tx)
            .await
            .context("chain.configure_instance(hub)")?;
        }
        Stream::Controller => {
            sqlx::query(
                "select chain.configure_instance($1::chain.stream, $2::bigint, (chain.tron_address_from_text($3))::text::chain_address, $4::bytes32_hex)",
            )
            .bind(stream.as_str())
            .bind(chain_id)
            .bind(contract_address_input)
            .bind(genesis_tip)
            .execute(&mut *tx)
            .await
            .context("chain.configure_instance(controller)")?;
        }
    }

    tx.commit().await.context("commit configure_instance tx")?;
    Ok(())
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
                block_hash: domain::BlockHash(
                    util::parse_b256_hex(&h).context("db block_hash invalid")?,
                ),
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
            block_hash: domain::BlockHash(
                util::parse_b256_hex(&h).context("db block_hash invalid")?,
            ),
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

    /// Base58 (T...) or 0x… (DB will convert via `chain.tron_address_from_text`)
    pub caller: domain::Caller,
    pub proved_tip: domain::Tip,
}

fn push_stream<'qb, 'args, Sep>(b: &mut Separated<'qb, 'args, Postgres, Sep>, stream: Stream)
where
    'args: 'qb,
    Sep: std::fmt::Display,
{
    b.push_bind(stream.as_str());
    b.push_unseparated("::chain.stream");
}

fn push_tron_address_from_text<'qb, 'args, Sep>(
    b: &mut Separated<'qb, 'args, Postgres, Sep>,
    address_text: impl Encode<'args, Postgres> + Type<Postgres> + 'args,
) where
    'args: 'qb,
    Sep: std::fmt::Display,
{
    b.push("chain.tron_address_from_text(");
    b.push_bind_unseparated(address_text);
    b.push_unseparated(")");
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
        push_stream(&mut b, row.stream);
        b.push_bind(row.chain_id);
        b.push_bind(row.contract_address.as_str());

        b.push_bind(row.block_number);
        b.push_bind(row.block_timestamp);
        b.push_bind(row.block_hash);

        b.push_bind(row.tx_hash);
        b.push_bind(row.log_index);
        b.push_bind(true); // canonical

        b.push_bind(row.event_seq);
        b.push_bind(row.prev_tip);
        b.push_bind(row.new_tip);
        b.push_bind(row.event_signature);
        b.push_bind(&row.abi_encoded_event_data);

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
        push_stream(&mut b, Stream::Controller);
        b.push_bind(row.chain_id);
        b.push_bind(row.contract_address.as_str());

        b.push_bind(row.block_number);
        b.push_bind(row.block_timestamp);
        b.push_bind(row.block_hash);

        b.push_bind(row.tx_hash);
        b.push_bind(row.log_index);
        b.push_bind(true);

        push_tron_address_from_text(&mut b, row.caller);
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
