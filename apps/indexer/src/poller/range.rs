use crate::{config::Stream, db};
use alloy::{providers::Provider, rpc::types::Filter, sol_types::SolEvent};
use anyhow::{Context, Result};
use std::time::Instant;
use tokio_util::sync::CancellationToken;
use tracing::Instrument;
use tracing::debug;
use untron_v3_bindings::{
    untron_controller_index::UntronControllerIndex, untron_v3_index::UntronV3Index,
};

use super::{helpers, logs::validate_logs, rows, state::PollState};

#[derive(Debug, Clone)]
pub(super) struct RangeMetrics {
    pub(super) from_block: u64,
    pub(super) to_block: u64,
    pub(super) event_logs: usize,
    pub(super) proof_logs: usize,

    pub(super) rpc_event_ms: u64,
    pub(super) rpc_proof_ms: u64,
    pub(super) ts_ms: u64,
    pub(super) decode_ms: u64,
    pub(super) db_ms: u64,
    pub(super) total_ms: u64,
}

pub(super) async fn process_range(
    dbh: &db::Db,
    shutdown: &CancellationToken,
    state: &mut PollState,
    from_block: u64,
    to_block: u64,
) -> Result<Option<RangeMetrics>> {
    let provider = state.provider.clone();
    process_range_with_provider(dbh, shutdown, state, &provider, from_block, to_block).await
}

pub(super) async fn process_range_with_provider(
    dbh: &db::Db,
    shutdown: &CancellationToken,
    state: &mut PollState,
    provider: &alloy::providers::DynProvider,
    from_block: u64,
    to_block: u64,
) -> Result<Option<RangeMetrics>> {
    let span = tracing::debug_span!(
        "process_range",
        stream = state.stream.as_str(),
        chain_id = state.chain_id,
        from_block,
        to_block
    );

    async move {
        let total_start = Instant::now();
        let event_appended_topic0 = match state.stream {
            Stream::Hub => UntronV3Index::EventAppended::SIGNATURE_HASH,
            Stream::Controller => UntronControllerIndex::EventAppended::SIGNATURE_HASH,
        };

        let filter = Filter::new()
            .address(state.contract_address_rpc)
            .from_block(from_block)
            .to_block(to_block)
            .event_signature(event_appended_topic0);

        let rpc_event_start = Instant::now();
        let event_logs = helpers::await_or_cancel(shutdown, async {
            match provider.get_logs(&filter).await {
                Ok(v) => Ok(v),
                Err(e) => {
                    state.telemetry.rpc_error("eth_getLogs_EventAppended");
                    Err(anyhow::Error::new(e).context(format!(
                        "eth_getLogs EventAppended [{from_block}..{to_block}]"
                    )))
                }
            }
        })
        .await?;
        let rpc_event_ms = rpc_event_start.elapsed().as_millis() as u64;
        state
            .telemetry
            .observe_rpc_latency_ms("eth_getLogs_EventAppended", rpc_event_ms);
        let Some(event_logs) = event_logs else {
            return Ok(None);
        };
        let event_logs = event_logs;

        let mut proof_logs = Vec::new();
        let mut rpc_proof_ms = 0u64;
        if state.stream == Stream::Controller {
            let proof_filter = Filter::new()
                .address(state.contract_address_rpc)
                .from_block(from_block)
                .to_block(to_block)
                .event_signature(UntronControllerIndex::IsEventChainTipCalled::SIGNATURE_HASH);
            let rpc_proof_start = Instant::now();
            let maybe = helpers::await_or_cancel(shutdown, async {
                match provider.get_logs(&proof_filter).await {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        state
                            .telemetry
                            .rpc_error("eth_getLogs_IsEventChainTipCalled");
                        Err(anyhow::Error::new(e).context(format!(
                            "eth_getLogs IsEventChainTipCalled [{from_block}..{to_block}]"
                        )))
                    }
                }
            })
            .await?;
            rpc_proof_ms = rpc_proof_start.elapsed().as_millis() as u64;
            state
                .telemetry
                .observe_rpc_latency_ms("eth_getLogs_IsEventChainTipCalled", rpc_proof_ms);
            let Some(logs) = maybe else {
                return Ok(None);
            };
            proof_logs = logs;
        }

        let mut event_logs = validate_logs(event_logs)?;
        let mut proof_logs = validate_logs(proof_logs)?;

        event_logs.sort_by_key(|l| (l.block_number, l.log_index));
        proof_logs.sort_by_key(|l| (l.block_number, l.log_index));

        let ts_start = Instant::now();
        state
            .timestamps
            .populate_timestamps(shutdown, provider, &event_logs, &proof_logs)
            .await
            .context("timestamp enrichment")?;
        if shutdown.is_cancelled() {
            return Ok(None);
        }
        let ts_ms = ts_start.elapsed().as_millis() as u64;
        state.telemetry.observe_timestamp_enrichment_ms(ts_ms);

        let decode_start = Instant::now();
        let mut event_rows = Vec::with_capacity(event_logs.len());
        for log in event_logs {
            let row = rows::decode_event_appended(state, log)?;
            event_rows.push(row);
        }

        let mut proof_rows = Vec::with_capacity(proof_logs.len());
        for log in proof_logs {
            let row = rows::decode_tip_proof(state, log)?;
            proof_rows.push(row);
        }
        let decode_ms = decode_start.elapsed().as_millis() as u64;

        let db_start = Instant::now();
        let inserted = helpers::await_or_cancel(shutdown, async {
            match db::insert_batch(dbh, &event_rows, &proof_rows).await {
                Ok(()) => Ok(()),
                Err(e) => {
                    state.telemetry.db_error("insert_batch");
                    Err(e)
                }
            }
        })
        .await?;
        if inserted.is_none() {
            return Ok(None);
        }
        let db_ms = db_start.elapsed().as_millis() as u64;
        state.telemetry.observe_db_latency_ms("insert_batch", db_ms);
        // best-effort: counts are accurate even if some rows were conflict-updated
        state
            .telemetry
            .rows_upserted("chain.event_appended", event_rows.len() as u64);
        if !proof_rows.is_empty() {
            state
                .telemetry
                .rows_upserted("chain.controller_tip_proofs", proof_rows.len() as u64);
        }

        let metrics = RangeMetrics {
            from_block,
            to_block,
            event_logs: event_rows.len(),
            proof_logs: proof_rows.len(),
            rpc_event_ms,
            rpc_proof_ms,
            ts_ms,
            decode_ms,
            db_ms,
            total_ms: total_start.elapsed().as_millis() as u64,
        };

        debug!(
            stream = state.stream.as_str(),
            from_block,
            to_block,
            event_logs = metrics.event_logs,
            proof_logs = metrics.proof_logs,
            rpc_event_ms = metrics.rpc_event_ms,
            rpc_proof_ms = metrics.rpc_proof_ms,
            ts_ms = metrics.ts_ms,
            decode_ms = metrics.decode_ms,
            db_ms = metrics.db_ms,
            total_ms = metrics.total_ms,
            "range processed"
        );

        Ok(Some(metrics))
    }
    .instrument(span)
    .await
}
