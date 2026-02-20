mod config;
mod db;
mod domain;
mod event_chain;
mod hub_deposit_processed;
mod metrics;
mod receiver_usdt;
mod rpc;
mod shared;

use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::time::Duration;
use tokio::time;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // trigger rebuild (watch paths) + enable OTLP logs rollout

    let mut otel = Some(untron_observability::init(untron_observability::Config {
        service_name: "indexer",
        service_version: env!("CARGO_PKG_VERSION"),
    })?);

    let config::AppConfig {
        database_url,
        streams,
        receiver_usdt: receiver_usdt_cfg,
        db_max_connections,
        block_header_concurrency,
        block_timestamp_cache_size,
        progress_interval,
        progress_tail_lag_blocks,
        hub_deposit_processed: hub_deposit_processed_cfg,
    } = config::load_config()?;
    let dbh = db::Db::connect(&database_url, db_max_connections).await?;
    // Keep this in sync with the latest migration file number.
    let _schema_version = db::ensure_schema_version(&dbh, 22).await?;

    let shutdown = CancellationToken::new();

    let mut join_set: tokio::task::JoinSet<Result<()>> = tokio::task::JoinSet::new();

    // If the controller stream is configured, we also run the TRC-20 receiver transfer indexer
    // using the same RPC providers and deployment block.
    let mut controller_for_receiver_usdt: Option<config::StreamConfig> = None;
    let mut controller_providers_for_receiver_usdt: Option<rpc::RpcProviders> = None;
    let mut controller_resolved_for_receiver_usdt: Option<db::ResolvedStream> = None;

    let mut hub_for_deposit_processed: Option<config::StreamConfig> = None;
    let mut hub_providers_for_deposit_processed: Option<rpc::RpcProviders> = None;
    let mut hub_resolved_for_deposit_processed: Option<db::ResolvedStream> = None;

    // Configure all streams in the DB before starting any ingestion tasks.
    // This avoids races where the hub stream observes controller-related hub events before
    // `chain.instance(stream='controller')` exists, which can seed projections with a zero genesis tip.
    let mut resolved_streams: Vec<(config::StreamConfig, db::ResolvedStream, rpc::RpcProviders)> =
        Vec::new();

    for stream_cfg in streams {
        let dbh = dbh.clone();

        let resolved = db::ensure_instance_config(
            &dbh,
            stream_cfg.stream,
            stream_cfg.chain_id,
            &stream_cfg.contract_address,
        )
        .await?;

        let providers =
            rpc::RpcProviders::from_config(stream_cfg.stream, stream_cfg.chain_id, &stream_cfg.rpc)
                .await?;

        if stream_cfg.stream == config::Stream::Controller {
            controller_for_receiver_usdt = Some(stream_cfg.clone());
            controller_providers_for_receiver_usdt = Some(providers.clone());
            controller_resolved_for_receiver_usdt = Some(resolved.clone());
        }

        if stream_cfg.stream == config::Stream::Hub {
            hub_for_deposit_processed = Some(stream_cfg.clone());
            hub_providers_for_deposit_processed = Some(providers.clone());
            hub_resolved_for_deposit_processed = Some(resolved.clone());
        }

        resolved_streams.push((stream_cfg, resolved, providers));
    }

    for (stream_cfg, resolved, providers) in resolved_streams {
        let dbh = dbh.clone();
        let shutdown = shutdown.clone();

        join_set.spawn(async move {
            let stream_label = stream_cfg.stream.as_str();
            let mut backoff = Duration::from_millis(250);
            loop {
                if shutdown.is_cancelled() {
                    return Ok(());
                }

                let res = event_chain::run_stream(event_chain::RunStreamParams {
                    dbh: dbh.clone(),
                    cfg: stream_cfg.clone(),
                    resolved: resolved.clone(),
                    providers: providers.clone(),
                    shutdown: shutdown.clone(),
                    block_header_concurrency,
                    block_timestamp_cache_size,
                    progress_interval,
                    progress_tail_lag_blocks,
                })
                .await;

                match res {
                    Ok(()) => {
                        // On shutdown/deploy, tasks can return Ok(()) via cancellation; don't log
                        // that as an error.
                        if shutdown.is_cancelled() {
                            return Ok(());
                        }
                        // Stream tasks should be effectively long-lived; an unexpected clean exit
                        // is service-affecting (it stops ingestion until restart).
                        error!(
                            stream = stream_label,
                            "stream task exited unexpectedly; restarting"
                        )
                    }
                    Err(e) => {
                        // Use Debug formatting for `anyhow::Error` to include the full cause chain.
                        error!(stream = stream_label, err = ?e, "stream task failed; restarting")
                    }
                }

                time::sleep(backoff).await;
                backoff = (backoff * 2).min(Duration::from_secs(5));
            }
        });
    }

    if let (Some(cfg), Some(providers), Some(resolved)) = (
        controller_for_receiver_usdt,
        controller_providers_for_receiver_usdt,
        controller_resolved_for_receiver_usdt,
    ) {
        let dbh = dbh.clone();
        let shutdown = shutdown.clone();

        // Receiver USDT indexer has its own env-driven knobs; it only requires controller RPC access + DB.
        if receiver_usdt_cfg.enabled {
            // KPI loop: how long deposits sit in recommended_action=subjective_pre_entitle.
            // We keep this separate from the ingestion loop so visibility doesn't depend on log ranges.
            {
                let dbh = dbh.clone();
                let shutdown = shutdown.clone();
                let chain_id = cfg.chain_id;
                join_set.spawn(async move {
                    let poll_interval = receiver_usdt_cfg
                        .poll_interval
                        .min(Duration::from_secs(30))
                        .max(Duration::from_secs(5));
                    receiver_usdt::run_subjective_pre_entitle_kpi(
                        receiver_usdt::RunSubjectivePreEntitleKpiParams {
                            dbh,
                            chain_id,
                            token: "usdt",
                            poll_interval,
                            shutdown,
                        },
                    )
                    .await
                });
            }

            join_set.spawn(async move {
                let mut backoff = Duration::from_millis(250);
                loop {
                    if shutdown.is_cancelled() {
                        return Ok(());
                    }

                    let res = receiver_usdt::run_receiver_usdt_indexer(
                        receiver_usdt::RunReceiverUsdtParams {
                            dbh: dbh.clone(),
                            controller_cfg: cfg.clone(),
                            resolved: resolved.clone(),
                            providers: providers.clone(),
                            receiver_usdt_cfg: receiver_usdt_cfg.clone(),
                            block_header_concurrency,
                            block_timestamp_cache_size,
                            progress_interval,
                            progress_tail_lag_blocks,
                            shutdown: shutdown.clone(),
                        },
                    )
                    .await;

                    match res {
                        Ok(()) => {
                            // On shutdown/deploy, tasks can return Ok(()) via cancellation; don't log
                            // that as an error.
                            if shutdown.is_cancelled() {
                                return Ok(());
                            }
                            // This task should be long-lived; a clean exit indicates receiver USDT
                            // discovery/backfill has stopped until restart.
                            error!("receiver_usdt task exited unexpectedly; restarting")
                        }
                        // Use Debug formatting for `anyhow::Error` to include the full cause chain.
                        Err(e) => error!(err = ?e, "receiver_usdt task failed; restarting"),
                    }

                    time::sleep(backoff).await;
                    backoff = (backoff * 2).min(Duration::from_secs(5));
                }
            });
        }
    }

    if hub_deposit_processed_cfg.enabled {
        if let (Some(_cfg), Some(providers), Some(resolved)) = (
            hub_for_deposit_processed,
            hub_providers_for_deposit_processed,
            hub_resolved_for_deposit_processed,
        ) {
            let dbh = dbh.clone();
            let shutdown = shutdown.clone();
            join_set.spawn(async move {
                let mut backoff = Duration::from_millis(250);
                loop {
                    if shutdown.is_cancelled() {
                        return Ok(());
                    }

                    let res = hub_deposit_processed::run_hub_deposit_processed_cache(
                        hub_deposit_processed::RunHubDepositProcessedParams {
                            dbh: dbh.clone(),
                            resolved: resolved.clone(),
                            providers: providers.clone(),
                            shutdown: shutdown.clone(),
                            poll_interval: hub_deposit_processed_cfg.poll_interval,
                            batch_size: hub_deposit_processed_cfg.batch_size,
                            recheck_after: hub_deposit_processed_cfg.recheck_after,
                            concurrency: hub_deposit_processed_cfg.concurrency,
                        },
                    )
                    .await;

                    match res {
                        Ok(()) => {
                            // On shutdown/deploy, tasks can return Ok(()) via cancellation; don't log
                            // that as an error.
                            if shutdown.is_cancelled() {
                                return Ok(());
                            }
                            // This task should be long-lived; a clean exit means depositProcessed
                            // caching is stopped until restart.
                            error!("hub_deposit_processed task exited unexpectedly; restarting")
                        }
                        Err(e) => error!(err = ?e, "hub_deposit_processed task failed; restarting"),
                    }

                    time::sleep(backoff).await;
                    backoff = (backoff * 2).min(Duration::from_secs(5));
                }
            });
        }
    }

    info!("indexer started");
    shutdown_signal().await?;
    info!("shutdown requested");
    shutdown.cancel();

    while let Some(res) = join_set.join_next().await {
        let res = res.context("stream task panicked")?;
        if let Err(e) = res {
            warn!(err = %e, "task exited with error during shutdown");
        }
    }

    if let Some(otel) = otel.take() {
        otel.shutdown().await;
    }
    Ok(())
}

async fn shutdown_signal() -> Result<()> {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let mut sigterm = signal(SignalKind::terminate()).context("install SIGTERM handler")?;
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = sigterm.recv() => {},
        }
        Ok(())
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.context("ctrl-c")?;
        Ok(())
    }
}
