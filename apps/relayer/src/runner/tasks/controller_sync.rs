use super::{HubIntent, TronIntent};
use crate::tron::wallet::encode_is_event_chain_tip;
use anyhow::{Context, Result};
use untron_v3_bindings::untron_v3::UntronV3Base::ControllerEvent;

use crate::runner::model::{Plan, StateUpdate};
use crate::runner::util::{parse_bytes32, parse_hex_bytes, parse_txid32, u256_to_u64};
use crate::runner::{RelayerContext, RelayerState, Tick};

fn plan_controller_tip_proof_decision(
    tip_hex: String,
    tip: alloy::primitives::FixedBytes<32>,
    tip_block: u64,
    tron_head: u64,
    block_lag: u64,
    resend_blocks: u64,
    proof_exists: bool,
    next_ok: Option<u64>,
) -> Plan<TronIntent> {
    if proof_exists {
        return Plan::none().update(StateUpdate::TipProofResendRemove { tip });
    }

    if !super::tron_block_finalized(tip_block, tron_head, 0, block_lag) {
        return Plan::none();
    }

    if next_ok.is_some_and(|next_ok| tron_head < next_ok) {
        return Plan::none();
    }

    Plan::intent(TronIntent::ProveControllerTip {
        tip_hex,
        tip,
        next_resend_ok_at: tron_head.saturating_add(resend_blocks),
    })
}

fn decode_controller_event(
    sig_hex: &str,
    data_hex: &str,
    block_number: u64,
    block_timestamp: u64,
) -> Result<ControllerEvent> {
    let sig = parse_bytes32(sig_hex)?;
    let data = parse_hex_bytes(data_hex)?;
    Ok(ControllerEvent {
        sig,
        data: data.into(),
        blockNumber: block_number,
        blockTimestamp: block_timestamp,
    })
}

pub async fn plan_controller_tip_proof(
    ctx: &RelayerContext,
    state: &RelayerState,
    tick: &Tick,
) -> Result<Plan<TronIntent>> {
    let Some(latest) = ctx.indexer.latest_event_appended("controller").await? else {
        return Ok(Plan::none());
    };

    let tip_hex = latest.new_tip.context("missing controller new_tip")?;
    let tip_block = latest
        .block_number
        .context("missing controller block_number")?;
    let tip_block_u64 = u64::try_from(tip_block).context("controller block_number out of range")?;
    let tip = parse_bytes32(&tip_hex)?;

    let proof_exists = ctx.indexer.controller_tip_proof(&tip_hex).await?.is_some();
    let next_ok = state.tip_proof_resend_after.get(&tip).copied();

    Ok(plan_controller_tip_proof_decision(
        tip_hex,
        tip,
        tip_block_u64,
        tick.tron_head,
        ctx.cfg.tron.block_lag,
        ctx.cfg.jobs.tip_proof_resend_blocks,
        proof_exists,
        next_ok,
    ))
}

pub async fn execute_controller_tip_proof(
    ctx: &RelayerContext,
    state: &mut RelayerState,
    intent: TronIntent,
) -> Result<()> {
    let TronIntent::ProveControllerTip {
        tip_hex,
        tip,
        next_resend_ok_at,
    } = intent
    else {
        anyhow::bail!("execute_controller_tip_proof called with non-tip intent");
    };

    let data = encode_is_event_chain_tip(tip);
    let txid = ctx
        .tron_write
        .broadcast_trigger_smart_contract(ctx.tron_controller, data, 0)
        .await?;

    tracing::info!(
        txid = %hex::encode(txid),
        proved_tip = %tip_hex,
        "sent isEventChainTip"
    );
    state.tip_proof_resend_after.insert(tip, next_resend_ok_at);
    Ok(())
}

pub async fn plan_relay_controller_chain(
    ctx: &RelayerContext,
    tick: &Tick,
) -> Result<Plan<HubIntent>> {
    let Some(latest) = ctx.indexer.latest_event_appended("controller").await? else {
        return Ok(Plan::none());
    };
    let target_tip = latest.new_tip.context("missing controller new_tip")?;
    let target_seq = latest.event_seq.context("missing controller event_seq")?;
    let target_tip_b32 = parse_bytes32(&target_tip)?;

    let proof = match ctx.indexer.controller_tip_proof(&target_tip).await? {
        Some(p) => p,
        None => return Ok(Plan::none()),
    };
    let proof_block = proof.block_number.context("missing proof block_number")?;
    let proof_block_u64 = u64::try_from(proof_block).context("proof block_number out of range")?;
    if !super::tron_block_finalized(
        proof_block_u64,
        tick.tron_head,
        ctx.cfg.jobs.tron_finality_blocks,
        ctx.cfg.tron.block_lag,
    ) {
        return Ok(Plan::none());
    }

    let hub_contract = ctx.hub_contract();
    let hub_tip = hub_contract.lastControllerEventTip().call().await?;
    if hub_tip == target_tip_b32 {
        return Ok(Plan::none());
    }

    let hub_seq_u256 = hub_contract.lastControllerEventSeq().call().await?;
    let hub_seq = u256_to_u64(hub_seq_u256).context("hub lastControllerEventSeq out of range")?;
    let target_seq_u64 = u64::try_from(target_seq).context("controller event_seq out of range")?;
    if hub_seq >= target_seq_u64 {
        return Ok(Plan::none());
    }

    let to_fetch = target_seq_u64 - hub_seq;
    let hub_seq_i64 = i64::try_from(hub_seq).context("hub seq out of range")?;
    let events = ctx
        .indexer
        .controller_events_from_seq(hub_seq_i64, to_fetch)
        .await?;
    if events.len() != to_fetch as usize {
        anyhow::bail!(
            "unexpected controller event range length: expected {}, got {}",
            to_fetch,
            events.len()
        );
    }

    let mut controller_events = Vec::with_capacity(events.len());
    for ev in events {
        let block_number = u64::try_from(ev.block_number.context("missing block_number")?)
            .context("controller block_number out of range")?;
        let block_timestamp = u64::try_from(ev.block_timestamp.context("missing block_timestamp")?)
            .context("controller block_timestamp out of range")?;
        controller_events.push(decode_controller_event(
            ev.event_signature
                .as_deref()
                .context("missing event_signature")?,
            ev.abi_encoded_event_data
                .as_deref()
                .context("missing abi_encoded_event_data")?,
            block_number,
            block_timestamp,
        )?);
    }

    let proof_txid = parse_txid32(proof.tx_hash.as_deref().context("missing proof tx_hash")?)?;

    Ok(Plan::intent(HubIntent::RelayControllerEventChain {
        proof_txid,
        events: controller_events,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::FixedBytes;

    #[test]
    fn tron_block_finalized_boundary() {
        assert!(super::super::tron_block_finalized(10, 11, 0, 1));
        assert!(!super::super::tron_block_finalized(10, 10, 0, 1));
        assert!(super::super::tron_block_finalized(10, 15, 3, 2));
        assert!(!super::super::tron_block_finalized(10, 14, 3, 2));
    }

    #[test]
    fn tip_proof_decision_removes_resend_when_proof_exists() {
        let tip = FixedBytes::from([7u8; 32]);
        let plan = plan_controller_tip_proof_decision(
            "0x".to_string(),
            tip,
            1,
            100,
            0,
            10,
            true,
            Some(123),
        );
        assert!(plan.intent.is_none());
        assert_eq!(plan.updates.len(), 1);
        match &plan.updates[0] {
            StateUpdate::TipProofResendRemove { tip: t } => assert_eq!(*t, tip),
            _ => panic!("expected TipProofResendRemove"),
        }
    }

    #[test]
    fn tip_proof_decision_respects_block_lag_and_backoff() {
        let tip = FixedBytes::from([1u8; 32]);
        let plan =
            plan_controller_tip_proof_decision("t".to_string(), tip, 100, 100, 1, 10, false, None);
        assert!(plan.intent.is_none());

        let plan = plan_controller_tip_proof_decision(
            "t".to_string(),
            tip,
            99,
            100,
            1,
            10,
            false,
            Some(101),
        );
        assert!(plan.intent.is_none());

        let plan = plan_controller_tip_proof_decision(
            "t".to_string(),
            tip,
            99,
            101,
            1,
            10,
            false,
            Some(101),
        );
        assert!(plan.intent.is_some());
    }

    #[test]
    fn decode_controller_event_validates_sig_len_and_hex() {
        let ok = decode_controller_event(&format!("0x{}", "11".repeat(32)), "0x", 1, 2).unwrap();
        assert_eq!(ok.blockNumber, 1);
        assert_eq!(ok.blockTimestamp, 2);

        let err = decode_controller_event("0x11", "0x", 1, 2).unwrap_err();
        assert!(err.to_string().contains("expected 32-byte"));

        let err =
            decode_controller_event(&format!("0x{}", "11".repeat(32)), "0xzz", 1, 2).unwrap_err();
        assert!(err.to_string().contains("decode hex"));
    }
}
