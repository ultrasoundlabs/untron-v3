use crate::{
    config::Stream,
    db::event_chain::{ControllerTipProofRow, EventAppendedRow},
    domain,
};
use alloy::primitives::U256;
use anyhow::{Context, Result};
use untron_v3_bindings::{
    untron_controller_index::UntronControllerIndex, untron_v3_index::UntronV3Index,
};

use super::{decode, state::PollState};
use crate::shared::{logs::ValidatedLog, timestamps};

pub(super) fn decode_event_appended(
    state: &mut PollState,
    log: ValidatedLog,
) -> Result<EventAppendedRow> {
    match state.stream {
        Stream::Hub => decode_hub_event_appended(state, log),
        Stream::Controller => decode_controller_event_appended(state, log),
    }
}

fn decode_hub_event_appended(state: &mut PollState, log: ValidatedLog) -> Result<EventAppendedRow> {
    let block_number = log.block_number;
    let block_timestamp = timestamps::block_timestamp_for_log(&mut state.timestamps.cache, &log)?;

    let decoded = log
        .log
        .log_decode::<UntronV3Index::EventAppended>()
        .map_err(|e| anyhow::anyhow!("EventAppended decode failed: {e}"))?;

    let ev = decoded.inner.data;
    let event_seq = u256_to_u64(ev.eventSeq)?;
    let semantic_sig: alloy::primitives::B256 = ev.eventSignature;
    let prev_tip: alloy::primitives::B256 = ev.prevTip;
    let new_tip: alloy::primitives::B256 = ev.newTip;

    let semantic =
        decode::decode_semantic_event(Stream::Hub, semantic_sig, &ev.abiEncodedEventData)?;
    let (event_type, args_json) = semantic.into_db_parts();

    Ok(EventAppendedRow {
        stream: Stream::Hub,
        chain_id: state.chain_id,
        contract_address: state.contract_address_db.clone(),
        block_number: i64::try_from(block_number)
            .context("block_number out of range for bigint")?,
        block_timestamp: i64::try_from(block_timestamp)
            .context("block_timestamp out of range for bigint")?,
        block_hash: domain::BlockHash(log.block_hash),
        tx_hash: domain::TxHash(log.tx_hash),
        log_index: i32::try_from(log.log_index).context("log_index out of range for int4")?,
        event_seq: i64::try_from(event_seq).context("event_seq out of range for bigint")?,
        prev_tip: domain::Tip(prev_tip),
        new_tip: domain::Tip(new_tip),
        event_signature: domain::EventSignature(semantic_sig),
        abi_encoded_event_data: domain::AbiEncodedEventData(ev.abiEncodedEventData),
        event_type: event_type.into_owned(),
        args_json,
    })
}

fn decode_controller_event_appended(
    state: &mut PollState,
    log: ValidatedLog,
) -> Result<EventAppendedRow> {
    let block_number = log.block_number;
    let block_timestamp = timestamps::block_timestamp_for_log(&mut state.timestamps.cache, &log)?;

    let decoded = log
        .log
        .log_decode::<UntronControllerIndex::EventAppended>()
        .map_err(|e| anyhow::anyhow!("EventAppended decode failed: {e}"))?;

    let ev = decoded.inner.data;
    let event_seq = u256_to_u64(ev.eventSeq)?;
    let semantic_sig: alloy::primitives::B256 = ev.eventSignature;
    let prev_tip: alloy::primitives::B256 = ev.prevTip;
    let new_tip: alloy::primitives::B256 = ev.newTip;

    let semantic =
        decode::decode_semantic_event(Stream::Controller, semantic_sig, &ev.abiEncodedEventData)?;
    let (event_type, args_json) = semantic.into_db_parts();

    Ok(EventAppendedRow {
        stream: Stream::Controller,
        chain_id: state.chain_id,
        contract_address: state.contract_address_db.clone(),

        block_number: i64::try_from(block_number)
            .context("block_number out of range for bigint")?,
        block_timestamp: i64::try_from(block_timestamp)
            .context("block_timestamp out of range for bigint")?,
        block_hash: domain::BlockHash(log.block_hash),

        tx_hash: domain::TxHash(log.tx_hash),
        log_index: i32::try_from(log.log_index).context("log_index out of range for int4")?,

        event_seq: i64::try_from(event_seq).context("event_seq out of range for bigint")?,
        prev_tip: domain::Tip(prev_tip),
        new_tip: domain::Tip(new_tip),
        event_signature: domain::EventSignature(semantic_sig),
        abi_encoded_event_data: domain::AbiEncodedEventData(ev.abiEncodedEventData),

        event_type: event_type.into_owned(),
        args_json,
    })
}

pub(super) fn decode_tip_proof(
    state: &mut PollState,
    log: ValidatedLog,
) -> Result<ControllerTipProofRow> {
    let contract_address_db = match &state.contract_address_db {
        domain::ContractAddressDb::Controller(a) => a.clone(),
        _ => anyhow::bail!("internal error: tip proof decoded for non-controller stream"),
    };

    let block_number = log.block_number;
    let block_timestamp = timestamps::block_timestamp_for_log(&mut state.timestamps.cache, &log)?;

    let decoded = log
        .log
        .log_decode::<UntronControllerIndex::IsEventChainTipCalled>()
        .map_err(|e| anyhow::anyhow!("IsEventChainTipCalled decode failed: {e}"))?;

    Ok(ControllerTipProofRow {
        chain_id: state.chain_id,
        contract_address: contract_address_db,
        block_number: i64::try_from(block_number)
            .context("block_number out of range for bigint")?,
        block_timestamp: i64::try_from(block_timestamp)
            .context("block_timestamp out of range for bigint")?,
        block_hash: domain::BlockHash(log.block_hash),
        tx_hash: domain::TxHash(log.tx_hash),
        log_index: i32::try_from(log.log_index).context("log_index out of range for int4")?,
        caller: domain::TronAddress::from_evm(decoded.inner.data.caller),
        proved_tip: domain::Tip(decoded.inner.data.eventChainTip),
    })
}

fn u256_to_u64(value: U256) -> Result<u64> {
    u64::try_from(value).with_context(|| format!("U256 too large for u64: {value}"))
}
