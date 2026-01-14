use crate::backoff::{BackoffConfig, BackoffState};
use crate::metrics::PoolTelemetry;
use anyhow::Result;
use one_click_sdk_rs::apis::{
    configuration::Configuration as OneClickConfiguration, one_click_api,
};
use one_click_sdk_rs::models::get_execution_status_response::Status;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct WatchConfig {
    pub poll_interval: Duration,
    pub max_wait: Duration,
}

#[derive(Debug, Clone)]
pub struct WatchRequest {
    pub deposit_address: String,
    pub origin_tx_hash: String,
}

#[derive(Debug, Clone)]
struct WatchItem {
    deposit_address: String,
    origin_tx_hash: String,
    created_at: Instant,
    last_status: Option<Status>,
    next_poll_at: Instant,
}

pub struct StatusWatcher {
    cfg: WatchConfig,
    oneclick: OneClickConfiguration,
    telemetry: PoolTelemetry,
    rx: mpsc::Receiver<WatchRequest>,
    active: HashMap<String, WatchItem>,
    backoff_cfg: BackoffConfig,
    backoff: Arc<Mutex<BackoffState>>,
}

impl StatusWatcher {
    pub fn new(
        cfg: WatchConfig,
        oneclick: OneClickConfiguration,
        telemetry: PoolTelemetry,
        rx: mpsc::Receiver<WatchRequest>,
        backoff_cfg: BackoffConfig,
        backoff: Arc<Mutex<BackoffState>>,
    ) -> Self {
        Self {
            cfg,
            oneclick,
            telemetry,
            rx,
            active: HashMap::new(),
            backoff_cfg,
            backoff,
        }
    }

    pub async fn run(mut self, shutdown: CancellationToken) -> Result<()> {
        let mut tick = tokio::time::interval(self.cfg.poll_interval.max(Duration::from_secs(1)));
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                _ = shutdown.cancelled() => break,
                maybe = self.rx.recv() => {
                    let Some(req) = maybe else { break };
                    self.insert(req);
                }
                _ = tick.tick() => {
                    self.poll_due().await?;
                }
            }
        }

        Ok(())
    }

    fn insert(&mut self, req: WatchRequest) {
        let key = req.deposit_address.trim().to_string();
        if key.is_empty() {
            return;
        }
        if self.active.contains_key(&key) {
            tracing::debug!(deposit_address = %key, "status watcher already tracking deposit");
            return;
        }

        let now = Instant::now();
        self.active.insert(
            key.clone(),
            WatchItem {
                deposit_address: key.clone(),
                origin_tx_hash: req.origin_tx_hash,
                created_at: now,
                last_status: None,
                next_poll_at: now,
            },
        );
        tracing::info!(deposit_address = %key, "status watcher tracking deposit");
    }

    async fn poll_due(&mut self) -> Result<()> {
        if self.active.is_empty() {
            return Ok(());
        }

        let now = Instant::now();
        let max_wait = self.cfg.max_wait;

        let due = self
            .active
            .values()
            .filter(|v| v.next_poll_at <= now)
            .map(|v| v.deposit_address.clone())
            .collect::<Vec<_>>();

        for deposit_address in due {
            let Some(item) = self.active.get_mut(&deposit_address) else {
                continue;
            };

            if max_wait > Duration::ZERO && now.duration_since(item.created_at) > max_wait {
                tracing::warn!(
                    deposit_address = %item.deposit_address,
                    origin_tx_hash = %item.origin_tx_hash,
                    waited_secs = now.duration_since(item.created_at).as_secs(),
                    "1click status watch timed out"
                );
                self.telemetry.oneclick_status_terminal("TIMEOUT");
                self.apply_backoff("TIMEOUT").await;
                self.active.remove(&deposit_address);
                continue;
            }

            let start = Instant::now();
            let res = one_click_api::get_execution_status(&self.oneclick, &item.deposit_address)
                .await
                .map_err(|e| anyhow::anyhow!("{e:?}"));
            let ms = start.elapsed().as_millis() as u64;

            match res {
                Ok(resp) => {
                    let status = resp.status;
                    let status_str = status_as_str(status);
                    self.telemetry.oneclick_status_poll_ms(true, status_str, ms);

                    if item.last_status != Some(status) {
                        tracing::info!(
                            deposit_address = %item.deposit_address,
                            origin_tx_hash = %item.origin_tx_hash,
                            status = status_str,
                            updated_at = %resp.updated_at,
                            intent_hashes = ?resp.swap_details.intent_hashes,
                            near_tx_hashes = ?resp.swap_details.near_tx_hashes,
                            origin_chain_txs = ?resp.swap_details.origin_chain_tx_hashes.iter().map(|t| t.hash.clone()).collect::<Vec<_>>(),
                            destination_chain_txs = ?resp.swap_details.destination_chain_tx_hashes.iter().map(|t| t.hash.clone()).collect::<Vec<_>>(),
                            "1click status updated"
                        );
                        item.last_status = Some(status);
                    }

                    if is_terminal(status) {
                        match status {
                            Status::Success => tracing::info!(
                                deposit_address = %item.deposit_address,
                                origin_tx_hash = %item.origin_tx_hash,
                                status = status_str,
                                updated_at = %resp.updated_at,
                                "1click status terminal"
                            ),
                            Status::Refunded | Status::Failed => tracing::warn!(
                                deposit_address = %item.deposit_address,
                                origin_tx_hash = %item.origin_tx_hash,
                                status = status_str,
                                updated_at = %resp.updated_at,
                                "1click status terminal"
                            ),
                            _ => tracing::info!(
                                deposit_address = %item.deposit_address,
                                origin_tx_hash = %item.origin_tx_hash,
                                status = status_str,
                                updated_at = %resp.updated_at,
                                "1click status terminal"
                            ),
                        }
                        self.telemetry.oneclick_status_terminal(status_str);
                        match status {
                            Status::Success => self.clear_backoff().await,
                            Status::Refunded => self.apply_backoff("REFUNDED").await,
                            Status::Failed => self.apply_backoff("FAILED").await,
                            _ => {}
                        }
                        self.active.remove(&deposit_address);
                    } else {
                        item.next_poll_at = now + self.cfg.poll_interval;
                    }
                }
                Err(err) => {
                    self.telemetry.oneclick_status_poll_ms(false, "ERR", ms);
                    tracing::warn!(
                        deposit_address = %item.deposit_address,
                        origin_tx_hash = %item.origin_tx_hash,
                        err = %err,
                        "1click status poll failed"
                    );
                    // Backoff a bit on errors.
                    item.next_poll_at = now + (self.cfg.poll_interval.max(Duration::from_secs(2)));
                }
            }
        }

        Ok(())
    }

    async fn apply_backoff(&self, reason: &'static str) {
        let mut st = self.backoff.lock().await;
        let dur = st.on_failure(self.backoff_cfg);
        let secs = dur.as_secs().max(1);
        self.telemetry.oneclick_backoff(reason, secs);
        tracing::warn!(reason, backoff_secs = secs, "entering 1click backoff");
    }

    async fn clear_backoff(&self) {
        let mut st = self.backoff.lock().await;
        st.on_success();
    }
}

fn is_terminal(status: Status) -> bool {
    matches!(status, Status::Success | Status::Refunded | Status::Failed)
}

fn status_as_str(status: Status) -> &'static str {
    match status {
        Status::KnownDepositTx => "KNOWN_DEPOSIT_TX",
        Status::PendingDeposit => "PENDING_DEPOSIT",
        Status::IncompleteDeposit => "INCOMPLETE_DEPOSIT",
        Status::Processing => "PROCESSING",
        Status::Success => "SUCCESS",
        Status::Refunded => "REFUNDED",
        Status::Failed => "FAILED",
    }
}
