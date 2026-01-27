use anyhow::{Context, Result};
use serde::Deserialize;
use std::time::{Duration, Instant};
use tracing::Instrument;
use untron_v3_indexer_client::{Client, types};

use crate::metrics::RelayerTelemetry;

#[derive(Debug, Clone, Deserialize)]
pub struct RelayerHubState {
    pub last_controller_event_tip: String,
    pub last_controller_event_seq: serde_json::Number,
    pub next_controller_event_index: serde_json::Number,
}

pub struct IndexerApi {
    base_url: String,
    http: reqwest::Client,
    client: Client,
    telemetry: RelayerTelemetry,
}

impl IndexerApi {
    pub fn new(base_url: &str, timeout: Duration, telemetry: RelayerTelemetry) -> Result<Self> {
        let base_url = base_url.trim_end_matches('/').to_string();
        let http = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .context("build indexer http client")?;
        let client = Client::new_with_client(&base_url, http.clone());
        Ok(Self {
            base_url,
            http,
            client,
            telemetry,
        })
    }

    async fn timed<T>(
        &self,
        op: &'static str,
        f: impl std::future::Future<Output = Result<T>>,
    ) -> Result<T> {
        let span = tracing::debug_span!("indexer.http", op);
        let start = Instant::now();
        let res = f.instrument(span).await;
        self.telemetry
            .indexer_http_ms(op, res.is_ok(), start.elapsed().as_millis() as u64);
        res
    }

    pub async fn health(&self) -> Result<()> {
        let rows = self
            .timed("health_get", async {
                self.client
                    .health_get()
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("indexer health_get: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
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
        self.timed("stream_ingest_summary_get", async {
            self.client
                .stream_ingest_summary_get()
                .send()
                .await
                .map_err(|e| anyhow::anyhow!("stream_ingest_summary_get: {e:?}"))
                .map(|r| r.into_inner())
        })
        .await
    }

    pub async fn relayer_hub_state(&self) -> Result<RelayerHubState> {
        self.timed("relayer_hub_state_get", async {
            let url = format!("{}/relayer_hub_state", self.base_url);
            let res = self
                .http
                .get(url)
                .send()
                .await
                .map_err(|e| anyhow::anyhow!("relayer_hub_state_get: {e:?}"))?;

            let status = res.status();
            if !status.is_success() {
                let body = res.text().await.unwrap_or_default();
                anyhow::bail!("relayer_hub_state_get: http {status} body={body:?}");
            }

            res.json::<Vec<RelayerHubState>>()
                .await
                .map_err(|e| anyhow::anyhow!("relayer_hub_state_get decode: {e:?}"))
                .and_then(|mut rows| {
                    rows.pop()
                        .ok_or_else(|| anyhow::anyhow!("relayer_hub_state_get: empty response"))
                })
        })
        .await
    }

    pub async fn receiver_usdt_indexer_status(
        &self,
    ) -> Result<Option<types::ReceiverUsdtIndexerStatus>> {
        let rows = self
            .timed("receiver_usdt_indexer_status_get", async {
                self.client
                    .receiver_usdt_indexer_status_get()
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("receiver_usdt_indexer_status_get: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    pub async fn receiver_usdt_balances(&self) -> Result<Vec<types::ReceiverUsdtBalances>> {
        self.timed("receiver_usdt_balances_get", async {
            self.client
                .receiver_usdt_balances_get()
                .send()
                .await
                .map_err(|e| anyhow::anyhow!("receiver_usdt_balances_get: {e:?}"))
                .map(|r| r.into_inner())
        })
        .await
    }

    pub async fn latest_event_appended(
        &self,
        stream: &str,
    ) -> Result<Option<types::EventAppended>> {
        let stream_filter = format!("eq.{stream}");
        let rows = self
            .timed("event_appended_get_latest", async {
                self.client
                    .event_appended_get()
                    .stream(stream_filter)
                    .order("event_seq.desc")
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("event_appended_get latest: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    pub async fn controller_tip_proof(
        &self,
        proved_tip: &str,
    ) -> Result<Option<types::ControllerTipProofs>> {
        let proved_tip_filter = format!("eq.{proved_tip}");
        let rows = self
            .timed("controller_tip_proofs_get", async {
                self.client
                    .controller_tip_proofs_get()
                    .proved_tip(proved_tip_filter)
                    .order("block_number.desc")
                    .limit("1")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("controller_tip_proofs_get: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    pub async fn controller_events_from_seq(
        &self,
        from_exclusive: i64,
        limit: u64,
    ) -> Result<Vec<types::EventAppended>> {
        let event_seq_filter = format!("gt.{from_exclusive}");
        self.timed("event_appended_get_controller_range", async {
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
        })
        .await
    }

    pub async fn receiver_usdt_transfer_actionability_pre_entitle(
        &self,
        limit: u64,
    ) -> Result<Vec<types::ReceiverUsdtTransferActionability>> {
        self.timed("receiver_usdt_transfer_actionability_get", async {
            self.client
                .receiver_usdt_transfer_actionability_get()
                .recommended_action("eq.pre_entitle")
                .order("block_number.asc")
                .limit(limit.to_string())
                .send()
                .await
                .map_err(|e| anyhow::anyhow!("receiver_usdt_transfer_actionability_get: {e:?}"))
                .map(|r| r.into_inner())
        })
        .await
    }

    pub async fn hub_protocol_config(&self) -> Result<Option<types::HubProtocolConfig>> {
        let rows = self
            .timed("hub_protocol_config_get", async {
                self.client
                    .hub_protocol_config_get()
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("hub_protocol_config_get: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    pub async fn hub_swap_rates(&self) -> Result<Vec<types::HubSwapRates>> {
        self.timed("hub_swap_rates_get", async {
            self.client
                .hub_swap_rates_get()
                .send()
                .await
                .map_err(|e| anyhow::anyhow!("hub_swap_rates_get: {e:?}"))
                .map(|r| r.into_inner())
        })
        .await
    }

    pub async fn controller_usdt(&self) -> Result<Option<types::ControllerUsdt>> {
        let rows = self
            .timed("controller_usdt_get", async {
                self.client
                    .controller_usdt_get()
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("controller_usdt_get: {e:?}"))
                    .map(|r| r.into_inner())
            })
            .await?;
        Ok(rows.into_iter().next())
    }

    pub async fn controller_payloads(&self) -> Result<Vec<types::ControllerPayloads>> {
        self.timed("controller_payloads_get", async {
            self.client
                .controller_payloads_get()
                .send()
                .await
                .map_err(|e| anyhow::anyhow!("controller_payloads_get: {e:?}"))
                .map(|r| r.into_inner())
        })
        .await
    }

    pub async fn hub_claims_created_for_token(
        &self,
        target_token: &str,
        limit: u64,
    ) -> Result<Vec<types::HubClaims>> {
        let token_filter = format!("eq.{target_token}");
        self.timed("hub_claims_get_created_for_token", async {
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
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relayer_hub_state_deserializes_from_postgrest_row() {
        let json = format!(
            r#"[{{"last_controller_event_tip":"0x{tip}","last_controller_event_seq":1,"next_controller_event_index":2}}]"#,
            tip = "11".repeat(32)
        );
        let rows: Vec<RelayerHubState> = serde_json::from_str(&json).unwrap();
        let row = rows.into_iter().next().unwrap();
        assert!(row.last_controller_event_tip.starts_with("0x"));
        assert_eq!(row.last_controller_event_seq.to_string(), "1");
        assert_eq!(row.next_controller_event_index.to_string(), "2");
    }
}
