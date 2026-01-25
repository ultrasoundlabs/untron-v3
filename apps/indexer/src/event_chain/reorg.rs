use crate::shared::rpc_telemetry::RpcTelemetry;
use crate::{config::Stream, db, domain};
use alloy::{providers::Provider, rpc::types::BlockNumberOrTag};
use anyhow::{Context, Result};
use serde_json::Value;
use std::time::Instant;
use tracing::{debug, warn};

pub async fn detect_reorg_start(
    dbh: &db::Db,
    provider: &impl Provider,
    pinned_providers: &[alloy::providers::DynProvider],
    stream: Stream,
    scan_depth: u64,
    rpc_telemetry: Option<&dyn RpcTelemetry>,
) -> Result<Option<u64>> {
    let Some(latest) = db::event_chain::latest_canonical_block_hash(dbh, stream).await? else {
        return Ok(None);
    };

    let latest_rpc_hash =
        match get_block_hash_opt(provider, latest.block_number, rpc_telemetry).await {
            Ok(Some(h)) => h,
            Ok(None) => {
                warn!(
                    stream = stream.as_str(),
                    block_number = latest.block_number,
                    "reorg check: latest block not found on RPC; skipping this tick"
                );
                return Ok(None);
            }
            Err(e) => {
                warn!(
                    stream = stream.as_str(),
                    block_number = latest.block_number,
                    err = %e,
                    "reorg check: failed to fetch latest block hash; skipping this tick"
                );
                return Ok(None);
            }
        };
    if latest_rpc_hash == latest.block_hash {
        debug!(
            stream = stream.as_str(),
            block_number = latest.block_number,
            "reorg check: latest block matches"
        );
        return Ok(None);
    }

    // Avoid false positives from a single bad/lagging RPC endpoint by checking at least one
    // additional pinned endpoint (when configured).
    if let Some(reason) = mismatch_not_confirmed(&latest, pinned_providers, rpc_telemetry).await? {
        warn!(
            stream = stream.as_str(),
            block_number = latest.block_number,
            stored_hash = %latest.block_hash,
            rpc_hash = %latest_rpc_hash,
            reason,
            "reorg check: mismatch not confirmed; skipping invalidation"
        );
        return Ok(None);
    }

    let scan_depth = scan_depth.max(1);
    let mut stored =
        db::event_chain::recent_canonical_block_hashes(dbh, stream, scan_depth).await?;
    if stored.is_empty() {
        // Shouldn't happen because `latest` exists, but handle defensively.
        return Ok(Some(latest.block_number));
    }

    stored.sort_by_key(|b| b.block_number);

    // Ensure the mismatching latest block is included in the search window.
    if stored.last().map(|b| b.block_number) != Some(latest.block_number) {
        stored.push(latest.clone());
        stored.sort_by_key(|b| b.block_number);
    }

    // Find the earliest mismatching stored block in O(log N) header calls.
    let mut left = 0usize;
    let mut right = stored.len();
    while left < right {
        let mid = (left + right) / 2;
        let b = &stored[mid];
        let rpc_hash = match get_block_hash_opt(provider, b.block_number, rpc_telemetry).await {
            Ok(Some(h)) => h,
            Ok(None) => {
                warn!(
                    stream = stream.as_str(),
                    block_number = b.block_number,
                    "reorg check: block not found during binary search; skipping this tick"
                );
                return Ok(None);
            }
            Err(e) => {
                warn!(
                    stream = stream.as_str(),
                    block_number = b.block_number,
                    err = %e,
                    "reorg check: failed to fetch block hash during binary search; skipping this tick"
                );
                return Ok(None);
            }
        };
        if rpc_hash == b.block_hash {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left >= stored.len() {
        // Inconsistent RPC view; treat as inconclusive and avoid invalidation.
        warn!(
            stream = stream.as_str(),
            block_number = latest.block_number,
            "reorg check: binary search found no mismatch; skipping invalidation"
        );
        return Ok(None);
    }

    Ok(Some(stored[left].block_number))
}

async fn get_block_hash_opt(
    provider: &impl Provider,
    block_number: u64,
    rpc_telemetry: Option<&dyn RpcTelemetry>,
) -> Result<Option<domain::BlockHash>> {
    // Tron JSON-RPC block responses are not Ethereum-typed (e.g. `stateRoot: "0x"`), which can
    // break strict decoding. For reorg detection we only need the block hash, so fetch raw JSON.
    let start = Instant::now();
    let block: Option<Value> = provider
        .client()
        .request(
            "eth_getBlockByNumber",
            (BlockNumberOrTag::Number(block_number), false),
        )
        .await
        .map_err(|e| {
            if let Some(rpc) = rpc_telemetry {
                rpc.rpc_error("eth_getBlockByNumber", "reorg");
                rpc.rpc_call(
                    "eth_getBlockByNumber",
                    "reorg",
                    false,
                    start.elapsed().as_millis() as u64,
                );
            }
            anyhow::Error::new(e)
        })
        .with_context(|| format!("get_block_by_number({block_number})"))?;

    let Some(block) = block else {
        return Ok(None);
    };

    if let Some(rpc) = rpc_telemetry {
        rpc.rpc_call(
            "eth_getBlockByNumber",
            "reorg",
            true,
            start.elapsed().as_millis() as u64,
        );
    }

    let hash = block
        .get("hash")
        .and_then(|v| v.as_str())
        .context("missing block.hash")?;
    let b256: alloy::primitives::B256 = hash
        .parse()
        .with_context(|| format!("invalid block.hash: {hash}"))?;
    Ok(Some(domain::BlockHash::from(b256)))
}

// Returns `Some(reason)` when a mismatch is not confirmed (and we should avoid invalidation).
async fn mismatch_not_confirmed(
    latest: &db::event_chain::StoredBlockHash,
    pinned_providers: &[alloy::providers::DynProvider],
    rpc_telemetry: Option<&dyn RpcTelemetry>,
) -> Result<Option<&'static str>> {
    if pinned_providers.is_empty() {
        return Ok(None);
    }

    // If any pinned endpoint still reports the stored hash, treat this as inconclusive.
    // If multiple endpoints exist, require multiple successful mismatching responses before
    // invalidating to avoid false positives from a single bad RPC.
    let required_successes = if pinned_providers.len() >= 2 { 2 } else { 1 };
    let mut successes = 0usize;

    for p in pinned_providers {
        let h = match get_block_hash_opt(p, latest.block_number, rpc_telemetry).await {
            Ok(Some(h)) => h,
            Ok(None) => continue,
            Err(_) => continue,
        };

        successes += 1;
        if h == latest.block_hash {
            return Ok(Some("another RPC still matches stored hash"));
        }

        if successes >= required_successes {
            return Ok(None);
        }
    }

    if successes == 0 {
        return Ok(Some("unable to confirm mismatch with pinned RPCs"));
    }

    Ok(Some("unable to confirm mismatch with multiple pinned RPCs"))
}
