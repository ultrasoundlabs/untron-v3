use crate::config::LifiConfig;
use alloy::primitives::{Address, Bytes, U256};
use anyhow::{Context, Result};
use reqwest::Url;
use serde::Deserialize;

#[derive(Clone)]
pub struct LifiClient {
    base_url: Url,
    api_key: Option<String>,
    integrator: Option<String>,
    slippage: f64,
    http: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct LifiQuote {
    pub approval_address: Address,
    pub to: Address,
    pub data: Bytes,
    pub value: U256,
    pub to_amount_min: U256,
}

impl LifiClient {
    pub fn new(cfg: &LifiConfig) -> Result<Self> {
        let base_url =
            Url::parse(cfg.base_url.trim_end_matches('/')).context("parse LI.FI base_url")?;
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .context("build LI.FI http client")?;
        Ok(Self {
            base_url,
            api_key: cfg.api_key.clone(),
            integrator: cfg.integrator.clone(),
            slippage: cfg.slippage,
            http,
        })
    }

    pub async fn quote_usdt_to_token(
        &self,
        chain_id: u64,
        usdt: Address,
        target_token: Address,
        amount_usdt: U256,
        swap_executor: Address,
    ) -> Result<LifiQuote> {
        let url = self
            .base_url
            .join("v1/quote")
            .context("join LI.FI /v1/quote")?;

        let mut req = self.http.get(url).query(&[
            ("fromChain", chain_id.to_string()),
            ("toChain", chain_id.to_string()),
            ("fromToken", usdt.to_string()),
            ("toToken", target_token.to_string()),
            ("fromAddress", swap_executor.to_string()),
            ("toAddress", swap_executor.to_string()),
            ("fromAmount", amount_usdt.to_string()),
            ("slippage", self.slippage.to_string()),
            ("allowDestinationCall", "false".to_string()),
        ]);

        if let Some(integrator) = &self.integrator {
            req = req.query(&[("integrator", integrator.clone())]);
        }
        if let Some(api_key) = &self.api_key {
            req = req.header("x-lifi-api-key", api_key);
        }

        let res = req.send().await.context("LI.FI /v1/quote request failed")?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            anyhow::bail!("LI.FI /v1/quote http {status} body={body:?}");
        }

        let step: StepResponse = res.json().await.context("decode LI.FI quote")?;

        // Basic sanity checks to avoid accidentally using a cross-chain route.
        if step.action.from_chain_id != chain_id || step.action.to_chain_id != chain_id {
            anyhow::bail!(
                "LI.FI quote returned unexpected chainIds: from={} to={} (expected {chain_id})",
                step.action.from_chain_id,
                step.action.to_chain_id
            );
        }
        let from_token: Address = step
            .action
            .from_token
            .address
            .parse()
            .context("parse LI.FI action.fromToken.address")?;
        let to_token: Address = step
            .action
            .to_token
            .address
            .parse()
            .context("parse LI.FI action.toToken.address")?;
        if from_token != usdt || to_token != target_token {
            anyhow::bail!(
                "LI.FI quote returned unexpected tokens: from={from_token} to={to_token} (expected {usdt} -> {target_token})"
            );
        }

        let to_amount_min = parse_u256_any(&step.estimate.to_amount_min)
            .context("parse LI.FI estimate.toAmountMin")?;
        let approval_address: Address = step
            .estimate
            .approval_address
            .parse()
            .context("parse LI.FI estimate.approvalAddress")?;

        let tx_to_raw = step
            .transaction_request
            .to
            .as_deref()
            .context("missing LI.FI transactionRequest.to")?;
        let tx_to: Address = tx_to_raw
            .parse()
            .with_context(|| format!("parse LI.FI transactionRequest.to: {tx_to_raw}"))?;

        let tx_data_raw = step
            .transaction_request
            .data
            .as_deref()
            .context("missing LI.FI transactionRequest.data")?;
        let tx_data = Bytes::from(parse_hex_bytes(tx_data_raw).context("parse LI.FI tx data")?);

        let value = match step.transaction_request.value {
            None => U256::ZERO,
            Some(v) => parse_u256_json(v).context("parse LI.FI transactionRequest.value")?,
        };

        Ok(LifiQuote {
            approval_address,
            to: tx_to,
            data: tx_data,
            value,
            to_amount_min,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StepResponse {
    action: Action,
    estimate: Estimate,
    transaction_request: TransactionRequest,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Action {
    from_chain_id: u64,
    to_chain_id: u64,
    from_token: TokenRef,
    to_token: TokenRef,
}

#[derive(Debug, Deserialize)]
struct TokenRef {
    address: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Estimate {
    to_amount_min: String,
    approval_address: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransactionRequest {
    to: Option<String>,
    data: Option<String>,
    value: Option<serde_json::Value>,
}

fn parse_u256_json(v: serde_json::Value) -> Result<U256> {
    match v {
        serde_json::Value::Null => Ok(U256::ZERO),
        serde_json::Value::Number(n) => {
            let s = n.to_string();
            parse_u256_any(&s)
        }
        serde_json::Value::String(s) => parse_u256_any(&s),
        other => anyhow::bail!("unexpected value type: {other:?}"),
    }
}

fn parse_u256_any(s: &str) -> Result<U256> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(U256::ZERO);
    }
    if let Some(hex) = trimmed.strip_prefix("0x") {
        return U256::from_str_radix(hex, 16).context("parse u256 hex");
    }
    let dec = trimmed.replace('_', "");
    U256::from_str_radix(&dec, 10).context("parse u256 decimal")
}

fn parse_hex_bytes(hex_bytes: &str) -> Result<Vec<u8>> {
    let s = hex_bytes.trim();
    let s = s.strip_prefix("0x").unwrap_or(s);
    hex::decode(s).context("decode hex")
}
