mod executors;
mod model;
mod tasks;
mod util;

use crate::uniswap_v4::UniswapV4Client;
use crate::{config::AppConfig, indexer::IndexerApi, metrics::RelayerTelemetry};
use aa::paymaster::PaymasterService;
use aa::{
    PaymasterFinalizationMode, Safe4337UserOpSender, Safe4337UserOpSenderConfig,
    Safe4337UserOpSenderOptions,
};
use alloy::{
    primitives::{FixedBytes, U256},
    providers::{DynProvider, Provider, ProviderBuilder},
};
use anyhow::{Context, Result};
use std::{
    collections::{HashMap, hash_map::Entry},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tron::{
    FeePolicy, JsonApiRentalProvider, TronAddress, TronGrpc, TronTxProofBuilder, TronWallet,
};
use untron_v3_bindings::untron_v3::UntronV3::UntronV3Instance;

use self::{
    executors::{HubExecutor, TronExecutor},
    model::{Plan, StateUpdate},
    tasks::HubIntent,
    util::{run_job, tron_head_block},
};
use futures::future::BoxFuture;

pub struct Relayer {
    ctx: RelayerContext,
    state: RelayerState,
}

pub struct Tick {
    pub tron_head: u64,
}

#[derive(Debug, Clone, Copy)]
struct IndexerTickInfo {
    ready: bool,

    hub_projection_caught_up: bool,
    controller_projection_caught_up: bool,

    hub_max_block_number: Option<u64>,
    controller_max_block_number: Option<u64>,

    hub_rpc_head_block_number: Option<u64>,
    hub_stream_lag_from_rpc_head_blocks: Option<u64>,
    controller_stream_lag_from_rpc_head_blocks: Option<u64>,

    receiver_count: u64,
    receiver_backfill_pending: u64,
    receiver_tail_next_block: Option<u64>,
    receiver_tail_lag_blocks: Option<u64>,
    allowed_receiver_tail_lag_blocks: u64,
}

struct PlannedTick {
    hub_candidates: Vec<HubCandidate>,
    pull_intent: Option<tasks::TronIntent>,
}

#[derive(Clone)]
pub struct RelayerContext {
    pub cfg: AppConfig,
    pub telemetry: RelayerTelemetry,

    pub indexer: Arc<IndexerApi>,

    pub hub_provider: DynProvider,
    pub hub_contract_address: alloy::primitives::Address,
    pub hub: HubExecutor,
    pub uniswap_v4: Option<UniswapV4Client>,

    pub tron_controller: TronAddress,
    active_tron_read_grpc: Arc<Mutex<TronGrpc>>,
    tron_read_grpc_urls: Vec<String>,
    tron_read_grpc_url_cursor: Arc<Mutex<usize>>,
    tron_read_grpc_api_key: Option<String>,
    pub tron_write: TronExecutor,
    pub tron_wallet: Arc<TronWallet>,
    pub tron_proof: Arc<TronTxProofBuilder>,
}

pub struct RelayerState {
    delayed_tron: HashMap<&'static str, u64>,
    tip_proof_resend_after: HashMap<FixedBytes<32>, u64>,
    rebalance_in_flight: Option<RebalanceInFlight>,
    rebalance_cursor: usize,
    energy_rental_cursor: usize,
    fill_cursor: usize,
    hub_pending_nonce: Option<U256>,
    hub_usdt_balance_cache: Option<HubUsdtBalanceCache>,
    hub_head_block_cache: Option<HubHeadBlockCache>,
    hub_swap_executor_cache: Option<HubSwapExecutorCache>,
    hub_safe_erc20_balance_cache: HashMap<alloy::primitives::Address, HubSafeErc20BalanceCache>,
    hub_lp_allowed_cache: Option<HubLpAllowedCache>,
}

#[derive(Debug, Clone, Copy)]
struct RebalanceInFlight {
    txid: [u8; 32],
    sent_at_tron_head: u64,
    pre_balance: U256,
    in_amount: U256,
}

#[derive(Debug, Clone, Copy)]
struct HubUsdtBalanceCache {
    balance: U256,
    fetched_at: Instant,
}

#[derive(Debug, Clone, Copy)]
struct HubHeadBlockCache {
    block_number: u64,
    fetched_at: Instant,
}

#[derive(Debug, Clone, Copy)]
struct HubSwapExecutorCache {
    swap_executor: alloy::primitives::Address,
    fetched_at: Instant,
}

#[derive(Debug, Clone, Copy)]
struct HubSafeErc20BalanceCache {
    balance: U256,
    fetched_at: Instant,
}

#[derive(Debug, Clone, Copy)]
struct HubLpAllowedCache {
    safe: alloy::primitives::Address,
    allowed: bool,
    fetched_at: Instant,
}

impl RelayerState {
    pub fn invalidate_hub_usdt_balance_cache(&mut self) {
        self.hub_usdt_balance_cache = None;
    }

    pub fn invalidate_hub_safe_erc20_balance_cache(&mut self) {
        self.hub_safe_erc20_balance_cache.clear();
    }

    pub async fn hub_usdt_balance(&mut self, ctx: &RelayerContext) -> Result<U256> {
        const HUB_USDT_BALANCE_CACHE_TTL: Duration = Duration::from_secs(30);

        if let Some(cached) = self.hub_usdt_balance_cache {
            if cached.fetched_at.elapsed() <= HUB_USDT_BALANCE_CACHE_TTL {
                return Ok(cached.balance);
            }
        }

        let hub_contract = ctx.hub_contract();
        let start = Instant::now();
        let balance_res = hub_contract.usdtBalance().call().await;
        ctx.telemetry.hub_rpc_ms(
            "usdtBalance",
            balance_res.is_ok(),
            start.elapsed().as_millis() as u64,
        );
        let balance = balance_res?;
        self.hub_usdt_balance_cache = Some(HubUsdtBalanceCache {
            balance,
            fetched_at: Instant::now(),
        });
        Ok(balance)
    }

    pub async fn hub_head_block_number(&mut self, ctx: &RelayerContext) -> Result<u64> {
        const HUB_HEAD_BLOCK_CACHE_TTL: Duration = Duration::from_secs(10);

        if let Some(cached) = self.hub_head_block_cache {
            if cached.fetched_at.elapsed() <= HUB_HEAD_BLOCK_CACHE_TTL {
                return Ok(cached.block_number);
            }
        }

        let start = Instant::now();
        let head_res = ctx.hub_provider.get_block_number().await;
        ctx.telemetry.hub_rpc_ms(
            "eth_blockNumber",
            head_res.is_ok(),
            start.elapsed().as_millis() as u64,
        );
        let head = head_res?;

        self.hub_head_block_cache = Some(HubHeadBlockCache {
            block_number: head,
            fetched_at: Instant::now(),
        });
        Ok(head)
    }

    pub async fn hub_swap_executor(
        &mut self,
        ctx: &RelayerContext,
    ) -> Result<alloy::primitives::Address> {
        const HUB_SWAP_EXECUTOR_CACHE_TTL: Duration = Duration::from_secs(60 * 60);

        if let Some(cached) = self.hub_swap_executor_cache {
            if cached.fetched_at.elapsed() <= HUB_SWAP_EXECUTOR_CACHE_TTL {
                return Ok(cached.swap_executor);
            }
        }

        let hub_contract = ctx.hub_contract();
        let start = Instant::now();
        let swap_exec_res = hub_contract.SWAP_EXECUTOR().call().await;
        ctx.telemetry.hub_rpc_ms(
            "SWAP_EXECUTOR",
            swap_exec_res.is_ok(),
            start.elapsed().as_millis() as u64,
        );
        let swap_executor = swap_exec_res?;

        self.hub_swap_executor_cache = Some(HubSwapExecutorCache {
            swap_executor,
            fetched_at: Instant::now(),
        });
        Ok(swap_executor)
    }

    pub async fn hub_is_lp_allowed(
        &mut self,
        ctx: &RelayerContext,
        safe: alloy::primitives::Address,
    ) -> Result<bool> {
        const HUB_IS_LP_ALLOWED_CACHE_TTL: Duration = Duration::from_secs(60 * 10);

        if let Some(cached) = self.hub_lp_allowed_cache {
            if cached.safe == safe && cached.fetched_at.elapsed() <= HUB_IS_LP_ALLOWED_CACHE_TTL {
                return Ok(cached.allowed);
            }
        }

        let hub_contract = ctx.hub_contract();
        let start = Instant::now();
        let allowed_res = hub_contract.isLpAllowed(safe).call().await;
        ctx.telemetry.hub_rpc_ms(
            "isLpAllowed",
            allowed_res.is_ok(),
            start.elapsed().as_millis() as u64,
        );
        let allowed = allowed_res?;

        self.hub_lp_allowed_cache = Some(HubLpAllowedCache {
            safe,
            allowed,
            fetched_at: Instant::now(),
        });
        Ok(allowed)
    }

    pub async fn hub_safe_erc20_balance_of(
        &mut self,
        ctx: &RelayerContext,
        token: alloy::primitives::Address,
        safe: alloy::primitives::Address,
    ) -> Result<U256> {
        const HUB_SAFE_ERC20_BALANCE_CACHE_TTL: Duration = Duration::from_secs(30);

        if let Some(cached) = self.hub_safe_erc20_balance_cache.get(&token).copied() {
            if cached.fetched_at.elapsed() <= HUB_SAFE_ERC20_BALANCE_CACHE_TTL {
                return Ok(cached.balance);
            }
        }

        let erc20 = crate::evm::IERC20::new(token, &ctx.hub_provider);
        let start = Instant::now();
        let bal_res = erc20.balanceOf(safe).call().await;
        ctx.telemetry.hub_rpc_ms(
            "ERC20.balanceOf",
            bal_res.is_ok(),
            start.elapsed().as_millis() as u64,
        );
        let balance = bal_res?;
        self.hub_safe_erc20_balance_cache.insert(
            token,
            HubSafeErc20BalanceCache {
                balance,
                fetched_at: Instant::now(),
            },
        );
        Ok(balance)
    }

    fn plan_tron_delay(
        &self,
        key: &'static str,
        lag: u64,
        tron_head: u64,
    ) -> (bool, Vec<StateUpdate>) {
        if lag == 0 {
            return (true, Vec::new());
        }
        match self.delayed_tron.get(key).copied() {
            None => (
                false,
                vec![StateUpdate::DelayedTronSet {
                    key,
                    until: tron_head.saturating_add(lag),
                }],
            ),
            Some(until) if tron_head < until => (false, Vec::new()),
            Some(_) => (true, vec![StateUpdate::DelayedTronClear { key }]),
        }
    }

    fn apply_updates(&mut self, updates: impl IntoIterator<Item = StateUpdate>) {
        for u in updates {
            match u {
                StateUpdate::DelayedTronSet { key, until } => match self.delayed_tron.entry(key) {
                    Entry::Vacant(v) => {
                        v.insert(until);
                    }
                    Entry::Occupied(mut o) => {
                        let cur = *o.get();
                        if until < cur {
                            o.insert(until);
                        }
                    }
                },
                StateUpdate::DelayedTronClear { key } => {
                    self.delayed_tron.remove(key);
                }
                StateUpdate::TipProofResendRemove { tip } => {
                    self.tip_proof_resend_after.remove(&tip);
                }
            }
        }
    }
}

impl RelayerContext {
    pub fn hub_contract(&self) -> UntronV3Instance<DynProvider> {
        UntronV3Instance::new(self.hub_contract_address, self.hub_provider.clone())
    }

    async fn ensure_tron_read_connected(&self, idx: usize, force_reconnect: bool) -> Result<()> {
        let len = self.tron_read_grpc_urls.len();
        if len == 0 {
            anyhow::bail!("no TRON_GRPC_URLS configured");
        }
        let idx = idx % len;

        let cur = *self.tron_read_grpc_url_cursor.lock().await;
        if idx == cur && !force_reconnect {
            return Ok(());
        }

        let url = self.tron_read_grpc_urls[idx].clone();
        let grpc = TronGrpc::connect(&url, self.tron_read_grpc_api_key.as_deref())
            .await
            .with_context(|| format!("connect TRON gRPC: {url}"))?;

        {
            let mut guard = self.active_tron_read_grpc.lock().await;
            *guard = grpc;
        }
        *self.tron_read_grpc_url_cursor.lock().await = idx;
        tracing::info!(tron_grpc = %url, "switched relayer Tron read endpoint");
        Ok(())
    }

    pub async fn with_tron_read_retry<T, F>(&self, op_name: &'static str, mut op: F) -> Result<T>
    where
        F: for<'a> FnMut(&'a mut TronGrpc) -> BoxFuture<'a, Result<T>>,
    {
        let len = self.tron_read_grpc_urls.len();
        if len == 0 {
            anyhow::bail!("no TRON_GRPC_URLS configured");
        }

        let start = *self.tron_read_grpc_url_cursor.lock().await;
        let attempts = if len == 1 { 2 } else { len };
        let mut last_err: Option<anyhow::Error> = None;

        for attempt in 0..attempts {
            let idx = (start + attempt) % len;
            let force_reconnect = len == 1 && attempt > 0;

            if let Err(err) = self.ensure_tron_read_connected(idx, force_reconnect).await {
                tracing::warn!(
                    tron_grpc = %self.tron_read_grpc_urls[idx],
                    op = op_name,
                    err = %err,
                    "failed to connect Tron gRPC endpoint"
                );
                last_err = Some(err);
                continue;
            }

            let mut grpc = self.active_tron_read_grpc.lock().await;
            match op(&mut grpc).await {
                Ok(v) => return Ok(v),
                Err(err) => {
                    tracing::warn!(
                        tron_grpc = %self.tron_read_grpc_urls[idx],
                        op = op_name,
                        err = %err,
                        "tron read operation failed; trying next endpoint"
                    );
                    last_err = Some(err);
                }
            }
        }

        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("all TRON_GRPC_URLS endpoints failed")))
    }
}

impl Relayer {
    pub async fn new(cfg: AppConfig, telemetry: RelayerTelemetry) -> Result<Self> {
        let mut cfg = cfg;
        let indexer = Arc::new(IndexerApi::new(
            &cfg.indexer.base_url,
            cfg.indexer.timeout,
            telemetry.clone(),
        )?);
        let per_try_timeout_ms: u64 = std::env::var("RPC_PER_TRY_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2_500);
        let client = untron_rpc_fallback::rpc_client_from_urls_csv(
            &cfg.hub.rpc_url,
            std::time::Duration::from_millis(per_try_timeout_ms),
        )
        .with_context(|| format!("connect hub rpc (fallback): {}", cfg.hub.rpc_url))?;
        let provider = ProviderBuilder::default().connect_client(client);
        let hub_provider = DynProvider::new(provider);
        let hub_contract_address = cfg.hub.untron_v3;
        let uniswap_v4 = if let Some(v4_cfg) = cfg.hub.uniswap_v4.as_ref() {
            let hub_chain_id = match cfg.hub.chain_id {
                Some(id) => id,
                None => {
                    let id = hub_provider.get_chain_id().await.context("eth_chainId")?;
                    cfg.hub.chain_id = Some(id);
                    id
                }
            };
            Some(
                UniswapV4Client::new(v4_cfg, hub_chain_id, hub_provider.clone())
                    .await
                    .context("init Uniswap v4 client")?,
            )
        } else {
            None
        };

        let hub_sender_cfg = Safe4337UserOpSenderConfig {
            rpc_url: cfg.hub.rpc_url.clone(),
            chain_id: cfg.hub.chain_id,
            entrypoint: cfg.hub.entrypoint,
            safe: cfg.hub.safe,
            safe_4337_module: cfg.hub.safe_4337_module,
            safe_deployment: cfg.hub.safe_deployment.clone(),
            bundler_urls: cfg.hub.bundler_urls.clone(),
            owner_private_key: cfg.hub.owner_private_key,
            paymasters: cfg
                .hub
                .paymasters
                .iter()
                .map(|pm| PaymasterService {
                    url: pm.url.clone(),
                    context: pm.context.clone(),
                })
                .collect(),
            options: Safe4337UserOpSenderOptions {
                check_bundler_entrypoints: true,
                paymaster_finalization: PaymasterFinalizationMode::SkipIfStubFinal,
            },
        };

        let hub_sender_inner = Safe4337UserOpSender::new(hub_sender_cfg).await?;
        let hub_safe = hub_sender_inner.safe_address();
        tracing::info!(safe = %hub_safe, "hub safe ready");
        cfg.hub.safe = Some(hub_safe);
        let hub_sender = Arc::new(Mutex::new(hub_sender_inner));
        let hub = HubExecutor::new(hub_sender, telemetry.clone());

        let grpc_urls = cfg.tron.grpc_urls.clone();
        let mut active_tron_read_grpc = None;
        let mut tron_read_grpc_idx = 0usize;
        let mut last_connect_err: Option<anyhow::Error> = None;
        for (idx, url) in grpc_urls.iter().enumerate() {
            match TronGrpc::connect(url, cfg.tron.api_key.as_deref()).await {
                Ok(g) => {
                    active_tron_read_grpc = Some(g);
                    tron_read_grpc_idx = idx;
                    break;
                }
                Err(err) => {
                    tracing::warn!(tron_grpc = %url, err = %err, "failed to connect Tron gRPC endpoint");
                    last_connect_err = Some(err);
                }
            }
        }
        let active_tron_read_grpc = active_tron_read_grpc.context("connect TRON gRPC")?;
        let active_tron_read_grpc = Arc::new(Mutex::new(active_tron_read_grpc));
        let tron_read_grpc_url_cursor = Arc::new(Mutex::new(tron_read_grpc_idx));
        if let Some(err) = last_connect_err {
            tracing::info!(err = %err, "using fallback Tron gRPC endpoint");
        }

        {
            let mut g = active_tron_read_grpc.lock().await;
            let _ = g.get_now_block2().await?;
        }

        let tron_controller = TronAddress::parse_text(&cfg.tron.controller_address)
            .context("parse TRON_CONTROLLER_ADDRESS")?;
        let tron_wallet = Arc::new(TronWallet::new(cfg.tron.private_key)?);
        let tron_grpc_write = {
            let g = active_tron_read_grpc.lock().await;
            Arc::new(Mutex::new(g.clone()))
        };
        let fee_policy = FeePolicy {
            // No env config: cap is effectively disabled.
            fee_limit_cap_sun: i64::MAX as u64,
            fee_limit_headroom_ppm: cfg.tron.fee_limit_headroom_ppm,
        };
        let energy_rental = cfg
            .tron
            .energy_rental_providers
            .clone()
            .into_iter()
            .map(JsonApiRentalProvider::new)
            .collect::<Vec<_>>();
        let tron_write = TronExecutor::new(
            tron_grpc_write,
            grpc_urls.clone(),
            cfg.tron.api_key.clone(),
            tron_read_grpc_idx,
            tron_wallet.clone(),
            fee_policy,
            energy_rental,
            cfg.tron.energy_rental_confirm_max_wait,
            telemetry.clone(),
        );
        let tron_proof = Arc::new(TronTxProofBuilder::new(cfg.jobs.tron_finality_blocks));
        let tron_read_grpc_api_key = cfg.tron.api_key.clone();

        Ok(Self {
            ctx: RelayerContext {
                cfg,
                telemetry,
                indexer,
                hub_provider,
                hub_contract_address,
                hub,
                uniswap_v4,
                tron_controller,
                active_tron_read_grpc,
                tron_read_grpc_urls: grpc_urls,
                tron_read_grpc_url_cursor,
                tron_read_grpc_api_key,
                tron_write,
                tron_wallet,
                tron_proof,
            },
            state: RelayerState {
                delayed_tron: HashMap::new(),
                tip_proof_resend_after: HashMap::new(),
                rebalance_in_flight: None,
                rebalance_cursor: 0,
                energy_rental_cursor: 0,
                fill_cursor: 0,
                hub_pending_nonce: None,
                hub_usdt_balance_cache: None,
                hub_head_block_cache: None,
                hub_swap_executor_cache: None,
                hub_safe_erc20_balance_cache: HashMap::new(),
                hub_lp_allowed_cache: None,
            },
        })
    }

    pub async fn run(mut self, shutdown: CancellationToken) -> Result<()> {
        let mut ticker = tokio::time::interval(self.ctx.cfg.jobs.tick_interval);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                _ = shutdown.cancelled() => {
                    tracing::info!("shutdown signal received");
                    return Ok(());
                }
                _ = ticker.tick() => {}
            }

            if let Err(err) = self.tick().await {
                tracing::error!(err = %err, "tick failed");
            }
        }
    }

    async fn tick(&mut self) -> Result<()> {
        let tick = self.collect_tick().await?;

        let hub_head = self.state.hub_head_block_number(&self.ctx).await.ok();
        let ix = match self.indexer_tick_info(&tick, hub_head).await {
            Ok(v) => v,
            Err(err) => {
                tracing::warn!(err = %err, "indexer readiness check failed; skipping tick");
                return Ok(());
            }
        };

        tracing::info!(
            tron_head = tick.tron_head,
            indexer_ready = ix.ready,
            hub_projection_caught_up = ix.hub_projection_caught_up,
            controller_projection_caught_up = ix.controller_projection_caught_up,
            hub_max_block_number = ?ix.hub_max_block_number,
            controller_max_block_number = ?ix.controller_max_block_number,
            hub_rpc_head_block_number = ?ix.hub_rpc_head_block_number,
            hub_stream_lag_from_rpc_head_blocks = ?ix.hub_stream_lag_from_rpc_head_blocks,
            controller_stream_lag_from_rpc_head_blocks = ?ix.controller_stream_lag_from_rpc_head_blocks,
            receiver_count = ix.receiver_count,
            receiver_backfill_pending = ix.receiver_backfill_pending,
            receiver_tail_next_block = ?ix.receiver_tail_next_block,
            receiver_tail_lag_blocks = ?ix.receiver_tail_lag_blocks,
            allowed_receiver_tail_lag_blocks = ix.allowed_receiver_tail_lag_blocks,
            "tick"
        );

        if !ix.ready {
            return Ok(());
        }

        let telemetry = self.ctx.telemetry.clone();

        let hub_locked = self.hub_locked_or_assume_locked().await;

        self.run_controller_tip_proof(&telemetry, &tick).await;

        let planned = self.plan_tick(&tick, hub_locked).await?;
        self.execute_hub_jobs(&telemetry, planned.hub_candidates)
            .await;
        self.execute_liquidity_tron(&telemetry, &tick, planned.pull_intent)
            .await;

        self.run_controller_rebalance(&telemetry, &tick).await;

        Ok(())
    }

    async fn collect_tick(&self) -> Result<Tick> {
        let tron_start = std::time::Instant::now();
        let tron_head_res = self
            .ctx
            .with_tron_read_retry("get_now_block2", |tron| Box::pin(tron_head_block(tron)))
            .await;
        self.ctx.telemetry.tron_grpc_ms(
            "get_now_block2",
            tron_head_res.is_ok(),
            tron_start.elapsed().as_millis() as u64,
        );
        let tron_head = tron_head_res?;
        Ok(Tick { tron_head })
    }

    async fn hub_locked_or_assume_locked(&mut self) -> bool {
        match self.hub_locked().await {
            Ok(v) => v,
            Err(err) => {
                tracing::warn!(err = %err, "failed to query hub AA nonce; skipping hub jobs");
                true
            }
        }
    }

    async fn run_controller_tip_proof(&mut self, telemetry: &RelayerTelemetry, tick: &Tick) {
        let _ = run_job(telemetry, tasks::JOB_CONTROLLER_TIP_PROOF, || async {
            let plan = tasks::plan_controller_tip_proof(&self.ctx, &self.state, tick).await?;
            self.state.apply_updates(plan.updates);
            let Some(intent) = plan.intent else {
                return Ok(());
            };
            tasks::execute_controller_tip_proof(&self.ctx, &mut self.state, intent).await
        })
        .await;
    }

    async fn run_controller_rebalance(&mut self, telemetry: &RelayerTelemetry, tick: &Tick) {
        let _ = run_job(telemetry, tasks::JOB_CONTROLLER_REBALANCE, || async {
            let plan = tasks::plan_controller_rebalance(&self.ctx, &mut self.state, tick).await?;
            self.state.apply_updates(plan.updates);
            let Some(intent) = plan.intent else {
                return Ok(());
            };
            tasks::execute_controller_rebalance(&self.ctx, &mut self.state, tick.tron_head, intent)
                .await
        })
        .await;
    }

    async fn plan_tick(&mut self, tick: &Tick, hub_locked: bool) -> Result<PlannedTick> {
        let hub_state = self.ctx.indexer.relayer_hub_state().await?;

        let (relay_plan, process_plan, pre_entitle_plan, deposit_lp_plan) = if hub_locked {
            tracing::warn!(
                "hub sender nonce locked (pending userop); skipping hub-dependent planning"
            );
            (
                Plan::<HubIntent>::none(),
                Plan::<HubIntent>::none(),
                Plan::<HubIntent>::none(),
                Plan::<HubIntent>::none(),
            )
        } else {
            let (relay, process, pre, deposit_lp) = tokio::join!(
                tasks::plan_relay_controller_chain(&self.ctx, tick, &hub_state),
                tasks::plan_process_controller_events(&self.ctx, &hub_state),
                tasks::plan_pre_entitle(&self.ctx, tick),
                tasks::plan_deposit_lp(&self.ctx, &mut self.state),
            );
            (relay?, process?, pre?, deposit_lp?)
        };

        let liquidity_plan = tasks::plan_liquidity(&self.ctx, &mut self.state, tick).await?;

        self.state.apply_updates(relay_plan.updates);
        self.state.apply_updates(process_plan.updates);
        self.state.apply_updates(pre_entitle_plan.updates);
        self.state.apply_updates(deposit_lp_plan.updates);
        self.state.apply_updates(liquidity_plan.updates);

        let mut hub_candidates = Vec::new();
        if let Some(intent) = relay_plan.intent {
            hub_candidates.push(HubCandidate::new(intent));
        }
        if let Some(intent) = process_plan.intent {
            hub_candidates.push(HubCandidate::new(intent));
        }
        if let Some(intent) = pre_entitle_plan.intent {
            hub_candidates.push(HubCandidate::new(intent));
        }
        if let Some(intent) = deposit_lp_plan.intent {
            hub_candidates.push(HubCandidate::new(intent));
        }

        let mut pull_intent: Option<tasks::TronIntent> = None;
        if let Some(liq) = liquidity_plan.intent {
            match liq {
                tasks::LiquidityIntent::Hub(h) => {
                    if hub_locked {
                        tracing::debug!(
                            "hub sender nonce locked; skipping hub-side liquidity intent"
                        );
                    } else {
                        hub_candidates.push(HubCandidate::new(h));
                    }
                }
                tasks::LiquidityIntent::Tron(t) => pull_intent = Some(t),
                tasks::LiquidityIntent::HubAndTron { hub, tron } => {
                    if hub_locked {
                        tracing::debug!(
                            "hub sender nonce locked; skipping hub-side liquidity intent"
                        );
                    } else {
                        hub_candidates.push(HubCandidate::new(hub));
                    }
                    pull_intent = Some(tron);
                }
            }
        }

        Ok(PlannedTick {
            hub_candidates,
            pull_intent,
        })
    }

    async fn execute_hub_jobs(
        &mut self,
        telemetry: &RelayerTelemetry,
        hub_candidates: Vec<HubCandidate>,
    ) {
        let Some(chosen) = choose_hub_candidate(hub_candidates) else {
            return;
        };
        let name = chosen.job_name;
        let _ = run_job(telemetry, name, || async {
            tasks::execute_hub_intent(&self.ctx, &mut self.state, chosen.intent).await
        })
        .await;
    }

    async fn execute_liquidity_tron(
        &mut self,
        telemetry: &RelayerTelemetry,
        tick: &Tick,
        intent: Option<tasks::TronIntent>,
    ) {
        let Some(intent) = intent else {
            return;
        };
        let _ = run_job(telemetry, tasks::JOB_PULL_FROM_RECEIVERS, || async {
            tasks::execute_liquidity_intent(
                &self.ctx,
                &mut self.state,
                tick,
                tasks::LiquidityIntent::Tron(intent),
            )
            .await
        })
        .await;
    }

    async fn hub_locked(&mut self) -> Result<bool> {
        let Some(pending) = self.state.hub_pending_nonce else {
            return Ok(false);
        };
        let current = self.ctx.hub.current_nonce().await?;
        if current > pending {
            self.state.hub_pending_nonce = None;
            return Ok(false);
        }
        Ok(true)
    }

    fn i64_to_u64_opt(v: Option<i64>) -> Option<u64> {
        v.and_then(|n| if n >= 0 { Some(n as u64) } else { None })
    }

    async fn indexer_tick_info(
        &self,
        tick: &Tick,
        hub_rpc_head_block_number: Option<u64>,
    ) -> Result<IndexerTickInfo> {
        self.ctx.indexer.health().await?;

        let summaries = self.ctx.indexer.stream_ingest_summary().await?;
        let mut hub_projection_caught_up = false;
        let mut controller_projection_caught_up = false;
        let mut hub_max_block_number = None;
        let mut controller_max_block_number = None;

        for s in summaries {
            let Some(stream) = s.stream else {
                continue;
            };
            let caught_up = s.is_projection_caught_up.unwrap_or(false);
            let max_block = Self::i64_to_u64_opt(s.max_block_number);

            if matches!(
                stream,
                untron_v3_indexer_client::types::StreamIngestSummaryStream::Hub
            ) {
                // `max_block_number` is the highest block that produced at least one indexed event,
                // not necessarily how far the ingester has scanned. On sparse streams, it can lag
                // far behind head even while the indexer is actively tailing.
                hub_projection_caught_up = caught_up;
                hub_max_block_number = max_block;
            } else if matches!(
                stream,
                untron_v3_indexer_client::types::StreamIngestSummaryStream::Controller
            ) {
                controller_projection_caught_up = caught_up;
                controller_max_block_number = max_block;
            }
        }

        let hub_stream_lag_from_rpc_head_blocks = hub_rpc_head_block_number
            .zip(hub_max_block_number)
            .map(|(head, max_block)| head.saturating_sub(max_block));
        let controller_stream_lag_from_rpc_head_blocks = hub_rpc_head_block_number
            .zip(controller_max_block_number)
            .map(|(head, max_block)| head.saturating_sub(max_block));

        if let Some(lag) = hub_stream_lag_from_rpc_head_blocks {
            self.ctx
                .telemetry
                .indexer_stream_head_lag_blocks("hub", lag);
        }
        if let Some(lag) = controller_stream_lag_from_rpc_head_blocks {
            self.ctx
                .telemetry
                .indexer_stream_head_lag_blocks("controller", lag);
        }

        let mut ready = hub_projection_caught_up && controller_projection_caught_up;
        if !ready {
            tracing::warn!(
                hub_projection_caught_up,
                controller_projection_caught_up,
                "indexer projections not caught up; skipping tick"
            );
        }

        let allowed_receiver_tail_lag_blocks = self.ctx.cfg.indexer.max_head_lag_blocks;

        let Some(rx) = self.ctx.indexer.receiver_usdt_indexer_status().await? else {
            tracing::warn!("indexer missing receiver_usdt_indexer_status; skipping tick");
            return Ok(IndexerTickInfo {
                ready: false,
                hub_projection_caught_up,
                controller_projection_caught_up,
                hub_max_block_number,
                controller_max_block_number,
                hub_rpc_head_block_number,
                hub_stream_lag_from_rpc_head_blocks,
                controller_stream_lag_from_rpc_head_blocks,
                receiver_count: 0,
                receiver_backfill_pending: 0,
                receiver_tail_next_block: None,
                receiver_tail_lag_blocks: None,
                allowed_receiver_tail_lag_blocks,
            });
        };

        let receiver_backfill_pending =
            Self::i64_to_u64_opt(rx.backfill_pending_receivers).unwrap_or_default();
        if receiver_backfill_pending != 0 {
            // Backfills can run for newly-discovered receivers without preventing the relayer
            // from operating on already-synced receivers. Downstream queries that depend on
            // receiver-USDT transfer state should exclude receivers that are still backfilling.
            tracing::warn!(
                backfills = receiver_backfill_pending,
                "receiver_usdt backfill pending"
            );
        }

        // If there are no tracked receivers, the receiver-usdt tailer may not advance tail_next_block.
        // In that case, treat receiver-usdt as ready once backfills are drained.
        let receiver_count = Self::i64_to_u64_opt(rx.receiver_count).unwrap_or_default();
        if receiver_count == 0 {
            return Ok(IndexerTickInfo {
                ready,
                hub_projection_caught_up,
                controller_projection_caught_up,
                hub_max_block_number,
                controller_max_block_number,
                hub_rpc_head_block_number,
                hub_stream_lag_from_rpc_head_blocks,
                controller_stream_lag_from_rpc_head_blocks,
                receiver_count,
                receiver_backfill_pending,
                receiver_tail_next_block: None,
                receiver_tail_lag_blocks: None,
                allowed_receiver_tail_lag_blocks,
            });
        }

        let Some(tail_next) = rx.tail_next_block else {
            tracing::warn!("receiver_usdt tail_next_block missing; skipping tick");
            return Ok(IndexerTickInfo {
                ready: false,
                hub_projection_caught_up,
                controller_projection_caught_up,
                hub_max_block_number,
                controller_max_block_number,
                hub_rpc_head_block_number,
                hub_stream_lag_from_rpc_head_blocks,
                controller_stream_lag_from_rpc_head_blocks,
                receiver_count,
                receiver_backfill_pending,
                receiver_tail_next_block: None,
                receiver_tail_lag_blocks: None,
                allowed_receiver_tail_lag_blocks,
            });
        };

        // tail_next_block is the NEXT block to query, so it should be close to head.
        let tail_next_u64 = tail_next.max(0) as u64;
        let lag = tick.tron_head.saturating_sub(tail_next_u64);
        self.ctx.telemetry.receiver_usdt_tail_lag_blocks(lag);
        if lag > allowed_receiver_tail_lag_blocks {
            tracing::warn!(
                lag,
                allowed = allowed_receiver_tail_lag_blocks,
                receiver_count,
                tron_head = tick.tron_head,
                tail_next_block = tail_next_u64,
                "receiver_usdt tail lag too high; skipping tick"
            );
            ready = false;
        }

        Ok(IndexerTickInfo {
            ready,
            hub_projection_caught_up,
            controller_projection_caught_up,
            hub_max_block_number,
            controller_max_block_number,
            hub_rpc_head_block_number,
            hub_stream_lag_from_rpc_head_blocks,
            controller_stream_lag_from_rpc_head_blocks,
            receiver_count,
            receiver_backfill_pending,
            receiver_tail_next_block: Some(tail_next_u64),
            receiver_tail_lag_blocks: Some(lag),
            allowed_receiver_tail_lag_blocks,
        })
    }
}

#[derive(Debug)]
struct HubCandidate {
    priority: u8,
    job_name: &'static str,
    intent: HubIntent,
}

impl HubCandidate {
    fn new(intent: HubIntent) -> Self {
        let (priority, job_name) = match &intent {
            HubIntent::RelayControllerEventChain { .. } => (0, tasks::JOB_RELAY_CONTROLLER_CHAIN),
            HubIntent::ProcessControllerEvents => (1, tasks::JOB_PROCESS_CONTROLLER_EVENTS),
            HubIntent::PreEntitle { .. } => (2, tasks::JOB_PRE_ENTITLE),
            HubIntent::SubjectivePreEntitle { .. } => (2, tasks::JOB_PRE_ENTITLE),
            HubIntent::FillClaims { .. } => (3, tasks::JOB_FILL_CLAIMS),
            HubIntent::DepositLp { .. } => (4, tasks::JOB_DEPOSIT_LP),
        };
        Self {
            priority,
            job_name,
            intent,
        }
    }
}

fn choose_hub_candidate(mut candidates: Vec<HubCandidate>) -> Option<HubCandidate> {
    if candidates.is_empty() {
        return None;
    }
    candidates.sort_by_key(|c| c.priority);
    let chosen = candidates.remove(0);
    for skipped in candidates {
        tracing::debug!(
            job = skipped.job_name,
            priority = skipped.priority,
            "hub intent ready but skipped (only one hub userop per tick)"
        );
    }
    Some(chosen)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_state() -> RelayerState {
        RelayerState {
            delayed_tron: HashMap::new(),
            tip_proof_resend_after: HashMap::new(),
            rebalance_in_flight: None,
            rebalance_cursor: 0,
            energy_rental_cursor: 0,
            fill_cursor: 0,
            hub_pending_nonce: None,
            hub_usdt_balance_cache: None,
            hub_head_block_cache: None,
            hub_swap_executor_cache: None,
            hub_safe_erc20_balance_cache: HashMap::new(),
            hub_lp_allowed_cache: None,
        }
    }

    #[test]
    fn tron_delay_plans_set_and_clear_updates() {
        let mut state = empty_state();

        let (ready, updates) = state.plan_tron_delay("k", 5, 10);
        assert!(!ready);
        assert_eq!(updates.len(), 1);
        state.apply_updates(updates);

        let (ready, updates) = state.plan_tron_delay("k", 5, 14);
        assert!(!ready);
        assert!(updates.is_empty());

        let (ready, updates) = state.plan_tron_delay("k", 5, 15);
        assert!(ready);
        assert_eq!(updates.len(), 1);
        state.apply_updates(updates);

        assert!(state.delayed_tron.get("k").is_none());
    }

    #[test]
    fn tron_delay_keeps_earliest_deadline() {
        let mut state = empty_state();

        state.apply_updates([StateUpdate::DelayedTronSet {
            key: "k",
            until: 100,
        }]);
        state.apply_updates([StateUpdate::DelayedTronSet {
            key: "k",
            until: 50,
        }]);

        assert_eq!(state.delayed_tron.get("k").copied(), Some(50));
    }

    #[test]
    fn choose_hub_candidate_picks_highest_priority() {
        let candidates = vec![
            HubCandidate::new(HubIntent::FillClaims {
                target_token: alloy::primitives::Address::ZERO,
                max_claims: 1,
                calls: Vec::new(),
                top_up_amount: alloy::primitives::U256::ZERO,
                swap_executor: alloy::primitives::Address::ZERO,
            }),
            HubCandidate::new(HubIntent::PreEntitle {
                receiver_salt: alloy::primitives::FixedBytes::ZERO,
                txid: [0u8; 32],
            }),
            HubCandidate::new(HubIntent::ProcessControllerEvents),
        ];

        let chosen = choose_hub_candidate(candidates).unwrap();
        assert_eq!(chosen.job_name, tasks::JOB_PROCESS_CONTROLLER_EVENTS);
    }
}
