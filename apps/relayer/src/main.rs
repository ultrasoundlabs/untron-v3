mod config;
mod evm;
mod indexer;
mod metrics;
mod runner;
mod uniswap_v4;

use anyhow::{Context, Result};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    // trigger rebuild (watch paths) + enable OTLP logs rollout

    let command = Command::parse(std::env::args().skip(1).collect::<Vec<_>>())?;

    let cfg = config::load_config()?;
    let mut otel = Some(untron_observability::init(untron_observability::Config {
        service_name: "relayer",
        service_version: env!("CARGO_PKG_VERSION"),
    })?);

    let telemetry = metrics::RelayerTelemetry::new();

    tracing::info!("relayer starting");
    tracing::info!(
        indexer = %cfg.indexer.base_url,
        hub_rpc = %cfg.hub.rpc_url,
        tron_grpc_urls = %cfg.tron.grpc_urls.join(","),
        command = command.name(),
        "config loaded"
    );

    let res = match command {
        Command::Run => run_service(cfg, telemetry).await,
        Command::DrainReceivers(opts) => {
            let mut relayer = runner::Relayer::new(cfg, telemetry).await?;
            relayer.drain_receivers(opts).await
        }
    };

    if let Some(otel) = otel.take() {
        otel.shutdown().await;
    }

    res
}

#[derive(Debug, Clone)]
enum Command {
    Run,
    DrainReceivers(runner::DrainReceiversOptions),
}

impl Command {
    fn parse(args: Vec<String>) -> Result<Self> {
        if args.is_empty() {
            return Ok(Self::Run);
        }

        match args[0].as_str() {
            "run" => Ok(Self::Run),
            "drain-receivers" => {
                let mut opts = runner::DrainReceiversOptions::default();
                let mut i = 1usize;
                while i < args.len() {
                    match args[i].as_str() {
                        "--rebalance" => opts.rebalance = true,
                        "--until-empty" => opts.until_empty = true,
                        "--max-rounds" => {
                            i += 1;
                            let value = args.get(i).context("--max-rounds requires a value")?;
                            opts.max_rounds = value.parse().context("parse --max-rounds")?;
                        }
                        "--poll-secs" => {
                            i += 1;
                            let value = args.get(i).context("--poll-secs requires a value")?;
                            opts.poll_secs = value.parse().context("parse --poll-secs")?;
                        }
                        "--observe-timeout-secs" => {
                            i += 1;
                            let value = args
                                .get(i)
                                .context("--observe-timeout-secs requires a value")?;
                            opts.observe_timeout_secs =
                                value.parse().context("parse --observe-timeout-secs")?;
                        }
                        "--help" | "-h" => {
                            print_usage();
                            std::process::exit(0);
                        }
                        other => anyhow::bail!("unknown drain-receivers argument: {other}"),
                    }
                    i += 1;
                }
                Ok(Self::DrainReceivers(opts))
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => anyhow::bail!("unknown command: {other}"),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::DrainReceivers(_) => "drain-receivers",
        }
    }
}

fn print_usage() {
    eprintln!(
        "Usage:\n  relayer [run]\n  relayer drain-receivers [--until-empty] [--rebalance] [--max-rounds N] [--poll-secs N] [--observe-timeout-secs N]"
    );
}

async fn run_service(cfg: config::AppConfig, telemetry: metrics::RelayerTelemetry) -> Result<()> {
    let shutdown = CancellationToken::new();

    let mut join_set = tokio::task::JoinSet::new();
    {
        let shutdown = shutdown.clone();
        let telemetry = telemetry.clone();
        join_set.spawn(async move {
            let relayer = runner::Relayer::new(cfg, telemetry).await?;
            relayer.run(shutdown).await
        });
    }

    tracing::info!("relayer started");

    let mut fatal: Option<anyhow::Error> = None;
    tokio::select! {
        res = shutdown_signal() => {
            res?;
            tracing::info!("shutdown requested");
        },
        res = join_set.join_next() => {
            if let Some(res) = res {
                let res = res.context("relayer task panicked")?;
                match res {
                    Ok(()) => fatal = Some(anyhow::anyhow!("relayer task exited unexpectedly")),
                    Err(e) => fatal = Some(e.context("relayer task failed")),
                }
            }
        }
    }

    shutdown.cancel();

    while let Some(res) = join_set.join_next().await {
        let res = res.context("relayer task panicked")?;
        if let Err(e) = res {
            fatal.get_or_insert_with(|| e.context("relayer task failed"));
        }
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
