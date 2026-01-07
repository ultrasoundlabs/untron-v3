mod executors;
mod model;
mod tasks;
mod util;

use crate::{config::AppConfig, indexer::IndexerApi, metrics::RelayerTelemetry};
use aa::paymaster::PaymasterService;
use aa::{
    PaymasterFinalizationMode, Safe4337UserOpSender, Safe4337UserOpSenderConfig,
    Safe4337UserOpSenderOptions,
};
use alloy::{
    primitives::{FixedBytes, U256},
    providers::{DynProvider, Provider, ProviderBuilder},
    rpc::client::{BuiltInConnectionString, RpcClient},
};
use anyhow::{Context, Result};
use std::{
    collections::{HashMap, hash_map::Entry},
    sync::Arc,
};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tracing::Instrument;
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

pub struct Relayer {
    ctx: RelayerContext,
    state: RelayerState,
}

pub struct Tick {
    pub hub_head: u64,
    pub tron_head: u64,
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

    pub tron_controller: TronAddress,
    pub tron_read: TronGrpc,
    pub tron_write: TronExecutor,
    pub tron_wallet: Arc<TronWallet>,
    pub tron_proof: Arc<TronTxProofBuilder>,
}

pub struct RelayerState {
    delayed_tron: HashMap<&'static str, u64>,
    tip_proof_resend_after: HashMap<FixedBytes<32>, u64>,
    rebalance_cursor: usize,
    energy_rental_cursor: usize,
    hub_pending_nonce: Option<U256>,
}

impl RelayerState {
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
}

impl Relayer {
    pub async fn new(cfg: AppConfig, telemetry: RelayerTelemetry) -> Result<Self> {
        let indexer = Arc::new(IndexerApi::new(
            &cfg.indexer.base_url,
            cfg.indexer.timeout,
            telemetry.clone(),
        )?);

        let transport = BuiltInConnectionString::connect(&cfg.hub.rpc_url)
            .await
            .with_context(|| format!("connect hub rpc: {}", cfg.hub.rpc_url))?;
        let client = RpcClient::builder().transport(transport, false);
        let provider = ProviderBuilder::default().connect_client(client);
        let hub_provider = DynProvider::new(provider);
        let hub_contract_address = cfg.hub.untron_v3;

        let hub_sender_cfg = Safe4337UserOpSenderConfig {
            rpc_url: cfg.hub.rpc_url.clone(),
            chain_id: cfg.hub.chain_id,
            entrypoint: cfg.hub.entrypoint,
            safe: cfg.hub.safe,
            safe_4337_module: cfg.hub.safe_4337_module,
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

        let hub_sender = Arc::new(Mutex::new(Safe4337UserOpSender::new(hub_sender_cfg).await?));
        let hub = HubExecutor::new(hub_sender, cfg.hub.untron_v3, telemetry.clone());

        let mut tron_read =
            TronGrpc::connect(&cfg.tron.grpc_url, cfg.tron.api_key.as_deref()).await?;

        let _ = tron_read.get_now_block2().await?;

        let tron_controller = TronAddress::parse_text(&cfg.tron.controller_address)
            .context("parse TRON_CONTROLLER_ADDRESS")?;
        let tron_wallet = Arc::new(TronWallet::new(cfg.tron.private_key)?);
        let tron_grpc_write = Arc::new(Mutex::new(tron_read.clone()));
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
            tron_wallet.clone(),
            fee_policy,
            energy_rental,
            telemetry.clone(),
        );
        let tron_proof = Arc::new(TronTxProofBuilder::new(cfg.jobs.tron_finality_blocks));

        Ok(Self {
            ctx: RelayerContext {
                cfg,
                telemetry,
                indexer,
                hub_provider,
                hub_contract_address,
                hub,
                tron_controller,
                tron_read,
                tron_write,
                tron_wallet,
                tron_proof,
            },
            state: RelayerState {
                delayed_tron: HashMap::new(),
                tip_proof_resend_after: HashMap::new(),
                rebalance_cursor: 0,
                energy_rental_cursor: 0,
                hub_pending_nonce: None,
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

        let ready = match self.indexer_ready(&tick).await {
            Ok(v) => v,
            Err(err) => {
                tracing::warn!(err = %err, "indexer readiness check failed; skipping tick");
                return Ok(());
            }
        };

        if !ready {
            tracing::warn!("indexer not caught up; skipping tick");
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
        let hub_span = tracing::debug_span!("hub.rpc", op = "get_block_number");
        let hub_start = std::time::Instant::now();
        let hub_head_res = async {
            self.ctx
                .hub_provider
                .get_block_number()
                .await
                .context("hub head")
        }
        .instrument(hub_span)
        .await;
        self.ctx.telemetry.hub_rpc_ms(
            "get_block_number",
            hub_head_res.is_ok(),
            hub_start.elapsed().as_millis() as u64,
        );
        let hub_head = hub_head_res?;

        let mut tron = self.ctx.tron_read.clone();
        let tron_start = std::time::Instant::now();
        let tron_head_res = tron_head_block(&mut tron).await;
        self.ctx.telemetry.tron_grpc_ms(
            "get_now_block2",
            tron_head_res.is_ok(),
            tron_start.elapsed().as_millis() as u64,
        );
        let tron_head = tron_head_res?;
        Ok(Tick {
            hub_head,
            tron_head,
        })
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
            let plan = tasks::plan_controller_rebalance(&self.ctx, &self.state, tick).await?;
            self.state.apply_updates(plan.updates);
            let Some(intent) = plan.intent else {
                return Ok(());
            };
            tasks::execute_controller_rebalance(&self.ctx, &mut self.state, intent).await
        })
        .await;
    }

    async fn plan_tick(&mut self, tick: &Tick, hub_locked: bool) -> Result<PlannedTick> {
        let (relay_plan, process_plan, pre_entitle_plan) = if hub_locked {
            tracing::warn!(
                "hub sender nonce locked (pending userop); skipping hub-dependent planning"
            );
            (
                Plan::<HubIntent>::none(),
                Plan::<HubIntent>::none(),
                Plan::<HubIntent>::none(),
            )
        } else {
            let (relay, process, pre) = tokio::join!(
                tasks::plan_relay_controller_chain(&self.ctx, tick),
                tasks::plan_process_controller_events(&self.ctx),
                tasks::plan_pre_entitle(&self.ctx, tick),
            );
            (relay?, process?, pre?)
        };

        let liquidity_plan = tasks::plan_liquidity(&self.ctx, &self.state, tick).await?;

        self.state.apply_updates(relay_plan.updates);
        self.state.apply_updates(process_plan.updates);
        self.state.apply_updates(pre_entitle_plan.updates);
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

    async fn indexer_ready(&self, tick: &Tick) -> Result<bool> {
        self.ctx.indexer.health().await?;

        let summaries = self.ctx.indexer.stream_ingest_summary().await?;
        let mut hub_ok = false;
        let mut controller_ok = false;

        for s in summaries {
            let Some(stream) = s.stream else {
                continue;
            };
            let caught_up = s.is_projection_caught_up.unwrap_or(false);
            let Some(max_block) = s.max_block_number else {
                continue;
            };
            if !caught_up {
                continue;
            }

            if matches!(
                stream,
                untron_v3_indexer_client::types::StreamIngestSummaryStream::Hub
            ) {
                let max_block_u64 =
                    u64::try_from(max_block).context("hub max_block_number out of range")?;
                let lag = tick.hub_head.saturating_sub(max_block_u64);
                hub_ok = lag <= self.ctx.cfg.indexer.max_head_lag_blocks;
            } else if matches!(
                stream,
                untron_v3_indexer_client::types::StreamIngestSummaryStream::Controller
            ) {
                let max_block_u64 =
                    u64::try_from(max_block).context("controller max_block_number out of range")?;
                let lag = tick.tron_head.saturating_sub(max_block_u64);
                controller_ok = lag <= self.ctx.cfg.indexer.max_head_lag_blocks;
            }
        }

        if !hub_ok || !controller_ok {
            return Ok(false);
        }

        let Some(rx) = self.ctx.indexer.receiver_usdt_indexer_status().await? else {
            return Ok(false);
        };

        let backfills = rx.backfill_pending_receivers.unwrap_or_default();
        if backfills != 0 {
            return Ok(false);
        }

        if let Some(tail_next) = rx.tail_next_block {
            // tail_next_block is the NEXT block to query, so it should be close to head.
            let tail_next_u64 = tail_next.max(0) as u64;
            let lag = tick.tron_head.saturating_sub(tail_next_u64);
            if lag > self.ctx.cfg.indexer.max_head_lag_blocks {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }

        Ok(true)
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
            HubIntent::FillClaims { .. } => (3, tasks::JOB_FILL_CLAIMS),
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
            rebalance_cursor: 0,
            energy_rental_cursor: 0,
            hub_pending_nonce: None,
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
