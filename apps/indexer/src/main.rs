mod config;
mod db;
mod domain;
mod event_chain;
mod metrics;
mod observability;
mod receiver_usdt;
mod rpc;
mod shared;

use anyhow::{Context, Result};
use dotenvy::dotenv;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let otel = observability::init("indexer")?;

    let config::AppConfig {
        database_url,
        streams,
        receiver_usdt: receiver_usdt_cfg,
        db_max_connections,
        block_header_concurrency,
        block_timestamp_cache_size,
        progress_interval,
    } = config::load_config()?;
    let dbh = db::Db::connect(&database_url, db_max_connections).await?;
    // Keep this in sync with the latest migration file number.
    let _schema_version = db::ensure_schema_version(&dbh, 5).await?;

    let shutdown = CancellationToken::new();

    let mut join_set = tokio::task::JoinSet::new();

    // If the controller stream is configured, we also run the TRC-20 receiver transfer indexer
    // using the same RPC providers and deployment block.
    let mut controller_for_receiver_usdt: Option<config::StreamConfig> = None;
    let mut controller_providers_for_receiver_usdt: Option<rpc::RpcProviders> = None;
    let mut controller_resolved_for_receiver_usdt: Option<db::ResolvedStream> = None;

    for stream_cfg in streams {
        let dbh = dbh.clone();

        let resolved = db::ensure_instance_config(
            &dbh,
            stream_cfg.stream,
            stream_cfg.chain_id,
            &stream_cfg.contract_address,
        )
        .await?;

        let providers = rpc::RpcProviders::from_config(&stream_cfg.rpc).await?;

        let shutdown = shutdown.clone();

        if stream_cfg.stream == config::Stream::Controller {
            controller_for_receiver_usdt = Some(stream_cfg.clone());
            controller_providers_for_receiver_usdt = Some(providers.clone());
            controller_resolved_for_receiver_usdt = Some(resolved.clone());
        }

        join_set.spawn(async move {
            event_chain::run_stream(event_chain::RunStreamParams {
                dbh,
                cfg: stream_cfg,
                resolved,
                providers,
                shutdown,
                block_header_concurrency,
                block_timestamp_cache_size,
                progress_interval,
            })
            .await
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
            join_set.spawn(async move {
                receiver_usdt::run_receiver_usdt_indexer(receiver_usdt::RunReceiverUsdtParams {
                    dbh,
                    controller_cfg: cfg,
                    resolved,
                    providers,
                    receiver_usdt_cfg,
                    block_header_concurrency,
                    block_timestamp_cache_size,
                    progress_interval,
                    shutdown,
                })
                .await
            });
        }
    }

    info!("indexer started");

    let mut fatal: Option<anyhow::Error> = None;
    tokio::select! {
        res = shutdown_signal() => {
            res?;
            info!("shutdown requested");
        },
        res = join_set.join_next() => {
            if let Some(res) = res {
                let res = res.context("stream task panicked")?;
                match res {
                    Ok(()) => fatal = Some(anyhow::anyhow!("stream task exited unexpectedly")),
                    Err(e) => fatal = Some(e.context("stream task failed")),
                }
            }
        }
    }

    shutdown.cancel();

    while let Some(res) = join_set.join_next().await {
        let res = res.context("stream task panicked")?;
        if let Err(e) = res {
            fatal.get_or_insert_with(|| e.context("stream task failed"));
        }
    }

    match fatal {
        Some(e) => Err(e),
        None => {
            otel.shutdown().await;
            Ok(())
        }
    }
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
