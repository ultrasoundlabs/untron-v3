use alloy::providers::Provider;
use anyhow::{Context, Result};
use futures::{StreamExt, stream};
use lru::LruCache;
use std::{collections::HashSet, num::NonZeroUsize, sync::Arc};
use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;

use super::logs::ValidatedLog;

#[derive(Clone)]
pub(super) struct BlockTimestampCache {
    inner: LruCache<u64, u64>,
}

impl BlockTimestampCache {
    pub(super) fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity.max(1)).expect("nonzero");
        Self {
            inner: LruCache::new(cap),
        }
    }

    pub(super) fn clear(&mut self) {
        self.inner.clear();
    }

    pub(super) fn get(&mut self, block_number: u64) -> Option<u64> {
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
pub(super) struct TimestampState {
    pub(super) cache: BlockTimestampCache,
    header_sem: Arc<Semaphore>,
    header_concurrency: usize,
}

impl TimestampState {
    pub(super) fn new(cache_size: usize, block_header_concurrency: usize) -> Self {
        let header_concurrency = block_header_concurrency.max(1);
        Self {
            cache: BlockTimestampCache::new(cache_size),
            header_sem: Arc::new(Semaphore::new(header_concurrency)),
            header_concurrency,
        }
    }

    pub(super) async fn populate_timestamps(
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
                        let start = std::time::Instant::now();
                        let block = provider
                            .get_block_by_number(alloy::rpc::types::BlockNumberOrTag::Number(block_number))
                            .await
                            .with_context(|| format!("get_block_by_number({block_number})"))?;
                        let Some(block) = block else {
                            anyhow::bail!("block {block_number} not found");
                        };
                        let _elapsed_ms = start.elapsed().as_millis() as u64;
                        Ok(Some((block_number, normalize_timestamp_seconds(block.header.inner.timestamp))))
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

pub(super) fn normalize_timestamp_seconds(timestamp: u64) -> u64 {
    // Guardrail for chains/endpoints that return milliseconds since epoch.
    if timestamp >= 20_000_000_000 {
        timestamp / 1000
    } else {
        timestamp
    }
}
