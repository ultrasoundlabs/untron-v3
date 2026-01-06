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

    pub async fn health(&self) -> Result<()> {
        let rows = self
            .client
            .health_get()
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("indexer health_get: {e:?}"))?
            .into_inner();
        let status = rows
            .first()
            .and_then(|r| r.status.as_deref())
            .unwrap_or_default();
        if status != "ok" {
            anyhow::bail!("indexer unhealthy: status={status:?}");
        }
        Ok(())
    }

    pub async fn stream_ingest_summary(&self) -> Result<Vec<types::StreamIngestSummary>> {
        self.client
            .stream_ingest_summary_get()
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("stream_ingest_summary_get: {e:?}"))
            .map(|r| r.into_inner())
    }

    pub async fn receiver_usdt_indexer_status(
        &self,
    ) -> Result<Option<types::ReceiverUsdtIndexerStatus>> {
        let rows = self
            .client
            .receiver_usdt_indexer_status_get()
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("receiver_usdt_indexer_status_get: {e:?}"))?
            .into_inner();
        Ok(rows.into_iter().next())
    }

    pub async fn receiver_usdt_balances(&self) -> Result<Vec<types::ReceiverUsdtBalances>> {
        self.client
            .receiver_usdt_balances_get()
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("receiver_usdt_balances_get: {e:?}"))
            .map(|r| r.into_inner())
    }

    pub async fn latest_event_appended(
        &self,
        stream: &str,
    ) -> Result<Option<types::EventAppended>> {
        let stream_filter = format!("eq.{stream}");
        let rows = self
            .client
            .event_appended_get()
            .stream(stream_filter)
            .order("event_seq.desc")
            .limit("1")
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("event_appended_get latest: {e:?}"))?
            .into_inner();
        Ok(rows.into_iter().next())
    }

    pub async fn controller_tip_proof(
        &self,
        proved_tip: &str,
    ) -> Result<Option<types::ControllerTipProofs>> {
        let proved_tip_filter = format!("eq.{proved_tip}");
        let rows = self
            .client
            .controller_tip_proofs_get()
            .proved_tip(proved_tip_filter)
            .order("block_number.desc")
            .limit("1")
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("controller_tip_proofs_get: {e:?}"))?
            .into_inner();
        Ok(rows.into_iter().next())
    }

    pub async fn controller_events_from_seq(
        &self,
        from_exclusive: i64,
        limit: u64,
    ) -> Result<Vec<types::EventAppended>> {
        let event_seq_filter = format!("gt.{from_exclusive}");
        self.client
            .event_appended_get()
            .stream("eq.controller")
            .event_seq(event_seq_filter)
            .order("event_seq.asc")
            .limit(limit.to_string())
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("event_appended_get controller range: {e:?}"))
            .map(|r| r.into_inner())
    }

    pub async fn receiver_usdt_transfer_actionability_pre_entitle(
        &self,
        limit: u64,
    ) -> Result<Vec<types::ReceiverUsdtTransferActionability>> {
        self.client
            .receiver_usdt_transfer_actionability_get()
            .recommended_action("eq.pre_entitle")
            .order("block_number.asc")
            .limit(limit.to_string())
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("receiver_usdt_transfer_actionability_get: {e:?}"))
            .map(|r| r.into_inner())
    }

    pub async fn hub_protocol_config(&self) -> Result<Option<types::HubProtocolConfig>> {
        let rows = self
            .client
            .hub_protocol_config_get()
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("hub_protocol_config_get: {e:?}"))?
            .into_inner();
        Ok(rows.into_iter().next())
    }

    pub async fn controller_usdt(&self) -> Result<Option<types::ControllerUsdt>> {
        let rows = self
            .client
            .controller_usdt_get()
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("controller_usdt_get: {e:?}"))?
            .into_inner();
        Ok(rows.into_iter().next())
    }

    pub async fn controller_payloads(&self) -> Result<Vec<types::ControllerPayloads>> {
        self.client
            .controller_payloads_get()
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("controller_payloads_get: {e:?}"))
            .map(|r| r.into_inner())
    }

    pub async fn hub_claims_created_for_token(
        &self,
        target_token: &str,
        limit: u64,
    ) -> Result<Vec<types::HubClaims>> {
        let token_filter = format!("eq.{target_token}");
        self.client
            .hub_claims_get()
            .target_token(token_filter)
            .status("eq.created")
            .order("queue_index.asc")
            .limit(limit.to_string())
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("hub_claims_get(created): {e:?}"))
            .map(|r| r.into_inner())
    }

    pub async fn hub_claims_created(&self, limit: u64) -> Result<Vec<types::HubClaims>> {
        self.client
            .hub_claims_get()
            .status("eq.created")
            .order("target_token.asc,queue_index.asc")
            .limit(limit.to_string())
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("hub_claims_get(created all): {e:?}"))
            .map(|r| r.into_inner())
    }
}
