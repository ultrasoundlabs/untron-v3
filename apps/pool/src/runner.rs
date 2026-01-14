use crate::backoff::{BackoffConfig, BackoffState};
use crate::config::AppConfig;
use crate::metrics::PoolTelemetry;
use crate::watcher::{StatusWatcher, WatchConfig, WatchRequest};
use anyhow::{Context, Result};
use one_click_sdk_rs::models::quote_request::{DepositType, RecipientType, RefundType, SwapType};
use one_click_sdk_rs::{
    apis::{configuration::Configuration as OneClickConfiguration, one_click_api},
    models::{QuoteRequest, QuoteResponse, SubmitDepositTxRequest},
};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use std::sync::Arc;
use std::time::Duration;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use tokio::sync::Mutex;
use tokio::time::MissedTickBehavior;
use tokio_util::sync::CancellationToken;
use tron::{
    FeePolicy, JsonApiRentalProvider, RentalContext, RentalResourceKind, TronAddress, TronGrpc,
    TronWallet, wallet::trc20_balance_of,
};

fn usdt_keep_units() -> alloy::primitives::U256 {
    alloy::primitives::U256::from(1u64)
}

fn build_oneclick_client(user_agent: &str, bearer_token: Option<&str>) -> Result<reqwest::Client> {
    let mut headers = HeaderMap::new();
    if let Some(token) = bearer_token {
        let v = format!("Bearer {token}");
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&v).context("invalid ONECLICK_BEARER_TOKEN")?,
        );
    }
    let mut builder = reqwest::Client::builder()
        // Avoid OS proxy auto-detection (can be flaky in sandboxed/macOS environments).
        .no_proxy()
        .user_agent(user_agent);

    if !headers.is_empty() {
        builder = builder.default_headers(headers);
    }

    builder.build().context("build oneclick reqwest client")
}

pub struct PoolService {
    cfg: AppConfig,
    telemetry: PoolTelemetry,

    tron: TronGrpc,
    tron_urls: Vec<String>,
    tron_url_cursor: usize,
    tron_api_key: Option<String>,
    tron_wallet: TronWallet,
    tron_usdt: TronAddress,
    fee_policy: FeePolicy,
    energy_rental: Vec<JsonApiRentalProvider>,
    energy_rental_cursor: usize,

    oneclick: OneClickConfiguration,
    watch_tx: tokio::sync::mpsc::Sender<WatchRequest>,
    backoff: Arc<Mutex<BackoffState>>,
}

impl PoolService {
    pub async fn new(cfg: AppConfig, telemetry: PoolTelemetry) -> Result<Self> {
        let tron_urls = cfg.tron.grpc_urls.clone();
        let tron_url_cursor = 0;
        let tron_api_key = cfg.tron.api_key.clone();
        let tron = TronGrpc::connect(&tron_urls[tron_url_cursor], tron_api_key.as_deref())
            .await
            .context("connect TRON gRPC")?;

        let tron_wallet = TronWallet::new(cfg.tron.private_key)?;
        let tron_usdt = TronAddress::parse_text(&cfg.tron.usdt_contract_address)
            .context("parse TRON_USDT_CONTRACT_ADDRESS")?;

        let fee_policy = FeePolicy {
            fee_limit_cap_sun: cfg.tron.fee_limit_cap_sun,
            fee_limit_headroom_ppm: cfg.tron.fee_limit_headroom_ppm,
        };

        let energy_rental = cfg
            .tron
            .energy_rental_providers
            .clone()
            .into_iter()
            .map(JsonApiRentalProvider::new)
            .collect::<Vec<_>>();
        if !energy_rental.is_empty() {
            let names = energy_rental
                .iter()
                .map(|p| p.name().to_string())
                .collect::<Vec<_>>()
                .join(",");
            tracing::info!(
                energy_rental_providers = energy_rental.len(),
                providers = %names,
                "energy rental configured"
            );
        } else {
            tracing::info!("energy rental not configured");
        }

        let user_agent = format!("untron-v3-pool/{}", env!("CARGO_PKG_VERSION"));
        let mut oneclick = OneClickConfiguration::new();
        oneclick.base_path = cfg.oneclick.base_url.clone();
        oneclick.user_agent = Some(user_agent.clone());
        oneclick.client = build_oneclick_client(&user_agent, cfg.oneclick.bearer_token.as_deref())?;

        let (watch_tx, _watch_rx) = tokio::sync::mpsc::channel::<WatchRequest>(1024);
        let backoff = Arc::new(Mutex::new(BackoffState::default()));

        Ok(Self {
            cfg,
            telemetry,
            tron,
            tron_urls,
            tron_url_cursor,
            tron_api_key,
            tron_wallet,
            tron_usdt,
            fee_policy,
            energy_rental,
            energy_rental_cursor: 0,
            oneclick,
            watch_tx,
            backoff,
        })
    }

    pub async fn run(mut self, shutdown: CancellationToken) -> Result<()> {
        let (watch_tx, watch_rx) = tokio::sync::mpsc::channel::<WatchRequest>(1024);
        self.watch_tx = watch_tx;
        {
            let watcher_cfg = WatchConfig {
                poll_interval: self.cfg.oneclick.status_poll_interval,
                max_wait: self.cfg.oneclick.status_max_wait,
            };
            let backoff_cfg = BackoffConfig {
                base: self.cfg.oneclick.backoff_base,
                max: self.cfg.oneclick.backoff_max,
            };
            let oneclick = self.oneclick.clone();
            let telemetry = self.telemetry.clone();
            let backoff = self.backoff.clone();
            let watcher_shutdown = shutdown.clone();
            tokio::spawn(async move {
                if let Err(err) = StatusWatcher::new(
                    watcher_cfg,
                    oneclick,
                    telemetry,
                    watch_rx,
                    backoff_cfg,
                    backoff,
                )
                .run(watcher_shutdown)
                .await
                {
                    tracing::error!(err = %err, "status watcher task failed");
                }
            });
        }

        let threshold = crate::util::parse_u256_decimal(
            "POOL_USDT_BALANCE_THRESHOLD",
            &self.cfg.jobs.usdt_balance_threshold,
        )?;

        let mut interval = tokio::time::interval(self.cfg.jobs.poll_interval);
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                _ = shutdown.cancelled() => break,
                _ = interval.tick() => {
                    let start = std::time::Instant::now();
                    match self.tick(threshold).await {
                        Ok(()) => self.telemetry.tick_ok(start.elapsed().as_millis() as u64),
                        Err(err) => {
                            self.telemetry.tick_err(start.elapsed().as_millis() as u64);
                            tracing::error!(err = %err, "pool tick failed");
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn tick(&mut self, threshold: alloy::primitives::U256) -> Result<()> {
        if let Some(remaining) = self.backoff.lock().await.in_cooldown() {
            tracing::warn!(
                remaining_secs = remaining.as_secs(),
                "1click backoff active; skipping sweep"
            );
            return Ok(());
        }

        let bal_start = std::time::Instant::now();
        let balance = self.trc20_balance().await;
        match &balance {
            Ok(_) => self
                .telemetry
                .tron_balance_ms(true, bal_start.elapsed().as_millis() as u64),
            Err(_) => self
                .telemetry
                .tron_balance_ms(false, bal_start.elapsed().as_millis() as u64),
        }
        let balance = balance.context("read TRC20 USDT balance")?;

        if balance <= threshold {
            tracing::debug!(
                balance = %balance,
                threshold = %threshold,
                "balance below threshold"
            );
            return Ok(());
        }

        // Keep 1 unit (0.000001 USDT) on the account to avoid edge cases with zero-balance receivers.
        let keep_units = usdt_keep_units();
        let swap_budget = balance.saturating_sub(keep_units);
        if swap_budget.is_zero() {
            tracing::info!(
                balance = %balance,
                keep_units = %keep_units,
                "balance above threshold but nothing available after keep; skipping"
            );
            return Ok(());
        }

        tracing::info!(
            balance = %balance,
            swap_budget = %swap_budget,
            keep_units = %keep_units,
            threshold = %threshold,
            "balance above threshold; creating swap quote"
        );

        let quote_start = std::time::Instant::now();
        let quote_resp = self.create_quote(swap_budget).await;
        match &quote_resp {
            Ok(_) => self
                .telemetry
                .oneclick_quote_ms(true, quote_start.elapsed().as_millis() as u64),
            Err(_) => self
                .telemetry
                .oneclick_quote_ms(false, quote_start.elapsed().as_millis() as u64),
        }
        let quote_resp = quote_resp.context("create one-click quote")?;
        let deposit_addr = quote_resp
            .quote
            .deposit_address
            .clone()
            .context("quote missing depositAddress")?;

        let amount_in =
            crate::util::parse_u256_decimal("quote.amountIn", &quote_resp.quote.amount_in)?;
        if amount_in.is_zero() {
            anyhow::bail!("quote.amountIn was zero");
        }
        if amount_in > swap_budget {
            anyhow::bail!(
                "quote.amountIn exceeds swap budget: amount_in={} swap_budget={}",
                amount_in,
                swap_budget
            );
        }

        let deposit_tron = TronAddress::parse_text(&deposit_addr)
            .context("parse quote depositAddress as Tron address")?;

        tracing::info!(
            deposit_address = %deposit_addr,
            amount_in = %amount_in,
            "broadcasting TRC20 transfer to one-click deposit address"
        );

        let transfer_start = std::time::Instant::now();
        let txid = self
            .broadcast_trc20_transfer(self.tron_usdt, deposit_tron, amount_in)
            .await;
        match &txid {
            Ok(_) => self
                .telemetry
                .tron_transfer_ms(true, transfer_start.elapsed().as_millis() as u64),
            Err(_) => self
                .telemetry
                .tron_transfer_ms(false, transfer_start.elapsed().as_millis() as u64),
        }
        let txid = txid.context("broadcast TRC20 transfer")?;

        let tx_hash = format!("0x{}", hex::encode(txid));

        let submit_start = std::time::Instant::now();
        match one_click_api::submit_deposit_tx(
            &self.oneclick,
            SubmitDepositTxRequest::new(tx_hash.clone(), deposit_addr.clone()),
        )
        .await
        {
            Ok(_) => {
                self.telemetry
                    .oneclick_submit_ms(true, submit_start.elapsed().as_millis() as u64);
                tracing::info!(%tx_hash, %deposit_addr, "one-click deposit submission ok");
            }
            Err(err) => {
                self.telemetry
                    .oneclick_submit_ms(false, submit_start.elapsed().as_millis() as u64);
                tracing::warn!(%tx_hash, %deposit_addr, err = %err, "one-click deposit submission failed (continuing)");
            }
        }

        if let Err(err) = self.watch_tx.try_send(WatchRequest {
            deposit_address: deposit_addr.clone(),
            origin_tx_hash: tx_hash.clone(),
        }) {
            tracing::warn!(deposit_address = %deposit_addr, err = %err, "failed to enqueue status watch");
        }

        Ok(())
    }

    async fn trc20_balance(&mut self) -> Result<alloy::primitives::U256> {
        let len = self.tron_urls.len();
        if len == 0 {
            anyhow::bail!("no TRON_GRPC_URLS configured");
        }

        let attempts = if len == 1 { 2 } else { len };
        for attempt in 0..attempts {
            let idx = (self.tron_url_cursor + attempt) % len;
            let force_reconnect = len == 1 && attempt > 0;
            if let Err(err) = self.ensure_tron_connected(idx, force_reconnect).await {
                tracing::warn!(
                    tron_grpc = %self.tron_urls[idx],
                    op = "trc20_balance_of",
                    err = %err,
                    "failed to connect Tron gRPC endpoint"
                );
                continue;
            }

            match trc20_balance_of(
                &mut self.tron,
                self.tron_usdt,
                self.tron_wallet.address(),
                self.tron_wallet.address(),
            )
            .await
            {
                Ok(v) => return Ok(v),
                Err(err) => {
                    tracing::warn!(
                        tron_grpc = %self.tron_urls[self.tron_url_cursor],
                        op = "trc20_balance_of",
                        err = %err,
                        "tron operation failed; trying next endpoint"
                    );
                }
            }
        }

        anyhow::bail!("all TRON_GRPC_URLS endpoints failed for op=trc20_balance_of")
    }

    async fn ensure_tron_connected(&mut self, idx: usize, force_reconnect: bool) -> Result<()> {
        let len = self.tron_urls.len();
        if len == 0 {
            anyhow::bail!("no TRON_GRPC_URLS configured");
        }
        let idx = idx % len;
        if idx == self.tron_url_cursor && !force_reconnect {
            return Ok(());
        }

        let url = self.tron_urls[idx].clone();
        let grpc = TronGrpc::connect(&url, self.tron_api_key.as_deref())
            .await
            .with_context(|| format!("connect TRON gRPC: {url}"))?;

        self.tron = grpc;
        self.tron_url_cursor = idx;
        tracing::info!(tron_grpc = %url, "switched Tron gRPC endpoint");
        Ok(())
    }

    async fn create_quote(&self, amount: alloy::primitives::U256) -> Result<QuoteResponse> {
        let now = OffsetDateTime::now_utc();
        let deadline = now
            .checked_add(time::Duration::seconds(
                i64::try_from(self.cfg.oneclick.deadline_secs).unwrap_or(i64::MAX),
            ))
            .unwrap_or(now);
        let deadline = deadline
            .format(&Rfc3339)
            .context("format one-click deadline as RFC3339")?;

        let mut req = QuoteRequest::new(
            false,
            SwapType::ExactInput,
            self.cfg.oneclick.slippage_bps,
            self.cfg.oneclick.origin_asset.clone(),
            DepositType::OriginChain,
            self.cfg.oneclick.destination_asset.clone(),
            amount.to_string(),
            self.tron_wallet.address().to_string(),
            RefundType::OriginChain,
            self.cfg.oneclick.beneficiary.clone(),
            RecipientType::DestinationChain,
            deadline,
        );
        req.referral = self.cfg.oneclick.referral.clone();

        one_click_api::get_quote(&self.oneclick, req)
            .await
            .map_err(|e| anyhow::anyhow!("{e:?}"))
    }

    async fn broadcast_trc20_transfer(
        &mut self,
        token_contract: TronAddress,
        recipient: TronAddress,
        amount: alloy::primitives::U256,
    ) -> Result<[u8; 32]> {
        let data = encode_trc20_transfer(recipient.evm(), amount);

        let len = self.tron_urls.len();
        if len == 0 {
            anyhow::bail!("no TRON_GRPC_URLS configured");
        }

        let attempts = if len == 1 { 2 } else { len };
        for attempt in 0..attempts {
            let idx = (self.tron_url_cursor + attempt) % len;
            let force_reconnect = len == 1 && attempt > 0;
            if let Err(err) = self.ensure_tron_connected(idx, force_reconnect).await {
                tracing::warn!(
                    tron_grpc = %self.tron_urls[idx],
                    op = "broadcast_trc20_transfer",
                    err = %err,
                    "failed to connect Tron gRPC endpoint"
                );
                continue;
            }

            let res = {
                let tron = &mut self.tron;
                let wallet = &self.tron_wallet;
                let fee_policy = self.fee_policy;
                let energy_rental = &self.energy_rental;
                let energy_rental_cursor = &mut self.energy_rental_cursor;
                let telemetry = &self.telemetry;
                broadcast_trc20_transfer_single(
                    tron,
                    wallet,
                    fee_policy,
                    energy_rental,
                    energy_rental_cursor,
                    telemetry,
                    self.cfg.tron.energy_rental_settle_delay,
                    token_contract,
                    data.clone(),
                )
                .await
            };

            match res {
                Ok(v) => return Ok(v),
                Err(err) => {
                    tracing::warn!(
                        tron_grpc = %self.tron_urls[self.tron_url_cursor],
                        op = "broadcast_trc20_transfer",
                        err = %err,
                        "tron operation failed; trying next endpoint"
                    );
                }
            }
        }

        anyhow::bail!("all TRON_GRPC_URLS endpoints failed for op=broadcast_trc20_transfer")
    }
}

async fn broadcast_trc20_transfer_single(
    tron: &mut TronGrpc,
    wallet: &TronWallet,
    fee_policy: FeePolicy,
    energy_rental: &[JsonApiRentalProvider],
    energy_rental_cursor: &mut usize,
    telemetry: &PoolTelemetry,
    energy_rental_settle_delay: Duration,
    token_contract: TronAddress,
    data: Vec<u8>,
) -> Result<[u8; 32]> {
    const MIN_ENERGY_RENTAL_AMOUNT: u64 = 32_000;

    let signed = wallet
        .build_and_sign_trigger_smart_contract(tron, token_contract, data, 0, fee_policy)
        .await?;

    // Preflight balance check: nodes commonly require `balance >= fee_limit` even if resources are rented.
    let account = tron
        .get_account(wallet.address().prefixed_bytes().to_vec())
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
    if !energy_rental.is_empty() {
        let res = tron
            .get_account_resource(wallet.address().prefixed_bytes().to_vec())
            .await?;
        let parsed = tron::resources::parse_account_resources(&res)?;
        let shortfall = signed
            .energy_required
            .saturating_sub(parsed.energy_available());

        tracing::info!(
            energy_required = signed.energy_required,
            energy_available = parsed.energy_available(),
            energy_shortfall = shortfall,
            min_rental_energy = MIN_ENERGY_RENTAL_AMOUNT,
            energy_rental_providers = energy_rental.len(),
            "energy rental decision"
        );

        if shortfall >= MIN_ENERGY_RENTAL_AMOUNT {
            let addr = wallet.address();
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

            let len = energy_rental.len();
            let start_cursor = *energy_rental_cursor;
            let order = rental_try_indices(start_cursor, len);

            let mut ok = false;
            for (attempt, idx) in order.into_iter().enumerate() {
                let p = &energy_rental[idx];
                let rent_start = std::time::Instant::now();
                match p.rent(&ctx).await {
                    Ok(attempt_res) if attempt_res.ok => {
                        telemetry.energy_rental_ms(
                            &attempt_res.provider,
                            true,
                            rent_start.elapsed().as_millis() as u64,
                        );
                        tracing::info!(
                            provider = %attempt_res.provider,
                            order_id = attempt_res.order_id.as_deref().unwrap_or(""),
                            energy = shortfall,
                            "energy rental ok"
                        );
                        *energy_rental_cursor =
                            rental_cursor_after_attempts(start_cursor, len, attempt + 1);
                        ok = true;
                        break;
                    }
                    Ok(attempt_res) => {
                        telemetry.energy_rental_ms(
                            &attempt_res.provider,
                            false,
                            rent_start.elapsed().as_millis() as u64,
                        );
                        tracing::warn!(
                            provider = %attempt_res.provider,
                            err = attempt_res.error.as_deref().unwrap_or("rental failed"),
                            "energy rental failed; trying next provider"
                        );
                        *energy_rental_cursor =
                            rental_cursor_after_attempts(start_cursor, len, attempt + 1);
                    }
                    Err(err) => {
                        telemetry.energy_rental_ms(
                            p.name(),
                            false,
                            rent_start.elapsed().as_millis() as u64,
                        );
                        tracing::warn!(
                            provider = %p.name(),
                            err = %err,
                            "energy rental errored; trying next provider"
                        );
                        *energy_rental_cursor =
                            rental_cursor_after_attempts(start_cursor, len, attempt + 1);
                    }
                }
            }

            if !ok {
                tracing::warn!(
                    energy = shortfall,
                    "all energy rental providers failed; falling back to paying TRX fees"
                );
            } else if !energy_rental_settle_delay.is_zero() {
                tracing::info!(
                    delay_ms = energy_rental_settle_delay.as_millis() as u64,
                    "waiting briefly for rented energy to settle"
                );
                tokio::time::sleep(energy_rental_settle_delay).await;
            }
        } else if shortfall > 0 {
            tracing::info!(
                energy = shortfall,
                min_energy = MIN_ENERGY_RENTAL_AMOUNT,
                "energy shortfall below minimum rental amount; skipping rental"
            );
        }
    }

    let ret = tron.broadcast_transaction(signed.tx).await?;
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

    tracing::info!(
        txid = %hex::encode(signed.txid),
        fee_limit_sun = signed.fee_limit_sun,
        energy_required = signed.energy_required,
        "tron tx broadcast ok"
    );

    Ok(signed.txid)
}

fn encode_trc20_transfer(
    to: alloy::primitives::Address,
    amount: alloy::primitives::U256,
) -> Vec<u8> {
    let selector = alloy::primitives::keccak256("transfer(address,uint256)".as_bytes());
    let mut out = Vec::with_capacity(4 + 32 + 32);
    out.extend_from_slice(&selector[..4]);

    let mut addr_word = [0u8; 32];
    addr_word[12..].copy_from_slice(to.as_slice());
    out.extend_from_slice(&addr_word);

    let amt_word: [u8; 32] = amount.to_be_bytes();
    out.extend_from_slice(&amt_word);
    out
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
    use alloy::primitives::{Address, U256, keccak256};
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;

    #[test]
    fn oneclick_client_accepts_valid_headers_and_rejects_invalid() {
        // Valid: should construct the client.
        assert!(build_oneclick_client("ua-test/1.2.3", Some("secret")).is_ok());
        assert!(build_oneclick_client("ua-test/1.2.3", None).is_ok());

        // Invalid header value should be rejected (HeaderValue does not allow newlines).
        let err = build_oneclick_client("ua-test/1.2.3", Some("bad\nvalue"))
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("ONECLICK_BEARER_TOKEN") || err.contains("oneclick"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn usdt_keep_units_leaves_one_min_unit() {
        let keep = usdt_keep_units();
        assert_eq!(U256::ZERO.saturating_sub(keep), U256::ZERO);
        assert_eq!(U256::from(1u64).saturating_sub(keep), U256::ZERO);
        assert_eq!(U256::from(2u64).saturating_sub(keep), U256::from(1u64));
        assert_eq!(
            U256::from(10_000_000u64).saturating_sub(keep),
            U256::from(9_999_999u64)
        );
    }

    // NOTE: This is ignored by default because some sandboxed CI environments disallow
    // binding to localhost sockets. Run manually in a normal dev environment with:
    // `cargo test -p pool -- --ignored`
    #[tokio::test]
    #[ignore]
    async fn oneclick_client_sends_user_agent_and_optional_bearer_auth() {
        fn start_one_shot_http_server() -> (String, mpsc::Receiver<String>) {
            let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
            let addr = listener.local_addr().expect("local_addr");
            let (tx, rx) = mpsc::channel::<String>();

            std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().expect("accept");
                let mut buf = Vec::new();
                let mut tmp = [0u8; 4096];
                loop {
                    let n = stream.read(&mut tmp).expect("read");
                    if n == 0 {
                        break;
                    }
                    buf.extend_from_slice(&tmp[..n]);
                    if buf.windows(4).any(|w| w == b"\r\n\r\n") {
                        break;
                    }
                }
                let req = String::from_utf8_lossy(&buf).into_owned();
                let _ = tx.send(req);
                let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK");
            });

            (format!("http://{addr}/"), rx)
        }

        let (url, rx) = start_one_shot_http_server();
        let client = build_oneclick_client("ua-test/1.2.3", Some("secret")).unwrap();
        let resp = client.get(url).send().await.unwrap();
        assert!(resp.status().is_success());

        let req = rx.recv().unwrap();
        let lower = req.to_ascii_lowercase();
        assert!(lower.contains("user-agent: ua-test/1.2.3"));
        assert!(lower.contains("authorization: bearer secret"));

        let (url2, rx2) = start_one_shot_http_server();
        let client2 = build_oneclick_client("ua-test/1.2.3", None).unwrap();
        let resp2 = client2.get(url2).send().await.unwrap();
        assert!(resp2.status().is_success());

        let req2 = rx2.recv().unwrap();
        let lower2 = req2.to_ascii_lowercase();
        assert!(lower2.contains("user-agent: ua-test/1.2.3"));
        assert!(!lower2.contains("authorization:"));
    }

    #[test]
    fn encode_trc20_transfer_layout() {
        let to = Address::from_slice(&[0x11u8; 20]);
        let amount = U256::from(123_456_789u64);
        let out = encode_trc20_transfer(to, amount);

        let selector = keccak256("transfer(address,uint256)".as_bytes());
        assert_eq!(&out[..4], &selector[..4]);

        let mut addr_word = [0u8; 32];
        addr_word[12..].copy_from_slice(to.as_slice());
        assert_eq!(&out[4..36], &addr_word);

        let amt_word: [u8; 32] = amount.to_be_bytes();
        assert_eq!(&out[36..68], &amt_word);
        assert_eq!(out.len(), 68);
    }

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
