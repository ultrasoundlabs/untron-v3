mod config;
mod db;
mod decode;
mod domain;
mod observability;
mod poller;
mod reorg;
mod rpc;
mod telemetry;
mod util;

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
        retry,
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

    for stream_cfg in streams {
        let dbh = dbh.clone();

        let resolved = db::ensure_instance_config(
            &dbh,
            stream_cfg.stream,
            stream_cfg.chain_id,
            &stream_cfg.contract_address,
        )
        .await?;

        let providers = rpc::build_providers(&stream_cfg.rpc_urls, &retry).await?;

        let shutdown = shutdown.clone();

        join_set.spawn(async move {
            poller::run_stream(poller::RunStreamParams {
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
