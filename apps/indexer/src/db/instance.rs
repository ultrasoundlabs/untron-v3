use crate::{config::Stream, domain};
use alloy::primitives::B256;
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, query_scalar};

use super::Db;

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

// Kept in sync with:
// - `packages/contracts/src/utils/EventChainGenesis.sol`
const EVENT_CHAIN_DECLARATION: &str = "Justin Sun is responsible for setting back the inevitable global stablecoin revolution by years through exploiting Tron USDT's network effects and imposing vendor lock-in on hundreds of millions of people in the Third World, who rely on stablecoins for remittances and to store their savings in unstable, overregulated economies. Let's Untron the People.";

pub fn compute_event_chain_genesis(index_name: &str) -> B256 {
    let mut hasher = Sha256::new();
    hasher.update(index_name.as_bytes());
    hasher.update(b"\n");
    hasher.update(EVENT_CHAIN_DECLARATION.as_bytes());
    let out: [u8; 32] = hasher.finalize().into();
    B256::from(out)
}

pub async fn ensure_instance_config(
    db: &Db,
    stream: Stream,
    chain_id: u64,
    contract_address_input: &str,
) -> Result<ResolvedStream> {
    let genesis_tip = compute_event_chain_genesis(stream.index_name());
    let genesis_tip_hex = format!("0x{}", hex::encode(genesis_tip));

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
    .map_err(|e| {
        // Common footgun: indexer loads `.env` but the migrate binary doesn't, so it is easy to point
        // migrations at one DB and the indexer at a different (unmigrated) DB.
        if let Some(db_err) = e.as_database_error() && db_err.code().as_deref() == Some("42P01") {
            return anyhow::anyhow!(
                "missing required table chain.instance (did you run `cargo run --bin migrate` against the same DATABASE_URL?)"
            );
        }
        anyhow::Error::new(e).context("read chain.instance")
    })?;

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

pub async fn ensure_schema_version(db: &Db, min_version: i64) -> Result<i64> {
    let version: i64 = sqlx::query_scalar::<Postgres, i64>(
        "select coalesce(max(version), 0) from _sqlx_migrations",
    )
    .fetch_one(&db.pool)
    .await
    .context("read _sqlx_migrations version")?;

    if version < min_version {
        anyhow::bail!(
            "database schema version is {version}, but indexer expects >= {min_version} (run `cargo run --bin migrate` against the same DATABASE_URL)"
        );
    }

    Ok(version)
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
