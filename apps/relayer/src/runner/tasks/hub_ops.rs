use super::HubIntent;
use anyhow::{Context, Result};
use untron_v3_bindings::untron_v3::UntronV3::{
    depositCall, fillCall, preEntitleCall, processControllerEventsCall,
    relayControllerEventChainCall, subjectivePreEntitleCall,
};

use crate::evm::{IERC20, MultiSend, MultiSendTx, encode_multisend_transactions};
use crate::indexer::RelayerHubState;
use crate::runner::model::Plan;
use crate::runner::util::{number_to_u256, parse_bytes32, parse_txid32};
use crate::runner::{RelayerContext, RelayerState, Tick};
use alloy::{
    primitives::{Address, FixedBytes, U256},
    sol_types::SolCall,
};
use std::time::Instant;
use tron::{DecodedTrc20Call, TronAddress, decode_trc20_call_data, decode_trigger_smart_contract};

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
    let start = Instant::now();
    let tron_usdt_res = hub_contract.tronUsdt().call().await;
    ctx.telemetry.hub_rpc_ms(
        "tronUsdt",
        tron_usdt_res.is_ok(),
        start.elapsed().as_millis() as u64,
    );
    let tron_usdt = tron_usdt_res.context("hub tronUsdt")?;
    if tron_usdt == Address::ZERO {
        tracing::warn!("hub tronUsdt is unset; skipping pre-entitle planning");
        return Ok(Plan::none());
    }

    let start = Instant::now();
    let controller_address_res = hub_contract.CONTROLLER_ADDRESS().call().await;
    ctx.telemetry.hub_rpc_ms(
        "CONTROLLER_ADDRESS",
        controller_address_res.is_ok(),
        start.elapsed().as_millis() as u64,
    );
    let controller_address = controller_address_res.context("hub CONTROLLER_ADDRESS")?;

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

    let mut tron = ctx.tron_read.clone();
    for row in rows.into_iter().take(20) {
        let receiver_salt_hex = row.receiver_salt.as_deref().unwrap_or_default();
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
        let processed = processed_res.context("hub depositProcessed")?;
        if processed {
            tracing::debug!(
                txid = %txid_hex,
                receiver_salt = %receiver_salt_hex,
                "pre-entitle candidate skipped (already processed on hub)"
            );
            continue;
        }

        let action = row.recommended_action.as_deref().unwrap_or("");

        let block_number_u64 = row
            .block_number
            .and_then(|n| u64::try_from(n).ok())
            .unwrap_or_default();
        let finalized_ok = pre_entitle_finalized(
            block_number_u64,
            tick.tron_head,
            ctx.cfg.jobs.tron_finality_blocks,
            ctx.cfg.tron.block_lag,
        );

        // Fetch and decode the Tron tx to explain / predict whether the hub's `preEntitle` call will
        // simulate successfully. This is intentionally verbose: it helps operators understand why
        // we pick preEntitle vs subjectivePreEntitle vs skipping (waiting for pullFromReceivers).
        let tx = match tron.get_transaction_by_id(txid).await {
            Ok(tx) => tx,
            Err(err) => {
                tracing::warn!(
                    txid = %txid_hex,
                    receiver_salt = %receiver_salt_hex,
                    err = %err,
                    "pre-entitle candidate skipped (failed to fetch tron tx)"
                );
                continue;
            }
        };
        let decoded = match decode_trigger_smart_contract(&tx) {
            Ok(d) => d,
            Err(err) => {
                tracing::info!(
                    txid = %txid_hex,
                    receiver_salt = %receiver_salt_hex,
                    recommended_action = %action,
                    err = %err,
                    "pre-entitle candidate skipped (not a TriggerSmartContract tx; hub preEntitle would revert)"
                );
                continue;
            }
        };

        let selector_hex = decoded
            .data
            .get(0..4)
            .map(hex::encode)
            .unwrap_or_else(|| "<missing>".to_string());

        let is_direct_tron_usdt_call = decoded.contract.evm() == tron_usdt;

        let decoded_trc20 = match decode_trc20_call_data(&decoded.data, decoded.owner) {
            Ok(v) => Some(v),
            Err(err) => {
                tracing::info!(
                    txid = %txid_hex,
                    receiver_salt = %receiver_salt_hex,
                    recommended_action = %action,
                    tron_owner = %decoded.owner,
                    tron_to = %decoded.contract,
                    tron_selector = %selector_hex,
                    err = %err,
                    "pre-entitle candidate not recognized as TRC-20 transfer; hub preEntitle would revert"
                );
                None
            }
        };

        let (call_kind, trc20_from, trc20_to, trc20_amount) = match decoded_trc20 {
            Some(DecodedTrc20Call::Transfer { from, to, amount }) => {
                ("transfer", Some(from), Some(to), Some(amount))
            }
            Some(DecodedTrc20Call::TransferFrom { from, to, amount }) => {
                ("transferFrom", Some(from), Some(to), Some(amount))
            }
            None => ("unknown", None, None, None),
        };

        let predicted_receiver = match hub_contract
            .predictReceiverAddress_1(controller_address, receiver_salt)
            .call()
            .await
        {
            Ok(addr) => Some(addr),
            Err(err) => {
                tracing::warn!(
                    txid = %txid_hex,
                    receiver_salt = %receiver_salt_hex,
                    err = %err,
                    "failed to predict receiver address on hub"
                );
                None
            }
        };

        let predicted_receiver_tron = predicted_receiver.map(|a| TronAddress::from_evm(a));
        let recipient_matches_receiver = predicted_receiver
            .zip(trc20_to)
            .map(|(pred, to)| pred == to.evm());

        tracing::debug!(
            txid = %txid_hex,
            receiver_salt = %receiver_salt_hex,
            recommended_action = %action,
            preentitle_time_ok = row.preentitle_time_ok,
            last_pull_timestamp = row.last_pull_timestamp,
            expected_lease_id = ?row.expected_lease_id,
            amount = ?row.amount,
            block_number = row.block_number,
            tron_head = tick.tron_head,
            finalized_ok,
            tron_owner = %decoded.owner,
            tron_to = %decoded.contract,
            tron_usdt = %TronAddress::from_evm(tron_usdt),
            is_direct_tron_usdt_call,
            tron_selector = %selector_hex,
            trc20_kind = %call_kind,
            trc20_from = ?trc20_from,
            trc20_to = ?trc20_to,
            trc20_amount = ?trc20_amount,
            predicted_receiver = ?predicted_receiver_tron,
            recipient_matches_receiver = ?recipient_matches_receiver,
            "pre-entitle candidate evaluated"
        );

        // The hub can only pre-entitle deposits that are direct calls to Tron USDT. If the USDT
        // transfer into the receiver was caused by another contract call (DEX, router, etc),
        // `preEntitle` will revert (NotTronUsdt). In that case, we should wait for finality and
        // then pull from the receiver.
        if !is_direct_tron_usdt_call {
            tracing::info!(
                txid = %txid_hex,
                receiver_salt = %receiver_salt_hex,
                tron_to = %decoded.contract,
                tron_usdt = %TronAddress::from_evm(tron_usdt),
                "pre-entitle skipped (Tron tx is not a direct call to Tron USDT); will rely on pullFromReceivers"
            );
            continue;
        }

        // If calldata doesn't decode (or doesn't target the predicted receiver), preEntitle would
        // revert; skip it to avoid noisy AA simulation failures.
        if decoded_trc20.is_none() {
            continue;
        }
        if recipient_matches_receiver == Some(false) {
            tracing::info!(
                txid = %txid_hex,
                receiver_salt = %receiver_salt_hex,
                trc20_to = ?trc20_to,
                predicted_receiver = ?predicted_receiver_tron,
                "pre-entitle skipped (Tron calldata recipient does not match predicted receiver); will rely on pullFromReceivers"
            );
            continue;
        }

        // For subjective pre-entitle we want to act as soon as we see the deposit.
        // If subjective isn't possible (missing fields / no principal), we fall back to objective,
        // which *does* wait for Tron finality.
        if action == "subjective_pre_entitle" {
            if let Some(principal) = safe_lp_principal {
                match (row.amount.as_ref(), row.expected_lease_id.as_ref()) {
                    (Some(amount), Some(lease_id_num)) => {
                        let raw_amount =
                            number_to_u256(amount).context("parse receiver transfer amount")?;
                        let lease_id =
                            number_to_u256(lease_id_num).context("parse expected_lease_id")?;

                        if principal >= raw_amount {
                            tracing::info!(
                                txid = %txid_hex,
                                receiver_salt = %receiver_salt_hex,
                                lease_id = %lease_id,
                                raw_amount = %raw_amount,
                                safe_lp_principal = %principal,
                                "pre-entitle decision: subjectivePreEntitle (principal sufficient)"
                            );
                            return Ok(Plan::intent(HubIntent::SubjectivePreEntitle {
                                txid,
                                lease_id,
                                raw_amount,
                            }));
                        }

                        tracing::info!(
                            txid = %txid_hex,
                            receiver_salt = %receiver_salt_hex,
                            lease_id = %lease_id,
                            raw_amount = %raw_amount,
                            safe_lp_principal = %principal,
                            "pre-entitle decision: skip subjectivePreEntitle (principal insufficient); will consider objective preEntitle"
                        );
                    }
                    (None, _) => tracing::warn!(
                        "subjective_pre_entitle row missing amount; falling back to objective preEntitle"
                    ),
                    (_, None) => tracing::warn!(
                        "subjective_pre_entitle row missing expected_lease_id; falling back to objective preEntitle"
                    ),
                }
            }
        }

        // Objective pre-entitle requires finalized Tron blocks.
        if row.block_number.is_none() {
            tracing::warn!(
                txid = %txid_hex,
                receiver_salt = %receiver_salt_hex,
                "pre-entitle candidate missing block_number; skipping"
            );
            continue;
        }
        if !finalized_ok {
            tracing::debug!(
                txid = %txid_hex,
                receiver_salt = %receiver_salt_hex,
                block_number = row.block_number,
                tron_head = tick.tron_head,
                tron_finality_blocks = ctx.cfg.jobs.tron_finality_blocks,
                tron_block_lag = ctx.cfg.tron.block_lag,
                "pre-entitle decision: wait (Tron tx not finalized yet)"
            );
            continue;
        }

        tracing::info!(
            txid = %txid_hex,
            receiver_salt = %receiver_salt_hex,
            block_number = row.block_number,
            "pre-entitle decision: preEntitle (objective proof)"
        );
        return Ok(Plan::intent(HubIntent::PreEntitle {
            receiver_salt,
            txid,
        }));
    }

    Ok(Plan::none())
}

pub async fn plan_deposit_lp(
    ctx: &RelayerContext,
    state: &mut RelayerState,
) -> Result<Plan<HubIntent>> {
    let Some(multisend) = ctx.cfg.hub.multisend else {
        // We need multisend to atomically approve + deposit from the Safe.
        return Ok(Plan::none());
    };
    let Some(safe) = ctx.cfg.hub.safe else {
        return Ok(Plan::none());
    };
    let Some(proto) = ctx.indexer.hub_protocol_config().await? else {
        return Ok(Plan::none());
    };
    let usdt_str = proto.usdt.as_deref().context("missing hub usdt")?;
    let usdt: Address = usdt_str.parse().context("invalid hub usdt address")?;

    let allowed = state
        .hub_is_lp_allowed(ctx, safe)
        .await
        .context("hub isLpAllowed")?;
    if !allowed {
        return Ok(Plan::none());
    }

    let bal = state
        .hub_safe_erc20_balance_of(ctx, usdt, safe)
        .await
        .context("hub usdt balanceOf(safe)")?;
    if bal.is_zero() {
        return Ok(Plan::none());
    }

    // Keep multisend referenced so the planner doesn't ignore configuration.
    let _ = multisend;
    Ok(Plan::intent(HubIntent::DepositLp { usdt, amount: bal }))
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
        HubIntent::DepositLp { .. } => "depositLp",
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
        HubIntent::DepositLp { usdt, amount } => {
            let multisend = ctx
                .cfg
                .hub
                .multisend
                .context("depositLp requires HUB_MULTISEND_ADDRESS")?;

            let approve0 = IERC20::approveCall {
                spender: ctx.hub_contract_address,
                amount: U256::ZERO,
            }
            .abi_encode();
            let approve_amt = IERC20::approveCall {
                spender: ctx.hub_contract_address,
                amount,
            }
            .abi_encode();
            let deposit_data = depositCall { amount }.abi_encode();

            let txs = vec![
                MultiSendTx {
                    operation: 0,
                    to: usdt,
                    value: U256::ZERO,
                    data: approve0.into(),
                },
                MultiSendTx {
                    operation: 0,
                    to: usdt,
                    value: U256::ZERO,
                    data: approve_amt.into(),
                },
                MultiSendTx {
                    operation: 0,
                    to: ctx.hub_contract_address,
                    value: U256::ZERO,
                    data: deposit_data.into(),
                },
            ];
            let packed = encode_multisend_transactions(&txs);
            let multisend_data = MultiSend::multiSendCall {
                transactions: packed.into(),
            }
            .abi_encode();

            (multisend, 1u8, multisend_data)
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
