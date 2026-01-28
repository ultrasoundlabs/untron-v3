use crate::db::receiver_usdt as receiverdb;
use crate::{
    config::{ReceiverUsdtConfig, Stream, StreamConfig},
    db::{self, ResolvedStream},
    receiver_usdt::range,
    receiver_usdt::telemetry::ReceiverUsdtTelemetry,
    rpc::RpcProviders,
    shared::{r#async, r#async::timed_await_or_cancel, timestamps},
};
use alloy::providers::Provider;
use anyhow::{Context, Result};
use std::{collections::HashMap, time::Duration};
use tokio::{task::JoinSet, time};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::shared::progress::ProgressReporter;
use crate::shared::rpc_telemetry::RpcTelemetry;
use futures::{StreamExt, stream};

pub struct RunReceiverUsdtParams {
    pub dbh: db::Db,
    pub controller_cfg: StreamConfig,
    pub resolved: ResolvedStream,
    pub providers: RpcProviders,
    pub receiver_usdt_cfg: ReceiverUsdtConfig,
    pub block_header_concurrency: usize,
    pub block_timestamp_cache_size: usize,
    pub progress_interval: Duration,
    pub progress_tail_lag_blocks: u64,
    pub shutdown: CancellationToken,
}

struct LoopCtx<'a> {
    dbh: &'a db::Db,
    shutdown: &'a CancellationToken,
    provider: &'a alloy::providers::DynProvider,
    controller_cfg: &'a StreamConfig,
    receiver_usdt_cfg: &'a ReceiverUsdtConfig,
    block_header_concurrency: usize,
    block_timestamp_cache_size: usize,
    progress_interval: Duration,
    progress_tail_lag_blocks: u64,
}

struct ProcessCtx<'a> {
    dbh: &'a db::Db,
    shutdown: &'a CancellationToken,
    provider: &'a alloy::providers::DynProvider,
    timestamps_state: &'a mut timestamps::TimestampState,
    progress: &'a mut ProgressReporter,
    receiver_usdt_cfg: &'a ReceiverUsdtConfig,
    telemetry: &'a ReceiverUsdtTelemetry,
    mode: &'static str,
    chain_id_i64: i64,
}

fn report_receiver_usdt_progress(
    progress: &mut ProgressReporter,
    head: u64,
    safe_head: u64,
    next_block: u64,
    receiver_count: usize,
    batch_size: usize,
) {
    progress.update_receiver_usdt_params(receiver_count, batch_size);
    progress.maybe_report(head, safe_head, next_block);
}

async fn scan_chunks(
    ctx: &mut ProcessCtx<'_>,
    receiver_map: &HashMap<alloy::primitives::Address, String>,
    to_addrs: &[alloy::primitives::Address],
    mut from_block: u64,
    safe_head: u64,
    max_chunks: usize,
) -> Result<Option<u64>> {
    let chunk_blocks = ctx.receiver_usdt_cfg.chunk_blocks.max(1);
    let mut chunks_done = 0usize;

    while from_block <= safe_head && chunks_done < max_chunks {
        if ctx.shutdown.is_cancelled() {
            return Ok(None);
        }

        let to_block = safe_head.min(from_block.saturating_add(chunk_blocks.saturating_sub(1)));
        if process_block_range(ctx, receiver_map, to_addrs, from_block, to_block)
            .await?
            .is_none()
        {
            return Ok(None);
        }

        from_block = to_block.saturating_add(1);
        chunks_done += 1;
    }

    Ok(Some(from_block))
}

pub async fn run_receiver_usdt_indexer(params: RunReceiverUsdtParams) -> Result<()> {
    let RunReceiverUsdtParams {
        dbh,
        controller_cfg,
        resolved,
        providers,
        receiver_usdt_cfg,
        block_header_concurrency,
        block_timestamp_cache_size,
        progress_interval,
        progress_tail_lag_blocks,
        shutdown,
    } = params;
    let local_shutdown = shutdown.child_token();
    let (stream, chain_id, contract_address_db) = resolved.into_parts();
    if stream != Stream::Controller {
        anyhow::bail!("receiver_usdt indexer requires controller stream");
    }

    let controller_address_evm =
        crate::domain::TronAddress::from_base58check(contract_address_db.as_str())
            .context("resolve controller address (tron base58 -> evm)")?
            .evm();

    let init_code_hash = range::fetch_receiver_init_code_hash(
        &local_shutdown,
        &providers.fallback,
        controller_address_evm,
    )
    .await
    .context("read receiver init code hash")?;

    info!(
        chain_id,
        controller = %contract_address_db,
        create2_prefix = receiver_usdt_cfg.controller_create2_prefix,
        to_batch_size = receiver_usdt_cfg.to_batch_size,
        chunk_blocks = receiver_usdt_cfg.chunk_blocks,
        poll_interval_secs = receiver_usdt_cfg.poll_interval.as_secs(),
        backfill_concurrency = receiver_usdt_cfg.backfill_concurrency,
        discovery_interval_secs = receiver_usdt_cfg.discovery_interval.as_secs(),
        "receiver usdt transfer indexer starting"
    );

    // Ensure tail cursor exists.
    let tail_next = receiverdb::ensure_tail_cursor(&dbh, controller_cfg.deployment_block).await?;
    info!(
        tail_next_block = tail_next,
        "receiver usdt tail cursor ready"
    );

    // Prime watchlist from env salts before starting pollers.
    upsert_watchlist_once(
        &dbh,
        &controller_cfg,
        &receiver_usdt_cfg,
        controller_address_evm,
        init_code_hash,
    )
    .await?;

    // On a fresh DB (tail cursor still at deployment), we rely on the tail loop to backfill from deployment
    // for all currently-known receivers. Avoid redundant per-receiver backfill for the initial seed set.
    if tail_next == controller_cfg.deployment_block {
        receiverdb::clear_all_backfill(&dbh).await?;
    }

    let mut join_set = JoinSet::new();

    // Discovery loop (env + hub lease salts).
    {
        let dbh = dbh.clone();
        let shutdown = local_shutdown.clone();
        let controller_cfg = controller_cfg.clone();
        let receiver_usdt_cfg = receiver_usdt_cfg.clone();
        join_set.spawn(async move {
            discovery_loop(
                &dbh,
                &shutdown,
                &controller_cfg,
                &receiver_usdt_cfg,
                controller_address_evm,
                init_code_hash,
            )
            .await
        });
    }

    // Tail loop (batched by recipient list).
    {
        let dbh = dbh.clone();
        let shutdown = local_shutdown.clone();
        let controller_cfg = controller_cfg.clone();
        let receiver_usdt_cfg = receiver_usdt_cfg.clone();
        let provider = providers.fallback.clone();
        join_set.spawn(async move {
            runner_loop(
                LoopCtx {
                    dbh: &dbh,
                    shutdown: &shutdown,
                    provider: &provider,
                    controller_cfg: &controller_cfg,
                    receiver_usdt_cfg: &receiver_usdt_cfg,
                    block_header_concurrency,
                    block_timestamp_cache_size,
                    progress_interval,
                    progress_tail_lag_blocks,
                },
                RunnerMode::Tail,
            )
            .await
        });
    }

    // Backfill loop (batched by cohort of receivers sharing the same backfill_next_block).
    for _ in 0..receiver_usdt_cfg.backfill_concurrency {
        let dbh = dbh.clone();
        let shutdown = local_shutdown.clone();
        let controller_cfg = controller_cfg.clone();
        let receiver_usdt_cfg = receiver_usdt_cfg.clone();
        let provider = providers.fallback.clone();
        join_set.spawn(async move {
            runner_loop(
                LoopCtx {
                    dbh: &dbh,
                    shutdown: &shutdown,
                    provider: &provider,
                    controller_cfg: &controller_cfg,
                    receiver_usdt_cfg: &receiver_usdt_cfg,
                    block_header_concurrency,
                    block_timestamp_cache_size,
                    progress_interval,
                    progress_tail_lag_blocks,
                },
                RunnerMode::Backfill,
            )
            .await
        });
    }

    // Wait for first task to exit; treat errors as fatal.
    let mut fatal: Option<anyhow::Error> = None;
    tokio::select! {
        _ = shutdown.cancelled() => {}
        res = join_set.join_next() => {
            if let Some(res) = res {
                let res = res.context("receiver_usdt task panicked")?;
                if let Err(e) = res {
                    fatal = Some(e);
                } else {
                    fatal = Some(anyhow::anyhow!("receiver_usdt task exited unexpectedly"));
                }
            }
        }
    }

    local_shutdown.cancel();
    while let Some(res) = join_set.join_next().await {
        let res = res.context("receiver_usdt task panicked")?;
        if let Err(e) = res {
            fatal.get_or_insert(e);
        }
    }

    match fatal {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

async fn discovery_loop(
    dbh: &db::Db,
    shutdown: &CancellationToken,
    controller_cfg: &StreamConfig,
    receiver_usdt_cfg: &ReceiverUsdtConfig,
    controller_address_evm: alloy::primitives::Address,
    init_code_hash: alloy::primitives::B256,
) -> Result<()> {
    let mut ticker = time::interval(receiver_usdt_cfg.discovery_interval);
    ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => return Ok(()),
            _ = ticker.tick() => {}
        }

        if let Err(e) = upsert_watchlist_once(
            dbh,
            controller_cfg,
            receiver_usdt_cfg,
            controller_address_evm,
            init_code_hash,
        )
        .await
        {
            warn!(err = %e, "receiver_usdt discovery tick failed");
        }
    }
}

async fn upsert_watchlist_once(
    dbh: &db::Db,
    controller_cfg: &StreamConfig,
    receiver_usdt_cfg: &ReceiverUsdtConfig,
    controller_address_evm: alloy::primitives::Address,
    init_code_hash: alloy::primitives::B256,
) -> Result<()> {
    receiverdb::upsert_watchlist_from_sources(
        dbh,
        controller_cfg.deployment_block,
        &receiver_usdt_cfg.preknown_receiver_salts,
        receiver_usdt_cfg.controller_create2_prefix,
        controller_address_evm,
        init_code_hash,
    )
    .await
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RunnerMode {
    Tail,
    Backfill,
}

async fn list_historical_controller_usdt_tokens_up_to(
    dbh: &db::Db,
    deployment_block: u64,
    to_block: u64,
) -> Result<Vec<alloy::primitives::Address>> {
    let points = receiverdb::usdt_set_points_up_to(dbh, deployment_block, to_block).await?;
    let mut out = Vec::new();
    for p in points {
        let evm = crate::domain::TronAddress::parse_text(&p.usdt_tron)
            .or_else(|_| crate::domain::TronAddress::parse_text(&p.usdt_evm))
            .context("parse controller usdt address")?
            .evm();
        if !out.contains(&evm) {
            out.push(evm);
        }
    }
    Ok(out)
}

async fn runner_loop(ctx: LoopCtx<'_>, mode: RunnerMode) -> Result<()> {
    let LoopCtx {
        dbh,
        shutdown,
        provider,
        controller_cfg,
        receiver_usdt_cfg,
        block_header_concurrency,
        block_timestamp_cache_size,
        progress_interval,
        progress_tail_lag_blocks,
    } = ctx;

    let mut timestamps_state =
        timestamps::TimestampState::new(block_timestamp_cache_size, block_header_concurrency);
    let chain_id_i64 = i64::try_from(controller_cfg.chain_id).context("chain_id out of range")?;

    let label = match mode {
        RunnerMode::Tail => "receiver_usdt_tail",
        RunnerMode::Backfill => "receiver_usdt_backfill",
    };
    let telemetry = ReceiverUsdtTelemetry::new();
    let mut progress = ProgressReporter::new_receiver_usdt(
        label,
        progress_interval,
        0,
        receiver_usdt_cfg.to_batch_size.max(1),
        progress_tail_lag_blocks,
    );

    let batch_size = receiver_usdt_cfg.to_batch_size.max(1);
    let mut process_ctx = ProcessCtx {
        dbh,
        shutdown,
        provider,
        timestamps_state: &mut timestamps_state,
        progress: &mut progress,
        receiver_usdt_cfg,
        telemetry: &telemetry,
        mode: label,
        chain_id_i64,
    };

    match mode {
        RunnerMode::Tail => {
            let mut from_block =
                receiverdb::ensure_tail_cursor(dbh, controller_cfg.deployment_block).await?;

            let mut ticker =
                time::interval(receiver_usdt_cfg.poll_interval.max(Duration::from_secs(1)));
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

            let mut watchlist_snapshot: Option<receiverdb::WatchlistSnapshot> = None;

            loop {
                tokio::select! {
                    _ = shutdown.cancelled() => return Ok(()),
                    _ = ticker.tick() => {}
                }

                let Some((head, head_ms)) = timed_await_or_cancel(shutdown, async {
                    provider.get_block_number().await.map_err(|e| {
                        telemetry.rpc_error("eth_blockNumber", "head");
                        anyhow::Error::new(e).context("Failed to get block number")
                    })
                })
                .await?
                else {
                    return Ok(());
                };
                telemetry.rpc_call("eth_blockNumber", "head", true, head_ms);

                // Cache receiver map across ticks; refresh only if watchlist changed.
                let watchlist_epoch = receiverdb::watchlist_last_updated_at_epoch(dbh)
                    .await?
                    .unwrap_or(0);
                let should_reload = match watchlist_snapshot.as_ref() {
                    Some(s) => s.updated_at_epoch != watchlist_epoch,
                    None => true,
                };
                if should_reload {
                    watchlist_snapshot = Some(receiverdb::load_watchlist_snapshot(dbh).await?);
                }
                let snapshot = watchlist_snapshot.as_ref().expect("set");
                let receiver_count = snapshot.to_addrs.len();

                let safe_head = head.saturating_sub(controller_cfg.confirmations);
                // If we don't know any receivers yet, there's nothing to scan, so treat this as
                // effectively tailing (idle) for progress purposes even though the cursor is still
                // at the deployment block.
                let progress_next_block = if receiver_count == 0 {
                    safe_head.saturating_add(1)
                } else {
                    from_block
                };
                report_receiver_usdt_progress(
                    process_ctx.progress,
                    head,
                    safe_head,
                    progress_next_block,
                    receiver_count,
                    batch_size,
                );
                if from_block > safe_head || receiver_count == 0 {
                    continue;
                }

                let Some(next) = scan_chunks(
                    &mut process_ctx,
                    &snapshot.receiver_map,
                    &snapshot.to_addrs,
                    from_block,
                    safe_head,
                    usize::MAX,
                )
                .await?
                else {
                    return Ok(());
                };
                from_block = next;
                receiverdb::update_tail_cursor(dbh, from_block).await?;
                report_receiver_usdt_progress(
                    process_ctx.progress,
                    head,
                    safe_head,
                    from_block,
                    receiver_count,
                    batch_size,
                );
            }
        }
        RunnerMode::Backfill => {
            let mut idle_backoff = Duration::from_secs(1);

            loop {
                r#async::sleep_or_cancel(shutdown, idle_backoff).await?;
                if shutdown.is_cancelled() {
                    return Ok(());
                }

                let Some(work) = receiverdb::next_backfill_work(
                    dbh,
                    controller_cfg.deployment_block,
                    receiver_usdt_cfg.to_batch_size,
                )
                .await?
                else {
                    idle_backoff = (idle_backoff * 2).min(Duration::from_secs(10));
                    continue;
                };
                idle_backoff = Duration::from_secs(1);

                let receiver_count = work.to_addrs.len();
                let start_block = work.start_block;
                let stop_at_or_above = work.stop_at_or_above;

                // Backfill one chunk for this cohort.
                let Some((head, head_ms)) = timed_await_or_cancel(shutdown, async {
                    provider.get_block_number().await.map_err(|e| {
                        telemetry.rpc_error("eth_blockNumber", "head");
                        anyhow::Error::new(e).context("Failed to get block number")
                    })
                })
                .await?
                else {
                    return Ok(());
                };
                telemetry.rpc_call("eth_blockNumber", "head", true, head_ms);
                let safe_head = head.saturating_sub(controller_cfg.confirmations);

                if start_block > safe_head {
                    report_receiver_usdt_progress(
                        process_ctx.progress,
                        head,
                        safe_head,
                        start_block,
                        receiver_count,
                        batch_size,
                    );
                    continue;
                }

                // Optimization: for newly discovered receivers, if current balanceOf is 0 for all
                // historically configured USDT tokens, then by protocol invariant no transfers
                // have ever happened to that receiver, so backfill can be skipped.
                //
                // This is safe because the Tron-side controller sweep leaves 1 base unit behind
                // after the first non-zero deposit, so balance never returns to 0.
                let usdt_tokens = list_historical_controller_usdt_tokens_up_to(
                    dbh,
                    controller_cfg.deployment_block,
                    safe_head,
                )
                .await?;
                let usdt_tokens = std::sync::Arc::new(usdt_tokens);
                if !usdt_tokens.is_empty() {
                    let provider = provider.clone();
                    let shutdown = shutdown.clone();
                    let telemetry = telemetry.clone();

                    let check_concurrency = block_header_concurrency.max(1).min(32);
                    let receivers = work
                        .receiver_map
                        .iter()
                        .map(|(addr, salt)| (*addr, salt.clone()))
                        .collect::<Vec<_>>();
                    let results = stream::iter(receivers.into_iter())
                        .map(|(receiver_addr, receiver_salt)| {
                            let provider = provider.clone();
                            let shutdown = shutdown.clone();
                            let telemetry = telemetry.clone();
                            let usdt_tokens = usdt_tokens.clone();
                            async move {
                                if shutdown.is_cancelled() {
                                    return Ok::<_, anyhow::Error>((receiver_salt, true));
                                }

                                for token in usdt_tokens.iter().copied() {
                                    let contract = untron_v3_bindings::erc20::ERC20::new(
                                        token,
                                        provider.clone(),
                                    );
                                    let call = contract.balanceOf(receiver_addr);
                                    let request = call
                                        .clone()
                                        .into_transaction_request()
                                        .normalized_data();

                                    let Some((return_data, ms)) =
                                        timed_await_or_cancel(&shutdown, async {
                                            provider
                                                .call(request)
                                                .block(alloy::eips::BlockId::latest())
                                                .await
                                                .map_err(|e| {
                                                    telemetry.rpc_error("eth_call", "erc20.balanceOf");
                                                    anyhow::Error::new(e).context("eth_call(erc20.balanceOf)")
                                                })
                                        })
                                        .await?
                                    else {
                                        return Ok((receiver_salt, true));
                                    };
                                    telemetry.rpc_call("eth_call", "erc20.balanceOf", true, ms);

                                    let decoded =
                                        <untron_v3_bindings::erc20::ERC20::balanceOfCall as alloy::sol_types::SolCall>::abi_decode_returns(
                                            return_data.as_ref(),
                                        )
                                        .context("decode erc20.balanceOf return")?;
                                    if decoded != alloy::primitives::U256::ZERO {
                                        return Ok((receiver_salt, true));
                                    }
                                }
                                Ok((receiver_salt, false))
                            }
                        })
                        .buffer_unordered(check_concurrency)
                        .collect::<Vec<_>>()
                        .await;

                    let mut nonzero_salts = Vec::new();
                    let mut zero_salts = Vec::new();
                    for r in results {
                        match r {
                            Ok((salt, has_nonzero)) => {
                                if has_nonzero {
                                    nonzero_salts.push(salt);
                                } else {
                                    zero_salts.push(salt);
                                }
                            }
                            Err(e) => {
                                warn!(err = %e, "receiver_usdt balanceOf precheck failed; falling back to normal backfill");
                                nonzero_salts = work.receiver_salts.clone();
                                zero_salts.clear();
                                break;
                            }
                        }
                    }

                    if !zero_salts.is_empty() {
                        receiverdb::clear_backfill_for_receiver_salts(dbh, &zero_salts).await?;
                    }

                    if nonzero_salts.is_empty() {
                        report_receiver_usdt_progress(
                            process_ctx.progress,
                            head,
                            safe_head,
                            stop_at_or_above,
                            0,
                            batch_size,
                        );
                        continue;
                    }

                    // Filter this cohort down to receivers that actually have non-zero balance.
                    let nonzero_set = nonzero_salts
                        .iter()
                        .cloned()
                        .collect::<std::collections::HashSet<_>>();
                    let mut filtered_map = HashMap::new();
                    let mut filtered_addrs = Vec::new();
                    for (addr, salt) in work.receiver_map.iter() {
                        if nonzero_set.contains(salt) {
                            filtered_map.insert(*addr, salt.clone());
                            filtered_addrs.push(*addr);
                        }
                    }
                    let work = receiverdb::BackfillWork {
                        start_block,
                        stop_at_or_above,
                        receiver_salts: nonzero_salts,
                        receiver_map: filtered_map,
                        to_addrs: filtered_addrs,
                    };

                    // Recompute receiver_count for progress + downstream chunking.
                    let receiver_count = work.to_addrs.len();

                    let Some(next_block) = scan_chunks(
                        &mut process_ctx,
                        &work.receiver_map,
                        &work.to_addrs,
                        start_block,
                        safe_head,
                        1,
                    )
                    .await?
                    else {
                        return Ok(());
                    };
                    receiverdb::advance_backfill_for_receiver_salts(
                        dbh,
                        &work.receiver_salts,
                        start_block,
                        next_block,
                        stop_at_or_above,
                    )
                    .await?;
                    report_receiver_usdt_progress(
                        process_ctx.progress,
                        head,
                        safe_head,
                        next_block,
                        receiver_count,
                        batch_size,
                    );
                    continue;
                }

                let Some(next_block) = scan_chunks(
                    &mut process_ctx,
                    &work.receiver_map,
                    &work.to_addrs,
                    start_block,
                    safe_head,
                    1,
                )
                .await?
                else {
                    return Ok(());
                };
                receiverdb::advance_backfill_for_receiver_salts(
                    dbh,
                    &work.receiver_salts,
                    start_block,
                    next_block,
                    stop_at_or_above,
                )
                .await?;
                report_receiver_usdt_progress(
                    process_ctx.progress,
                    head,
                    safe_head,
                    next_block,
                    receiver_count,
                    batch_size,
                );
            }
        }
    }
}

async fn process_block_range(
    ctx: &mut ProcessCtx<'_>,
    receiver_map: &HashMap<alloy::primitives::Address, String>,
    to_addrs: &[alloy::primitives::Address],
    from_block: u64,
    to_block: u64,
) -> Result<Option<()>> {
    // Split by controller USDT token changes, so transfers are indexed under the correct token address.
    let points = receiverdb::usdt_set_points_up_to(ctx.dbh, from_block, to_block).await?;
    let segments = compute_usdt_segments(points, from_block, to_block)?;

    if segments.is_empty() {
        // No known USDT token configured in this window; skip.
        return Ok(Some(()));
    }

    // Batch recipients to avoid huge topic arrays.
    let batch_size = ctx.receiver_usdt_cfg.to_batch_size.max(1);
    for (token_evm, token_tron, seg_from, seg_to) in segments {
        if seg_from > seg_to {
            continue;
        }

        for chunk in to_addrs.chunks(batch_size) {
            let res = range::process_token_range(
                ctx.dbh,
                ctx.shutdown,
                ctx.provider,
                ctx.timestamps_state,
                ctx.telemetry,
                ctx.mode,
                range::ReceiverSet {
                    to_addrs: chunk,
                    addr_to_salt: receiver_map,
                },
                range::TokenRange {
                    chain_id: ctx.chain_id_i64,
                    token_evm,
                    token_tron: &token_tron,
                    from_block: seg_from,
                    to_block: seg_to,
                },
            )
            .await;

            match res {
                Ok(Some(metrics)) => {
                    ctx.telemetry.observe_range(
                        ctx.mode,
                        token_tron.as_str(),
                        chunk.len() as u64,
                        metrics.logs,
                        metrics.rows,
                        metrics.rpc_ms,
                        metrics.db_ms,
                        metrics.total_ms,
                    );
                    ctx.progress.observe_range(metrics);
                }
                Ok(None) => return Ok(None),
                Err(e) => return Err(e),
            }
        }
    }

    Ok(Some(()))
}

fn compute_usdt_segments(
    points: Vec<receiverdb::UsdtSetPoint>,
    from_block: u64,
    to_block: u64,
) -> Result<Vec<(alloy::primitives::Address, String, u64, u64)>> {
    let mut segments = Vec::new();

    let mut cur_token_evm: Option<alloy::primitives::Address> = None;
    let mut cur_token_tron: Option<String> = None;
    let mut cur_start = from_block;

    for p in points {
        let evm = crate::domain::TronAddress::parse_text(&p.usdt_tron)
            .or_else(|_| crate::domain::TronAddress::parse_text(&p.usdt_evm))
            .context("parse controller usdt address")?
            .evm();

        if cur_token_evm.is_none() {
            cur_token_evm = Some(evm);
            cur_token_tron = Some(p.usdt_tron);
            cur_start = from_block.max(p.block_number);
            continue;
        }

        let token_evm = cur_token_evm.expect("set");
        let token_tron = cur_token_tron.clone().expect("set");

        if cur_start <= p.block_number.min(to_block) {
            segments.push((
                token_evm,
                token_tron,
                cur_start,
                p.block_number.min(to_block),
            ));
        }

        cur_token_evm = Some(evm);
        cur_token_tron = Some(p.usdt_tron);
        cur_start = p.block_number.min(to_block);
    }

    if let (Some(token_evm), Some(token_tron)) = (cur_token_evm, cur_token_tron.clone())
        && cur_start <= to_block
    {
        segments.push((token_evm, token_tron, cur_start, to_block));
    }

    Ok(segments)
}
