use super::HubIntent;
use anyhow::{Context, Result};
use untron_v3_bindings::untron_v3::UntronV3::{
    fillCall, preEntitleCall, processControllerEventsCall, relayControllerEventChainCall,
};

use crate::runner::model::Plan;
use crate::runner::util::{parse_bytes32, parse_txid32};
use crate::runner::{RelayerContext, RelayerState, Tick};
use alloy::{
    primitives::{FixedBytes, U256},
    sol_types::SolCall,
};
use std::time::Instant;

fn pre_entitle_finalized(
    block_number: u64,
    tron_head: u64,
    finality_blocks: u64,
    block_lag: u64,
) -> bool {
    super::tron_block_finalized(block_number, tron_head, finality_blocks, block_lag)
}

pub async fn plan_process_controller_events(ctx: &RelayerContext) -> Result<Plan<HubIntent>> {
    let hub_contract = ctx.hub_contract();
    let start = Instant::now();
    let next_idx_res = hub_contract.nextControllerEventIndex().call().await;
    ctx.telemetry.hub_rpc_ms(
        "nextControllerEventIndex",
        next_idx_res.is_ok(),
        start.elapsed().as_millis() as u64,
    );
    let next_idx = next_idx_res?;

    let start = Instant::now();
    let last_seq_res = hub_contract.lastControllerEventSeq().call().await;
    ctx.telemetry.hub_rpc_ms(
        "lastControllerEventSeq",
        last_seq_res.is_ok(),
        start.elapsed().as_millis() as u64,
    );
    let last_seq = last_seq_res?;
    if next_idx >= last_seq {
        return Ok(Plan::none());
    }
    Ok(Plan::intent(HubIntent::ProcessControllerEvents))
}

pub async fn plan_pre_entitle(ctx: &RelayerContext, tick: &Tick) -> Result<Plan<HubIntent>> {
    let rows = ctx
        .indexer
        .receiver_usdt_transfer_actionability_pre_entitle(20)
        .await?;
    if rows.is_empty() {
        return Ok(Plan::none());
    }

    let hub_contract = ctx.hub_contract();
    for row in rows.into_iter().take(20) {
        let block_number = row
            .block_number
            .context("missing receiver transfer block_number")?;
        let block_number_u64 =
            u64::try_from(block_number).context("receiver transfer block_number out of range")?;
        if !pre_entitle_finalized(
            block_number_u64,
            tick.tron_head,
            ctx.cfg.jobs.tron_finality_blocks,
            ctx.cfg.tron.block_lag,
        ) {
            continue;
        }

        let receiver_salt = parse_bytes32(
            row.receiver_salt
                .as_deref()
                .context("missing receiver_salt")?,
        )?;
        let txid_hex = row.tx_hash.as_deref().context("missing tx_hash")?;
        let txid = parse_txid32(txid_hex)?;
        let txid_b32 = FixedBytes::from_slice(&txid);

        let start = Instant::now();
        let processed_res = hub_contract.depositProcessed(txid_b32).call().await;
        ctx.telemetry.hub_rpc_ms(
            "depositProcessed",
            processed_res.is_ok(),
            start.elapsed().as_millis() as u64,
        );
        if processed_res.context("hub depositProcessed")? {
            continue;
        }

        return Ok(Plan::intent(HubIntent::PreEntitle {
            receiver_salt,
            txid,
        }));
    }

    Ok(Plan::none())
}

pub async fn execute_hub_intent(
    ctx: &RelayerContext,
    state: &mut RelayerState,
    intent: HubIntent,
) -> Result<()> {
    let name = match &intent {
        HubIntent::RelayControllerEventChain { .. } => "relayControllerEventChain",
        HubIntent::ProcessControllerEvents => "processControllerEvents",
        HubIntent::PreEntitle { .. } => "preEntitle",
        HubIntent::FillClaims { .. } => "fill",
    };

    let data = match intent {
        HubIntent::RelayControllerEventChain { proof_txid, events } => {
            let mut tron = ctx.tron_read.clone();
            let start = Instant::now();
            let bundle_res = ctx.tron_proof.build(&mut tron, proof_txid).await;
            ctx.telemetry
                .tron_proof_ms(bundle_res.is_ok(), start.elapsed().as_millis() as u64);
            let bundle = bundle_res?;
            let blocks: [alloy::primitives::Bytes; 20] =
                std::array::from_fn(|i| bundle.blocks[i].clone().into());

            relayControllerEventChainCall {
                blocks,
                encodedTx: bundle.encoded_tx.into(),
                proof: bundle.proof,
                index: bundle.index,
                events,
            }
            .abi_encode()
        }
        HubIntent::ProcessControllerEvents => processControllerEventsCall {
            maxEvents: U256::from(ctx.cfg.jobs.process_controller_max_events),
        }
        .abi_encode(),
        HubIntent::PreEntitle {
            receiver_salt,
            txid,
        } => {
            let mut tron = ctx.tron_read.clone();
            let start = Instant::now();
            let bundle_res = ctx.tron_proof.build(&mut tron, txid).await;
            ctx.telemetry
                .tron_proof_ms(bundle_res.is_ok(), start.elapsed().as_millis() as u64);
            let bundle = bundle_res?;
            let blocks: [alloy::primitives::Bytes; 20] =
                std::array::from_fn(|i| bundle.blocks[i].clone().into());
            preEntitleCall {
                receiverSalt: receiver_salt,
                blocks,
                encodedTx: bundle.encoded_tx.into(),
                proof: bundle.proof,
                index: bundle.index,
            }
            .abi_encode()
        }
        HubIntent::FillClaims {
            target_token,
            max_claims,
        } => fillCall {
            targetToken: target_token,
            maxClaims: U256::from(max_claims),
            calls: Vec::new(),
        }
        .abi_encode(),
    };

    ctx.hub.submit(state, name, data).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_entitle_finalized_boundary() {
        assert!(pre_entitle_finalized(10, 12, 1, 1));
        assert!(!pre_entitle_finalized(10, 11, 1, 1));
    }
}
