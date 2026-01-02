use anyhow::{Context, Result};
use clap::{Args, Parser, ValueEnum};
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
    pub rpc_urls: Vec<String>,
    /// "hub": 0x… EVM address. "controller": Tron base58check (T…) OR 0x… EVM address.
    pub contract_address: String,
    pub deployment_block: u64,

    pub confirmations: u64,
    pub poll_interval: Duration,
    pub chunk_blocks: u64,
    pub reorg_scan_depth: u64,
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_rate_limit_retries: u32,
    pub initial_backoff_ms: u64,
    pub compute_units_per_second: u64,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub streams: Vec<StreamConfig>,

    pub retry: RetryConfig,
    pub db_max_connections: u32,

    pub block_header_concurrency: usize,
    pub block_timestamp_cache_size: usize,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum StreamSelection {
    Hub,
    Controller,
    All,
}

#[derive(Debug, Clone, Parser)]
#[command(name = "indexer", disable_help_subcommand = true)]
struct Cli {
    /// Only run the selected stream (default: run all configured streams).
    #[arg(long, value_enum)]
    stream: Option<StreamSelection>,

    #[arg(env = "DATABASE_URL")]
    database_url: String,

    #[command(flatten)]
    retry: RetryArgs,

    #[arg(env = "DB_MAX_CONNECTIONS", default_value_t = 5)]
    db_max_connections: u32,

    #[arg(env = "BLOCK_HEADER_CONCURRENCY", default_value_t = 16)]
    block_header_concurrency: usize,

    #[arg(env = "BLOCK_TIMESTAMP_CACHE_SIZE", default_value_t = 2048)]
    block_timestamp_cache_size: usize,

    #[command(flatten)]
    hub: HubArgs,

    #[command(flatten)]
    controller: ControllerArgs,
}

#[derive(Debug, Clone, Args)]
struct RetryArgs {
    #[arg(env = "RPC_MAX_RATE_LIMIT_RETRIES", default_value_t = 8)]
    max_rate_limit_retries: u32,

    #[arg(env = "RPC_INITIAL_BACKOFF_MS", default_value_t = 250)]
    initial_backoff_ms: u64,

    #[arg(env = "RPC_COMPUTE_UNITS_PER_SECOND", default_value_t = 500)]
    compute_units_per_second: u64,
}

#[derive(Debug, Clone, Args)]
struct HubArgs {
    #[arg(env = "HUB_RPC_URLS")]
    rpc_urls: Option<String>,

    #[arg(env = "HUB_CHAIN_ID")]
    chain_id: Option<u64>,

    #[arg(env = "HUB_CONTRACT_ADDRESS")]
    contract_address: Option<String>,

    #[arg(env = "HUB_DEPLOYMENT_BLOCK")]
    deployment_block: Option<u64>,

    #[arg(env = "HUB_CONFIRMATIONS")]
    confirmations: Option<u64>,

    #[arg(env = "HUB_POLL_INTERVAL_SECS")]
    poll_interval_secs: Option<u64>,

    #[arg(env = "HUB_CHUNK_BLOCKS")]
    chunk_blocks: Option<u64>,

    #[arg(env = "HUB_REORG_SCAN_DEPTH")]
    reorg_scan_depth: Option<u64>,
}

#[derive(Debug, Clone, Args)]
struct ControllerArgs {
    #[arg(env = "CONTROLLER_RPC_URLS")]
    rpc_urls: Option<String>,

    #[arg(env = "CONTROLLER_CHAIN_ID")]
    chain_id: Option<u64>,

    #[arg(env = "CONTROLLER_CONTRACT_ADDRESS")]
    contract_address: Option<String>,

    #[arg(env = "CONTROLLER_DEPLOYMENT_BLOCK")]
    deployment_block: Option<u64>,

    #[arg(env = "CONTROLLER_CONFIRMATIONS")]
    confirmations: Option<u64>,

    #[arg(env = "CONTROLLER_POLL_INTERVAL_SECS")]
    poll_interval_secs: Option<u64>,

    #[arg(env = "CONTROLLER_CHUNK_BLOCKS")]
    chunk_blocks: Option<u64>,

    #[arg(env = "CONTROLLER_REORG_SCAN_DEPTH")]
    reorg_scan_depth: Option<u64>,
}

pub fn load_config() -> Result<AppConfig> {
    let cli = Cli::parse();

    let retry = RetryConfig {
        max_rate_limit_retries: cli.retry.max_rate_limit_retries,
        initial_backoff_ms: cli.retry.initial_backoff_ms,
        compute_units_per_second: cli.retry.compute_units_per_second,
    };

    let mut streams = Vec::new();
    if let Some(cfg) = load_stream_config(Stream::Hub, StreamArgRefs::from(&cli.hub))? {
        streams.push(cfg);
    }
    if let Some(cfg) = load_stream_config(Stream::Controller, StreamArgRefs::from(&cli.controller))?
    {
        streams.push(cfg);
    }

    if let Some(only) = cli.stream {
        match only {
            StreamSelection::Hub => streams.retain(|s| s.stream == Stream::Hub),
            StreamSelection::Controller => streams.retain(|s| s.stream == Stream::Controller),
            StreamSelection::All => {}
        }
    }

    if streams.is_empty() {
        anyhow::bail!(
            "no streams configured (set HUB_* and/or CONTROLLER_* env vars; optionally pass --stream hub|controller|all)"
        );
    }

    Ok(AppConfig {
        database_url: cli.database_url,
        streams,
        retry,
        db_max_connections: cli.db_max_connections,
        block_header_concurrency: cli.block_header_concurrency,
        block_timestamp_cache_size: cli.block_timestamp_cache_size,
    })
}

fn load_stream_config(stream: Stream, args: StreamArgRefs<'_>) -> Result<Option<StreamConfig>> {
    let defaults = match stream {
        Stream::Hub => StreamDefaults {
            confirmations: 12,
            poll_interval: Duration::from_secs(5),
            chunk_blocks: 5_000,
            reorg_scan_depth: 512,
        },
        Stream::Controller => StreamDefaults {
            confirmations: 20,
            poll_interval: Duration::from_secs(5),
            chunk_blocks: 1_000,
            reorg_scan_depth: 1_024,
        },
    };

    let rpc_urls = match args.rpc_urls_raw {
        Some(raw) => parse_list(raw),
        None => return Ok(None),
    };
    if rpc_urls.is_empty() {
        return Ok(None);
    }

    let prefix = stream.as_str().to_uppercase();

    let chain_id = args
        .chain_id
        .with_context(|| format!("{prefix}_CHAIN_ID must be set when {prefix}_RPC_URLS is set"))?;

    let contract_address = args.contract_address.with_context(|| {
        format!("{prefix}_CONTRACT_ADDRESS must be set when {prefix}_RPC_URLS is set")
    })?;

    let deployment_block = args.deployment_block.with_context(|| {
        format!("{prefix}_DEPLOYMENT_BLOCK must be set when {prefix}_RPC_URLS is set")
    })?;

    let confirmations = args.confirmations.unwrap_or(defaults.confirmations);

    let poll_interval_secs = args
        .poll_interval_secs
        .unwrap_or(defaults.poll_interval.as_secs());
    let poll_interval = Duration::from_secs(poll_interval_secs.max(1));

    let chunk_blocks = args.chunk_blocks.unwrap_or(defaults.chunk_blocks);
    let reorg_scan_depth = args.reorg_scan_depth.unwrap_or(defaults.reorg_scan_depth);

    Ok(Some(StreamConfig {
        stream,
        chain_id,
        rpc_urls,
        contract_address: contract_address.to_string(),
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

#[derive(Clone, Copy)]
struct StreamArgRefs<'a> {
    rpc_urls_raw: Option<&'a str>,
    chain_id: Option<u64>,
    contract_address: Option<&'a str>,
    deployment_block: Option<u64>,
    confirmations: Option<u64>,
    poll_interval_secs: Option<u64>,
    chunk_blocks: Option<u64>,
    reorg_scan_depth: Option<u64>,
}

impl<'a> From<&'a HubArgs> for StreamArgRefs<'a> {
    fn from(value: &'a HubArgs) -> Self {
        Self {
            rpc_urls_raw: value.rpc_urls.as_deref(),
            chain_id: value.chain_id,
            contract_address: value.contract_address.as_deref(),
            deployment_block: value.deployment_block,
            confirmations: value.confirmations,
            poll_interval_secs: value.poll_interval_secs,
            chunk_blocks: value.chunk_blocks,
            reorg_scan_depth: value.reorg_scan_depth,
        }
    }
}

impl<'a> From<&'a ControllerArgs> for StreamArgRefs<'a> {
    fn from(value: &'a ControllerArgs) -> Self {
        Self {
            rpc_urls_raw: value.rpc_urls.as_deref(),
            chain_id: value.chain_id,
            contract_address: value.contract_address.as_deref(),
            deployment_block: value.deployment_block,
            confirmations: value.confirmations,
            poll_interval_secs: value.poll_interval_secs,
            chunk_blocks: value.chunk_blocks,
            reorg_scan_depth: value.reorg_scan_depth,
        }
    }
}

fn parse_list(raw: &str) -> Vec<String> {
    raw.split(|c: char| c == ',' || c.is_whitespace())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}
