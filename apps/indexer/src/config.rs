use anyhow::{Context, Result};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stream {
    Hub,
    Controller,
}

impl Stream {
    pub const fn as_str(self) -> &'static str {
        match self {
            Stream::Hub => "hub",
            Stream::Controller => "controller",
        }
    }

    pub const fn index_name(self) -> &'static str {
        match self {
            Stream::Hub => "UntronV3Index",
            Stream::Controller => "UntronControllerIndex",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub stream: Stream,
    pub chain_id: u64,
    pub rpc: crate::rpc::RpcConfig,
    /// "hub": 0x… EVM address. "controller": Tron base58check (T…) OR 0x… EVM address.
    pub contract_address: String,
    pub deployment_block: u64,

    pub confirmations: u64,
    pub poll_interval: Duration,
    pub chunk_blocks: u64,
    pub reorg_scan_depth: u64,
}

#[derive(Debug, Clone)]
pub struct ReceiverUsdtConfig {
    pub enabled: bool,
    pub preknown_receiver_salts: Vec<String>,
    pub controller_create2_prefix: u8,
    pub poll_interval: Duration,
    pub chunk_blocks: u64,
    pub to_batch_size: usize,
    pub backfill_concurrency: usize,
    pub discovery_interval: Duration,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub streams: Vec<StreamConfig>,
    pub receiver_usdt: ReceiverUsdtConfig,
    pub db_max_connections: u32,

    pub block_header_concurrency: usize,
    pub block_timestamp_cache_size: usize,

    /// How often each stream emits an INFO progress summary while running.
    pub progress_interval: Duration,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct BaseEnv {
    database_url: String,

    db_max_connections: u32,

    block_header_concurrency: usize,

    block_timestamp_cache_size: usize,

    #[serde(rename = "indexer_progress_interval_secs")]
    progress_interval_secs: u64,

    /// Optional: only run this stream ("hub" | "controller" | "all").
    #[serde(rename = "indexer_stream")]
    stream: Option<String>,
}

impl Default for BaseEnv {
    fn default() -> Self {
        Self {
            database_url: String::new(),
            db_max_connections: DEFAULT_DB_MAX_CONNECTIONS,
            block_header_concurrency: DEFAULT_BLOCK_HEADER_CONCURRENCY,
            block_timestamp_cache_size: DEFAULT_BLOCK_TIMESTAMP_CACHE_SIZE,
            progress_interval_secs: DEFAULT_PROGRESS_INTERVAL_SECS,
            stream: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct RetryEnv {
    #[serde(rename = "rpc_max_rate_limit_retries")]
    max_rate_limit_retries: u32,

    #[serde(rename = "rpc_initial_backoff_ms")]
    initial_backoff_ms: u64,

    #[serde(rename = "rpc_compute_units_per_second")]
    compute_units_per_second: u64,
}

impl Default for RetryEnv {
    fn default() -> Self {
        Self {
            max_rate_limit_retries: DEFAULT_RPC_MAX_RATE_LIMIT_RETRIES,
            initial_backoff_ms: DEFAULT_RPC_INITIAL_BACKOFF_MS,
            compute_units_per_second: DEFAULT_RPC_COMPUTE_UNITS_PER_SECOND,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct ReceiverUsdtEnv {
    #[serde(rename = "trc20_enabled")]
    enabled: bool,

    #[serde(default)]
    preknown_receiver_salts: String,

    #[serde(rename = "untron_controller_create2_prefix")]
    controller_create2_prefix: String,

    #[serde(rename = "trc20_poll_interval_secs")]
    poll_interval_secs: u64,

    #[serde(rename = "trc20_chunk_blocks")]
    chunk_blocks: u64,

    #[serde(rename = "trc20_to_batch_size")]
    to_batch_size: usize,

    #[serde(rename = "trc20_backfill_concurrency")]
    backfill_concurrency: usize,

    #[serde(rename = "trc20_discovery_interval_secs")]
    discovery_interval_secs: u64,
}

impl Default for ReceiverUsdtEnv {
    fn default() -> Self {
        Self {
            enabled: true,
            preknown_receiver_salts: String::new(),
            controller_create2_prefix: DEFAULT_UNTRON_CONTROLLER_CREATE2_PREFIX.to_string(),
            poll_interval_secs: DEFAULT_TRC20_POLL_INTERVAL_SECS,
            chunk_blocks: DEFAULT_TRC20_CHUNK_BLOCKS,
            to_batch_size: DEFAULT_TRC20_TO_BATCH_SIZE,
            backfill_concurrency: DEFAULT_TRC20_BACKFILL_CONCURRENCY,
            discovery_interval_secs: DEFAULT_TRC20_DISCOVERY_INTERVAL_SECS,
        }
    }
}

#[derive(Debug, Deserialize)]
struct StreamEnv {
    chain_id: u64,
    contract_address: String,
    deployment_block: u64,

    confirmations: Option<u64>,
    poll_interval_secs: Option<u64>,
    chunk_blocks: Option<u64>,
    reorg_scan_depth: Option<u64>,
}

pub fn load_config() -> Result<AppConfig> {
    let base: BaseEnv = envy::from_env().context("load base env config")?;
    if base.database_url.trim().is_empty() {
        anyhow::bail!("DATABASE_URL must be set");
    }
    let retry_env: RetryEnv = envy::from_env().context("load retry env config")?;
    let receiver_usdt_env: ReceiverUsdtEnv =
        envy::from_env().context("load receiver_usdt env config")?;

    let retry = crate::rpc::RetryConfig {
        max_rate_limit_retries: retry_env.max_rate_limit_retries,
        initial_backoff_ms: retry_env.initial_backoff_ms,
        compute_units_per_second: retry_env.compute_units_per_second,
    };

    let mut streams = Vec::new();
    if let Some(cfg) = load_stream_config(Stream::Hub, "HUB_", &retry)? {
        streams.push(cfg);
    }
    if let Some(cfg) = load_stream_config(Stream::Controller, "CONTROLLER_", &retry)? {
        streams.push(cfg);
    }

    if let Some(only) = base.stream.as_deref() {
        match only.to_lowercase().as_str() {
            "hub" => streams.retain(|s| s.stream == Stream::Hub),
            "controller" => streams.retain(|s| s.stream == Stream::Controller),
            "all" => {}
            other => {
                anyhow::bail!("invalid INDEXER_STREAM value: {other} (expected hub|controller|all)")
            }
        };
    }

    if streams.is_empty() {
        anyhow::bail!(
            "no streams configured (set HUB_* and/or CONTROLLER_* env vars; optionally set INDEXER_STREAM=hub|controller|all)"
        );
    }

    let preknown_receiver_salts = parse_list(&receiver_usdt_env.preknown_receiver_salts);
    let controller_create2_prefix = parse_bytes1(&receiver_usdt_env.controller_create2_prefix)
        .context("UNTRON_CONTROLLER_CREATE2_PREFIX")?;

    Ok(AppConfig {
        database_url: base.database_url,
        streams,
        receiver_usdt: ReceiverUsdtConfig {
            enabled: receiver_usdt_env.enabled,
            preknown_receiver_salts,
            controller_create2_prefix,
            poll_interval: Duration::from_secs(receiver_usdt_env.poll_interval_secs.max(1)),
            chunk_blocks: receiver_usdt_env.chunk_blocks.max(1),
            to_batch_size: receiver_usdt_env.to_batch_size.max(1),
            backfill_concurrency: receiver_usdt_env.backfill_concurrency.max(1),
            discovery_interval: Duration::from_secs(
                receiver_usdt_env.discovery_interval_secs.max(5),
            ),
        },
        db_max_connections: base.db_max_connections,
        block_header_concurrency: base.block_header_concurrency,
        block_timestamp_cache_size: base.block_timestamp_cache_size,
        progress_interval: Duration::from_secs(base.progress_interval_secs.max(1)),
    })
}

fn load_stream_config(
    stream: Stream,
    prefix: &'static str,
    retry: &crate::rpc::RetryConfig,
) -> Result<Option<StreamConfig>> {
    let defaults = match stream {
        Stream::Hub => StreamDefaults {
            // Default to "developer-friendly" settings (works well on anvil and most local RPCs):
            // - 0 confirmations so new blocks are indexed immediately
            // - 1s polling for low latency
            // - moderate chunk size with adaptive shrink-on-error
            // - shallower reorg scan depth (reorg detection still runs every tick)
            confirmations: 0,
            poll_interval: Duration::from_secs(1),
            chunk_blocks: 2_000,
            reorg_scan_depth: 128,
        },
        Stream::Controller => StreamDefaults {
            confirmations: 0,
            poll_interval: Duration::from_secs(1),
            chunk_blocks: 2_000,
            reorg_scan_depth: 256,
        },
    };

    let env_var = format!("{prefix}RPC_URLS");
    let Ok(rpc_urls_raw) = std::env::var(&env_var) else {
        return Ok(None);
    };
    let rpc_urls = parse_list(&rpc_urls_raw);
    if rpc_urls.is_empty() {
        return Ok(None);
    }

    let env: StreamEnv = envy::prefixed(prefix)
        .from_env()
        .with_context(|| format!("load {prefix} stream env"))?;
    let chain_id = env.chain_id;

    let contract_address = env.contract_address;

    let deployment_block = env.deployment_block;

    let confirmations = env.confirmations.unwrap_or(defaults.confirmations);

    let poll_interval_secs = env
        .poll_interval_secs
        .unwrap_or(defaults.poll_interval.as_secs());
    let poll_interval = Duration::from_secs(poll_interval_secs.max(1));

    let chunk_blocks = env.chunk_blocks.unwrap_or(defaults.chunk_blocks);
    let reorg_scan_depth = env.reorg_scan_depth.unwrap_or(defaults.reorg_scan_depth);

    Ok(Some(StreamConfig {
        stream,
        chain_id,
        rpc: crate::rpc::RpcConfig {
            urls: rpc_urls,
            retry: retry.clone(),
        },
        contract_address,
        deployment_block,
        confirmations,
        poll_interval,
        chunk_blocks: chunk_blocks.max(1),
        reorg_scan_depth: reorg_scan_depth.max(1),
    }))
}

struct StreamDefaults {
    confirmations: u64,
    poll_interval: Duration,
    chunk_blocks: u64,
    reorg_scan_depth: u64,
}

fn parse_list(raw: &str) -> Vec<String> {
    raw.split(|c: char| c == ',' || c.is_whitespace())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

fn parse_bytes1(value: &str) -> Result<u8> {
    let trimmed = value.trim();
    let normalized = trimmed.strip_prefix("0x").unwrap_or(trimmed);
    if normalized.is_empty() || normalized.len() > 2 {
        anyhow::bail!("invalid bytes1 value \"{value}\" (expected 0xNN)");
    }
    let byte = u8::from_str_radix(normalized, 16)
        .with_context(|| format!("invalid bytes1 value \"{value}\" (expected 0x00..0xff)"))?;
    Ok(byte)
}

const DEFAULT_DB_MAX_CONNECTIONS: u32 = 5;
const DEFAULT_BLOCK_HEADER_CONCURRENCY: usize = 16;
const DEFAULT_BLOCK_TIMESTAMP_CACHE_SIZE: usize = 2048;
const DEFAULT_PROGRESS_INTERVAL_SECS: u64 = 5;

const DEFAULT_RPC_MAX_RATE_LIMIT_RETRIES: u32 = 8;
const DEFAULT_RPC_INITIAL_BACKOFF_MS: u64 = 250;
const DEFAULT_RPC_COMPUTE_UNITS_PER_SECOND: u64 = 500;

const DEFAULT_UNTRON_CONTROLLER_CREATE2_PREFIX: &str = "0x41";
const DEFAULT_TRC20_POLL_INTERVAL_SECS: u64 = 2;
const DEFAULT_TRC20_CHUNK_BLOCKS: u64 = 2000;
const DEFAULT_TRC20_TO_BATCH_SIZE: usize = 50;
const DEFAULT_TRC20_BACKFILL_CONCURRENCY: usize = 2;
const DEFAULT_TRC20_DISCOVERY_INTERVAL_SECS: u64 = 30;
