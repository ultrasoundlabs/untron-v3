use crate::shared::rpc_telemetry::RpcTelemetry;
use alloy::providers::{DynProvider, Provider};
use anyhow::Result;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Shares `eth_blockNumber` across multiple consumers on the same chain.
///
/// First caller within `ttl` fetches via RPC; subsequent callers within the window return the
/// cached value without an RPC. Failures are not cached.
pub struct HeadCache {
    provider: DynProvider,
    ttl: Duration,
    inner: Mutex<Option<(u64, Instant)>>,
}

impl HeadCache {
    pub fn new(provider: DynProvider, ttl: Duration) -> Arc<Self> {
        Arc::new(Self {
            provider,
            ttl,
            inner: Mutex::new(None),
        })
    }

    pub async fn get(&self, telemetry: &dyn RpcTelemetry, purpose: &'static str) -> Result<u64> {
        let mut guard = self.inner.lock().await;
        if let Some((head, at)) = *guard
            && at.elapsed() < self.ttl
        {
            return Ok(head);
        }

        let start = Instant::now();
        let head = self.provider.get_block_number().await.map_err(|e| {
            telemetry.rpc_error("eth_blockNumber", purpose);
            anyhow::Error::new(e).context("eth_blockNumber")
        })?;
        let ms = start.elapsed().as_millis() as u64;
        telemetry.rpc_call("eth_blockNumber", purpose, true, ms);
        *guard = Some((head, Instant::now()));
        Ok(head)
    }
}
