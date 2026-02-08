use crate::config::UniswapV4Config;
use alloy::sol_types::{SolCall, SolValue};
use alloy::{
    primitives::{
        Address, Bytes, U256,
        aliases::{I24, U24},
    },
    providers::DynProvider,
};
use anyhow::{Context, Result};
use core::convert::TryFrom;
use std::collections::{HashMap, HashSet, VecDeque};
use uniswap_v4_sdk::prelude::{
    Actions, BestTradeOptions, HookOptions, Pool, SettleAllParams, SimpleTickDataProvider,
    TakeAllParams, Trade, V4Planner, encode_route_to_path, has_permission, has_swap_permissions,
    sdk_core::{
        entities::{BaseCurrencyCore, FractionBase},
        prelude::{BaseCurrency, BigInt, Currency, CurrencyAmount, Percent},
    },
};

type V4Pool = Pool<SimpleTickDataProvider<DynProvider>>;
const PERMIT2_ADDRESS: Address =
    alloy::primitives::address!("000000000022D473030F116dDEE9F6B43aC78BA3");
const UNIVERSAL_ROUTER_COMMAND_V4_SWAP: u8 = 0x10;
const V4_ACTION_SWAP_EXACT_IN_SINGLE: u8 = 0x06;
const V4_ACTION_SWAP_EXACT_IN: u8 = 0x07;

alloy::sol! {
    struct URPoolKey {
        address currency0;
        address currency1;
        uint24 fee;
        int24 tickSpacing;
        address hooks;
    }

    struct URPathKey {
        address intermediateCurrency;
        uint24 fee;
        int24 tickSpacing;
        address hooks;
        bytes hookData;
    }

    struct URSwapExactInSingleParams {
        URPoolKey poolKey;
        bool zeroForOne;
        uint128 amountIn;
        uint128 amountOutMinimum;
        bytes hookData;
    }

    struct URSwapExactInParams {
        address currencyIn;
        URPathKey[] path;
        uint256[] maxHopSlippage;
        uint128 amountIn;
        uint128 amountOutMinimum;
    }

    interface IUniversalRouter {
        function execute(bytes calldata commands, bytes[] calldata inputs, uint256 deadline)
            external
            payable;
    }

    #[sol(rpc)]
    interface IV4SwapRouterState {
        function poolManager() external view returns (address);
    }
}

#[derive(Clone)]
pub struct UniswapV4Client {
    swap_router: Address,
    slippage: f64,
    pools: Vec<V4Pool>,
    currency_by_address: HashMap<Address, Currency>,
    adjacency: HashMap<Address, HashSet<Address>>,
}

#[derive(Debug, Clone)]
pub struct UniswapV4Quote {
    pub approval_address: Address,
    pub to: Address,
    pub data: Bytes,
    pub value: U256,
    pub to_amount_min: U256,
}

impl UniswapV4Client {
    pub async fn new(cfg: &UniswapV4Config, chain_id: u64, provider: DynProvider) -> Result<Self> {
        let known = uniswap_v4_sdk::prelude::sdk_core::addresses::CHAIN_TO_ADDRESSES_MAP
            .get(&chain_id)
            .copied();
        let pool_manager = cfg
            .pool_manager
            .or_else(|| known.and_then(|v| v.v4_pool_manager))
            .context("missing Uniswap v4 pool manager address for chain (set UNISWAP_V4_POOL_MANAGER_ADDRESS)")?;
        let swap_router = cfg
            .swap_router
            .context("missing Uniswap swap router address (set UNISWAP_V4_SWAP_ROUTER_ADDRESS)")?;

        if known.and_then(|v| v.v4_position_manager) == Some(swap_router) {
            anyhow::bail!(
                "UNISWAP_V4_SWAP_ROUTER_ADDRESS points to the v4 PositionManager; \
                 this swap path requires a router endpoint (e.g. Universal Router), not PositionManager"
            );
        }
        let router_pool_manager = IV4SwapRouterState::new(swap_router, provider.clone())
            .poolManager()
            .call()
            .await
            .with_context(|| {
                format!(
                    "failed to read poolManager() from UNISWAP_V4_SWAP_ROUTER_ADDRESS={swap_router}"
                )
            })?;
        if router_pool_manager != pool_manager {
            anyhow::bail!(
                "UNISWAP_V4_SWAP_ROUTER_ADDRESS={swap_router} is wired to poolManager={router_pool_manager}, \
                 but relayer is configured for poolManager={pool_manager}. \
                 Use the router deployment for this chain."
            );
        }

        let mut pools = Vec::with_capacity(cfg.allowed_pools.len());
        let mut currency_by_address = HashMap::new();
        let mut adjacency: HashMap<Address, HashSet<Address>> = HashMap::new();

        for (idx, p) in cfg.allowed_pools.iter().enumerate() {
            let fee = U24::from_str_radix(&p.fee.to_string(), 10)
                .with_context(|| format!("invalid U24 fee in allowed pool[{idx}]"))?;
            let tick_spacing = I24::from_dec_str(&p.tick_spacing.to_string())
                .with_context(|| format!("invalid I24 tick_spacing in allowed pool[{idx}]"))?;
            let pool = V4Pool::from_pool_key_with_tick_data_provider(
                chain_id,
                pool_manager,
                p.currency0,
                p.currency1,
                fee,
                tick_spacing,
                p.hooks,
                provider.clone(),
                None,
            )
            .await
            .with_context(|| format!("load allowed Uniswap v4 pool[{idx}]"))?;

            let hook_before_swap = has_permission(pool.hooks, HookOptions::BeforeSwap);
            let hook_after_swap = has_permission(pool.hooks, HookOptions::AfterSwap);
            let hook_before_swap_returns_delta =
                has_permission(pool.hooks, HookOptions::BeforeSwapReturnsDelta);
            let hook_after_swap_returns_delta =
                has_permission(pool.hooks, HookOptions::AfterSwapReturnsDelta);
            let has_swap_impacting_hooks = has_swap_permissions(pool.hooks)
                || hook_before_swap_returns_delta
                || hook_after_swap_returns_delta;

            if has_swap_impacting_hooks {
                tracing::warn!(
                    hooks = %pool.hooks,
                    currency0 = %currency_address(&pool.currency0),
                    currency1 = %currency_address(&pool.currency1),
                    fee = %pool.fee,
                    tick_spacing = %pool.tick_spacing,
                    hook_before_swap,
                    hook_after_swap,
                    hook_before_swap_returns_delta,
                    hook_after_swap_returns_delta,
                    "skipping allowed Uniswap v4 pool with swap-impacting hooks (unsupported by sdk simulator)"
                );
                continue;
            }

            let a = currency_address(&pool.currency0);
            let b = currency_address(&pool.currency1);
            adjacency.entry(a).or_default().insert(b);
            adjacency.entry(b).or_default().insert(a);

            if !a.is_zero() {
                currency_by_address
                    .entry(a)
                    .or_insert_with(|| pool.currency0.clone());
            }
            if !b.is_zero() {
                currency_by_address
                    .entry(b)
                    .or_insert_with(|| pool.currency1.clone());
            }

            tracing::info!(
                pool_index = idx,
                currency0 = %a,
                currency1 = %b,
                fee = %pool.fee,
                tick_spacing = %pool.tick_spacing,
                hooks = %pool.hooks,
                liquidity = %pool.liquidity,
                "loaded allowed Uniswap v4 pool"
            );
            pools.push(pool);
        }

        if pools.is_empty() {
            anyhow::bail!(
                "no usable Uniswap v4 pools were loaded from UNISWAP_V4_ALLOWED_POOLS_JSON"
            );
        }

        Ok(Self {
            swap_router,
            slippage: cfg.slippage,
            pools,
            currency_by_address,
            adjacency,
        })
    }

    pub fn reachable_targets_from_usdt(&self, usdt: Address) -> HashSet<Address> {
        if !self.adjacency.contains_key(&usdt) {
            return HashSet::new();
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        visited.insert(usdt);
        queue.push_back(usdt);

        while let Some(cur) = queue.pop_front() {
            let Some(nexts) = self.adjacency.get(&cur) else {
                continue;
            };
            for next in nexts {
                if visited.insert(*next) {
                    queue.push_back(*next);
                }
            }
        }

        visited.remove(&usdt);
        visited.retain(|a| !a.is_zero());
        visited
    }

    pub async fn quote_usdt_to_token(
        &self,
        usdt: Address,
        target_token: Address,
        amount_usdt: U256,
    ) -> Result<UniswapV4Quote> {
        if amount_usdt.is_zero() {
            anyhow::bail!("amount_usdt is zero");
        }
        if usdt == target_token {
            anyhow::bail!("usdt and target token are identical");
        }

        let reachable = self.reachable_targets_from_usdt(usdt);
        if !reachable.contains(&target_token) {
            anyhow::bail!("target token is not reachable from USDT via allowed Uniswap v4 pools");
        }

        let currency_in = self
            .currency_by_address
            .get(&usdt)
            .cloned()
            .context("USDT not found in allowed Uniswap v4 pools")?;
        let currency_out = self
            .currency_by_address
            .get(&target_token)
            .cloned()
            .context("target token not found in allowed Uniswap v4 pools")?;

        let amount_in =
            CurrencyAmount::from_raw_amount(currency_in.clone(), u256_to_bigint(amount_usdt)?)
                .map_err(|e| anyhow::anyhow!("build exact-in amount failed: {e:?}"))?;

        let mut best: Vec<Trade<Currency, Currency, SimpleTickDataProvider<DynProvider>>> =
            Vec::new();
        Trade::<Currency, Currency, SimpleTickDataProvider<DynProvider>>::best_trade_exact_in(
            self.pools.clone(),
            &amount_in,
            &currency_out,
            BestTradeOptions {
                max_num_results: Some(1),
                max_hops: Some(3),
            },
            Vec::new(),
            None,
            &mut best,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Uniswap v4 route simulation failed: {e:?}"))?;

        let trade = best
            .into_iter()
            .next()
            .context("no Uniswap v4 route found for USDT -> target token")?;

        let slippage_tolerance = slippage_percent(self.slippage);
        let min_out_amount = trade
            .minimum_amount_out(slippage_tolerance.clone(), None)
            .map_err(|e| anyhow::anyhow!("compute minimum amount out failed: {e:?}"))?;
        let to_amount_min = bigint_to_u256(&min_out_amount.quotient())?;

        let mut planner = V4Planner::default();
        let route = trade.route();
        let amount_in_u128 = u128::try_from(amount_usdt)
            .map_err(|_| anyhow::anyhow!("amount_in does not fit uint128 for v4 router"))?;
        let amount_out_min_u128 = u128::try_from(to_amount_min)
            .map_err(|_| anyhow::anyhow!("amount_out_min does not fit uint128 for v4 router"))?;

        // Encode swap action params using current v4-router ABI to avoid SDK/router drift.
        if route.pools.len() == 1 {
            let pool = &route.pools[0];
            let currency_in_addr = currency_address(&route.path_input);
            let currency0_addr = currency_address(&pool.currency0);
            let currency1_addr = currency_address(&pool.currency1);
            let zero_for_one = currency_in_addr == currency0_addr;
            if !zero_for_one && currency_in_addr != currency1_addr {
                anyhow::bail!("route input currency does not match single-hop pool currencies");
            }

            let fee_u32: u32 = pool.fee.to::<u32>();
            let swap_single = URSwapExactInSingleParams {
                poolKey: URPoolKey {
                    currency0: currency0_addr,
                    currency1: currency1_addr,
                    fee: U24::from(fee_u32),
                    tickSpacing: pool.tick_spacing,
                    hooks: pool.hooks,
                },
                zeroForOne: zero_for_one,
                amountIn: amount_in_u128,
                amountOutMinimum: amount_out_min_u128,
                hookData: Bytes::default(),
            };
            planner.actions.push(V4_ACTION_SWAP_EXACT_IN_SINGLE);
            planner.params.push(swap_single.abi_encode().into());
        } else {
            let encoded_path = encode_route_to_path(route, false);
            let mut path = Vec::with_capacity(encoded_path.len());
            for key in encoded_path {
                let fee_u32 = u32::try_from(key.fee)
                    .map_err(|_| anyhow::anyhow!("route fee does not fit uint24"))?;
                path.push(URPathKey {
                    intermediateCurrency: key.intermediateCurrency,
                    fee: U24::from(fee_u32),
                    tickSpacing: key.tickSpacing,
                    hooks: key.hooks,
                    hookData: key.hookData,
                });
            }

            let swap_multi = URSwapExactInParams {
                currencyIn: currency_address(&route.path_input),
                path,
                maxHopSlippage: Vec::new(),
                amountIn: amount_in_u128,
                amountOutMinimum: amount_out_min_u128,
            };
            planner.actions.push(V4_ACTION_SWAP_EXACT_IN);
            planner.params.push(swap_multi.abi_encode().into());
        }

        planner.add_action(&Actions::SETTLE_ALL(SettleAllParams {
            currency: usdt,
            maxAmount: amount_usdt,
        }));
        planner.add_action(&Actions::TAKE_ALL(TakeAllParams {
            currency: target_token,
            minAmount: to_amount_min,
        }));

        let commands = Bytes::from(vec![UNIVERSAL_ROUTER_COMMAND_V4_SWAP]);
        let inputs = vec![planner.finalize()];
        let calldata = IUniversalRouter::executeCall {
            commands,
            inputs,
            deadline: U256::MAX,
        }
        .abi_encode();

        Ok(UniswapV4Quote {
            approval_address: PERMIT2_ADDRESS,
            to: self.swap_router,
            data: calldata.into(),
            value: U256::ZERO,
            to_amount_min,
        })
    }
}

fn currency_address(currency: &Currency) -> Address {
    if currency.is_native() {
        Address::ZERO
    } else {
        currency.address()
    }
}

fn slippage_percent(slippage: f64) -> Percent {
    let ppm: u32 = if !slippage.is_finite() || slippage <= 0.0 {
        0
    } else if slippage >= 1.0 {
        1_000_000
    } else {
        (slippage * 1_000_000.0).round() as u32
    };
    Percent::new(ppm, 1_000_000u32)
}

fn u256_to_bigint(value: U256) -> Result<BigInt> {
    let s = value.to_string();
    BigInt::from_str(&s).map_err(|_| anyhow::anyhow!("amount is out of BigInt range: {s}"))
}

fn bigint_to_u256(value: &BigInt) -> Result<U256> {
    let s = value.to_string();
    if s.starts_with('-') {
        anyhow::bail!("negative amount from route simulation");
    }
    U256::from_str_radix(&s, 10).with_context(|| format!("parse u256 from bigint: {s}"))
}
