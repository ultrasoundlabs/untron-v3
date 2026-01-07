use alloy::{
    eips::BlockId, providers::Provider, rpc::types::Filter, sol_types::SolCall, sol_types::SolEvent,
};
use anyhow::{Context, Result};
use sqlx::types::BigDecimal;
use std::{collections::HashMap, time::Instant};
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, debug};

use crate::{
    db::receiver_usdt as db,
    receiver_usdt::telemetry::ReceiverUsdtTelemetry,
    shared::{
        r#async,
        progress::RangeMetrics,
        timestamps::{TimestampState, block_timestamp_for_log},
    },
};
use untron_v3_bindings::{erc20::ERC20, untron_controller::UntronController};

pub(crate) struct ReceiverSet<'a> {
    pub(crate) to_addrs: &'a [alloy::primitives::Address],
    pub(crate) addr_to_salt: &'a HashMap<alloy::primitives::Address, String>,
}

pub(crate) struct TokenRange<'a> {
    pub(crate) chain_id: i64,
    pub(crate) token_evm: alloy::primitives::Address,
    pub(crate) token_tron: &'a str,
    pub(crate) from_block: u64,
    pub(crate) to_block: u64,
}

pub(crate) async fn process_token_range(
    dbh: &crate::db::Db,
    shutdown: &CancellationToken,
    provider: &alloy::providers::DynProvider,
    timestamps_state: &mut TimestampState,
    telemetry: &ReceiverUsdtTelemetry,
    mode: &'static str,
    receivers: ReceiverSet<'_>,
    range: TokenRange<'_>,
) -> Result<Option<RangeMetrics>> {
    if receivers.to_addrs.is_empty() {
        return Ok(Some(RangeMetrics {
            from_block: range.from_block,
            to_block: range.to_block,
            logs: 0,
            rows: 0,
            rpc_ms: 0,
            ts_ms: 0,
            decode_ms: 0,
            db_ms: 0,
            total_ms: 0,
        }));
    }

    let TokenRange {
        chain_id,
        token_evm,
        token_tron,
        from_block,
        to_block,
    } = range;

    let span = tracing::debug_span!(
        "receiver_usdt_range",
        from_block,
        to_block,
        token = %token_tron,
        receiver_count = receivers.to_addrs.len()
    );

    async move {
        let start = Instant::now();

        let mut topic2 = alloy::rpc::types::Topic::default();
        for a in receivers.to_addrs {
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
        let logs = crate::shared::logs::validate_and_sort_logs(raw_logs)?;
        let logs_count = logs.len() as u64;

        let Some(((), ts_ms)) = r#async::timed_await_or_cancel(shutdown, async {
            timestamps_state
                .populate_timestamps(shutdown, provider, &logs, &[])
                .await
                .inspect_err(|_| {
                    telemetry.error(mode, token_tron, "timestamp");
                })
                .context("timestamp enrichment")
        })
        .await?
        else {
            return Ok(None);
        };
        if shutdown.is_cancelled() {
            return Ok(None);
        }

        let decode_start = Instant::now();
        let token = token_tron.to_string();
        let mut rows = Vec::with_capacity(logs.len());
        for l in logs {
            let decoded = l.log.log_decode::<ERC20::Transfer>().map_err(|e| {
                telemetry.error(mode, token_tron, "decode");
                anyhow::anyhow!("Transfer decode failed: {e}")
            })?;

            let from: alloy::primitives::Address = decoded.inner.data.from;
            let to: alloy::primitives::Address = decoded.inner.data.to;
            let value: alloy::primitives::U256 = decoded.inner.data.value;

            let Some(receiver_salt) = receivers.addr_to_salt.get(&to) else {
                continue;
            };

            let block_timestamp = block_timestamp_for_log(&mut timestamps_state.cache, &l)?;

            rows.push(db::TransferRow {
                chain_id,
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
            telemetry.error(mode, token_tron, "db");
        })?;
        let db_ms = db_start.elapsed().as_millis() as u64;
        let total_ms = start.elapsed().as_millis() as u64;

        debug!(
            from_block,
            to_block,
            token = %token_tron,
            logs = logs_count,
            rows = rows.len(),
            rpc_ms,
            ts_ms,
            decode_ms,
            db_ms,
            total_ms,
            "receiver_usdt range processed"
        );

        Ok(Some(RangeMetrics {
            from_block,
            to_block,
            logs: logs_count,
            rows: rows.len() as u64,
            rpc_ms,
            ts_ms,
            decode_ms,
            db_ms,
            total_ms,
        }))
    }
    .instrument(span)
    .await
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
