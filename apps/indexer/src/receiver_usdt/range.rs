use alloy::{
    eips::BlockId, providers::Provider, rpc::types::Filter, sol_types::SolCall, sol_types::SolEvent,
};
use anyhow::{Context, Result};
use sqlx::types::BigDecimal;
use std::{collections::HashMap, time::Instant};
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, debug};

use crate::shared::rpc_telemetry::RpcTelemetry;
use crate::{
    db::receiver_usdt as db,
    receiver_usdt::telemetry::ReceiverUsdtTelemetry,
    shared::{
        r#async,
        logs::ValidatedLog,
        progress::RangeMetrics,
        timestamps::{BlockTimestampCache, block_timestamp_for_log},
    },
};
use untron_v3_bindings::{erc20::ERC20, untron_controller::UntronController};

pub(crate) struct ReceiverSet<'a> {
    pub(crate) addr_to_salt: &'a HashMap<alloy::primitives::Address, String>,
}

pub(crate) struct TokenRange<'a> {
    pub(crate) chain_id: i64,
    pub(crate) token_evm: alloy::primitives::Address,
    pub(crate) token_tron: &'a str,
    pub(crate) from_block: u64,
    pub(crate) to_block: u64,
}

pub(crate) struct TransferLogBatch {
    pub(crate) token_evm: alloy::primitives::Address,
    pub(crate) token_tron: String,
    pub(crate) from_block: u64,
    pub(crate) to_block: u64,
    pub(crate) receiver_count: usize,
    pub(crate) logs: Vec<ValidatedLog>,
    pub(crate) rpc_ms: u64,
    started_at: Instant,
}

pub(crate) async fn fetch_token_range_logs(
    shutdown: &CancellationToken,
    provider: &alloy::providers::DynProvider,
    telemetry: &ReceiverUsdtTelemetry,
    mode: &'static str,
    to_addrs: &[alloy::primitives::Address],
    range: TokenRange<'_>,
) -> Result<Option<TransferLogBatch>> {
    if to_addrs.is_empty() {
        return Ok(Some(TransferLogBatch {
            token_evm: range.token_evm,
            token_tron: range.token_tron.to_string(),
            from_block: range.from_block,
            to_block: range.to_block,
            receiver_count: 0,
            logs: Vec::new(),
            rpc_ms: 0,
            started_at: Instant::now(),
        }));
    }

    let TokenRange {
        token_evm,
        token_tron,
        from_block,
        to_block,
        ..
    } = range;

    let span = tracing::debug_span!(
        "receiver_usdt_fetch_logs",
        from_block,
        to_block,
        token = %token_tron,
        receiver_count = to_addrs.len()
    );

    async move {
        let start = Instant::now();

        let mut topic2 = alloy::rpc::types::Topic::default();
        for a in to_addrs {
            topic2 = topic2.extend(*a);
        }

        let filter = Filter::new()
            .address(token_evm)
            .from_block(from_block)
            .to_block(to_block)
            .event_signature(ERC20::Transfer::SIGNATURE_HASH)
            .topic2(topic2);

        let Some((raw_logs, rpc_ms)) = r#async::timed_await_or_cancel(shutdown, async {
            provider.get_logs(&filter).await.map_err(|e| {
                telemetry.error(mode, token_tron, "rpc");
                anyhow::Error::new(e).context(format!(
                    "eth_getLogs receiver_usdt Transfer [{from_block}..{to_block}]"
                ))
            })
        })
        .await?
        else {
            return Ok(None);
        };
        telemetry.rpc_call("eth_getLogs", "receiver_usdt Transfer", true, rpc_ms);
        let logs = crate::shared::logs::validate_and_sort_logs(raw_logs)?;

        debug!(
            from_block,
            to_block,
            token = %token_tron,
            logs = logs.len(),
            rpc_ms,
            "receiver_usdt transfer logs fetched"
        );

        Ok(Some(TransferLogBatch {
            token_evm,
            token_tron: token_tron.to_string(),
            from_block,
            to_block,
            rpc_ms,
            receiver_count: to_addrs.len(),
            logs,
            started_at: start,
        }))
    }
    .instrument(span)
    .await
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn decode_and_insert_token_range_logs(
    dbh: &crate::db::Db,
    timestamps_cache: &mut BlockTimestampCache,
    telemetry: &ReceiverUsdtTelemetry,
    mode: &'static str,
    receivers: ReceiverSet<'_>,
    range: TokenRange<'_>,
    batch: &TransferLogBatch,
    ts_ms: u64,
) -> Result<RangeMetrics> {
    let decode_start = Instant::now();
    let token = range.token_tron.to_string();
    let mut rows = Vec::with_capacity(batch.logs.len());
    for l in &batch.logs {
        let decoded = l.log.log_decode::<ERC20::Transfer>().map_err(|e| {
            telemetry.error(mode, range.token_tron, "decode");
            anyhow::anyhow!("Transfer decode failed: {e}")
        })?;

        let from: alloy::primitives::Address = decoded.inner.data.from;
        let to: alloy::primitives::Address = decoded.inner.data.to;
        let value: alloy::primitives::U256 = decoded.inner.data.value;

        let Some(receiver_salt) = receivers.addr_to_salt.get(&to) else {
            continue;
        };

        let block_timestamp = block_timestamp_for_log(timestamps_cache, l)?;

        rows.push(db::TransferRow {
            chain_id: range.chain_id,
            token: token.clone(),
            receiver_salt: receiver_salt.clone(),
            sender: crate::domain::TronAddress::from_evm(from).to_string(),
            recipient: crate::domain::TronAddress::from_evm(to).to_string(),
            amount: value
                .to_string()
                .parse::<BigDecimal>()
                .context("parse Transfer amount as BigDecimal")?,
            block_number: i64::try_from(l.block_number)
                .context("block_number out of range for bigint")?,
            block_timestamp: i64::try_from(block_timestamp)
                .context("block_timestamp out of range for bigint")?,
            block_hash: format!("0x{}", hex::encode(l.block_hash)),
            tx_hash: format!("0x{}", hex::encode(l.tx_hash)),
            log_index: i32::try_from(l.log_index).context("log_index out of range for int4")?,
        });
    }
    let decode_ms = decode_start.elapsed().as_millis() as u64;

    let db_start = Instant::now();
    db::insert_transfers(dbh, &rows).await.inspect_err(|_| {
        telemetry.error(mode, range.token_tron, "db");
    })?;
    let db_ms = db_start.elapsed().as_millis() as u64;
    let total_ms = batch.started_at.elapsed().as_millis() as u64;

    debug!(
        from_block = range.from_block,
        to_block = range.to_block,
        token = %range.token_tron,
        logs = batch.logs.len(),
        rows = rows.len(),
        rpc_ms = batch.rpc_ms,
        ts_ms,
        decode_ms,
        db_ms,
        total_ms,
        "receiver_usdt range processed"
    );

    Ok(RangeMetrics {
        from_block: range.from_block,
        to_block: range.to_block,
        logs: batch.logs.len() as u64,
        rows: rows.len() as u64,
        rpc_ms: batch.rpc_ms,
        ts_ms,
        decode_ms,
        db_ms,
        total_ms,
    })
}

pub(crate) async fn fetch_receiver_init_code_hash(
    shutdown: &CancellationToken,
    provider: &alloy::providers::DynProvider,
    controller_address: alloy::primitives::Address,
) -> Result<alloy::primitives::B256> {
    let contract = UntronController::new(controller_address, provider.clone());
    let call = contract.receiverBytecode();

    // Tron JSON-RPC accepts `data` but may reject `input` (and may even error if both are present).
    // Alloy defaults to `input`, so normalize into `data`-only.
    let request = call.clone().into_transaction_request().normalized_data();

    let return_data = r#async::await_or_cancel(shutdown, async {
        provider
            .call(request)
            .block(BlockId::latest())
            .await
            .map_err(|e| anyhow::Error::new(e).context("eth_call(receiverBytecode)"))
    })
    .await?
    .unwrap_or_default();

    if return_data.is_empty() {
        return Ok(alloy::primitives::B256::ZERO);
    }

    let decoded = <UntronController::receiverBytecodeCall as SolCall>::abi_decode_returns(
        return_data.as_ref(),
    )
    .map_err(|e| anyhow::Error::new(e).context("decode receiverBytecode() return"))?;
    if decoded.is_empty() {
        return Ok(alloy::primitives::B256::ZERO);
    }

    Ok(alloy::primitives::keccak256(decoded))
}
