use crate::db::receiver_usdt as receiverdb;
use crate::{
    config::{ReceiverUsdtConfig, Stream, StreamConfig},
    db::{self, ResolvedStream},
    receiver_usdt::range,
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

pub struct RunReceiverUsdtParams {
    pub dbh: db::Db,
    pub controller_cfg: StreamConfig,
    pub resolved: ResolvedStream,
    pub providers: RpcProviders,
    pub receiver_usdt_cfg: ReceiverUsdtConfig,
    pub block_header_concurrency: usize,
    pub block_timestamp_cache_size: usize,
    pub progress_interval: Duration,
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
}

struct ProcessCtx<'a> {
    dbh: &'a db::Db,
    shutdown: &'a CancellationToken,
    provider: &'a alloy::providers::DynProvider,
    timestamps_state: &'a mut timestamps::TimestampState,
    progress: &'a mut ProgressReporter,
    receiver_usdt_cfg: &'a ReceiverUsdtConfig,
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
        shutdown,
    } = params;
    let (stream, chain_id, contract_address_db) = resolved.into_parts();
    if stream != Stream::Controller {
        anyhow::bail!("receiver_usdt indexer requires controller stream");
    }

    let controller_address_evm =
        crate::domain::TronAddress::from_base58check(contract_address_db.as_str())
            .context("resolve controller address (tron base58 -> evm)")?
            .evm();

    let init_code_hash = range::fetch_receiver_init_code_hash(
        &shutdown,
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
        let shutdown = shutdown.clone();
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
        let shutdown = shutdown.clone();
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
                },
                RunnerMode::Tail,
            )
            .await
        });
    }

    // Backfill loop (batched by cohort of receivers sharing the same backfill_next_block).
    for _ in 0..receiver_usdt_cfg.backfill_concurrency {
        let dbh = dbh.clone();
        let shutdown = shutdown.clone();
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

    shutdown.cancel();
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
    } = ctx;

    let mut timestamps_state =
        timestamps::TimestampState::new(block_timestamp_cache_size, block_header_concurrency);
    let chain_id_i64 = i64::try_from(controller_cfg.chain_id).context("chain_id out of range")?;

    let label = match mode {
        RunnerMode::Tail => "receiver_usdt_tail",
        RunnerMode::Backfill => "receiver_usdt_backfill",
    };
    let mut progress = ProgressReporter::new_receiver_usdt(
        label,
        progress_interval,
        0,
        receiver_usdt_cfg.to_batch_size.max(1),
    );

    let batch_size = receiver_usdt_cfg.to_batch_size.max(1);
    let mut process_ctx = ProcessCtx {
        dbh,
        shutdown,
        provider,
        timestamps_state: &mut timestamps_state,
        progress: &mut progress,
        receiver_usdt_cfg,
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

                let Some(head) = timed_await_or_cancel(shutdown, async {
                    provider
                        .get_block_number()
                        .await
                        .map_err(|e| anyhow::Error::new(e).context("Failed to get block number"))
                })
                .await?
                .map(|(v, _ms)| v) else {
                    return Ok(());
                };

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
                report_receiver_usdt_progress(
                    process_ctx.progress,
                    head,
                    safe_head,
                    from_block,
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
                let Some(head) = timed_await_or_cancel(shutdown, async {
                    provider
                        .get_block_number()
                        .await
                        .map_err(|e| anyhow::Error::new(e).context("Failed to get block number"))
                })
                .await?
                .map(|(v, _ms)| v) else {
                    return Ok(());
                };
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
                receiverdb::advance_backfill_batch(dbh, start_block, next_block, stop_at_or_above)
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
            match range::process_token_range(
                ctx.dbh,
                ctx.shutdown,
                ctx.provider,
                ctx.timestamps_state,
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
            .await?
            {
                Some(metrics) => ctx.progress.observe_range(metrics),
                None => return Ok(None),
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
