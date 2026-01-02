use crate::{
    config::StreamConfig,
    db::{self, ResolvedStream},
    reorg,
    rpc::RpcProviders,
};
use alloy::providers::Provider;
use anyhow::{Context, Result};
use std::time::Duration;
use tokio::time;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use super::{errors, helpers, policy, range, state::PollState, timestamps};

pub async fn run_stream(
    dbh: db::Db,
    cfg: StreamConfig,
    resolved: ResolvedStream,
    providers: RpcProviders,
    shutdown: CancellationToken,
    block_header_concurrency: usize,
    block_timestamp_cache_size: usize,
) -> Result<()> {
    let (stream, chain_id, contract_address_db) = resolved.into_parts();
    let contract_address_rpc =
        helpers::resolve_rpc_contract_address(stream, contract_address_db.as_str())?;

    let mut from_block = db::resume_from_block(&dbh, stream, cfg.deployment_block).await?;
    info!(
        stream = stream.as_str(),
        chain_id,
        contract_db = %contract_address_db,
        contract_rpc = %contract_address_rpc,
        from_block,
        "stream starting"
    );

    let mut state = PollState {
        stream,
        chain_id: i64::try_from(chain_id).context("chain_id out of range for bigint")?,
        contract_address_db,
        contract_address_rpc,
        confirmations: cfg.confirmations,
        reorg_scan_depth: cfg.reorg_scan_depth,
        chunk_target: cfg.chunk_blocks.max(1),
        chunk_current: cfg.chunk_blocks.max(1),
        pinned_providers: providers.pinned,
        provider: providers.fallback,
        timestamps: timestamps::TimestampState::new(
            block_timestamp_cache_size,
            block_header_concurrency,
        ),
    };

    let mut ticker = time::interval(cfg.poll_interval.max(Duration::from_secs(1)));
    ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

    const TRANSIENT_BACKOFF_INITIAL: Duration = Duration::from_millis(250);
    const TRANSIENT_BACKOFF_MAX: Duration = Duration::from_secs(2);

    let mut transient_attempts: u32 = 0;
    let mut transient_backoff = TRANSIENT_BACKOFF_INITIAL;

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!(stream = state.stream.as_str(), "shutdown signal received");
                return Ok(());
            }
            _ = ticker.tick() => {}
        }

        let Some(head) = helpers::await_or_cancel(&shutdown, async {
            state
                .provider
                .get_block_number()
                .await
                .context("eth_blockNumber")
        })
        .await?
        else {
            return Ok(());
        };
        let head: u64 = head;

        let safe_head = head.saturating_sub(state.confirmations);
        debug!(
            stream = state.stream.as_str(),
            head, safe_head, from_block, "tick"
        );

        if let Some(reorg_start) = helpers::await_or_cancel(&shutdown, async {
            reorg::detect_reorg_start(
                &dbh,
                &state.provider,
                &state.pinned_providers,
                state.stream,
                state.reorg_scan_depth,
            )
            .await
        })
        .await?
        .flatten()
        {
            warn!(
                stream = state.stream.as_str(),
                reorg_start, "reorg detected; invalidating"
            );

            if helpers::await_or_cancel(
                &shutdown,
                db::invalidate_from_block(&dbh, state.stream, reorg_start),
            )
            .await?
            .is_none()
            {
                return Ok(());
            }
            state.timestamps.cache.clear();
            from_block = from_block.min(reorg_start);
        }

        while from_block <= safe_head {
            if shutdown.is_cancelled() {
                return Ok(());
            }

            let to_block = safe_head.min(from_block.saturating_add(state.chunk_current - 1));

            match range::process_range(&dbh, &shutdown, &mut state, from_block, to_block).await {
                Ok(()) => {
                    from_block = to_block.saturating_add(1);
                    transient_attempts = 0;
                    transient_backoff = TRANSIENT_BACKOFF_INITIAL;
                    state.chunk_current =
                        policy::grow_chunk(state.chunk_current, state.chunk_target);
                }
                Err(e) => {
                    if errors::looks_like_transient(&e)
                        && transient_attempts < policy::MAX_TRANSIENT_RETRIES
                    {
                        transient_attempts += 1;
                        warn!(
                            stream = state.stream.as_str(),
                            from_block,
                            to_block,
                            attempt = transient_attempts,
                            err = %e,
                            "transient RPC error; retrying range"
                        );
                        let backoff = transient_backoff;
                        transient_backoff = (transient_backoff * 2).min(TRANSIENT_BACKOFF_MAX);
                        helpers::sleep_or_cancel(&shutdown, backoff).await?;
                        if shutdown.is_cancelled() {
                            return Ok(());
                        }
                        continue;
                    }

                    if state.chunk_current > 1 && errors::looks_like_range_too_large(&e) {
                        transient_attempts = 0;
                        transient_backoff = TRANSIENT_BACKOFF_INITIAL;
                        state.chunk_current = policy::shrink_chunk(state.chunk_current);
                        warn!(
                            stream = state.stream.as_str(),
                            from_block,
                            to_block,
                            chunk_blocks = state.chunk_current,
                            err = %e,
                            "eth_getLogs failed; shrinking chunk"
                        );
                        continue;
                    }

                    if state.chunk_current > 1 {
                        transient_attempts = 0;
                        transient_backoff = TRANSIENT_BACKOFF_INITIAL;
                        state.chunk_current = policy::shrink_chunk(state.chunk_current);
                        warn!(
                            stream = state.stream.as_str(),
                            from_block,
                            to_block,
                            chunk_blocks = state.chunk_current,
                            err = %e,
                            "range processing failed; shrinking chunk"
                        );
                        continue;
                    }

                    // chunk_current == 1: try each pinned provider for this block
                    if !state.pinned_providers.is_empty() {
                        transient_attempts = 0;
                        transient_backoff = TRANSIENT_BACKOFF_INITIAL;
                        warn!(
                            stream = state.stream.as_str(),
                            block = from_block,
                            err = %e,
                            "range processing failed; attempting pinned providers"
                        );

                        let mut repaired = false;
                        for idx in 0..state.pinned_providers.len() {
                            if shutdown.is_cancelled() {
                                return Ok(());
                            }
                            let pinned = state.pinned_providers[idx].clone();
                            match range::process_range_with_provider(
                                &dbh, &shutdown, &mut state, &pinned, from_block, to_block,
                            )
                            .await
                            {
                                Ok(()) => {
                                    info!(
                                        stream = state.stream.as_str(),
                                        block = from_block,
                                        pinned_index = idx,
                                        "repair succeeded"
                                    );
                                    repaired = true;
                                    break;
                                }
                                Err(e2) => {
                                    warn!(
                                        stream = state.stream.as_str(),
                                        block = from_block,
                                        pinned_index = idx,
                                        err = %e2,
                                        "pinned provider failed"
                                    );
                                }
                            }
                        }

                        if repaired {
                            from_block = to_block.saturating_add(1);
                            continue;
                        }
                    }

                    error!(
                        stream = state.stream.as_str(),
                        from_block,
                        to_block,
                        err = %e,
                        "range processing failed permanently"
                    );
                    return Err(e);
                }
            }
        }
    }
}
