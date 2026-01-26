mod backoff;
mod config;
mod metrics;
mod runner;
mod util;
mod watcher;

use anyhow::{Context, Result};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    // trigger rebuild (watch paths) + enable OTLP logs rollout

    let cfg = config::load_config()?;
    let mut otel = Some(untron_observability::init(untron_observability::Config {
        service_name: "pool",
        service_version: env!("CARGO_PKG_VERSION"),
    })?);

    let telemetry = metrics::PoolTelemetry::new();

    tracing::info!("pool starting");
    tracing::info!(
        tron_address = %tron::TronWallet::new(cfg.tron.private_key)?.address(),
        "wallet loaded"
    );
    tracing::info!(
        tron_grpc_urls = %cfg.tron.grpc_urls.join(","),
        usdt_contract = %cfg.tron.usdt_contract_address,
        oneclick_base_url = %cfg.oneclick.base_url,
        origin_asset = %cfg.oneclick.origin_asset,
        destination_asset = %cfg.oneclick.destination_asset,
        "config loaded"
    );

    let shutdown = CancellationToken::new();

    let mut join_set = tokio::task::JoinSet::new();
    {
        let shutdown = shutdown.clone();
        let telemetry = telemetry.clone();
        join_set.spawn(async move {
            let svc = runner::PoolService::new(cfg, telemetry).await?;
            svc.run(shutdown).await
        });
    }

    tracing::info!("pool started");

    let mut fatal: Option<anyhow::Error> = None;
    tokio::select! {
        res = shutdown_signal() => {
            res?;
            tracing::info!("shutdown requested");
        },
        res = join_set.join_next() => {
            if let Some(res) = res {
                let res = res.context("pool task panicked")?;
                match res {
                    Ok(()) => fatal = Some(anyhow::anyhow!("pool task exited unexpectedly")),
                    Err(e) => fatal = Some(e.context("pool task failed")),
                }
            }
        }
    }

    shutdown.cancel();

    while let Some(res) = join_set.join_next().await {
        let res = res.context("pool task panicked")?;
        if let Err(e) = res {
            fatal.get_or_insert_with(|| e.context("pool task failed"));
        }
    }

    if let Some(otel) = otel.take() {
        otel.shutdown().await;
    }
    fatal.map_or(Ok(()), Err)
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
