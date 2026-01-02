use crate::{config::Stream, db};
use alloy::{providers::Provider, rpc::types::Filter, sol_types::SolEvent};
use anyhow::{Context, Result};
use tokio_util::sync::CancellationToken;
use untron_v3_bindings::{
    untron_controller_index::UntronControllerIndex, untron_v3_index::UntronV3Index,
};

use super::{helpers, logs::validate_logs, rows, state::PollState};

pub(super) async fn process_range(
    dbh: &db::Db,
    shutdown: &CancellationToken,
    state: &mut PollState,
    from_block: u64,
    to_block: u64,
) -> Result<()> {
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
) -> Result<()> {
    let event_appended_topic0 = match state.stream {
        Stream::Hub => UntronV3Index::EventAppended::SIGNATURE_HASH,
        Stream::Controller => UntronControllerIndex::EventAppended::SIGNATURE_HASH,
    };

    let filter = Filter::new()
        .address(state.contract_address_rpc)
        .from_block(from_block)
        .to_block(to_block)
        .event_signature(event_appended_topic0);

    let event_logs = helpers::await_or_cancel(shutdown, async {
        provider
            .get_logs(&filter)
            .await
            .with_context(|| format!("eth_getLogs EventAppended [{from_block}..{to_block}]"))
    })
    .await?
    .unwrap_or_default();
    if shutdown.is_cancelled() {
        return Ok(());
    }

    let mut proof_logs = Vec::new();
    if state.stream == Stream::Controller {
        let proof_filter = Filter::new()
            .address(state.contract_address_rpc)
            .from_block(from_block)
            .to_block(to_block)
            .event_signature(UntronControllerIndex::IsEventChainTipCalled::SIGNATURE_HASH);
        proof_logs = helpers::await_or_cancel(shutdown, async {
            provider.get_logs(&proof_filter).await.with_context(|| {
                format!("eth_getLogs IsEventChainTipCalled [{from_block}..{to_block}]")
            })
        })
        .await?
        .unwrap_or_default();
        if shutdown.is_cancelled() {
            return Ok(());
        }
    }

    let mut event_logs = validate_logs(event_logs)?;
    let mut proof_logs = validate_logs(proof_logs)?;

    event_logs.sort_by_key(|l| (l.block_number, l.log_index));
    proof_logs.sort_by_key(|l| (l.block_number, l.log_index));

    state
        .timestamps
        .populate_timestamps(shutdown, provider, &event_logs, &proof_logs)
        .await
        .context("timestamp enrichment")?;
    if shutdown.is_cancelled() {
        return Ok(());
    }

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

    if helpers::await_or_cancel(shutdown, db::insert_batch(dbh, &event_rows, &proof_rows))
        .await?
        .is_none()
    {
        return Ok(());
    }
    Ok(())
}
