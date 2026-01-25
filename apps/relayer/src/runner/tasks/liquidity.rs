use super::{HubIntent, TronIntent};
use anyhow::{Context, Result};
use tron::{TronAddress, wallet::encode_pull_from_receivers};

use crate::runner::model::{Plan, StateUpdate};
use crate::runner::util::{number_to_u256, parse_bytes32};
use crate::runner::{RelayerContext, RelayerState, Tick};
use alloy::primitives::{Address, FixedBytes, U256};
use untron_v3_indexer_client::types;

#[derive(Debug, Clone)]
pub enum LiquidityIntent {
    Hub(HubIntent),
    Tron(TronIntent),
    HubAndTron { hub: HubIntent, tron: TronIntent },
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

fn sum_unfinalized_pre_entitle_amount(
    rows: impl IntoIterator<Item = types::ReceiverUsdtTransferActionability>,
    tron_head: u64,
    finality_blocks: u64,
    block_lag: u64,
) -> Result<U256> {
    let mut sum = U256::ZERO;
    for r in rows {
        let Some(block_number) = r.block_number else {
            continue;
        };
        let Ok(block_number_u64) = u64::try_from(block_number) else {
            continue;
        };
        if super::tron_block_finalized(block_number_u64, tron_head, finality_blocks, block_lag) {
            continue;
        }
        let Some(amount) = r.amount else {
            continue;
        };
        let amt = number_to_u256(&amount)?;
        sum = sum.checked_add(amt).context("deposit sum overflow")?;
    }
    Ok(sum)
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

    let claims = ctx
        .indexer
        .hub_claims_created_for_token(usdt, ctx.cfg.jobs.fill_max_claims.max(1))
        .await?;

    let ready_claims_amt = claims
        .iter()
        .try_fold(U256::ZERO, |acc, c| -> Result<U256> {
            let amt = number_to_u256(
                c.amount_usdt
                    .as_ref()
                    .context("missing claim amount_usdt")?,
            )?;
            acc.checked_add(amt).context("claim sum overflow")
        })?;
    let has_ready_claims = !claims.is_empty();

    // Include imminent (unfinalized) deposits that should become claims via preEntitle soon.
    let pre_entitle_rows = ctx
        .indexer
        .receiver_usdt_transfer_actionability_pre_entitle(200)
        .await?;
    let pending_pre_entitle_amt = sum_unfinalized_pre_entitle_amount(
        pre_entitle_rows,
        tick.tron_head,
        ctx.cfg.jobs.tron_finality_blocks,
        ctx.cfg.tron.block_lag,
    )?;

    let projected_demand = ready_claims_amt
        .checked_add(pending_pre_entitle_amt)
        .context("projected demand overflow")?;

    if projected_demand.is_zero() {
        return Ok(Plan::none().update(StateUpdate::DelayedTronClear {
            key: "pull_from_receivers",
        }));
    }

    let usdt_balance = state.hub_usdt_balance(ctx).await?;

    let hub_intent = if has_ready_claims && usdt_balance >= ready_claims_amt {
        Some(HubIntent::FillClaims {
            target_token: usdt_addr,
            max_claims: ctx.cfg.jobs.fill_max_claims.max(1),
        })
    } else {
        None
    };

    let tron_plan = if projected_demand > usdt_balance {
        plan_pull_from_receivers(ctx, state, tick, projected_demand).await?
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

async fn plan_pull_from_receivers(
    ctx: &RelayerContext,
    state: &RelayerState,
    tick: &Tick,
    total_claims: U256,
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
    let selected = select_receiver_salts(rows, desired)?;

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
    use serde_json::Number;

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
    fn select_receiver_salts_returns_empty_for_zero_desired() {
        let rows = vec![(b32(1), U256::from(10u64))];
        let selected = select_receiver_salts(rows, U256::ZERO).unwrap();
        assert!(selected.is_empty());
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

    #[test]
    fn sum_unfinalized_pre_entitle_amount_counts_only_unfinalized() {
        fn row(block_number: i64, amount: u64) -> types::ReceiverUsdtTransferActionability {
            types::ReceiverUsdtTransferActionability {
                block_number: Some(block_number),
                amount: Some(Number::from(amount)),
                ..Default::default()
            }
        }

        // Finality rule: block_number + finality_blocks + block_lag <= head => finalized.
        let tron_head = 100;
        let finality_blocks = 10;
        let block_lag = 0;

        let rows = vec![
            row(80, 5),  // 80 + 10 <= 100 => finalized (excluded)
            row(91, 7),  // 91 + 10 > 100 => unfinalized (included)
            row(-1, 99), // out of range (skipped)
        ];

        let sum = sum_unfinalized_pre_entitle_amount(rows, tron_head, finality_blocks, block_lag)
            .unwrap();
        assert_eq!(sum, U256::from(7u64));
    }
}

pub async fn execute_liquidity_intent(
    ctx: &RelayerContext,
    state: &mut RelayerState,
    tick: &Tick,
    intent: LiquidityIntent,
) -> Result<()> {
    match intent {
        LiquidityIntent::Hub(hub) => super::hub_ops::execute_hub_intent(ctx, state, hub).await,
        LiquidityIntent::Tron(tron) => execute_pull_from_receivers(ctx, state, tick, tron).await,
        LiquidityIntent::HubAndTron { hub, tron } => {
            super::hub_ops::execute_hub_intent(ctx, state, hub).await?;
            execute_pull_from_receivers(ctx, state, tick, tron).await
        }
    }
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
