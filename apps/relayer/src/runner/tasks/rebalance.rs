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
use alloy::primitives::U256;

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

fn apply_priority_limits(
    priority: &[TronAddress],
    limits: &[U256],
    in_amount: U256,
) -> Vec<TronAddress> {
    if priority.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    for (i, &addr) in priority.iter().enumerate() {
        let limit = limits.get(i).copied().unwrap_or(U256::ZERO);
        // limit=0 means "always preferred".
        if limit.is_zero() || in_amount <= limit {
            out.push(addr);
        }
    }
    out
}

pub async fn plan_controller_rebalance(
    ctx: &RelayerContext,
    state: &mut RelayerState,
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

    // Single-flight: if a prior rebalance is still "in flight" and we haven't observed its effect,
    // don't spam additional rebalance transactions.
    const REBALANCE_IN_FLIGHT_TIMEOUT_BLOCKS: u64 = 40;
    if let Some(lock) = state.rebalance_in_flight {
        // Clear the lock once we observe the controller balance drop by (roughly) the expected amount.
        let expected_post = lock.pre_balance.saturating_sub(lock.in_amount);
        let epsilon = U256::from(1u64);
        let effect_observed = balance <= expected_post.saturating_add(epsilon);

        if effect_observed {
            tracing::info!(
                txid = %hex::encode(lock.txid),
                pre_balance = %lock.pre_balance,
                in_amount = %lock.in_amount,
                balance_now = %balance,
                "rebalance effect observed; clearing in-flight lock"
            );
            state.rebalance_in_flight = None;
        } else {
            let timeout_at = lock
                .sent_at_tron_head
                .saturating_add(REBALANCE_IN_FLIGHT_TIMEOUT_BLOCKS);
            if tick.tron_head < timeout_at {
                tracing::debug!(
                    txid = %hex::encode(lock.txid),
                    sent_at_tron_head = lock.sent_at_tron_head,
                    tron_head = tick.tron_head,
                    pre_balance = %lock.pre_balance,
                    in_amount = %lock.in_amount,
                    balance_now = %balance,
                    "rebalance in-flight; skipping new rebalanceUsdt"
                );
                return Ok(Plan::none());
            }

            tracing::warn!(
                txid = %hex::encode(lock.txid),
                sent_at_tron_head = lock.sent_at_tron_head,
                tron_head = tick.tron_head,
                pre_balance = %lock.pre_balance,
                in_amount = %lock.in_amount,
                balance_now = %balance,
                "rebalance in-flight timed out; clearing lock and allowing retry"
            );
            state.rebalance_in_flight = None;
        }
    }

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
    // Apply prioritized rebalancers, optionally capped by size.
    // Limits are positional: aligned with CONTROLLER_REBALANCE_PRIORITIZED_REBALANCERS.
    let effective_priority = apply_priority_limits(
        &ctx.cfg.jobs.controller_rebalance_prioritized_rebalancers,
        &ctx.cfg
            .jobs
            .controller_rebalance_prioritized_rebalancers_limits_usdt,
        in_amount,
    );
    parsed = order_rebalancers_by_priority(parsed, &effective_priority);

    Ok(Plan::intent(TronIntent::RebalanceUsdt {
        rebalancers: parsed,
        pre_balance: balance,
        in_amount,
    })
    .extend_updates(updates))
}

pub async fn execute_controller_rebalance(
    ctx: &RelayerContext,
    state: &mut RelayerState,
    tron_head: u64,
    intent: TronIntent,
) -> Result<()> {
    let TronIntent::RebalanceUsdt {
        rebalancers,
        pre_balance,
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
                    pre_balance = %pre_balance,
                    in_amount = %in_amount,
                    "sent rebalanceUsdt"
                );

                // Single-flight lock: do not attempt another rebalance until we observe the
                // controller balance drop (or we time out).
                state.rebalance_in_flight = Some(crate::runner::RebalanceInFlight {
                    txid,
                    sent_at_tron_head: tron_head,
                    pre_balance,
                    in_amount,
                });

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
    fn prioritized_rebalancers_can_be_capped_by_size() {
        let oneclick =
            TronAddress::parse_text("0x0000000000000000000000000000000000000001").unwrap();
        let lz = TronAddress::parse_text("0x0000000000000000000000000000000000000002").unwrap();

        let priority = vec![oneclick, lz];
        let limits = vec![U256::from(10_000u64), U256::ZERO];

        // Below cap: both are preferred in order.
        assert_eq!(
            apply_priority_limits(&priority, &limits, U256::from(9_999u64)),
            vec![oneclick, lz]
        );

        // Above cap: oneclick no longer preferred, lz still preferred.
        assert_eq!(
            apply_priority_limits(&priority, &limits, U256::from(10_001u64)),
            vec![lz]
        );
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
