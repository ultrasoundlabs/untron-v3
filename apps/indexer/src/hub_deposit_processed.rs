use crate::{config, db, rpc};
use alloy::primitives::FixedBytes;
use anyhow::{Context, Result};
use futures::{StreamExt, stream};
use std::str::FromStr;
use std::time::Duration;
use tokio::time;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use untron_v3_bindings::untron_v3::UntronV3;

pub struct RunHubDepositProcessedParams {
    pub dbh: db::Db,
    pub resolved: db::ResolvedStream,
    pub providers: rpc::RpcProviders,
    pub shutdown: CancellationToken,
    pub poll_interval: Duration,
    pub batch_size: usize,
    pub recheck_after: Duration,
    pub concurrency: usize,
}

pub async fn run_hub_deposit_processed_cache(params: RunHubDepositProcessedParams) -> Result<()> {
    let RunHubDepositProcessedParams {
        dbh,
        resolved,
        providers,
        shutdown,
        poll_interval,
        batch_size,
        recheck_after,
        concurrency,
    } = params;

    let (stream, _chain_id, contract_address_db) = resolved.into_parts();
    if stream != config::Stream::Hub {
        anyhow::bail!("hub_deposit_processed_cache requires hub resolved stream");
    }

    let hub_addr: alloy::primitives::Address = contract_address_db
        .as_str()
        .parse()
        .context("parse hub contract address")?;

    let hub = UntronV3::new(hub_addr, &providers.fallback);

    let poll_interval = poll_interval.max(Duration::from_secs(1));
    let recheck_after = recheck_after.max(Duration::from_secs(1));
    let batch_size = batch_size.max(1);
    let concurrency = concurrency.max(1).min(50);

    info!(
        hub = %hub_addr,
        interval_s = poll_interval.as_secs(),
        batch_size,
        recheck_after_s = recheck_after.as_secs(),
        "hub depositProcessed cache task started"
    );

    loop {
        if shutdown.is_cancelled() {
            return Ok(());
        }

        let candidates = db::deposit_processed::list_unprocessed_subjective_pre_entitle_txids(
            &dbh,
            batch_size,
            recheck_after.as_secs(),
        )
        .await?;

        if candidates.is_empty() {
            time::sleep(poll_interval).await;
            continue;
        }

        debug!(
            n = candidates.len(),
            "checking depositProcessed(txId) for candidates"
        );

        let attempted = candidates.len();
        let results_raw: Vec<Result<(String, bool)>> = stream::iter(candidates.into_iter())
            .map(|tx_hash| {
                let hub = hub.clone();
                async move {
                    let txid = FixedBytes::<32>::from_str(&tx_hash)
                        .with_context(|| format!("invalid tx_hash bytes32: {tx_hash}"))?;
                    let start = std::time::Instant::now();
                    let processed_res = hub.depositProcessed(txid).call().await;
                    let ok = processed_res.is_ok();
                    debug!(
                        ok,
                        ms = start.elapsed().as_millis() as u64,
                        "hub.depositProcessed eth_call"
                    );
                    let processed = processed_res.with_context(|| {
                        format!("depositProcessed eth_call failed for txid={tx_hash}")
                    })?;
                    Ok::<_, anyhow::Error>((tx_hash, processed))
                }
            })
            .buffer_unordered(concurrency)
            .collect()
            .await;

        let mut failed: usize = 0;
        let mut results: Vec<(String, bool)> = Vec::new();
        results.reserve(attempted);

        for r in results_raw {
            match r {
                Ok(v) => results.push(v),
                Err(e) => {
                    failed += 1;
                    warn!(err = ?e, "depositProcessed check failed");
                }
            }
        }

        if attempted > 0 {
            let ok = results.len();
            // If we are systematically failing these checks, deposits may remain stuck in
            // subjective_pre_entitle even though they are actually processed.
            if ok == 0 && attempted >= 10 {
                error!(
                    attempted,
                    failed,
                    "depositProcessed checks are failing (0 successes); deposit processing may be stalled"
                );
            } else if failed * 2 > attempted && attempted >= 20 {
                error!(
                    attempted,
                    failed,
                    "depositProcessed checks have a high failure rate; deposit processing may be degraded"
                );
            }
        }

        if !results.is_empty() {
            db::deposit_processed::upsert_deposit_processed_cache(&dbh, &results).await?;
        }

        // If we are still behind, don't wait the full interval.
        time::sleep(if results.len() >= batch_size {
            Duration::from_millis(200)
        } else {
            poll_interval
        })
        .await;
    }
}
