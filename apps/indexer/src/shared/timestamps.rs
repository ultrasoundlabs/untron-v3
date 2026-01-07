use alloy::providers::Provider;
use anyhow::{Context, Result};
use futures::{StreamExt, stream};
use lru::LruCache;
use serde_json::Value;
use std::{collections::HashSet, num::NonZeroUsize, sync::Arc};
use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;

use super::logs::ValidatedLog;

#[derive(Clone)]
pub struct BlockTimestampCache {
    inner: LruCache<u64, u64>,
}

impl BlockTimestampCache {
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity.max(1)).expect("nonzero");
        Self {
            inner: LruCache::new(cap),
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn get(&mut self, block_number: u64) -> Option<u64> {
        self.inner.get(&block_number).copied()
    }

    fn peek(&self, block_number: u64) -> Option<u64> {
        self.inner.peek(&block_number).copied()
    }

    fn insert(&mut self, block_number: u64, timestamp: u64) {
        self.inner.put(block_number, timestamp);
    }
}

#[derive(Clone)]
pub struct TimestampState {
    pub cache: BlockTimestampCache,
    header_sem: Arc<Semaphore>,
    header_concurrency: usize,
}

impl TimestampState {
    pub fn new(cache_size: usize, block_header_concurrency: usize) -> Self {
        let header_concurrency = block_header_concurrency.max(1);
        Self {
            cache: BlockTimestampCache::new(cache_size),
            header_sem: Arc::new(Semaphore::new(header_concurrency)),
            header_concurrency,
        }
    }

    pub async fn populate_timestamps(
        &mut self,
        shutdown: &CancellationToken,
        provider: &alloy::providers::DynProvider,
        event_logs: &[ValidatedLog],
        proof_logs: &[ValidatedLog],
    ) -> Result<()> {
        let mut blocks: HashSet<u64> = HashSet::new();
        for l in event_logs.iter().chain(proof_logs.iter()) {
            blocks.insert(l.block_number);
            if let Some(ts) = l.block_timestamp {
                self.cache
                    .insert(l.block_number, normalize_timestamp_seconds(ts));
            }
        }

        let missing = blocks
            .into_iter()
            .filter(|b| self.cache.peek(*b).is_none())
            .collect::<Vec<_>>();

        if missing.is_empty() {
            return Ok(());
        }

        let provider = provider.clone();
        let sem = self.header_sem.clone();
        let shutdown_child = shutdown.clone();
        let concurrency = self.header_concurrency;

        let mut tasks = stream::iter(missing).map(move |block_number| {
            let provider = provider.clone();
            let sem = sem.clone();
            let shutdown = shutdown_child.clone();
            async move {
                tokio::select! {
                    _ = shutdown.cancelled() => Ok::<Option<(u64, u64)>, anyhow::Error>(None),
                    permit = sem.acquire_owned() => {
                        let _permit = permit.expect("semaphore closed");
                        // Some EVM-compatible RPCs (notably Tron) return nonstandard block fields
                        // (e.g. `stateRoot: "0x"`) that can fail strict typed decoding. We only
                        // need the block timestamp, so fetch the block as raw JSON and parse it.
                        let block: Option<Value> = provider
                            .client()
                            .request(
                                "eth_getBlockByNumber",
                                (alloy::rpc::types::BlockNumberOrTag::Number(block_number), false),
                            )
                            .await
                            .map_err(anyhow::Error::new)
                            .with_context(|| format!("eth_getBlockByNumber({block_number})"))?;
                        let Some(block) = block else {
                            anyhow::bail!("block {block_number} not found");
                        };
                        let ts = parse_block_timestamp(&block)
                            .with_context(|| format!("parse block.timestamp for block {block_number}"))?;
                        Ok(Some((block_number, normalize_timestamp_seconds(ts))))
                    }
                }
            }
        })
        .buffer_unordered(concurrency);

        while let Some(res) = tasks.next().await {
            if shutdown.is_cancelled() {
                return Ok(());
            }
            if let Some((block_number, ts)) = res? {
                self.cache.insert(block_number, ts);
            }
        }

        Ok(())
    }
}

fn parse_block_timestamp(block: &Value) -> Result<u64> {
    let ts = block.get("timestamp").context("missing block.timestamp")?;

    match ts {
        Value::String(s) => parse_quantity_u64(s).context("timestamp is not a valid quantity"),
        Value::Number(n) => n
            .as_u64()
            .context("timestamp JSON number is not representable as u64"),
        _ => anyhow::bail!("timestamp has unexpected JSON type"),
    }
}

fn parse_quantity_u64(value: &str) -> Result<u64> {
    let trimmed = value.trim();
    let Some(hex) = trimmed.strip_prefix("0x") else {
        return trimmed
            .parse::<u64>()
            .with_context(|| format!("invalid decimal u64: {value}"));
    };
    if hex.is_empty() {
        anyhow::bail!("invalid hex quantity: {value}");
    }
    u64::from_str_radix(hex, 16).with_context(|| format!("invalid hex quantity: {value}"))
}

pub fn normalize_timestamp_seconds(timestamp: u64) -> u64 {
    if timestamp >= 20_000_000_000 {
        timestamp / 1000
    } else {
        timestamp
    }
}

pub fn block_timestamp_for_log(cache: &mut BlockTimestampCache, log: &ValidatedLog) -> Result<u64> {
    let block_number = log.block_number;
    log.block_timestamp
        .map(normalize_timestamp_seconds)
        .or_else(|| cache.get(block_number))
        .with_context(|| format!("missing block_timestamp for block {block_number}"))
}
