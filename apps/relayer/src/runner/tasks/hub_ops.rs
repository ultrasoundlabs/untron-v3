use super::HubIntent;
use anyhow::{Context, Result};
use untron_v3_bindings::untron_v3::UntronV3::{
    fillCall, preEntitleCall, processControllerEventsCall, relayControllerEventChainCall,
    subjectivePreEntitleCall,
};

use crate::evm::{IERC20, MultiSend, MultiSendTx, encode_multisend_transactions};
use crate::indexer::RelayerHubState;
use crate::runner::model::Plan;
use crate::runner::util::{number_to_u256, parse_bytes32, parse_txid32};
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

pub async fn plan_process_controller_events(
    _ctx: &RelayerContext,
    hub_state: &RelayerHubState,
) -> Result<Plan<HubIntent>> {
    let next_idx = number_to_u256(&hub_state.next_controller_event_index)?;
    let last_seq = number_to_u256(&hub_state.last_controller_event_seq)?;
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

    let needs_subjective = rows
        .iter()
        .any(|r| r.recommended_action.as_deref() == Some("subjective_pre_entitle"));
    let safe = ctx.cfg.hub.safe;
    let safe_lp_principal = if needs_subjective {
        match safe {
            Some(safe) => {
                let start = Instant::now();
                let principal_res = hub_contract.lpPrincipal(safe).call().await;
                ctx.telemetry.hub_rpc_ms(
                    "lpPrincipal",
                    principal_res.is_ok(),
                    start.elapsed().as_millis() as u64,
                );
                Some(principal_res.context("hub lpPrincipal")?)
            }
            None => None,
        }
    } else {
        None
    };

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

        let action = row.recommended_action.as_deref().unwrap_or("");
        if action == "subjective_pre_entitle" {
            if let Some(principal) = safe_lp_principal {
                let Some(amount) = row.amount.as_ref() else {
                    tracing::warn!(
                        "subjective_pre_entitle row missing amount; falling back to preEntitle"
                    );
                    return Ok(Plan::intent(HubIntent::PreEntitle {
                        receiver_salt,
                        txid,
                    }));
                };
                let raw_amount =
                    number_to_u256(amount).context("parse receiver transfer amount")?;

                let Some(lease_id_num) = row.expected_lease_id.as_ref() else {
                    tracing::warn!(
                        "subjective_pre_entitle row missing expected_lease_id; falling back to preEntitle"
                    );
                    return Ok(Plan::intent(HubIntent::PreEntitle {
                        receiver_salt,
                        txid,
                    }));
                };
                let lease_id = number_to_u256(lease_id_num).context("parse expected_lease_id")?;

                if principal >= raw_amount {
                    return Ok(Plan::intent(HubIntent::SubjectivePreEntitle {
                        txid,
                        lease_id,
                        raw_amount,
                    }));
                }
            }

            // No (or insufficient) Safe LP principal: fall back to objective proof, same as `pre_entitle`.
            return Ok(Plan::intent(HubIntent::PreEntitle {
                receiver_salt,
                txid,
            }));
        }

        // Default: objective proof for rows that already have a subjective claim (`pre_entitle`).
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
        HubIntent::SubjectivePreEntitle { .. } => "subjectivePreEntitle",
        HubIntent::FillClaims { .. } => "fill",
    };

    let (to, operation, data) = match intent {
        HubIntent::RelayControllerEventChain { proof_txid, events } => {
            let mut tron = ctx.tron_read.clone();
            let start = Instant::now();
            let bundle_res = ctx.tron_proof.build(&mut tron, proof_txid).await;
            ctx.telemetry
                .tron_proof_ms(bundle_res.is_ok(), start.elapsed().as_millis() as u64);
            let bundle = bundle_res?;
            let blocks: [alloy::primitives::Bytes; 20] =
                std::array::from_fn(|i| bundle.blocks[i].clone().into());

            let data = relayControllerEventChainCall {
                blocks,
                encodedTx: bundle.encoded_tx.into(),
                proof: bundle.proof,
                index: bundle.index,
                events,
            }
            .abi_encode();
            (ctx.hub_contract_address, 0u8, data)
        }
        HubIntent::ProcessControllerEvents => (
            ctx.hub_contract_address,
            0u8,
            processControllerEventsCall {
                maxEvents: U256::from(ctx.cfg.jobs.process_controller_max_events),
            }
            .abi_encode(),
        ),
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
            let data = preEntitleCall {
                receiverSalt: receiver_salt,
                blocks,
                encodedTx: bundle.encoded_tx.into(),
                proof: bundle.proof,
                index: bundle.index,
            }
            .abi_encode();
            (ctx.hub_contract_address, 0u8, data)
        }
        HubIntent::SubjectivePreEntitle {
            txid,
            lease_id,
            raw_amount,
        } => {
            let txid_b32 = FixedBytes::from_slice(&txid);
            let data = subjectivePreEntitleCall {
                txId: txid_b32,
                leaseId: lease_id,
                rawAmount: raw_amount,
            }
            .abi_encode();
            (ctx.hub_contract_address, 0u8, data)
        }
        HubIntent::FillClaims {
            target_token,
            max_claims,
            calls,
            top_up_amount,
            swap_executor,
        } => {
            let fill_data = fillCall {
                targetToken: target_token,
                maxClaims: U256::from(max_claims),
                calls,
            }
            .abi_encode();

            if top_up_amount.is_zero() {
                (ctx.hub_contract_address, 0u8, fill_data)
            } else {
                let multisend = ctx
                    .cfg
                    .hub
                    .multisend
                    .context("top_up_amount set but HUB_MULTISEND_ADDRESS is not configured")?;

                let transfer_data = IERC20::transferCall {
                    to: swap_executor,
                    amount: top_up_amount,
                }
                .abi_encode();

                let txs = vec![
                    MultiSendTx {
                        operation: 0,
                        to: target_token,
                        value: U256::ZERO,
                        data: transfer_data.into(),
                    },
                    MultiSendTx {
                        operation: 0,
                        to: ctx.hub_contract_address,
                        value: U256::ZERO,
                        data: fill_data.into(),
                    },
                ];
                let packed = encode_multisend_transactions(&txs);
                let multisend_data = MultiSend::multiSendCall {
                    transactions: packed.into(),
                }
                .abi_encode();

                (multisend, 1u8, multisend_data)
            }
        }
    };

    ctx.hub.submit(state, name, to, data, operation).await
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
