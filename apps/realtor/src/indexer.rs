use anyhow::{Context, Result};
use std::time::{Duration, Instant};
use untron_v3_indexer_client::{Client, types};

use crate::metrics::RealtorTelemetry;

pub struct IndexerApi {
    client: Client,
    telemetry: RealtorTelemetry,
}

#[derive(Debug, Clone)]
pub struct BridgerPair {
    pub target_token: String,
    pub target_chain_id: u64,
}

impl IndexerApi {
    pub fn new(base_url: &str, timeout: Duration, telemetry: RealtorTelemetry) -> Result<Self> {
        let base_url = base_url.trim_end_matches('/').to_string();
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .context("build indexer http client")?;
        Ok(Self {
            client: Client::new_with_client(&base_url, client),
            telemetry,
        })
    }

    async fn timed<T>(
        &self,
        op: &'static str,
        f: impl std::future::Future<Output = Result<T>>,
    ) -> Result<T> {
        let start = Instant::now();
        let res = f.await;
        self.telemetry
            .indexer_http_ms(op, res.is_ok(), start.elapsed().as_millis() as u64);
        res
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn beneficiary_has_filled_claims(
        &self,
        beneficiary_addr_checksum: &str,
    ) -> Result<bool> {
        let beneficiary_filter = format!("eq.{beneficiary_addr_checksum}");
        let rows = self
            .timed("hub_claims_get_filled_by_beneficiary", async {
                self.client
                    .hub_claims_get()
                    .beneficiary(beneficiary_filter)
                    .status("eq.filled")
                    .select("lease_id")
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("hub_claims_get filled by beneficiary: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(!rows.is_empty())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn realtor_effective_config(
        &self,
        realtor_addr_checksum: &str,
    ) -> Result<Option<types::RealtorEffectiveConfig>> {
        let realtor_filter = format!("eq.{realtor_addr_checksum}");
        let rows = self
            .timed("realtor_effective_config_get", async {
                self.client
                    .realtor_effective_config_get()
                    .realtor(realtor_filter)
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("realtor_effective_config_get: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn receiver_salt_candidates(
        &self,
        order: &str,
        limit: u64,
        require_free: bool,
        require_nonzero_balance: bool,
    ) -> Result<Vec<types::ReceiverSaltCandidates>> {
        self.timed("receiver_salt_candidates_get", async {
            let mut req = self
                .client
                .receiver_salt_candidates_get()
                .order(order)
                .limit(limit.to_string());

            if require_free {
                req = req.is_free("eq.true");
            }
            if require_nonzero_balance {
                req = req.has_balance("eq.true");
            }

            req.send()
                .await
                .map_err(|e| anyhow::anyhow!("receiver_salt_candidates_get: {e:?}"))
                .map(|r| r.into_inner())
        })
        .await
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn bridger_pairs_current(&self) -> Result<Vec<BridgerPair>> {
        let rows = self
            .timed("hub_bridgers_get_current", async {
                self.client
                    .hub_bridgers_get()
                    .valid_to_seq("is.null")
                    .select("target_token,target_chain_id")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("hub_bridgers_get current: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;

        let mut out = Vec::new();
        for r in rows {
            let Some(target_token) = r.target_token else {
                continue;
            };
            let Some(target_chain_id) = r.target_chain_id.and_then(|v| u64::try_from(v).ok())
            else {
                continue;
            };
            out.push(BridgerPair {
                target_token,
                target_chain_id,
            });
        }
        Ok(out)
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn bridger_pair_is_supported(
        &self,
        target_token_checksum: &str,
        target_chain_id: u64,
    ) -> Result<bool> {
        let token_filter = format!("eq.{}", target_token_checksum);
        let chain_filter = format!("eq.{}", target_chain_id);
        let rows = self
            .timed("hub_bridgers_get_by_pair", async {
                self.client
                    .hub_bridgers_get()
                    .target_token(token_filter)
                    .target_chain_id(chain_filter)
                    .valid_to_seq("is.null")
                    .select("target_token")
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("hub_bridgers_get by pair: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(!rows.is_empty())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn receiver_salt_candidate(
        &self,
        receiver_salt_hex: &str,
    ) -> Result<Option<types::ReceiverSaltCandidates>> {
        let receiver_salt_filter = format!("eq.{receiver_salt_hex}");
        let rows = self
            .timed("receiver_salt_candidates_get_by_salt", async {
                self.client
                    .receiver_salt_candidates_get()
                    .receiver_salt(receiver_salt_filter)
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("receiver_salt_candidates_get by salt: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn latest_lease_by_receiver_salt(
        &self,
        receiver_salt_hex: &str,
    ) -> Result<Option<types::HubLeases>> {
        let receiver_salt_filter = format!("eq.{receiver_salt_hex}");
        let rows = self
            .timed("hub_leases_get_latest_by_receiver_salt", async {
                self.client
                    .hub_leases_get()
                    .receiver_salt(receiver_salt_filter)
                    .order("lease_number.desc")
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("hub_leases_get latest by receiver_salt: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn hub_lease(&self, lease_id: u64) -> Result<Option<types::HubLeases>> {
        let lease_filter = format!("eq.{lease_id}");
        let rows = self
            .timed("hub_leases_get_by_lease_id", async {
                self.client
                    .hub_leases_get()
                    .lease_id(lease_filter)
                    .valid_to_seq("is.null")
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("hub_leases_get by lease_id: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn hub_lease_nonce(&self, lease_id: u64) -> Result<Option<types::HubLeaseNonces>> {
        let lease_filter = format!("eq.{lease_id}");
        let rows = self
            .timed("hub_lease_nonces_get_by_lease_id", async {
                self.client
                    .hub_lease_nonces_get()
                    .lease_id(lease_filter)
                    .valid_to_seq("is.null")
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("hub_lease_nonces_get by lease_id: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn hub_protocol_config(&self) -> Result<Option<types::HubProtocolConfig>> {
        let rows = self
            .timed("hub_protocol_config_get", async {
                self.client
                    .hub_protocol_config_get()
                    .valid_to_seq("is.null")
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("hub_protocol_config_get: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn hub_swap_rate(
        &self,
        target_token_checksum: &str,
    ) -> Result<Option<types::HubSwapRates>> {
        let token_filter = format!("eq.{target_token_checksum}");
        let rows = self
            .timed("hub_swap_rates_get_by_token", async {
                self.client
                    .hub_swap_rates_get()
                    .target_token(token_filter)
                    .valid_to_seq("is.null")
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("hub_swap_rates_get by target_token: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn hub_chain(&self, target_chain_id: u64) -> Result<Option<types::HubChains>> {
        let chain_filter = format!("eq.{target_chain_id}");
        let rows = self
            .timed("hub_chains_get_by_chain_id", async {
                self.client
                    .hub_chains_get()
                    .target_chain_id(chain_filter)
                    .valid_to_seq("is.null")
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("hub_chains_get by target_chain_id: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    /// Fetches the aggregated `api.lease_view` row (single PostgREST request).
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn lease_view_row(&self, lease_id: u64) -> Result<Option<types::LeaseView>> {
        let lease_filter = format!("eq.{lease_id}");
        let rows = self
            .timed("lease_view_get_by_lease_id", async {
                self.client
                    .lease_view_get()
                    .lease_id(lease_filter)
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("lease_view_get by lease_id: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn receiver_addresses_by_salt(
        &self,
        receiver_salt_hex: &str,
    ) -> Result<Option<(String, String)>> {
        let salt_filter = format!("eq.{receiver_salt_hex}");
        let rows = self
            .timed("receiver_usdt_balances_get_by_salt", async {
                self.client
                    .receiver_usdt_balances_get()
                    .receiver_salt(salt_filter)
                    .select("receiver,receiver_evm")
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!("receiver_usdt_balances_get by receiver_salt: {e:?}")
                    })
                    .map(|r| r.into_inner())
            })
            .await?;

        let Some(row) = rows.into_iter().next() else {
            return Ok(None);
        };
        let receiver = row
            .receiver
            .ok_or_else(|| anyhow::anyhow!("receiver_usdt_balances missing receiver"))?;
        let receiver_evm = row
            .receiver_evm
            .ok_or_else(|| anyhow::anyhow!("receiver_usdt_balances missing receiver_evm"))?;
        Ok(Some((receiver, receiver_evm)))
    }
}
