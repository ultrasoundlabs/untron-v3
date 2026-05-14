use crate::shared::rpc_telemetry::RpcTelemetry;
use crate::{config::Stream, db, shared::progress::RangeMetrics};
use alloy::{providers::Provider, rpc::types::Filter, sol_types::SolEvent};
use anyhow::{Context, Result};
use std::time::Instant;
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, debug};
use untron_v3_bindings::{
    untron_controller_index::UntronControllerIndex, untron_v3_index::UntronV3Index,
};

use super::{rows, state::PollState};
use crate::shared::{r#async, logs};

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
        let proof_topic0 = UntronControllerIndex::IsEventChainTipCalled::SIGNATURE_HASH;

        // Single getLogs with topic0 OR-list (controller carries both EventAppended +
        // IsEventChainTipCalled; hub carries only EventAppended). Logs are partitioned locally.
        let topic0_filter: Vec<alloy::primitives::B256> = match state.stream {
            Stream::Hub => vec![event_appended_topic0],
            Stream::Controller => vec![event_appended_topic0, proof_topic0],
        };

        let filter = Filter::new()
            .address(state.contract_address_rpc)
            .from_block(from_block)
            .to_block(to_block)
            .event_signature(topic0_filter);

        let Some((raw_logs, rpc_event_ms)) = r#async::timed_await_or_cancel(shutdown, async {
            provider.get_logs(&filter).await.map_err(|e| {
                state.telemetry.rpc_error("eth_getLogs", "event_chain");
                anyhow::Error::new(e)
                    .context(format!("eth_getLogs event_chain [{from_block}..{to_block}]"))
            })
        })
        .await?
        else {
            return Ok(None);
        };
        state
            .telemetry
            .rpc_call("eth_getLogs", "event_chain", true, rpc_event_ms);

        let (raw_event_logs, raw_proof_logs): (Vec<_>, Vec<_>) =
            raw_logs.into_iter().partition(|l| {
                l.topics()
                    .first()
                    .map(|t| *t == event_appended_topic0)
                    .unwrap_or(false)
            });
        let event_logs = logs::validate_and_sort_logs(raw_event_logs)?;
        let proof_logs = if state.stream == Stream::Controller {
            logs::validate_and_sort_logs(raw_proof_logs)?
        } else {
            Vec::new()
        };

        let Some(((), ts_ms)) = r#async::timed_await_or_cancel(shutdown, async {
            state
                .timestamps
                .populate_timestamps(
                    shutdown,
                    provider,
                    &event_logs,
                    &proof_logs,
                    Some(&state.telemetry),
                )
                .await
                .context("timestamp enrichment")
        })
        .await?
        else {
            return Ok(None);
        };
        if shutdown.is_cancelled() {
            return Ok(None);
        }
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
        let inserted = r#async::await_or_cancel(shutdown, async {
            match db::event_chain::insert_batch(dbh, &event_rows, &proof_rows).await {
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

        // Advance ingestion cursor only after the range commit succeeds.
        // This makes ingestion resume explicit and prevents permanent holes.
        let Some(((), advance_ms)) = r#async::timed_await_or_cancel(shutdown, async {
            db::event_chain::advance_ingest_cursor(dbh, state.stream, to_block.saturating_add(1))
                .await
        })
        .await?
        else {
            return Ok(None);
        };
        state
            .telemetry
            .observe_db_latency_ms("advance_ingest_cursor", advance_ms);

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

        let event_logs_count = event_rows.len();
        let proof_logs_count = proof_rows.len();
        let total_ms = total_start.elapsed().as_millis() as u64;

        state.telemetry.observe_range(
            from_block,
            to_block,
            event_logs_count as u64,
            proof_logs_count as u64,
            total_ms,
        );

        let metrics = RangeMetrics {
            from_block,
            to_block,
            logs: (event_logs_count + proof_logs_count) as u64,
            rows: (event_logs_count + proof_logs_count) as u64,
            rpc_ms: rpc_event_ms,
            ts_ms,
            decode_ms,
            db_ms,
            total_ms,
        };

        debug!(
            stream = state.stream.as_str(),
            from_block,
            to_block,
            event_logs = event_logs_count,
            proof_logs = proof_logs_count,
            rpc_event_ms,
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
