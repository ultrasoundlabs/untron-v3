use super::RelayerState;
use crate::metrics::RelayerTelemetry;
use aa::Safe4337UserOpSender;
use alloy::{
    network::EthereumWallet,
    primitives::{Address, B256, U256},
    providers::{DynProvider, Provider, ProviderBuilder},
    rpc::types::eth::transaction::{TransactionInput, TransactionRequest},
    signers::local::PrivateKeySigner,
};
use anyhow::{Context, Result};
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Mutex;
use tron::{
    ChainFees, FeeLimitPolicy, JsonApiRentalProvider, RentalContext, RentalResourceKind,
    TronAddress, TronGrpc, TronWallet, protocol::TriggerSmartContract,
};

#[derive(Clone)]
pub(crate) struct DirectHubExecutor {
    provider: DynProvider,
    from: Address,
    chain_id: u64,
}

#[derive(Clone, Copy)]
struct DirectHubTxSubmission {
    tx_hash: B256,
}

impl DirectHubExecutor {
    pub(crate) fn new(rpc_url: &str, chain_id: u64, private_key: [u8; 32]) -> Result<Self> {
        let signer =
            PrivateKeySigner::from_bytes(&private_key.into()).context("invalid direct tx private key")?;
        let from = signer.address();
        let wallet = EthereumWallet::from(signer);
        let client = untron_rpc_fallback::rpc_client_from_urls_csv(rpc_url, Duration::from_secs(4))
            .with_context(|| format!("connect hub rpc (fallback csv): {rpc_url}"))?;
        let provider = ProviderBuilder::default().wallet(wallet).connect_client(client);
        Ok(Self {
            provider: DynProvider::new(provider),
            from,
            chain_id,
        })
    }

    async fn submit_call(&self, to: Address, data: Vec<u8>) -> Result<DirectHubTxSubmission> {
        let tx = TransactionRequest {
            from: Some(self.from),
            chain_id: Some(self.chain_id),
            to: Some(to.into()),
            input: TransactionInput::new(data.into()),
            ..Default::default()
        };

        let nonce = self
            .provider
            .get_transaction_count(self.from)
            .await
            .context("eth_getTransactionCount direct hub tx")?;

        let gas_price: u128 = self
            .provider
            .get_gas_price()
            .await
            .context("eth_gasPrice direct hub tx")?;
        let priority: u128 = self
            .provider
            .get_max_priority_fee_per_gas()
            .await
            .unwrap_or(gas_price / 10);

        let gas_limit: u64 = self
            .provider
            .estimate_gas(tx.clone())
            .await
            .context("eth_estimateGas direct hub tx")?;
        let gas_limit = gas_limit.saturating_mul(12).saturating_div(10).max(100_000);

        let tx = TransactionRequest {
            nonce: Some(nonce),
            gas: Some(gas_limit),
            max_fee_per_gas: Some(gas_price),
            max_priority_fee_per_gas: Some(priority),
            ..tx
        };

        let pending = self
            .provider
            .send_transaction(tx)
            .await
            .context("send direct hub tx")?;
        let tx_hash = *pending.tx_hash();

        Ok(DirectHubTxSubmission { tx_hash })
    }

    async fn is_pending(&self, tx_hash: B256) -> Result<bool> {
        Ok(self
            .provider
            .get_transaction_receipt(tx_hash)
            .await
            .context("eth_getTransactionReceipt direct hub tx")?
            .is_none())
    }
}

fn is_userop_gas_cap_error(err: &anyhow::Error) -> bool {
    err.chain().any(|cause| {
        cause
            .to_string()
            .contains("gas limits exceed the max gas per userOp")
    })
}

#[derive(Clone)]
pub struct HubExecutor {
    sender: Arc<Mutex<Safe4337UserOpSender>>,
    direct: Option<DirectHubExecutor>,
    telemetry: RelayerTelemetry,
}

impl HubExecutor {
    pub fn new(
        sender: Arc<Mutex<Safe4337UserOpSender>>,
        direct: Option<DirectHubExecutor>,
        telemetry: RelayerTelemetry,
    ) -> Self {
        Self {
            sender,
            direct,
            telemetry,
        }
    }

    pub async fn current_nonce(&self) -> Result<U256> {
        let start = Instant::now();
        let sender = self.sender.lock().await;
        let res = sender.current_nonce().await;
        self.telemetry.hub_rpc_ms(
            "EntryPoint.getNonce",
            res.is_ok(),
            start.elapsed().as_millis() as u64,
        );
        res
    }

    pub async fn submit(
        &self,
        state: &mut RelayerState,
        job_name: &'static str,
        intent_name: &'static str,
        to: Address,
        data: Vec<u8>,
        operation: u8,
    ) -> Result<()> {
        let data_len = data.len();
        let data_selector = match data.get(0..4) {
            Some(sel) => format!("0x{}", hex::encode(sel)),
            None => "0x".to_string(),
        };

        let debug_dump_calldata = job_name == "relay_controller_chain"
            && std::env::var("RELAYER_DEBUG_DUMP_RELAY_CONTROLLER_CALLDATA")
                .map(|v| v != "0" && v.to_lowercase() != "false")
                .unwrap_or(false);

        // Preserve calldata for debug logging on failure (bundler often returns error.data=null).
        let data_for_log = if debug_dump_calldata {
            Some(data.clone())
        } else {
            None
        };
        let data_for_direct = data.clone();
        let data_for_send = data;

        let start = Instant::now();
        let submission = {
            let mut sender = self.sender.lock().await;
            let nonce_start = Instant::now();
            let nonce_res = sender.current_nonce().await;
            self.telemetry.hub_rpc_ms(
                "EntryPoint.getNonce",
                nonce_res.is_ok(),
                nonce_start.elapsed().as_millis() as u64,
            );
            let nonce = nonce_res?;

            sender
                .send_call_with_nonce_operation(nonce, to, data_for_send, operation)
                .await
        };

        match submission {
            Ok(sub) => {
                self.telemetry
                    .hub_submit_ms(job_name, true, start.elapsed().as_millis() as u64);
                self.telemetry.hub_userop_ok();
                state.invalidate_hub_usdt_balance_cache();
                state.invalidate_hub_safe_erc20_balance_cache();
                state.hub_pending_nonce = Some(sub.nonce);
                state.hub_job_on_success(job_name);
                tracing::info!(userop_hash = %sub.userop_hash, job = %job_name, intent = %intent_name, "submitted hub userop");
                Ok(())
            }
            Err(err) => {
                if job_name == "relay_controller_chain" && operation == 0 && is_userop_gas_cap_error(&err)
                {
                    if let Some(direct) = &self.direct {
                        tracing::warn!(
                            job = %job_name,
                            intent = %intent_name,
                            to = %to,
                            err = %format!("{err:#}"),
                            "AA rejected relay_controller_chain due to userop gas cap; falling back to direct hub tx"
                        );

                        match direct.submit_call(to, data_for_direct).await {
                            Ok(sub) => {
                                self.telemetry.hub_submit_ms(
                                    job_name,
                                    true,
                                    start.elapsed().as_millis() as u64,
                                );
                                state.invalidate_hub_usdt_balance_cache();
                                state.invalidate_hub_safe_erc20_balance_cache();
                                state.hub_direct_relay_pending_tx = Some(sub.tx_hash);
                                state.hub_job_on_success(job_name);
                                tracing::info!(
                                    tx_hash = %sub.tx_hash,
                                    job = %job_name,
                                    intent = %intent_name,
                                    "submitted direct hub tx fallback"
                                );
                                return Ok(());
                            }
                            Err(direct_err) => {
                                tracing::error!(
                                    job = %job_name,
                                    intent = %intent_name,
                                    to = %to,
                                    err = %format!("{direct_err:#}"),
                                    raw_err = ?direct_err,
                                    "direct hub tx fallback failed"
                                );
                            }
                        }
                    }
                }

                self.telemetry
                    .hub_submit_ms(job_name, false, start.elapsed().as_millis() as u64);
                self.telemetry.hub_userop_err();
                state.hub_job_on_failure(job_name, state.last_tron_head);

                if debug_dump_calldata {
                    // Safe4337 smart account address; useful for reproducing the call via `cast call --from`.
                    let safe = {
                        let sender = self.sender.lock().await;
                        sender.safe_address()
                    };
                    let hex0x =
                        format!("0x{}", hex::encode(data_for_log.as_deref().unwrap_or(&[])));
                    // Some log backends drop/trim very long lines; emit chunked logs.
                    tracing::error!(
                        job = %job_name,
                        intent = %intent_name,
                        from = %safe,
                        to = %to,
                        operation,
                        data_len,
                        data_selector = %data_selector,
                        calldata_len = hex0x.len(),
                        "debug: dumping failing hub calldata"
                    );
                    const CHUNK: usize = 900;
                    for (i, chunk) in hex0x.as_bytes().chunks(CHUNK).enumerate() {
                        let s = std::str::from_utf8(chunk).unwrap_or("<non-utf8>");
                        tracing::error!(
                            job = %job_name,
                            intent = %intent_name,
                            chunk_index = i,
                            chunk_total = (hex0x.len() + CHUNK - 1) / CHUNK,
                            calldata_chunk = %s,
                            "debug: hub calldata chunk"
                        );
                    }
                }

                // During incidents, we need the *full* anyhow chain + underlying RPC payloads.
                // `%format!("{err:#}")` often loses structured JSON-RPC error data.
                tracing::error!(
                    job = %job_name,
                    intent = %intent_name,
                    to = %to,
                    operation,
                    data_len,
                    data_selector = %data_selector,
                    err = %format!("{err:#}"),
                    raw_err = ?err,
                    root_cause = ?err.root_cause(),
                    "hub userop submit failed"
                );
                Err(err)
            }
        }
    }

    pub async fn relay_controller_chain_direct_pending(&self, tx_hash: B256) -> Result<bool> {
        let Some(direct) = &self.direct else {
            return Ok(false);
        };
        direct.is_pending(tx_hash).await
    }
}

#[derive(Clone)]
pub struct TronExecutor {
    active_grpc: Arc<Mutex<TronGrpc>>,
    grpc_urls: Vec<String>,
    grpc_api_key: Option<String>,
    grpc_api_key_header: String,
    grpc_url_cursor: Arc<Mutex<usize>>,
    wallet: Arc<TronWallet>,
    energy_rental: Vec<JsonApiRentalProvider>,
    energy_rental_confirm_max_wait: Duration,
    require_energy_rental: bool,
    rental_cap_per_hour: u32,
    rental_cap_per_day: u32,
    rental_cooldown_after_trip: Duration,
    write_staleness_guard: Duration,
    write_preflight_simulation: bool,
    tx_cap_per_kind_per_hour: u32,
    fee_limit_policy: FeeLimitPolicy,
    telemetry: RelayerTelemetry,
}

impl TronExecutor {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        active_grpc: Arc<Mutex<TronGrpc>>,
        grpc_urls: Vec<String>,
        grpc_api_key: Option<String>,
        grpc_api_key_header: String,
        initial_grpc_url_cursor: usize,
        wallet: Arc<TronWallet>,
        energy_rental: Vec<JsonApiRentalProvider>,
        energy_rental_confirm_max_wait: Duration,
        require_energy_rental: bool,
        rental_cap_per_hour: u32,
        rental_cap_per_day: u32,
        rental_cooldown_after_trip: Duration,
        write_staleness_guard: Duration,
        write_preflight_simulation: bool,
        tx_cap_per_kind_per_hour: u32,
        chain_fees: ChainFees,
        fee_limit_headroom_ppm: u64,
        fee_limit_ceiling_sun: u64,
        telemetry: RelayerTelemetry,
    ) -> Self {
        Self {
            active_grpc,
            grpc_urls,
            grpc_api_key,
            grpc_api_key_header,
            grpc_url_cursor: Arc::new(Mutex::new(initial_grpc_url_cursor)),
            wallet,
            energy_rental,
            energy_rental_confirm_max_wait,
            require_energy_rental,
            rental_cap_per_hour,
            rental_cap_per_day,
            rental_cooldown_after_trip,
            write_staleness_guard,
            write_preflight_simulation,
            tx_cap_per_kind_per_hour,
            fee_limit_policy: FeeLimitPolicy {
                fees: Some(chain_fees),
                headroom_ppm: fee_limit_headroom_ppm,
                ceiling_sun: fee_limit_ceiling_sun,
            },
            telemetry,
        }
    }

    pub async fn broadcast_trigger_smart_contract(
        &self,
        state: &mut RelayerState,
        kind: &'static str,
        contract: TronAddress,
        data: Vec<u8>,
        call_value_sun: i64,
    ) -> Result<[u8; 32]> {
        // Layer 4 breaker: per-kind rate cap. Must come BEFORE any gRPC work so a tripped
        // breaker has zero cost — no staleness check, no preflight, no estimate, no rental,
        // no broadcast. Each kind has its own deque + cooldown, so a tripped tip_proof does
        // not block pullFromReceivers.
        self.enforce_kind_budget(state, kind)?;

        let start = Instant::now();
        let len = self.grpc_urls.len();
        if len == 0 {
            anyhow::bail!("no TRON_GRPC_URLS configured");
        }

        let start_cursor = *self.grpc_url_cursor.lock().await;
        let attempts = if len == 1 { 2 } else { len };
        let mut txid: Result<[u8; 32]> = Err(anyhow::anyhow!(
            "all TRON_GRPC_URLS endpoints failed for op=broadcast_trigger_smart_contract"
        ));

        for attempt in 0..attempts {
            let idx = (start_cursor + attempt) % len;
            let force_reconnect = len == 1 && attempt > 0;

            if let Err(err) = self.ensure_tron_connected(idx, force_reconnect).await {
                tracing::warn!(
                    tron_grpc = %self.grpc_urls[idx],
                    op = "broadcast_trigger_smart_contract",
                    err = %err,
                    "failed to connect Tron gRPC endpoint"
                );
                txid = Err(err);
                continue;
            }

            let mut grpc = self.active_grpc.lock().await;
            match self
                .broadcast_trigger_smart_contract_managed(
                    &mut grpc,
                    state,
                    contract,
                    data.clone(),
                    call_value_sun,
                )
                .await
            {
                Ok(v) => {
                    txid = Ok(v);
                    break;
                }
                Err(err) => {
                    tracing::warn!(
                        tron_grpc = %self.grpc_urls[idx],
                        op = "broadcast_trigger_smart_contract",
                        err = %err,
                        "tron write operation failed; trying next endpoint"
                    );
                    txid = Err(err);
                }
            }
        }

        match txid {
            Ok(txid) => {
                self.telemetry
                    .tron_broadcast_ms(true, start.elapsed().as_millis() as u64);
                self.telemetry.tron_tx_ok();
                Ok(txid)
            }
            Err(err) => {
                self.telemetry
                    .tron_broadcast_ms(false, start.elapsed().as_millis() as u64);
                self.telemetry.tron_tx_err();
                Err(err)
            }
        }
    }

    pub async fn estimate_trigger_smart_contract_energy(
        &self,
        contract: TronAddress,
        data: Vec<u8>,
        call_value_sun: i64,
    ) -> Result<u64> {
        let len = self.grpc_urls.len();
        if len == 0 {
            anyhow::bail!("no TRON_GRPC_URLS configured");
        }

        let owner = self.wallet.address().prefixed_bytes().to_vec();
        let contract_addr = contract.prefixed_bytes().to_vec();

        let start_cursor = *self.grpc_url_cursor.lock().await;
        let attempts = if len == 1 { 2 } else { len };
        let mut last_err: Option<anyhow::Error> = None;

        for attempt in 0..attempts {
            let idx = (start_cursor + attempt) % len;
            let force_reconnect = len == 1 && attempt > 0;

            if let Err(err) = self.ensure_tron_connected(idx, force_reconnect).await {
                tracing::warn!(
                    tron_grpc = %self.grpc_urls[idx],
                    op = "estimate_trigger_smart_contract_energy",
                    err = %err,
                    "failed to connect Tron gRPC endpoint"
                );
                last_err = Some(err);
                continue;
            }

            let mut grpc = self.active_grpc.lock().await;
            let resp = match grpc
                .estimate_energy(TriggerSmartContract {
                    owner_address: owner.clone(),
                    contract_address: contract_addr.clone(),
                    call_value: call_value_sun,
                    data: data.clone(),
                    call_token_value: 0,
                    token_id: 0,
                })
                .await
            {
                Ok(v) => v,
                Err(err) => {
                    tracing::warn!(
                        tron_grpc = %self.grpc_urls[idx],
                        op = "estimate_trigger_smart_contract_energy",
                        err = %err,
                        "tron write operation failed; trying next endpoint"
                    );
                    last_err = Some(err);
                    continue;
                }
            };

            // Tron returns Ok with `result.result=false` and `energy_required=0` when the
            // simulator hits its CPU budget (OutOfTimeException). Callers that treat the
            // returned u64 as authoritative would silently underestimate — bypassing the
            // splitter's energy_limit cap and broadcasting the full receiver set. Surface
            // estimator failure as an Err so the splitter falls back to its binary-search
            // path, which treats Err as "too big" and shrinks.
            if let Some(ret) = &resp.result
                && !ret.result
            {
                let msg = String::from_utf8_lossy(&ret.message);
                tracing::warn!(
                    tron_grpc = %self.grpc_urls[idx],
                    op = "estimate_trigger_smart_contract_energy",
                    code = ret.code,
                    sim_message = %msg,
                    "estimator returned unsuccessful result; treating as estimator failure"
                );
                last_err = Some(anyhow::anyhow!(
                    "estimate_energy unsuccessful: code={} msg={}",
                    ret.code,
                    msg
                ));
                continue;
            }

            let energy_required =
                u64::try_from(resp.energy_required).context("energy_required out of range")?;
            return Ok(energy_required);
        }

        Err(last_err.unwrap_or_else(|| {
            anyhow::anyhow!(
                "all TRON_GRPC_URLS endpoints failed for op=estimate_trigger_smart_contract_energy"
            )
        }))
    }

    async fn ensure_tron_connected(&self, idx: usize, force_reconnect: bool) -> Result<()> {
        let len = self.grpc_urls.len();
        if len == 0 {
            anyhow::bail!("no TRON_GRPC_URLS configured");
        }
        let idx = idx % len;

        let cur = *self.grpc_url_cursor.lock().await;
        if idx == cur && !force_reconnect {
            return Ok(());
        }

        let url = self.grpc_urls[idx].clone();
        let grpc = TronGrpc::connect(
            &url,
            self.grpc_api_key.as_deref(),
            &self.grpc_api_key_header,
        )
        .await
        .with_context(|| format!("connect TRON gRPC: {url}"))?;
        {
            let mut guard = self.active_grpc.lock().await;
            *guard = grpc;
        }
        *self.grpc_url_cursor.lock().await = idx;
        tracing::info!(tron_grpc = %url, "switched relayer Tron write endpoint");
        Ok(())
    }

    async fn broadcast_trigger_smart_contract_managed(
        &self,
        grpc: &mut TronGrpc,
        state: &mut RelayerState,
        contract: TronAddress,
        data: Vec<u8>,
        call_value_sun: i64,
    ) -> Result<[u8; 32]> {
        const MIN_ENERGY_RENTAL_AMOUNT: u64 = 32_000;

        // Staleness guard — if the node we're about to broadcast through is lagging,
        // its view of our account resources is stale, every rejection looks like a
        // resource error, and retries spin up rentals against a fiction. Refuse early.
        if !self.write_staleness_guard.is_zero() {
            let head = grpc
                .get_now_block2()
                .await
                .context("get_now_block2 (staleness guard)")?;
            let head_ts_ms = head
                .block_header
                .as_ref()
                .and_then(|bh| bh.raw_data.as_ref())
                .map(|rd| rd.timestamp)
                .unwrap_or(0);
            let now_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as i64)
                .unwrap_or(0);
            let age_secs = ((now_ms - head_ts_ms).max(0) / 1000) as u64;
            if age_secs > self.write_staleness_guard.as_secs() {
                anyhow::bail!(
                    "tron write endpoint head is stale (age={}s, guard={}s); refusing to broadcast to avoid firing rentals against a node that cannot see recent state",
                    age_secs,
                    self.write_staleness_guard.as_secs(),
                );
            }
        }

        // Pre-flight constant-call simulation. Zero cost; catches "would revert" before
        // we sign, rent, or burn anything. `trigger_constant_contract` returns the same
        // result.result boolean that estimate_energy does, but also gives the revert msg.
        if self.write_preflight_simulation {
            let sim = grpc
                .trigger_constant_contract(TriggerSmartContract {
                    owner_address: self.wallet.address().prefixed_bytes().to_vec(),
                    contract_address: contract.prefixed_bytes().to_vec(),
                    call_value: call_value_sun,
                    data: data.clone(),
                    call_token_value: 0,
                    token_id: 0,
                })
                .await
                .context("trigger_constant_contract (preflight)")?;
            if let Some(ret) = sim.result
                && !ret.result
            {
                let msg = String::from_utf8_lossy(&ret.message);
                // Tron's `triggerconstantcontract` imposes a fixed CPU-time budget on the
                // simulation that is unrelated to on-chain execution limits. Complex contract
                // calls (e.g. pullFromReceivers looping over several receivers) can hit the
                // budget and report `Program$OutOfTimeException: CPU timeout for '<op>'` — this
                // does NOT mean the real tx would revert, just that the simulator was too slow.
                // Treat those as "preflight inconclusive": log loudly and proceed with the real
                // broadcast (which has its own fee_limit ceiling + per-kind breaker for safety).
                // Real reverts — require() failures, assertions, invalid state — do not contain
                // these markers, so they still bail here before any TRX is touched.
                if msg.contains("OutOfTimeException") || msg.contains("CPU timeout") {
                    tracing::warn!(
                        code = ret.code,
                        sim_message = %msg,
                        "preflight hit simulator CPU budget (OutOfTime); proceeding — the real tx has a fee_limit ceiling and the per-kind breaker"
                    );
                } else {
                    anyhow::bail!(
                        "preflight simulation reports revert: code={} msg_hex=0x{} msg_utf8={}",
                        ret.code,
                        hex::encode(&ret.message),
                        msg,
                    );
                }
            }
        }

        let signed = self
            .wallet
            .build_and_sign_trigger_smart_contract(
                grpc,
                contract,
                data,
                call_value_sun,
                self.fee_limit_policy,
            )
            .await?;

        tracing::info!(
            txid = %format!("0x{}", hex::encode(signed.txid)),
            energy_required = signed.energy_required,
            tx_size_bytes = signed.tx_size_bytes,
            fee_limit_sun = signed.fee_limit_sun,
            call_value_sun,
            "signed tron tx"
        );

        // Cover any energy shortfall from rental providers. When `require_energy_rental` is set,
        // sub-minimum shortfalls still rent (rounded up to the provider minimum), and if rental
        // fails the broadcast is aborted instead of burning wallet TRX.
        if !self.energy_rental.is_empty() {
            let res = grpc
                .get_account_resource(self.wallet.address().prefixed_bytes().to_vec())
                .await?;
            let parsed = tron::resources::parse_account_resources(&res)?;
            let energy_available = parsed.energy_available();
            let shortfall = signed.energy_required.saturating_sub(energy_available);

            // Bandwidth feasibility check: even with all the rented energy in the world the
            // broadcast still fails on `Account resource insufficient` if the wallet can't
            // cover the per-byte network fee. Renting energy in that state wastes the rental
            // (apitrx-style providers debit the prepaid pool whether the tx lands or not),
            // so abort early if the wallet can neither use free bandwidth nor burn TRX for it.
            let bandwidth_needed = signed.tx_size_bytes;
            let bandwidth_available = parsed
                .net_available()
                .saturating_add(parsed.free_net_available());
            if bandwidth_available < bandwidth_needed {
                let shortfall_bytes = bandwidth_needed - bandwidth_available;
                let tx_fee_per_byte = self
                    .fee_limit_policy
                    .fees
                    .map(|f| f.tx_fee_sun_per_byte)
                    .unwrap_or(1000);
                let trx_burn_needed_sun = shortfall_bytes.saturating_mul(tx_fee_per_byte);

                let acct = grpc
                    .get_account(self.wallet.address().prefixed_bytes().to_vec())
                    .await?;
                let balance_sun = u64::try_from(acct.balance.max(0)).unwrap_or(0);
                if balance_sun < trx_burn_needed_sun {
                    tracing::error!(
                        bandwidth_needed,
                        bandwidth_available,
                        bandwidth_shortfall_bytes = shortfall_bytes,
                        trx_burn_needed_sun,
                        wallet_balance_sun = balance_sun,
                        "wallet bandwidth and TRX both insufficient; aborting before rental to avoid wasting prepaid pool on a tx that cannot broadcast"
                    );
                    anyhow::bail!(
                        "wallet cannot pay bandwidth (need {} sun, balance {} sun); refusing to rent energy for a doomed broadcast",
                        trx_burn_needed_sun,
                        balance_sun
                    );
                }
            }

            let should_attempt_rental =
                shortfall > 0 && (self.require_energy_rental || shortfall >= MIN_ENERGY_RENTAL_AMOUNT);

            tracing::info!(
                energy_required = signed.energy_required,
                energy_available,
                energy_shortfall = shortfall,
                min_rental_energy = MIN_ENERGY_RENTAL_AMOUNT,
                require_rental = self.require_energy_rental,
                will_attempt_rental = should_attempt_rental,
                "tron tx energy plan"
            );

            if should_attempt_rental {
                // Layer 3 breaker: rental-rate cap. Refuses to initiate a rental if we've
                // already fired too many this hour/day, or if we're in post-trip cooldown.
                // This is the guard that makes the 200-rentals-in-10-min scenario impossible.
                self.enforce_rental_budget(state)?;

                let rent_amount = shortfall.max(MIN_ENERGY_RENTAL_AMOUNT);
                let addr = self.wallet.address();
                let addr_hex41 = format!("0x{}", hex::encode(addr.prefixed_bytes()));
                let addr_evm_hex = format!("0x{}", hex::encode(addr.evm().as_slice()));
                let txid_hex = format!("0x{}", hex::encode(signed.txid));

                let ctx = RentalContext {
                    resource: RentalResourceKind::Energy,
                    amount: rent_amount,
                    address_base58check: addr.to_string(),
                    address_hex41: addr_hex41,
                    address_evm_hex: addr_evm_hex,
                    txid: Some(txid_hex),
                };

                let len = self.energy_rental.len();
                let start_cursor = state.energy_rental_cursor;
                let order = rental_try_indices(start_cursor, len);

                let mut ok = false;
                for (attempt, idx) in order.into_iter().enumerate() {
                    let p = &self.energy_rental[idx];
                    match p.rent(&ctx).await {
                        Ok(attempt_res) if attempt_res.ok => {
                            tracing::info!(
                                provider = %attempt_res.provider,
                                order_id = attempt_res.order_id.as_deref().unwrap_or(""),
                                energy = rent_amount,
                                "energy rental ok"
                            );
                            state.energy_rental_cursor =
                                rental_cursor_after_attempts(start_cursor, len, attempt + 1);
                            ok = true;
                            break;
                        }
                        Ok(attempt_res) => {
                            tracing::warn!(
                                provider = %attempt_res.provider,
                                err = attempt_res.error.as_deref().unwrap_or("rental failed"),
                                "energy rental failed; trying next provider"
                            );
                            state.energy_rental_cursor =
                                rental_cursor_after_attempts(start_cursor, len, attempt + 1);
                        }
                        Err(err) => {
                            tracing::warn!(provider = %p.name(), err = %err, "energy rental errored; trying next provider");
                            state.energy_rental_cursor =
                                rental_cursor_after_attempts(start_cursor, len, attempt + 1);
                        }
                    }
                }

                if !ok {
                    if self.require_energy_rental {
                        anyhow::bail!(
                            "TRON_REQUIRE_ENERGY_RENTAL=true and all {} rental provider(s) failed; aborting broadcast to preserve wallet TRX (energy_required={}, energy_available={}, shortfall={})",
                            self.energy_rental.len(),
                            signed.energy_required,
                            energy_available,
                            shortfall,
                        );
                    }
                    tracing::warn!(
                        energy = rent_amount,
                        "all energy rental providers failed; falling back to paying TRX fees"
                    );
                } else if !self.energy_rental_confirm_max_wait.is_zero() {
                    let addr = self.wallet.address().prefixed_bytes().to_vec();
                    wait_for_energy_available_after_rental(
                        grpc,
                        addr,
                        signed.energy_required,
                        self.energy_rental_confirm_max_wait,
                    )
                    .await?;
                }
            }
        } else if self.require_energy_rental {
            anyhow::bail!(
                "TRON_REQUIRE_ENERGY_RENTAL=true but no rental providers are configured",
            );
        }

        let ret = grpc.broadcast_transaction(signed.tx).await?;
        if !ret.result {
            tracing::warn!(
                tron_return_code = ret.code,
                tron_return_message_hex = %format!("0x{}", hex::encode(&ret.message)),
                tron_return_message_utf8 = %String::from_utf8_lossy(&ret.message),
                "tron broadcast rejected"
            );
            anyhow::bail!(
                "broadcast failed: msg_hex=0x{}, msg_utf8={}",
                hex::encode(&ret.message),
                String::from_utf8_lossy(&ret.message),
            );
        }

        Ok(signed.txid)
    }

    /// Enforces the per-kind rate breaker. On trip, records the cooldown in `state` and bails.
    /// Called at the very start of every `broadcast_trigger_smart_contract` invocation,
    /// before any gRPC cost is incurred.
    fn enforce_kind_budget(&self, state: &mut RelayerState, kind: &'static str) -> Result<()> {
        let now = Instant::now();

        // Evict this kind's attempts older than 1h so the deque doesn't grow without bound.
        let attempts = state.tx_attempts_per_kind.entry(kind).or_default();
        let hour = Duration::from_secs(3600);
        while attempts.front().is_some_and(|t| now.duration_since(*t) > hour) {
            attempts.pop_front();
        }

        let paused_until = state.tx_paused_until_per_kind.get(kind).copied();
        let decision = evaluate_kind_budget(now, paused_until, attempts, self.tx_cap_per_kind_per_hour);

        match decision {
            RentalBudgetDecision::Cooldown { remaining_secs } => {
                tracing::error!(
                    kind,
                    cooldown_remaining_secs = remaining_secs,
                    recent_attempts = attempts.len(),
                    "per-kind breaker tripped; refusing broadcast"
                );
                anyhow::bail!(
                    "per-kind breaker [{kind}] in cooldown for {remaining_secs}s more"
                );
            }
            RentalBudgetDecision::Trip { reason } => {
                let cooldown = self.rental_cooldown_after_trip;
                state
                    .tx_paused_until_per_kind
                    .insert(kind, now + cooldown);
                tracing::error!(
                    kind,
                    reason,
                    recent_attempts = attempts.len(),
                    cap_per_hour = self.tx_cap_per_kind_per_hour,
                    cooldown_secs = cooldown.as_secs(),
                    "per-kind breaker TRIPPED; pausing this tx kind"
                );
                anyhow::bail!(
                    "per-kind breaker [{kind}] tripped ({reason}); paused for {}s",
                    cooldown.as_secs()
                );
            }
            RentalBudgetDecision::Proceed => {
                attempts.push_back(now);
                tracing::debug!(
                    kind,
                    recent_attempts = attempts.len(),
                    cap_per_hour = self.tx_cap_per_kind_per_hour,
                    "per-kind attempt recorded against budget"
                );
                Ok(())
            }
        }
    }

    /// Enforces the rental-rate breaker. On trip, records the cooldown in `state` and bails.
    /// Called immediately before any attempt to contact a rental provider.
    fn enforce_rental_budget(&self, state: &mut RelayerState) -> Result<()> {
        let now = Instant::now();

        // Evict attempts older than 24h so the deque doesn't grow without bound.
        let day = Duration::from_secs(24 * 3600);
        while state
            .rental_attempts
            .front()
            .is_some_and(|t| now.duration_since(*t) > day)
        {
            state.rental_attempts.pop_front();
        }

        let decision = evaluate_rental_budget(
            now,
            state.rental_paused_until,
            &state.rental_attempts,
            self.rental_cap_per_hour,
            self.rental_cap_per_day,
        );

        match decision {
            RentalBudgetDecision::Cooldown { remaining_secs } => {
                tracing::error!(
                    cooldown_remaining_secs = remaining_secs,
                    recent_rental_attempts = state.rental_attempts.len(),
                    "rental breaker tripped; refusing to initiate another rental"
                );
                anyhow::bail!(
                    "rental breaker in cooldown for {remaining_secs}s more (recent attempts: {})",
                    state.rental_attempts.len()
                );
            }
            RentalBudgetDecision::Trip { reason } => {
                let cooldown = self.rental_cooldown_after_trip;
                state.rental_paused_until = Some(now + cooldown);
                tracing::error!(
                    reason,
                    recent_rental_attempts = state.rental_attempts.len(),
                    cap_per_hour = self.rental_cap_per_hour,
                    cap_per_day = self.rental_cap_per_day,
                    cooldown_secs = cooldown.as_secs(),
                    "rental breaker TRIPPED; pausing rentals (and therefore writes requiring rental)"
                );
                anyhow::bail!(
                    "rental breaker tripped ({reason}); paused for {}s",
                    cooldown.as_secs()
                );
            }
            RentalBudgetDecision::Proceed => {
                // Record the attempt we're about to make. Done before calling providers so a
                // provider error still counts toward the cap — a flapping provider can't
                // bypass the cap by failing every call.
                state.rental_attempts.push_back(now);
                tracing::debug!(
                    recent_rental_attempts = state.rental_attempts.len(),
                    cap_per_hour = self.rental_cap_per_hour,
                    cap_per_day = self.rental_cap_per_day,
                    "rental attempt recorded against budget"
                );
                Ok(())
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RentalBudgetDecision {
    Proceed,
    Cooldown { remaining_secs: u64 },
    Trip { reason: &'static str },
}

fn evaluate_kind_budget(
    now: Instant,
    paused_until: Option<Instant>,
    attempts: &std::collections::VecDeque<Instant>,
    cap_per_hour: u32,
) -> RentalBudgetDecision {
    if let Some(until) = paused_until
        && now < until
    {
        return RentalBudgetDecision::Cooldown {
            remaining_secs: until.saturating_duration_since(now).as_secs(),
        };
    }
    if cap_per_hour == 0 {
        return RentalBudgetDecision::Proceed;
    }
    let hour = Duration::from_secs(3600);
    let in_last_hour = attempts
        .iter()
        .filter(|t| now.duration_since(**t) <= hour)
        .count() as u32;
    if in_last_hour >= cap_per_hour {
        return RentalBudgetDecision::Trip { reason: "hourly cap" };
    }
    RentalBudgetDecision::Proceed
}

fn evaluate_rental_budget(
    now: Instant,
    paused_until: Option<Instant>,
    attempts: &std::collections::VecDeque<Instant>,
    cap_per_hour: u32,
    cap_per_day: u32,
) -> RentalBudgetDecision {
    if let Some(until) = paused_until
        && now < until
    {
        return RentalBudgetDecision::Cooldown {
            remaining_secs: until.saturating_duration_since(now).as_secs(),
        };
    }
    let hour = Duration::from_secs(3600);
    let in_last_hour = attempts
        .iter()
        .filter(|t| now.duration_since(**t) <= hour)
        .count() as u32;
    let in_last_day = attempts.len() as u32;
    if cap_per_hour > 0 && in_last_hour >= cap_per_hour {
        return RentalBudgetDecision::Trip { reason: "hourly cap" };
    }
    if cap_per_day > 0 && in_last_day >= cap_per_day {
        return RentalBudgetDecision::Trip { reason: "daily cap" };
    }
    RentalBudgetDecision::Proceed
}

async fn wait_for_energy_available_after_rental(
    grpc: &mut TronGrpc,
    address: Vec<u8>,
    energy_required: u64,
    max_wait: Duration,
) -> Result<()> {
    if energy_required == 0 || max_wait.is_zero() {
        return Ok(());
    }

    let start = tokio::time::Instant::now();
    let mut delay = Duration::from_millis(100);
    let max_delay = Duration::from_secs(1);
    let mut tries: u32 = 0;
    let mut last_available: Option<u64> = None;

    loop {
        tries = tries.saturating_add(1);
        match grpc.get_account_resource(address.clone()).await {
            Ok(res) => {
                let parsed = tron::resources::parse_account_resources(&res)?;
                let available = parsed.energy_available();
                last_available = Some(available);
                if available >= energy_required {
                    tracing::info!(
                        energy_required,
                        energy_available = available,
                        tries,
                        elapsed_ms = start.elapsed().as_millis() as u64,
                        "rented energy settled"
                    );
                    return Ok(());
                }
            }
            Err(err) => {
                tracing::debug!(
                    err = %err,
                    tries,
                    elapsed_ms = start.elapsed().as_millis() as u64,
                    "failed to query account resources while waiting for rented energy"
                );
            }
        }

        if start.elapsed() >= max_wait {
            tracing::warn!(
                energy_required,
                energy_available = last_available.unwrap_or(0),
                tries,
                elapsed_ms = start.elapsed().as_millis() as u64,
                "rented energy did not settle before timeout; broadcasting anyway"
            );
            return Ok(());
        }

        tokio::time::sleep(delay).await;
        delay = delay.saturating_mul(2);
        if delay > max_delay {
            delay = max_delay;
        }
    }
}

fn rental_try_indices(start_cursor: usize, len: usize) -> Vec<usize> {
    if len == 0 {
        return Vec::new();
    }
    let start = start_cursor % len;
    (0..len).map(|i| (start + i) % len).collect()
}

fn rental_cursor_after_attempts(start_cursor: usize, len: usize, attempts: usize) -> usize {
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
    fn rental_try_indices_rotates_from_cursor() {
        assert_eq!(rental_try_indices(0, 3), vec![0, 1, 2]);
        assert_eq!(rental_try_indices(2, 3), vec![2, 0, 1]);
        assert_eq!(rental_try_indices(5, 3), vec![2, 0, 1]);
        assert!(rental_try_indices(0, 0).is_empty());
    }

    #[test]
    fn rental_cursor_after_attempts_advances() {
        assert_eq!(rental_cursor_after_attempts(0, 3, 1), 1);
        assert_eq!(rental_cursor_after_attempts(2, 3, 1), 0);
        assert_eq!(rental_cursor_after_attempts(2, 3, 2), 1);
        assert_eq!(rental_cursor_after_attempts(2, 3, 3), 2);
        assert_eq!(rental_cursor_after_attempts(10, 0, 1), 0);
    }

    #[test]
    fn rental_budget_proceeds_when_empty() {
        let now = Instant::now();
        let q = std::collections::VecDeque::new();
        assert_eq!(
            evaluate_rental_budget(now, None, &q, 5, 20),
            RentalBudgetDecision::Proceed
        );
    }

    #[test]
    fn rental_budget_trips_hourly_cap() {
        let now = Instant::now();
        let mut q = std::collections::VecDeque::new();
        // 5 recent attempts; cap is 5 → next one must trip.
        for i in 0..5 {
            q.push_back(now - Duration::from_secs(60 * i));
        }
        assert_eq!(
            evaluate_rental_budget(now, None, &q, 5, 20),
            RentalBudgetDecision::Trip { reason: "hourly cap" }
        );
    }

    #[test]
    fn rental_budget_trips_daily_cap_when_hourly_ok() {
        let now = Instant::now();
        let mut q = std::collections::VecDeque::new();
        // All attempts are 2h+ old (not in hourly window) but still in the daily window.
        for i in 0..20 {
            q.push_back(now - Duration::from_secs(2 * 3600 + 60 * i));
        }
        assert_eq!(
            evaluate_rental_budget(now, None, &q, 5, 20),
            RentalBudgetDecision::Trip { reason: "daily cap" }
        );
    }

    #[test]
    fn rental_budget_honors_cooldown() {
        let now = Instant::now();
        let q = std::collections::VecDeque::new();
        let until = now + Duration::from_secs(42);
        match evaluate_rental_budget(now, Some(until), &q, 5, 20) {
            RentalBudgetDecision::Cooldown { remaining_secs } => {
                assert!(
                    remaining_secs <= 42 && remaining_secs >= 41,
                    "got {remaining_secs}"
                );
            }
            d => panic!("expected Cooldown, got {d:?}"),
        }
    }

    #[test]
    fn rental_budget_ignores_expired_cooldown() {
        let now = Instant::now();
        let q = std::collections::VecDeque::new();
        let until = now - Duration::from_secs(1);
        assert_eq!(
            evaluate_rental_budget(now, Some(until), &q, 5, 20),
            RentalBudgetDecision::Proceed
        );
    }

    #[test]
    fn rental_budget_zero_cap_disables() {
        let now = Instant::now();
        let mut q = std::collections::VecDeque::new();
        for i in 0..50 {
            q.push_back(now - Duration::from_secs(i));
        }
        // cap_per_hour=0 disables the hourly cap; cap_per_day=0 disables the daily cap.
        assert_eq!(
            evaluate_rental_budget(now, None, &q, 0, 0),
            RentalBudgetDecision::Proceed
        );
    }

    #[test]
    fn kind_budget_proceeds_when_empty() {
        let now = Instant::now();
        let q = std::collections::VecDeque::new();
        assert_eq!(
            evaluate_kind_budget(now, None, &q, 3),
            RentalBudgetDecision::Proceed
        );
    }

    #[test]
    fn kind_budget_trips_at_cap() {
        let now = Instant::now();
        let mut q = std::collections::VecDeque::new();
        // 3 recent attempts; cap is 3 → next trips.
        for i in 0..3 {
            q.push_back(now - Duration::from_secs(60 * i));
        }
        assert_eq!(
            evaluate_kind_budget(now, None, &q, 3),
            RentalBudgetDecision::Trip { reason: "hourly cap" }
        );
    }

    #[test]
    fn kind_budget_honors_cooldown() {
        let now = Instant::now();
        let q = std::collections::VecDeque::new();
        let until = now + Duration::from_secs(42);
        match evaluate_kind_budget(now, Some(until), &q, 3) {
            RentalBudgetDecision::Cooldown { remaining_secs } => {
                assert!(remaining_secs <= 42 && remaining_secs >= 41);
            }
            d => panic!("expected Cooldown, got {d:?}"),
        }
    }

    #[test]
    fn kind_budget_zero_cap_disables() {
        let now = Instant::now();
        let mut q = std::collections::VecDeque::new();
        for i in 0..50 {
            q.push_back(now - Duration::from_secs(i));
        }
        assert_eq!(
            evaluate_kind_budget(now, None, &q, 0),
            RentalBudgetDecision::Proceed
        );
    }

    #[test]
    fn rental_budget_hourly_window_excludes_old_attempts() {
        let now = Instant::now();
        let mut q = std::collections::VecDeque::new();
        // 4 recent + 10 older-than-hour → only 4 in the hourly window, under cap 5.
        for _ in 0..4 {
            q.push_back(now - Duration::from_secs(30 * 60));
        }
        for i in 0..10 {
            q.push_back(now - Duration::from_secs(2 * 3600 + i));
        }
        assert_eq!(
            evaluate_rental_budget(now, None, &q, 5, 100),
            RentalBudgetDecision::Proceed
        );
    }
}
