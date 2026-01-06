use anyhow::{Context, Result};
use std::time::Duration;
use untron_v3_indexer_client::{
    apis::{self, configuration::Configuration},
    models,
};

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

    pub async fn health(&self) -> Result<()> {
        let rows =
            apis::health_api::health_get(&self.cfg, None, None, None, None, None, None, None, None)
                .await
                .context("indexer health_get")?;
        let status = rows
            .first()
            .and_then(|r| r.status.as_deref())
            .unwrap_or_default();
        if status != "ok" {
            anyhow::bail!("indexer unhealthy: status={status:?}");
        }
        Ok(())
    }

    pub async fn stream_ingest_summary(&self) -> Result<Vec<models::StreamIngestSummary>> {
        apis::stream_ingest_summary_api::stream_ingest_summary_get(
            &self.cfg, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None,
        )
        .await
        .context("stream_ingest_summary_get")
    }

    pub async fn receiver_usdt_indexer_status(
        &self,
    ) -> Result<Option<models::ReceiverUsdtIndexerStatus>> {
        let rows = apis::receiver_usdt_indexer_status_api::receiver_usdt_indexer_status_get(
            &self.cfg, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        )
        .await
        .context("receiver_usdt_indexer_status_get")?;
        Ok(rows.into_iter().next())
    }

    pub async fn receiver_usdt_balances(&self) -> Result<Vec<models::ReceiverUsdtBalances>> {
        apis::receiver_usdt_balances_api::receiver_usdt_balances_get(
            &self.cfg, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        )
        .await
        .context("receiver_usdt_balances_get")
    }

    pub async fn latest_event_appended(
        &self,
        stream: &str,
    ) -> Result<Option<models::EventAppended>> {
        let stream_filter = format!("eq.{stream}");
        let rows = apis::event_appended_api::event_appended_get(
            &self.cfg,
            Some(stream_filter.as_str()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("event_seq.desc"),
            None,
            None,
            None,
            Some("1"),
            None,
        )
        .await
        .context("event_appended_get latest")?;
        Ok(rows.into_iter().next())
    }

    pub async fn controller_tip_proof(
        &self,
        proved_tip: &str,
    ) -> Result<Option<models::ControllerTipProofs>> {
        let proved_tip_filter = format!("eq.{proved_tip}");
        let rows = apis::controller_tip_proofs_api::controller_tip_proofs_get(
            &self.cfg,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(proved_tip_filter.as_str()),
            None,
            Some("block_number.desc"),
            None,
            None,
            None,
            Some("1"),
            None,
        )
        .await
        .context("controller_tip_proofs_get")?;
        Ok(rows.into_iter().next())
    }

    pub async fn controller_events_from_seq(
        &self,
        from_exclusive: i64,
        limit: u64,
    ) -> Result<Vec<models::EventAppended>> {
        let event_seq_filter = format!("gt.{from_exclusive}");
        let limit_str = limit.to_string();
        apis::event_appended_api::event_appended_get(
            &self.cfg,
            Some("eq.controller"),           // stream
            Some(event_seq_filter.as_str()), // event_seq
            None,                            // prev_tip
            None,                            // new_tip
            None,                            // event_signature
            None,                            // abi_encoded_event_data
            None,                            // event_type
            None,                            // args
            None,                            // block_number
            None,                            // block_timestamp
            None,                            // block_time
            None,                            // block_hash
            None,                            // tx_hash
            None,                            // log_index
            None,                            // select
            Some("event_seq.asc"),           // order
            None,                            // range
            None,                            // range_unit
            None,                            // offset
            Some(limit_str.as_str()),        // limit
            None,                            // prefer
        )
        .await
        .context("event_appended_get controller range")
    }

    pub async fn receiver_usdt_transfer_actionability_pre_entitle(
        &self,
        limit: u64,
    ) -> Result<Vec<models::ReceiverUsdtTransferActionability>> {
        let limit_str = limit.to_string();
        apis::receiver_usdt_transfer_actionability_api::receiver_usdt_transfer_actionability_get(
            &self.cfg,
            None,                     // chain_id
            None,                     // token
            None,                     // receiver_salt
            None,                     // sender
            None,                     // recipient
            None,                     // amount
            None,                     // block_number
            None,                     // block_timestamp
            None,                     // block_time
            None,                     // block_hash
            None,                     // tx_hash
            None,                     // log_index
            None,                     // claim_origin
            None,                     // claim_lease_id
            None,                     // claim_id
            None,                     // claim_status
            None,                     // claim_amount_usdt
            None,                     // expected_lease_id
            None,                     // last_pull_timestamp
            None,                     // preentitle_time_ok
            Some("eq.pre_entitle"),   // recommended_action
            None,                     // select
            Some("block_number.asc"), // order
            None,                     // range
            None,                     // range_unit
            None,                     // offset
            Some(limit_str.as_str()), // limit
            None,                     // prefer
        )
        .await
        .context("receiver_usdt_transfer_actionability_get")
    }

    pub async fn hub_protocol_config(&self) -> Result<Option<models::HubProtocolConfig>> {
        let rows = apis::hub_protocol_config_api::hub_protocol_config_get(
            &self.cfg, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None,
        )
        .await
        .context("hub_protocol_config_get")?;
        Ok(rows.into_iter().next())
    }

    pub async fn controller_usdt(&self) -> Result<Option<models::ControllerUsdt>> {
        let rows = apis::controller_usdt_api::controller_usdt_get(
            &self.cfg, None, None, None, None, None, None, None, None, None, None,
        )
        .await
        .context("controller_usdt_get")?;
        Ok(rows.into_iter().next())
    }

    pub async fn controller_payloads(&self) -> Result<Vec<models::ControllerPayloads>> {
        apis::controller_payloads_api::controller_payloads_get(
            &self.cfg, None, None, None, None, None, None, None, None, None, None, None,
        )
        .await
        .context("controller_payloads_get")
    }

    pub async fn hub_claims_created_for_token(
        &self,
        target_token: &str,
        limit: u64,
    ) -> Result<Vec<models::HubClaims>> {
        let token_filter = format!("eq.{target_token}");
        let limit_str = limit.to_string();
        apis::hub_claims_api::hub_claims_get(
            &self.cfg,
            None,                        // lease_id
            None,                        // claim_id
            None,                        // valid_from_seq
            None,                        // valid_to_seq
            Some(token_filter.as_str()), // target_token
            None,                        // queue_index
            None,                        // amount_usdt
            None,                        // target_chain_id
            None,                        // beneficiary
            None,                        // origin
            None,                        // origin_id
            None,                        // origin_actor
            None,                        // origin_token
            None,                        // origin_timestamp
            None,                        // origin_raw_amount
            Some("eq.created"),          // status
            None,                        // select
            Some("queue_index.asc"),     // order
            None,                        // range
            None,                        // range_unit
            None,                        // offset
            Some(limit_str.as_str()),    // limit
            None,                        // prefer
        )
        .await
        .context("hub_claims_get(created)")
    }

    pub async fn hub_claims_created(&self, limit: u64) -> Result<Vec<models::HubClaims>> {
        let limit_str = limit.to_string();
        apis::hub_claims_api::hub_claims_get(
            &self.cfg,
            None,                                     // lease_id
            None,                                     // claim_id
            None,                                     // valid_from_seq
            None,                                     // valid_to_seq
            None,                                     // target_token
            None,                                     // queue_index
            None,                                     // amount_usdt
            None,                                     // target_chain_id
            None,                                     // beneficiary
            None,                                     // origin
            None,                                     // origin_id
            None,                                     // origin_actor
            None,                                     // origin_token
            None,                                     // origin_timestamp
            None,                                     // origin_raw_amount
            Some("eq.created"),                       // status
            None,                                     // select
            Some("target_token.asc,queue_index.asc"), // order
            None,                                     // range
            None,                                     // range_unit
            None,                                     // offset
            Some(limit_str.as_str()),                 // limit
            None,                                     // prefer
        )
        .await
        .context("hub_claims_get(created all)")
    }
}
