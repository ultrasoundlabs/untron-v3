use anyhow::{Context, Result};
use std::time::Duration;
use untron_v3_indexer_client::{Client, types};

pub struct IndexerApi {
    client: Client,
}

impl IndexerApi {
    pub fn new(base_url: &str, timeout: Duration) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .context("build indexer http client")?;
        Ok(Self {
            client: Client::new_with_client(base_url.trim_end_matches('/'), client),
        })
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn beneficiary_has_filled_claims(
        &self,
        beneficiary_addr_lower_hex: &str,
    ) -> Result<bool> {
        let beneficiary_filter = format!("eq.{beneficiary_addr_lower_hex}");
        let rows = self
            .client
            .hub_claims_get()
            .beneficiary(beneficiary_filter)
            .status("eq.filled")
            .select("lease_id")
            .limit("1")
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("hub_claims_get filled by beneficiary: {e:?}"))?
            .into_inner();
        Ok(!rows.is_empty())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn realtor_effective_config(
        &self,
        realtor_addr_lower_hex: &str,
    ) -> Result<Option<types::RealtorEffectiveConfig>> {
        let realtor_filter = format!("eq.{realtor_addr_lower_hex}");
        let rows = self
            .client
            .realtor_effective_config_get()
            .realtor(realtor_filter)
            .limit("1")
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("realtor_effective_config_get: {e:?}"))?
            .into_inner();
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
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn receiver_salt_candidate(
        &self,
        receiver_salt_hex: &str,
    ) -> Result<Option<types::ReceiverSaltCandidates>> {
        let receiver_salt_filter = format!("eq.{receiver_salt_hex}");
        let rows = self
            .client
            .receiver_salt_candidates_get()
            .receiver_salt(receiver_salt_filter)
            .limit("1")
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("receiver_salt_candidates_get by salt: {e:?}"))?
            .into_inner();
        Ok(rows.into_iter().next())
    }
}
