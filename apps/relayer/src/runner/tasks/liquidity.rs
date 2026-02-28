use super::{HubIntent, TronIntent};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use tron::{
    TronAddress, decode_trc20_call_data, decode_trigger_smart_contract,
    wallet::encode_pull_from_receivers,
};

use crate::evm::{IAllowanceTransfer, IERC20};
use crate::runner::model::{Plan, StateUpdate};
use crate::runner::util::{number_to_u256, parse_bytes32, parse_txid32};
use crate::runner::{RelayerContext, RelayerState, Tick};
use alloy::primitives::{
    Address, FixedBytes, U256,
    aliases::{U48, U160},
};
use alloy::providers::Provider;
use alloy::sol_types::SolCall;
use untron_v3_bindings::untron_v3::UntronV3::Call as SwapCall;
use untron_v3_indexer_client::types;

#[derive(Debug, Clone)]
pub enum LiquidityIntent {
    Hub(HubIntent),
    Tron(TronIntent),
    HubAndTron { hub: HubIntent, tron: TronIntent },
}

#[derive(Clone)]
struct TokenCandidate {
    addr: Address,
    /// Exact token string to use in PostgREST filters (case-sensitive).
    filter: String,
    /// Swap rate in target token units per 1e6 USDT units.
    rate_ppm: Option<U256>,
}

fn compute_desired_liquidity(
    total_liquidity: U256,
    total_claims: U256,
    pull_liquidity_ppm: u64,
) -> Result<U256> {
    let ppm = U256::from(pull_liquidity_ppm);
    let pct_amt = total_liquidity
        .checked_mul(ppm)
        .and_then(|x| x.checked_div(U256::from(1_000_000u64)))
        .context("pct liquidity overflow")?;
    Ok(pct_amt.max(total_claims))
}

fn select_receiver_salts(
    mut rows: Vec<(FixedBytes<32>, U256)>,
    desired: U256,
) -> Result<Vec<FixedBytes<32>>> {
    if desired.is_zero() {
        return Ok(Vec::new());
    }

    rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut selected = Vec::new();
    let mut acc = U256::ZERO;
    for (salt, bal) in rows {
        if acc >= desired {
            break;
        }
        // Skip dust: receivers keep 1 unit (0.000001 USDT) by design.
        if bal <= U256::from(1u64) {
            continue;
        }
        selected.push(salt);
        acc = acc.checked_add(bal).context("acc overflow")?;
    }

    Ok(selected)
}

pub async fn plan_liquidity(
    ctx: &RelayerContext,
    state: &mut RelayerState,
    tick: &Tick,
) -> Result<Plan<LiquidityIntent>> {
    let Some(proto) = ctx.indexer.hub_protocol_config().await? else {
        return Ok(Plan::none().update(StateUpdate::DelayedTronClear {
            key: "pull_from_receivers",
        }));
    };
    let usdt = proto.usdt.as_deref().context("missing hub usdt")?;
    let usdt_addr: Address = usdt.parse().context("invalid hub usdt address")?;

    let mut candidates = Vec::new();
    candidates.push(TokenCandidate {
        addr: usdt_addr,
        filter: usdt.to_string(),
        rate_ppm: None,
    });
    let reachable_from_usdt = ctx
        .uniswap_v4
        .as_ref()
        .map(|v4| v4.reachable_targets_from_usdt(usdt_addr))
        .unwrap_or_else(HashSet::new);

    let swap_rates = ctx.indexer.hub_swap_rates().await?;
    for r in swap_rates {
        let Some(token_str) = r.target_token.as_deref() else {
            continue;
        };
        let Ok(addr) = token_str.parse::<Address>() else {
            continue;
        };
        if addr == Address::ZERO || addr == usdt_addr {
            continue;
        }
        if !reachable_from_usdt.contains(&addr) {
            continue;
        }
        let Some(rate_ppm_i64) = r.rate_ppm else {
            continue;
        };
        let Ok(rate_ppm_u64) = u64::try_from(rate_ppm_i64) else {
            continue;
        };
        if rate_ppm_u64 == 0 {
            continue;
        }

        // De-dupe by address.
        if candidates.iter().any(|c: &TokenCandidate| c.addr == addr) {
            continue;
        }
        candidates.push(TokenCandidate {
            addr,
            filter: token_str.to_string(),
            rate_ppm: Some(U256::from(rate_ppm_u64)),
        });
    }
    if candidates.len() > 1 {
        candidates[1..].sort_by(|a, b| a.addr.as_slice().cmp(b.addr.as_slice()));
    }

    let mut ready_claims_amt = U256::ZERO;
    let mut created_claims = Vec::new();
    for c in &candidates {
        let claims = ctx
            .indexer
            .hub_claims_created_for_token(&c.filter, ctx.cfg.jobs.fill_max_claims.max(1))
            .await?;
        ready_claims_amt = ready_claims_amt
            .checked_add(sum_claim_amounts_usdt(&claims)?)
            .context("claim sum overflow")?;
        created_claims.push((c.clone(), claims));
    }

    // With subjective pre-entitlement, deposits do not become claims until a sponsor creates them.
    // Liquidity planning should therefore be driven by *claims* rather than raw (possibly unfinalized)
    // receiver deposits.
    let projected_demand = ready_claims_amt;

    // If a receiver deposit is *not* directly pre-entitleable (e.g. a DEX/router call that caused a
    // USDT transfer), the hub's `preEntitle` path will never work. In that case, the relayer should
    // wait for Tron finality and then pull from the receiver so the controller event chain can
    // account for it.
    let forced_pull_salts = find_unentitleable_receiver_salts(ctx, tick).await?;

    if projected_demand.is_zero() {
        if forced_pull_salts.is_empty() {
            return Ok(Plan::none().update(StateUpdate::DelayedTronClear {
                key: "pull_from_receivers",
            }));
        }

        let tron_plan = plan_pull_specific_receivers(ctx, state, tick, &forced_pull_salts).await?;
        let intent = tron_plan.intent.map(LiquidityIntent::Tron);
        return Ok(Plan {
            intent,
            updates: tron_plan.updates,
        });
    }

    let usdt_balance = state.hub_usdt_balance(ctx).await?;

    let hub_intent = plan_hub_fill(ctx, state, usdt_addr, usdt_balance, &created_claims).await?;

    let tron_plan = if projected_demand > usdt_balance {
        plan_pull_from_receivers(ctx, state, tick, projected_demand, &forced_pull_salts).await?
    } else if !forced_pull_salts.is_empty() {
        plan_pull_specific_receivers(ctx, state, tick, &forced_pull_salts).await?
    } else {
        Plan::none().update(StateUpdate::DelayedTronClear {
            key: "pull_from_receivers",
        })
    };

    let intent = match (hub_intent, tron_plan.intent) {
        (Some(hub), Some(tron)) => Some(LiquidityIntent::HubAndTron { hub, tron }),
        (Some(hub), None) => Some(LiquidityIntent::Hub(hub)),
        (None, Some(tron)) => Some(LiquidityIntent::Tron(tron)),
        (None, None) => None,
    };

    Ok(Plan {
        intent,
        updates: tron_plan.updates,
    })
}

fn sum_claim_amounts_usdt(claims: &[types::HubClaims]) -> Result<U256> {
    claims
        .iter()
        .try_fold(U256::ZERO, |acc, c| -> Result<U256> {
            let amt = number_to_u256(
                c.amount_usdt
                    .as_ref()
                    .context("missing claim amount_usdt")?,
            )?;
            acc.checked_add(amt).context("claim sum overflow")
        })
}

fn plan_fillable_claim_batch(
    claims: &[types::HubClaims],
    available_usdt: U256,
) -> Result<(u64, U256)> {
    let mut remaining = available_usdt;
    let mut total = U256::ZERO;
    let mut count: u64 = 0;
    for c in claims {
        let amt = number_to_u256(
            c.amount_usdt
                .as_ref()
                .context("missing claim amount_usdt")?,
        )?;
        if remaining < amt {
            break;
        }
        remaining = remaining.checked_sub(amt).context("usdt underflow")?;
        total = total.checked_add(amt).context("claim sum overflow")?;
        count = count.saturating_add(1);
    }
    Ok((count, total))
}

fn compute_expected_out_total(
    claims: &[types::HubClaims],
    max_claims: u64,
    rate_ppm: U256,
) -> Result<U256> {
    if max_claims == 0 {
        return Ok(U256::ZERO);
    }
    let mut out = U256::ZERO;
    for c in claims.iter().take(max_claims as usize) {
        let amt = number_to_u256(
            c.amount_usdt
                .as_ref()
                .context("missing claim amount_usdt")?,
        )?;
        let q = amt
            .checked_mul(rate_ppm)
            .and_then(|x| x.checked_div(U256::from(1_000_000u64)))
            .context("expectedOutTotal overflow")?;
        out = out.checked_add(q).context("expectedOutTotal overflow")?;
    }
    Ok(out)
}

async fn plan_hub_fill(
    ctx: &RelayerContext,
    state: &mut RelayerState,
    usdt_addr: Address,
    usdt_balance: U256,
    created_claims: &[(TokenCandidate, Vec<types::HubClaims>)],
) -> Result<Option<HubIntent>> {
    if created_claims.is_empty() || usdt_balance.is_zero() {
        return Ok(None);
    }

    let l = created_claims.len();
    if state.fill_cursor >= l {
        state.fill_cursor = 0;
    }

    let allow_topup = ctx
        .cfg
        .hub
        .uniswap_v4
        .as_ref()
        .map(|c| c.allow_topup)
        .unwrap_or(false);

    let hub_chain_id = match ctx.cfg.hub.chain_id {
        Some(id) => id,
        None => {
            let start = std::time::Instant::now();
            let id_res = ctx.hub_provider.get_chain_id().await;
            ctx.telemetry.hub_rpc_ms(
                "eth_chainId",
                id_res.is_ok(),
                start.elapsed().as_millis() as u64,
            );
            id_res.context("eth_chainId")?
        }
    };
    let hub_contract = ctx.hub_contract();

    'candidate: for offset in 0..l {
        let idx = (state.fill_cursor + offset) % l;
        let (c, claims) = &created_claims[idx];
        if claims.is_empty() {
            continue;
        }

        let (max_claims, total_usdt) = plan_fillable_claim_batch(claims, usdt_balance)?;
        if max_claims == 0 || total_usdt.is_zero() {
            continue;
        }

        // USDT queue needs no swap calls.
        if c.addr == usdt_addr {
            state.fill_cursor = (idx + 1) % l;
            return Ok(Some(HubIntent::FillClaims {
                target_token: c.addr,
                max_claims,
                calls: Vec::new(),
                top_up_amount: U256::ZERO,
                swap_executor: Address::ZERO,
            }));
        }

        let Some(uniswap_v4) = &ctx.uniswap_v4 else {
            continue;
        };
        let Some(rate_ppm) = c.rate_ppm else {
            continue;
        };

        let swap_rate_start = std::time::Instant::now();
        let onchain_rate_res = hub_contract.swapRatePpm(c.addr).call().await;
        ctx.telemetry.hub_rpc_ms(
            "swapRatePpm",
            onchain_rate_res.is_ok(),
            swap_rate_start.elapsed().as_millis() as u64,
        );
        let onchain_rate = match onchain_rate_res {
            Ok(v) => v,
            Err(err) => {
                tracing::warn!(err = %err, token = %c.addr, "failed to query swapRatePpm; skipping token");
                continue;
            }
        };
        if onchain_rate.is_zero() {
            tracing::warn!(
                token = %c.addr,
                "on-chain swapRatePpm is zero; skipping non-USDT fill"
            );
            continue;
        }

        let mut required_remote_chains = HashSet::new();
        for claim in claims.iter().take(max_claims as usize) {
            let target_chain = claim
                .target_chain_id
                .and_then(|v| u64::try_from(v).ok())
                .unwrap_or(hub_chain_id);
            if target_chain != hub_chain_id {
                required_remote_chains.insert(target_chain);
            }
        }
        for target_chain in required_remote_chains {
            let bridger_start = std::time::Instant::now();
            let bridger_res = hub_contract
                .bridgers(c.addr, U256::from(target_chain))
                .call()
                .await;
            ctx.telemetry.hub_rpc_ms(
                "bridgers",
                bridger_res.is_ok(),
                bridger_start.elapsed().as_millis() as u64,
            );
            let bridger = match bridger_res {
                Ok(v) => v,
                Err(err) => {
                    tracing::warn!(
                        err = %err,
                        token = %c.addr,
                        target_chain,
                        "failed to query bridger; skipping token"
                    );
                    continue 'candidate;
                }
            };
            if bridger == Address::ZERO {
                tracing::warn!(
                    token = %c.addr,
                    target_chain,
                    "missing bridger for claim target chain; skipping token"
                );
                continue 'candidate;
            }
        }

        let expected_out_total = compute_expected_out_total(claims, max_claims, rate_ppm)?;
        if expected_out_total.is_zero() {
            continue;
        }

        let swap_executor = match state.hub_swap_executor(ctx).await {
            Ok(v) => v,
            Err(err) => {
                tracing::warn!(err = %err, "failed to fetch SWAP_EXECUTOR; skipping non-USDT fill");
                continue;
            }
        };

        let quote = match uniswap_v4
            .quote_usdt_to_token(usdt_addr, c.addr, total_usdt)
            .await
        {
            Ok(q) => q,
            Err(err) => {
                tracing::warn!(
                    err = %err,
                    token = %c.addr,
                    total_usdt = %total_usdt,
                    "Uniswap v4 quote failed; skipping token this tick"
                );
                continue;
            }
        };
        if !quote.value.is_zero() {
            tracing::warn!(
                token = %c.addr,
                value = %quote.value,
                "Uniswap v4 quote requires native value; SwapExecutor is not funded with ETH"
            );
            continue;
        }

        let top_up_needed = if quote.to_amount_min >= expected_out_total {
            U256::ZERO
        } else {
            expected_out_total - quote.to_amount_min
        };

        if !top_up_needed.is_zero() {
            if !allow_topup {
                tracing::warn!(
                    token = %c.addr,
                    needed = %top_up_needed,
                    "swap output below expected; top-up disabled"
                );
                continue;
            }
            let Some(multisend) = ctx.cfg.hub.multisend else {
                tracing::warn!(
                    token = %c.addr,
                    needed = %top_up_needed,
                    "swap output below expected; HUB_MULTISEND_ADDRESS not set"
                );
                continue;
            };
            let Some(safe) = ctx.cfg.hub.safe else {
                tracing::warn!(
                    token = %c.addr,
                    needed = %top_up_needed,
                    "swap output below expected; Safe address unknown"
                );
                continue;
            };

            let bal = match state.hub_safe_erc20_balance_of(ctx, c.addr, safe).await {
                Ok(v) => v,
                Err(err) => {
                    tracing::warn!(err = %err, token = %c.addr, "failed to query Safe token balance");
                    continue;
                }
            };
            if bal < top_up_needed {
                tracing::warn!(
                    token = %c.addr,
                    needed = %top_up_needed,
                    balance = %bal,
                    "swap output below expected; insufficient Safe balance to top up"
                );
                continue;
            }

            // If multisend is configured, we can atomically transfer + fill. Just keep it referenced
            // so the planner doesn't ignore the configuration.
            let _ = multisend;
        }

        let approve0 = IERC20::approveCall {
            spender: quote.approval_address,
            amount: U256::ZERO,
        }
        .abi_encode();
        let approve_amt = IERC20::approveCall {
            spender: quote.approval_address,
            amount: total_usdt,
        }
        .abi_encode();
        let total_usdt_be = total_usdt.to_be_bytes::<32>();
        if total_usdt_be[..12].iter().any(|b| *b != 0) {
            anyhow::bail!("USDT amount exceeds uint160");
        }
        let permit2_amount = U160::from_be_slice(&total_usdt_be[12..]);
        let permit2_approve = IAllowanceTransfer::approveCall {
            token: usdt_addr,
            spender: quote.to,
            amount: permit2_amount,
            expiration: U48::MAX,
        }
        .abi_encode();

        let calls = vec![
            SwapCall {
                to: usdt_addr,
                value: U256::ZERO,
                data: approve0.into(),
            },
            SwapCall {
                to: usdt_addr,
                value: U256::ZERO,
                data: approve_amt.into(),
            },
            SwapCall {
                to: quote.approval_address,
                value: U256::ZERO,
                data: permit2_approve.into(),
            },
            SwapCall {
                to: quote.to,
                value: quote.value,
                data: quote.data,
            },
        ];

        state.fill_cursor = (idx + 1) % l;
        return Ok(Some(HubIntent::FillClaims {
            target_token: c.addr,
            max_claims,
            calls,
            top_up_amount: top_up_needed,
            swap_executor,
        }));
    }

    Ok(None)
}

async fn find_unentitleable_receiver_salts(
    ctx: &RelayerContext,
    tick: &Tick,
) -> Result<Vec<FixedBytes<32>>> {
    let hub_contract = ctx.hub_contract();
    let start = std::time::Instant::now();
    let tron_usdt_res = hub_contract.tronUsdt().call().await;
    ctx.telemetry.hub_rpc_ms(
        "tronUsdt",
        tron_usdt_res.is_ok(),
        start.elapsed().as_millis() as u64,
    );
    let tron_usdt = tron_usdt_res.context("hub tronUsdt")?;
    if tron_usdt == Address::ZERO {
        return Ok(Vec::new());
    }

    let rows = ctx
        .indexer
        .receiver_usdt_transfer_actionability_pre_entitle(20)
        .await?;
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    for row in rows.into_iter().take(20) {
        let receiver_salt_hex = row.receiver_salt.as_deref().unwrap_or_default();
        let txid_hex = match row.tx_hash.as_deref() {
            Some(v) => v,
            None => continue,
        };

        let block_number = match row.block_number {
            Some(n) => u64::try_from(n).ok(),
            None => None,
        };
        let Some(block_number) = block_number else {
            continue;
        };
        if !super::tron_block_finalized(
            block_number,
            tick.tron_head,
            ctx.cfg.jobs.tron_finality_blocks,
            ctx.cfg.tron.block_lag,
        ) {
            continue;
        }
        let txid = parse_txid32(txid_hex)?;

        let tx = ctx
            .with_tron_read_retry("get_transaction_by_id", |tron| {
                Box::pin(async move { tron.get_transaction_by_id(txid).await })
            })
            .await
            .context("tron get_transaction_by_id")?;
        let decoded = match decode_trigger_smart_contract(&tx) {
            Ok(d) => d,
            Err(err) => {
                tracing::warn!(txid = %txid_hex, err = %err, "failed to decode tron tx; skipping forced pull detection for row");
                continue;
            }
        };
        let selector_hex = decoded
            .data
            .get(0..4)
            .map(hex::encode)
            .unwrap_or_else(|| "<missing>".to_string());
        let decoded_trc20 = decode_trc20_call_data(&decoded.data, decoded.owner).ok();
        if decoded.contract.evm() == tron_usdt {
            continue;
        }

        let receiver_salt = parse_bytes32(receiver_salt_hex)?;
        tracing::info!(
            txid = %txid_hex,
            receiver_salt = %receiver_salt_hex,
            recommended_action = ?row.recommended_action,
            preentitle_time_ok = row.preentitle_time_ok,
            amount = ?row.amount,
            expected_lease_id = ?row.expected_lease_id,
            tron_to = %decoded.contract,
            tron_usdt = %TronAddress::from_evm(tron_usdt),
            tron_selector = %selector_hex,
            trc20_call = ?decoded_trc20,
            "found unentitleable receiver deposit; will pullFromReceivers after finality"
        );
        return Ok(vec![receiver_salt]);
    }

    Ok(Vec::new())
}

async fn plan_pull_specific_receivers(
    ctx: &RelayerContext,
    state: &RelayerState,
    tick: &Tick,
    receiver_salts: &[FixedBytes<32>],
) -> Result<Plan<TronIntent>> {
    if receiver_salts.is_empty() {
        return Ok(Plan::none());
    }

    let Some(controller_usdt) = ctx.indexer.controller_usdt().await? else {
        return Ok(Plan::none().update(StateUpdate::DelayedTronClear {
            key: "pull_from_receivers",
        }));
    };
    let token_tron = controller_usdt
        .usdt
        .as_deref()
        .context("missing controller usdt")?;
    let token_tron = TronAddress::parse_text(token_tron).context("parse controller usdt")?;

    let (ready, updates) = state.plan_tron_delay(
        "pull_from_receivers",
        ctx.cfg.tron.block_lag,
        tick.tron_head,
    );
    if !ready {
        return Ok(Plan::none().extend_updates(updates));
    }

    Ok(Plan::intent(TronIntent::PullFromReceivers {
        token_tron,
        receiver_salts: receiver_salts.to_vec(),
    })
    .extend_updates(updates))
}

async fn plan_pull_from_receivers(
    ctx: &RelayerContext,
    state: &RelayerState,
    tick: &Tick,
    total_claims: U256,
    forced_receiver_salts: &[FixedBytes<32>],
) -> Result<Plan<TronIntent>> {
    let Some(controller_usdt) = ctx.indexer.controller_usdt().await? else {
        return Ok(Plan::none().update(StateUpdate::DelayedTronClear {
            key: "pull_from_receivers",
        }));
    };
    let token_tron = controller_usdt
        .usdt
        .as_deref()
        .context("missing controller usdt")?;
    let token_tron = TronAddress::parse_text(token_tron).context("parse controller usdt")?;

    let balances = ctx.indexer.receiver_usdt_balances().await?;
    if balances.is_empty() {
        return Ok(Plan::none().update(StateUpdate::DelayedTronClear {
            key: "pull_from_receivers",
        }));
    }

    let mut rows = Vec::new();
    let mut balance_by_salt = HashMap::new();
    let mut total_liquidity = U256::ZERO;
    for r in balances {
        let Some(salt_hex) = r.receiver_salt else {
            continue;
        };
        let Some(bal) = r.balance_amount else {
            continue;
        };
        let bal = number_to_u256(&bal)?;
        total_liquidity = total_liquidity
            .checked_add(bal)
            .context("receiver liquidity overflow")?;
        let salt = parse_bytes32(&salt_hex)?;
        balance_by_salt.insert(salt, bal);
        rows.push((salt, bal));
    }
    if total_liquidity.is_zero() {
        return Ok(Plan::none().update(StateUpdate::DelayedTronClear {
            key: "pull_from_receivers",
        }));
    }

    let desired = compute_desired_liquidity(
        total_liquidity,
        total_claims,
        ctx.cfg.jobs.pull_liquidity_ppm,
    )?;
    let mut selected = select_receiver_salts(rows, desired)?;
    for salt in forced_receiver_salts {
        if selected.iter().any(|s| s == salt) {
            continue;
        }
        selected.push(*salt);
    }
    selected.sort_by(|a, b| {
        let a_bal = balance_by_salt.get(a).copied().unwrap_or(U256::ZERO);
        let b_bal = balance_by_salt.get(b).copied().unwrap_or(U256::ZERO);
        b_bal.cmp(&a_bal).then_with(|| a.cmp(b))
    });

    if selected.is_empty() {
        return Ok(Plan::none().update(StateUpdate::DelayedTronClear {
            key: "pull_from_receivers",
        }));
    }

    let (ready, updates) = state.plan_tron_delay(
        "pull_from_receivers",
        ctx.cfg.tron.block_lag,
        tick.tron_head,
    );
    if !ready {
        return Ok(Plan::none().extend_updates(updates));
    }

    Ok(Plan::intent(TronIntent::PullFromReceivers {
        token_tron,
        receiver_salts: selected,
    })
    .extend_updates(updates))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b32(n: u8) -> FixedBytes<32> {
        FixedBytes::from([n; 32])
    }

    #[test]
    fn compute_desired_uses_max_of_claims_and_ppm_liquidity() {
        let total_liquidity = U256::from(1_000_000u64);
        let total_claims = U256::from(900_000u64);
        let desired = compute_desired_liquidity(total_liquidity, total_claims, 500_000).unwrap();
        assert_eq!(desired, U256::from(900_000u64));

        let desired = compute_desired_liquidity(total_liquidity, total_claims, 950_000).unwrap();
        assert_eq!(desired, U256::from(950_000u64));
    }

    #[test]
    fn select_receiver_salts_picks_largest_until_desired() {
        let rows = vec![
            (b32(1), U256::from(10u64)),
            (b32(2), U256::from(7u64)),
            (b32(3), U256::from(3u64)),
        ];
        let selected = select_receiver_salts(rows, U256::from(11u64)).unwrap();
        assert_eq!(selected, vec![b32(1), b32(2)]);
    }

    #[test]
    fn select_receiver_salts_skips_dust_balances() {
        let rows = vec![
            (b32(1), U256::from(1u64)),
            (b32(2), U256::from(2u64)),
            (b32(3), U256::from(0u64)),
        ];
        let selected = select_receiver_salts(rows, U256::from(2u64)).unwrap();
        assert_eq!(selected, vec![b32(2)]);
    }
}

pub async fn execute_liquidity_intent(
    ctx: &RelayerContext,
    state: &mut RelayerState,
    tick: &Tick,
    intent: LiquidityIntent,
) -> Result<()> {
    match intent {
        LiquidityIntent::Hub(hub) => {
            super::hub_ops::execute_hub_intent(ctx, state, "liquidity_hub", hub).await
        }
        LiquidityIntent::Tron(tron) => execute_pull_from_receivers(ctx, state, tick, tron).await,
        LiquidityIntent::HubAndTron { hub, tron } => {
            super::hub_ops::execute_hub_intent(ctx, state, "liquidity_hub", hub).await?;
            execute_pull_from_receivers(ctx, state, tick, tron).await
        }
    }
}

async fn cap_pull_from_receivers_by_energy_limit(
    ctx: &RelayerContext,
    token_tron: TronAddress,
    receiver_salts: &[FixedBytes<32>],
) -> Vec<FixedBytes<32>> {
    let energy_limit = ctx.cfg.tron.pull_from_receivers_energy_limit;
    if energy_limit == 0 || receiver_salts.len() <= 1 {
        return receiver_salts.to_vec();
    }

    let estimate = |count: usize| async move {
        let data = encode_pull_from_receivers(token_tron.evm(), &receiver_salts[..count]);
        ctx.tron_write
            .estimate_trigger_smart_contract_energy(ctx.tron_controller, data, 0)
            .await
    };

    let total = receiver_salts.len();
    let total_energy = match estimate(total).await {
        Ok(v) => v,
        Err(err) => {
            tracing::warn!(
                receivers = total,
                energy_limit,
                err = %err,
                "failed to estimate pullFromReceivers energy; keeping full receiver set"
            );
            return receiver_salts.to_vec();
        }
    };

    if total_energy <= energy_limit {
        return receiver_salts.to_vec();
    }

    let first_energy = match estimate(1).await {
        Ok(v) => v,
        Err(err) => {
            tracing::warn!(
                receivers = total,
                energy_limit,
                err = %err,
                "failed to estimate pullFromReceivers energy for single receiver; keeping full receiver set"
            );
            return receiver_salts.to_vec();
        }
    };

    if first_energy > energy_limit {
        tracing::warn!(
            receivers_total = total,
            receivers_selected = 1,
            estimated_energy_total = total_energy,
            estimated_energy_selected = first_energy,
            energy_limit,
            "pullFromReceivers exceeds configured energy limit even for a single receiver; sending one"
        );
        return vec![receiver_salts[0]];
    }

    let mut lo = 1usize;
    let mut hi = total.saturating_sub(1);
    let mut best = 1usize;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        match estimate(mid).await {
            Ok(mid_energy) if mid_energy <= energy_limit => {
                best = mid;
                lo = mid.saturating_add(1);
            }
            Ok(_) => {
                hi = mid.saturating_sub(1);
            }
            Err(err) => {
                tracing::warn!(
                    receivers_mid = mid,
                    energy_limit,
                    err = %err,
                    "failed to estimate pullFromReceivers energy while splitting; trying smaller receiver set"
                );
                hi = mid.saturating_sub(1);
            }
        }
    }

    if best < total {
        tracing::info!(
            receivers_total = total,
            receivers_selected = best,
            estimated_energy_total = total_energy,
            energy_limit,
            "split pullFromReceivers by configured energy limit"
        );
    }

    receiver_salts[..best].to_vec()
}

pub async fn execute_pull_from_receivers(
    ctx: &RelayerContext,
    state: &mut RelayerState,
    _tick: &Tick,
    intent: TronIntent,
) -> Result<()> {
    let TronIntent::PullFromReceivers {
        token_tron,
        receiver_salts,
    } = intent
    else {
        anyhow::bail!("execute_pull_from_receivers called with wrong intent");
    };

    if receiver_salts.is_empty() {
        return Ok(());
    }

    let receiver_salts =
        cap_pull_from_receivers_by_energy_limit(ctx, token_tron, &receiver_salts).await;
    let data = encode_pull_from_receivers(token_tron.evm(), &receiver_salts);

    let txid = ctx
        .tron_write
        .broadcast_trigger_smart_contract(state, ctx.tron_controller, data, 0)
        .await?;

    tracing::info!(
        txid = %hex::encode(txid),
        receivers = receiver_salts.len(),
        "sent pullFromReceivers"
    );
    Ok(())
}
