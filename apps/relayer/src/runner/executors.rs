use super::RelayerState;
use crate::metrics::RelayerTelemetry;
use aa::Safe4337UserOpSender;
use alloy::primitives::{Address, U256};
use anyhow::{Context, Result};
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Mutex;
use tron::{
    FeePolicy, JsonApiRentalProvider, RentalContext, RentalResourceKind, TronAddress, TronGrpc,
    TronWallet,
};

#[derive(Clone)]
pub struct HubExecutor {
    sender: Arc<Mutex<Safe4337UserOpSender>>,
    telemetry: RelayerTelemetry,
}

impl HubExecutor {
    pub fn new(sender: Arc<Mutex<Safe4337UserOpSender>>, telemetry: RelayerTelemetry) -> Self {
        Self { sender, telemetry }
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
        name: &'static str,
        to: Address,
        data: Vec<u8>,
        operation: u8,
    ) -> Result<()> {
        let data_len = data.len();
        let data_selector = match data.get(0..4) {
            Some(sel) => format!("0x{}", hex::encode(sel)),
            None => "0x".to_string(),
        };
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
                .send_call_with_nonce_operation(nonce, to, data, operation)
                .await
        };

        match submission {
            Ok(sub) => {
                self.telemetry
                    .hub_submit_ms(name, true, start.elapsed().as_millis() as u64);
                self.telemetry.hub_userop_ok();
                state.invalidate_hub_usdt_balance_cache();
                state.invalidate_hub_safe_erc20_balance_cache();
                state.hub_pending_nonce = Some(sub.nonce);
                tracing::info!(userop_hash = %sub.userop_hash, %name, "submitted hub userop");
                Ok(())
            }
            Err(err) => {
                self.telemetry
                    .hub_submit_ms(name, false, start.elapsed().as_millis() as u64);
                self.telemetry.hub_userop_err();
                tracing::error!(
                    %name,
                    to = %to,
                    operation,
                    data_len,
                    data_selector = %data_selector,
                    err = %format!("{err:#}"),
                    "hub userop submit failed"
                );
                Err(err)
            }
        }
    }
}

#[derive(Clone)]
pub struct TronExecutor {
    active_grpc: Arc<Mutex<TronGrpc>>,
    grpc_urls: Vec<String>,
    grpc_api_key: Option<String>,
    grpc_url_cursor: Arc<Mutex<usize>>,
    wallet: Arc<TronWallet>,
    fee_policy: FeePolicy,
    energy_rental: Vec<JsonApiRentalProvider>,
    energy_rental_confirm_max_wait: Duration,
    telemetry: RelayerTelemetry,
}

impl TronExecutor {
    pub fn new(
        active_grpc: Arc<Mutex<TronGrpc>>,
        grpc_urls: Vec<String>,
        grpc_api_key: Option<String>,
        initial_grpc_url_cursor: usize,
        wallet: Arc<TronWallet>,
        fee_policy: FeePolicy,
        energy_rental: Vec<JsonApiRentalProvider>,
        energy_rental_confirm_max_wait: Duration,
        telemetry: RelayerTelemetry,
    ) -> Self {
        Self {
            active_grpc,
            grpc_urls,
            grpc_api_key,
            grpc_url_cursor: Arc::new(Mutex::new(initial_grpc_url_cursor)),
            wallet,
            fee_policy,
            energy_rental,
            energy_rental_confirm_max_wait,
            telemetry,
        }
    }

    pub async fn broadcast_trigger_smart_contract(
        &self,
        state: &mut RelayerState,
        contract: TronAddress,
        data: Vec<u8>,
        call_value_sun: i64,
    ) -> Result<[u8; 32]> {
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
        let grpc = TronGrpc::connect(&url, self.grpc_api_key.as_deref())
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

        let signed = self
            .wallet
            .build_and_sign_trigger_smart_contract(
                grpc,
                contract,
                data,
                call_value_sun,
                self.fee_policy,
            )
            .await?;

        // Preflight balance check: nodes commonly require `balance >= fee_limit` even if resources are rented.
        let account = grpc
            .get_account(self.wallet.address().prefixed_bytes().to_vec())
            .await?;
        let fee_limit_i64 = i64::try_from(signed.fee_limit_sun).unwrap_or(i64::MAX);
        if account.balance < fee_limit_i64 {
            anyhow::bail!(
                "insufficient TRX for fee_limit: balance={} sun, fee_limit={} sun",
                account.balance,
                fee_limit_i64
            );
        }

        // Attempt energy rental for the shortfall (best-effort, fall back to paying TRX).
        if !self.energy_rental.is_empty() {
            let res = grpc
                .get_account_resource(self.wallet.address().prefixed_bytes().to_vec())
                .await?;
            let parsed = tron::resources::parse_account_resources(&res)?;
            let shortfall = signed
                .energy_required
                .saturating_sub(parsed.energy_available());

            if shortfall >= MIN_ENERGY_RENTAL_AMOUNT {
                let addr = self.wallet.address();
                let addr_hex41 = format!("0x{}", hex::encode(addr.prefixed_bytes()));
                let addr_evm_hex = format!("0x{}", hex::encode(addr.evm().as_slice()));
                let txid_hex = format!("0x{}", hex::encode(signed.txid));

                let ctx = RentalContext {
                    resource: RentalResourceKind::Energy,
                    amount: shortfall,
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
                                energy = shortfall,
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
                    tracing::warn!(
                        energy = shortfall,
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
            } else if shortfall > 0 {
                tracing::debug!(
                    energy = shortfall,
                    min_energy = MIN_ENERGY_RENTAL_AMOUNT,
                    "energy shortfall below minimum rental amount; skipping rental"
                );
            }
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
}
