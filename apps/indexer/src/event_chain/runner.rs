use crate::shared::rpc_telemetry::RpcTelemetry;
use crate::{
    config::StreamConfig,
    db::{self, ResolvedStream},
    metrics::StreamTelemetry,
    rpc::RpcProviders,
    shared::{r#async, r#async::timed_await_or_cancel},
};
use alloy::providers::Provider;
use anyhow::{Context, Result};
use std::time::Duration;
use tokio::time;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use super::{errors, range, reorg, state::PollState};
use crate::shared::progress::ProgressReporter;

const MAX_TRANSIENT_RETRIES: u32 = 3;
const TRANSIENT_BACKOFF_INITIAL: Duration = Duration::from_millis(250);
const TRANSIENT_BACKOFF_MAX: Duration = Duration::from_secs(2);

fn grow_chunk(current: u64, target: u64) -> u64 {
    if current >= target {
        return current;
    }
    current.saturating_mul(2).min(target)
}

fn shrink_chunk(current: u64) -> u64 {
    (current / 2).max(1)
}

fn shrink_chunk_with_backoff_reset(
    progress: &mut ProgressReporter,
    state: &mut PollState,
    transient_attempts: &mut u32,
    transient_backoff: &mut Duration,
    from_block: u64,
    to_block: u64,
    err: &anyhow::Error,
    msg: &'static str,
) {
    *transient_attempts = 0;
    *transient_backoff = TRANSIENT_BACKOFF_INITIAL;
    state.chunk_current = shrink_chunk(state.chunk_current);
    progress.on_chunk_shrink();
    progress.update_event_chain_chunk_blocks(state.chunk_current);
    warn!(
        stream = state.stream.as_str(),
        from_block,
        to_block,
        chunk_blocks = state.chunk_current,
        err = %err,
        "{msg}"
    );
}

fn resolve_rpc_contract_address(
    stream: crate::config::Stream,
    contract_address_db: &str,
) -> Result<alloy::primitives::Address> {
    match stream {
        crate::config::Stream::Hub => contract_address_db
            .parse::<alloy::primitives::Address>()
            .with_context(|| format!("invalid hub contract address: {contract_address_db}")),
        crate::config::Stream::Controller => {
            Ok(crate::domain::TronAddress::from_base58check(contract_address_db)?.evm())
        }
    }
}

pub struct RunStreamParams {
    pub dbh: db::Db,
    pub cfg: StreamConfig,
    pub resolved: ResolvedStream,
    pub providers: RpcProviders,
    pub shutdown: CancellationToken,
    pub block_header_concurrency: usize,
    pub block_timestamp_cache_size: usize,
    pub progress_interval: Duration,
    pub progress_tail_lag_blocks: u64,
}

pub async fn run_stream(params: RunStreamParams) -> Result<()> {
    let RunStreamParams {
        dbh,
        cfg,
        resolved,
        providers,
        shutdown,
        block_header_concurrency,
        block_timestamp_cache_size,
        progress_interval,
        progress_tail_lag_blocks,
    } = params;
    let (stream, chain_id, contract_address_db) = resolved.into_parts();
    let contract_address_rpc = resolve_rpc_contract_address(stream, contract_address_db.as_str())?;

    let mut from_block =
        db::event_chain::resume_from_block(&dbh, stream, cfg.deployment_block).await?;
    info!(
        stream = stream.as_str(),
        chain_id,
        contract_db = %contract_address_db,
        contract_rpc = %contract_address_rpc,
        from_block,
        confirmations = cfg.confirmations,
        poll_interval_secs = cfg.poll_interval.as_secs(),
        chunk_blocks = cfg.chunk_blocks,
        reorg_scan_depth = cfg.reorg_scan_depth,
        "stream starting"
    );

    let mut progress = ProgressReporter::new_event_chain(
        stream.as_str(),
        progress_interval,
        cfg.chunk_blocks.max(1),
        progress_tail_lag_blocks,
    );
    let telemetry = StreamTelemetry::new(stream, chain_id);

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
        timestamps: crate::shared::timestamps::TimestampState::new(
            block_timestamp_cache_size,
            block_header_concurrency,
        ),
        telemetry,
    };

    let mut ticker = time::interval(cfg.poll_interval.max(Duration::from_secs(1)));
    ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

    let mut transient_attempts: u32 = 0;
    let mut transient_backoff = TRANSIENT_BACKOFF_INITIAL;
    let mut head_attempts: u32 = 0;
    let mut head_backoff = TRANSIENT_BACKOFF_INITIAL;

    let mut last_gap_check = std::time::Instant::now()
        .checked_sub(Duration::from_secs(3600))
        .unwrap_or_else(std::time::Instant::now);

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!(stream = state.stream.as_str(), "shutdown signal received");
                return Ok(());
            }
            _ = ticker.tick() => {}
        }

        let head_res = timed_await_or_cancel(&shutdown, async {
            state.provider.get_block_number().await.map_err(|e| {
                state.telemetry.rpc_error("eth_blockNumber", "head");
                anyhow::Error::new(e).context("eth_blockNumber")
            })
        })
        .await;

        let Some((head, head_ms)) = (match head_res {
            Ok(opt) => opt,
            Err(e) if errors::looks_like_transient(&e) => {
                head_attempts = head_attempts.saturating_add(1);
                progress.on_transient_retry();
                warn!(
                    stream = state.stream.as_str(),
                    attempt = head_attempts,
                    backoff_ms = head_backoff.as_millis() as u64,
                    err = %e,
                    "transient RPC error; retrying eth_blockNumber"
                );
                r#async::sleep_or_cancel(&shutdown, head_backoff).await?;
                head_backoff = (head_backoff * 2).min(TRANSIENT_BACKOFF_MAX);
                continue;
            }
            Err(e) => return Err(e),
        }) else {
            return Ok(());
        };

        head_attempts = 0;
        head_backoff = TRANSIENT_BACKOFF_INITIAL;
        state
            .telemetry
            .rpc_call("eth_blockNumber", "head", true, head_ms);
        let head: u64 = head;

        let safe_head = head.saturating_sub(state.confirmations);
        state
            .telemetry
            .set_chain_position(head, safe_head, from_block, state.chunk_current);
        debug!(
            stream = state.stream.as_str(),
            head, safe_head, from_block, "tick"
        );

        progress.update_event_chain_chunk_blocks(state.chunk_current);
        progress.maybe_report(head, safe_head, from_block);

        // Detect canonical event_seq gaps early; these wedge projections.
        // Cheap check: missing = max(event_seq) - count(*) for canonical rows.
        if last_gap_check.elapsed() >= Duration::from_secs(30) {
            last_gap_check = std::time::Instant::now();
            let gaps_res = timed_await_or_cancel(&shutdown, async {
                db::event_chain::canonical_seq_gap_count(&dbh, state.stream).await
            })
            .await;
            match gaps_res {
                Ok(Some((gaps, ms))) => {
                    state
                        .telemetry
                        .observe_db_latency_ms("canonical_seq_gap_count", ms);
                    let gaps_u64 = u64::try_from(gaps).unwrap_or(0);
                    state.telemetry.set_seq_gaps(gaps_u64);
                    if gaps > 0 {
                        warn!(
                            stream = state.stream.as_str(),
                            gaps,
                            next_block = from_block,
                            "detected canonical event_seq gaps; projections may wedge until backfilled"
                        );
                    }
                }
                Ok(None) => return Ok(()),
                Err(e) => {
                    warn!(stream = state.stream.as_str(), err = %e, "gap check failed; continuing");
                }
            }
        }

        let reorg_res = r#async::await_or_cancel(&shutdown, async {
            reorg::detect_reorg_start(
                &dbh,
                &state.provider,
                &state.pinned_providers,
                state.stream,
                state.reorg_scan_depth,
                Some(&state.telemetry),
            )
            .await
        })
        .await;

        let reorg_start = match reorg_res {
            Ok(None) => return Ok(()),
            Ok(Some(v)) => v,
            Err(e) if errors::looks_like_transient(&e) => {
                warn!(
                    stream = state.stream.as_str(),
                    err = %e,
                    "transient RPC error; skipping reorg check this tick"
                );
                None
            }
            Err(e) => return Err(e),
        };

        if let Some(reorg_start) = reorg_start {
            warn!(
                stream = state.stream.as_str(),
                reorg_start, "reorg detected; invalidating"
            );

            let invalidated_blocks = from_block.saturating_sub(reorg_start);
            progress.on_reorg(invalidated_blocks);
            state.telemetry.reorg_detected();

            if r#async::await_or_cancel(
                &shutdown,
                db::event_chain::invalidate_from_block(&dbh, state.stream, reorg_start),
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

            let to_block =
                safe_head.min(from_block.saturating_add(state.chunk_current.saturating_sub(1)));

            match range::process_range(&dbh, &shutdown, &mut state, from_block, to_block).await {
                Ok(Some(metrics)) => {
                    progress.observe_range(metrics);
                    from_block = metrics.to_block.saturating_add(1);
                    transient_attempts = 0;
                    transient_backoff = TRANSIENT_BACKOFF_INITIAL;
                    state.chunk_current = grow_chunk(state.chunk_current, state.chunk_target);
                    progress.update_event_chain_chunk_blocks(state.chunk_current);
                    progress.maybe_report(head, safe_head, from_block);
                }
                Ok(None) => return Ok(()),
                Err(e) => {
                    if errors::looks_like_transient(&e)
                        && transient_attempts < MAX_TRANSIENT_RETRIES
                    {
                        transient_attempts += 1;
                        progress.on_transient_retry();
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
                        r#async::sleep_or_cancel(&shutdown, backoff).await?;
                        if shutdown.is_cancelled() {
                            return Ok(());
                        }
                        continue;
                    }

                    if state.chunk_current > 1 && errors::looks_like_range_too_large(&e) {
                        shrink_chunk_with_backoff_reset(
                            &mut progress,
                            &mut state,
                            &mut transient_attempts,
                            &mut transient_backoff,
                            from_block,
                            to_block,
                            &e,
                            "eth_getLogs failed; shrinking chunk",
                        );
                        continue;
                    }

                    if state.chunk_current > 1 {
                        shrink_chunk_with_backoff_reset(
                            &mut progress,
                            &mut state,
                            &mut transient_attempts,
                            &mut transient_backoff,
                            from_block,
                            to_block,
                            &e,
                            "range processing failed; shrinking chunk",
                        );
                        continue;
                    }

                    // chunk_current == 1: try each pinned provider for this block
                    if !state.pinned_providers.is_empty() {
                        transient_attempts = 0;
                        transient_backoff = TRANSIENT_BACKOFF_INITIAL;
                        progress.on_pinned_repair_attempt();
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
                                Ok(Some(metrics)) => {
                                    progress.on_pinned_repair_success();
                                    progress.observe_range(metrics);
                                    info!(
                                        stream = state.stream.as_str(),
                                        block = from_block,
                                        pinned_index = idx,
                                        "repair succeeded"
                                    );
                                    repaired = true;
                                    from_block = metrics.to_block.saturating_add(1);
                                    progress.maybe_report(head, safe_head, from_block);
                                    break;
                                }
                                Ok(None) => return Ok(()),
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
