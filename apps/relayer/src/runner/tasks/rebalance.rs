use super::TronIntent;
use anyhow::{Context, Result};
use std::collections::HashSet;
use tron::{
    TronAddress,
    wallet::{encode_rebalance_usdt, trc20_balance_of},
};

use crate::runner::model::{Plan, StateUpdate};
use crate::runner::util::parse_u256_decimal;
use crate::runner::{RelayerContext, RelayerState, Tick};

fn order_rebalancers_by_priority(
    available: Vec<TronAddress>,
    priority: &[TronAddress],
) -> Vec<TronAddress> {
    if priority.is_empty() || available.is_empty() {
        return available;
    }

    let available_set: HashSet<TronAddress> = available.iter().copied().collect();
    let mut out = Vec::with_capacity(available.len());
    let mut used = HashSet::new();

    for &p in priority {
        if available_set.contains(&p) && used.insert(p) {
            out.push(p);
        }
    }
    for &a in &available {
        if used.insert(a) {
            out.push(a);
        }
    }
    out
}

pub async fn plan_controller_rebalance(
    ctx: &RelayerContext,
    state: &RelayerState,
    tick: &Tick,
) -> Result<Plan<TronIntent>> {
    let Some(controller_usdt) = ctx.indexer.controller_usdt().await? else {
        return Ok(Plan::none().update(StateUpdate::DelayedTronClear {
            key: "rebalance_usdt",
        }));
    };
    let token_tron = controller_usdt
        .usdt
        .as_deref()
        .context("missing controller usdt")?;
    let token_tron = TronAddress::parse_text(token_tron).context("parse controller usdt")?;

    let mut tron = ctx.tron_read.clone();
    let balance = trc20_balance_of(
        &mut tron,
        token_tron,
        ctx.tron_controller,
        ctx.tron_wallet.address(),
    )
    .await?;

    let threshold = parse_u256_decimal(&ctx.cfg.jobs.controller_rebalance_threshold_usdt)?;
    if balance <= threshold {
        return Ok(Plan::none().update(StateUpdate::DelayedTronClear {
            key: "rebalance_usdt",
        }));
    }

    let (ready, updates) =
        state.plan_tron_delay("rebalance_usdt", ctx.cfg.tron.block_lag, tick.tron_head);
    if !ready {
        return Ok(Plan::none().extend_updates(updates));
    }

    let keep = parse_u256_decimal(&ctx.cfg.jobs.controller_rebalance_keep_usdt)?;
    let in_amount = balance.saturating_sub(keep);
    if in_amount.is_zero() {
        return Ok(Plan::none()
            .update(StateUpdate::DelayedTronClear {
                key: "rebalance_usdt",
            })
            .extend_updates(updates));
    }

    let payloads = ctx.indexer.controller_payloads().await?;
    if payloads.is_empty() {
        return Ok(Plan::none()
            .update(StateUpdate::DelayedTronClear {
                key: "rebalance_usdt",
            })
            .extend_updates(updates));
    }

    let mut rebalancers = payloads
        .into_iter()
        .filter_map(|p| p.rebalancer)
        .collect::<Vec<_>>();
    if rebalancers.is_empty() {
        return Ok(Plan::none()
            .update(StateUpdate::DelayedTronClear {
                key: "rebalance_usdt",
            })
            .extend_updates(updates));
    }
    rebalancers.sort();

    let mut parsed = Vec::new();
    let mut seen = HashSet::new();
    for s in rebalancers {
        let addr = TronAddress::parse_text(&s).with_context(|| format!("parse rebalancer: {s}"))?;
        if seen.insert(addr) {
            parsed.push(addr);
        }
    }
    parsed = order_rebalancers_by_priority(
        parsed,
        &ctx.cfg.jobs.controller_rebalance_prioritized_rebalancers,
    );

    Ok(Plan::intent(TronIntent::RebalanceUsdt {
        rebalancers: parsed,
        in_amount,
    })
    .extend_updates(updates))
}

pub async fn execute_controller_rebalance(
    ctx: &RelayerContext,
    state: &mut RelayerState,
    intent: TronIntent,
) -> Result<()> {
    let TronIntent::RebalanceUsdt {
        rebalancers,
        in_amount,
    } = intent
    else {
        anyhow::bail!("execute_controller_rebalance called with wrong intent");
    };

    if rebalancers.is_empty() {
        return Ok(());
    }

    let len = rebalancers.len();
    let start_cursor = state.rebalance_cursor;
    let order = rebalance_try_indices(start_cursor, len);

    for (attempt, idx) in order.into_iter().enumerate() {
        let reb = rebalancers[idx];
        let data = encode_rebalance_usdt(reb.evm(), in_amount);

        match ctx
            .tron_write
            .broadcast_trigger_smart_contract(state, ctx.tron_controller, data, 0)
            .await
        {
            Ok(txid) => {
                tracing::info!(
                    txid = %hex::encode(txid),
                    rebalancer = %reb,
                    in_amount = %in_amount,
                    "sent rebalanceUsdt"
                );
                state.rebalance_cursor =
                    rebalance_cursor_after_attempts(start_cursor, len, attempt + 1);
                return Ok(());
            }
            Err(err) => {
                tracing::warn!(
                    rebalancer = %reb,
                    err = %err,
                    "rebalanceUsdt failed; trying next rebalancer"
                );
                state.rebalance_cursor =
                    rebalance_cursor_after_attempts(start_cursor, len, attempt + 1);
            }
        }
    }

    Ok(())
}

fn rebalance_try_indices(start_cursor: usize, len: usize) -> Vec<usize> {
    if len == 0 {
        return Vec::new();
    }
    let start = start_cursor % len;
    (0..len).map(|i| (start + i) % len).collect()
}

fn rebalance_cursor_after_attempts(start_cursor: usize, len: usize, attempts: usize) -> usize {
    if len == 0 {
        return 0;
    }
    let start = start_cursor % len;
    (start + attempts) % len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prioritized_rebalancers_come_first() {
        let a = TronAddress::parse_text("0x0000000000000000000000000000000000000001").unwrap();
        let b = TronAddress::parse_text("0x0000000000000000000000000000000000000002").unwrap();
        let c = TronAddress::parse_text("0x0000000000000000000000000000000000000003").unwrap();

        let out = order_rebalancers_by_priority(vec![a, b, c], &[c, a]);
        assert_eq!(out, vec![c, a, b]);
    }

    #[test]
    fn rebalance_order_rotates_from_cursor() {
        assert_eq!(rebalance_try_indices(0, 3), vec![0, 1, 2]);
        assert_eq!(rebalance_try_indices(2, 3), vec![2, 0, 1]);
        assert_eq!(rebalance_try_indices(5, 3), vec![2, 0, 1]);
    }

    #[test]
    fn rebalance_cursor_advances_by_attempts() {
        assert_eq!(rebalance_cursor_after_attempts(2, 3, 1), 0);
        assert_eq!(rebalance_cursor_after_attempts(2, 3, 2), 1);
        assert_eq!(rebalance_cursor_after_attempts(2, 3, 3), 2);
        assert_eq!(rebalance_cursor_after_attempts(5, 3, 1), 0);
    }
}
