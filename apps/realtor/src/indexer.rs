use anyhow::{Context, Result};
use serde::Deserialize;
use std::time::Duration;
use untron_v3_indexer_client::apis::{self, configuration::Configuration};

#[derive(Debug, Clone, Deserialize)]
pub struct RealtorEffectiveConfigRow {
    pub allowed: bool,

    pub min_fee_ppm: i64,
    pub min_flat_fee: serde_json::Number,
    pub max_duration_seconds: i64,

    pub lease_rate_max_leases: serde_json::Number,
    pub lease_rate_window_seconds: serde_json::Number,
    #[serde(default)]
    pub lease_rate_remaining: Option<serde_json::Number>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReceiverSaltCandidateRow {
    #[serde(default)]
    pub receiver_salt: Option<String>,
    #[serde(default)]
    pub has_balance: Option<bool>,
    #[serde(default)]
    pub nukeable_after: Option<i64>,
    #[serde(default)]
    pub is_free: Option<bool>,
}

pub struct IndexerApi {
    cfg: Configuration,
}

impl IndexerApi {
    pub fn new(base_url: &str, timeout: Duration) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .context("build indexer http client")?;
        Ok(Self {
            cfg: Configuration {
                base_path: base_url.trim_end_matches('/').to_string(),
                client,
                ..Default::default()
            },
        })
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn beneficiary_has_filled_claims(
        &self,
        beneficiary_addr_lower_hex: &str,
    ) -> Result<bool> {
        let beneficiary_filter = format!("eq.{beneficiary_addr_lower_hex}");
        let rows = apis::hub_claims_api::hub_claims_get(
            &self.cfg,
            None,                              // lease_id
            None,                              // claim_id
            None,                              // valid_from_seq
            None,                              // valid_to_seq
            None,                              // target_token
            None,                              // queue_index
            None,                              // amount_usdt
            None,                              // target_chain_id
            Some(beneficiary_filter.as_str()), // beneficiary
            None,                              // origin
            None,                              // origin_id
            None,                              // origin_actor
            None,                              // origin_token
            None,                              // origin_timestamp
            None,                              // origin_raw_amount
            Some("eq.filled"),                 // status
            Some("lease_id"),                  // select
            None,                              // order
            None,                              // range
            None,                              // range_unit
            None,                              // offset
            Some("1"),                         // limit
            None,                              // prefer
        )
        .await
        .context("hub_claims_get filled by beneficiary")?;
        Ok(!rows.is_empty())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn realtor_effective_config(
        &self,
        realtor_addr_lower_hex: &str,
    ) -> Result<Option<RealtorEffectiveConfigRow>> {
        let url = format!("{}/realtor_effective_config", self.cfg.base_path);
        let realtor_filter = format!("eq.{realtor_addr_lower_hex}");
        let resp = self
            .cfg
            .client
            .get(url)
            .query(&[("realtor", realtor_filter.as_str()), ("limit", "1")])
            .send()
            .await
            .context("GET realtor_effective_config")?;
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("realtor_effective_config http {status}: {text}");
        }
        let rows = resp
            .json::<Vec<RealtorEffectiveConfigRow>>()
            .await
            .context("decode realtor_effective_config json")?;
        Ok(rows.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn receiver_salt_candidates(
        &self,
        order: &str,
        limit: u64,
        require_free: bool,
        require_nonzero_balance: bool,
    ) -> Result<Vec<ReceiverSaltCandidateRow>> {
        let url = format!("{}/receiver_salt_candidates", self.cfg.base_path);
        let limit = limit.to_string();
        let mut req = self
            .cfg
            .client
            .get(url)
            .query(&[("order", order), ("limit", limit.as_str())]);

        if require_free {
            req = req.query(&[("is_free", "eq.true")]);
        }
        if require_nonzero_balance {
            req = req.query(&[("has_balance", "eq.true")]);
        }

        let resp = req.send().await.context("GET receiver_salt_candidates")?;
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("receiver_salt_candidates http {status}: {text}");
        }
        resp.json::<Vec<ReceiverSaltCandidateRow>>()
            .await
            .context("decode receiver_salt_candidates json")
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn receiver_salt_candidate(
        &self,
        receiver_salt_hex: &str,
    ) -> Result<Option<ReceiverSaltCandidateRow>> {
        let url = format!("{}/receiver_salt_candidates", self.cfg.base_path);
        let receiver_salt_filter = format!("eq.{receiver_salt_hex}");
        let resp = self
            .cfg
            .client
            .get(url)
            .query(&[
                ("receiver_salt", receiver_salt_filter.as_str()),
                ("limit", "1"),
            ])
            .send()
            .await
            .context("GET receiver_salt_candidates by salt")?;
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("receiver_salt_candidates http {status}: {text}");
        }
        let rows = resp
            .json::<Vec<ReceiverSaltCandidateRow>>()
            .await
            .context("decode receiver_salt_candidates json")?;
        Ok(rows.into_iter().next())
    }
}
