use alloy::primitives::Address;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::net::SocketAddr;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
pub struct PaymasterServiceConfig {
    pub url: String,
    #[serde(default)]
    pub context: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub indexer: IndexerConfig,
    pub hub: HubConfig,
    pub leasing: LeasingDefaults,
}

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub bind: SocketAddr,
}

#[derive(Debug, Clone)]
pub struct IndexerConfig {
    pub base_url: String,
    pub timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct HubConfig {
    pub rpc_url: String,
    pub chain_id: Option<u64>,
    pub untron_v3: Address,

    pub entrypoint: Address,
    pub safe: Address,
    pub safe_4337_module: Address,

    pub bundler_urls: Vec<String>,

    pub owner_private_key: [u8; 32],

    pub paymasters: Vec<PaymasterServiceConfig>,
}

#[derive(Debug, Clone)]
pub struct LeasingDefaults {
    pub lessee: Address,
    pub lease_fee_ppm: u32,
    pub flat_fee: u64,
    pub duration_seconds: u64,

    pub target_chain_id: u64,
    pub target_token: Address,
    pub beneficiary: Address,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Env {
    api_bind: String,

    indexer_api_base_url: String,

    indexer_timeout_secs: u64,

    hub_rpc_url: String,

    hub_chain_id: Option<u64>,

    hub_untron_v3_address: String,

    hub_entrypoint_address: String,

    hub_safe_address: String,

    hub_safe_4337_module_address: String,

    hub_owner_private_key_hex: String,

    #[serde(default)]
    hub_bundler_urls: String,

    #[serde(default)]
    hub_paymasters_json: String,

    lease_default_lessee: String,

    lease_default_fee_ppm: u32,

    lease_default_flat_fee: u64,

    lease_default_duration_secs: u64,

    lease_default_target_chain_id: u64,

    lease_default_target_token: String,

    lease_default_beneficiary: String,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            api_bind: "0.0.0.0:3000".to_string(),
            indexer_api_base_url: String::new(),
            indexer_timeout_secs: 10,
            hub_rpc_url: String::new(),
            hub_chain_id: None,
            hub_untron_v3_address: String::new(),
            hub_entrypoint_address: String::new(),
            hub_safe_address: String::new(),
            hub_safe_4337_module_address: String::new(),
            hub_owner_private_key_hex: String::new(),
            hub_bundler_urls: String::new(),
            hub_paymasters_json: String::new(),
            lease_default_lessee: String::new(),
            lease_default_fee_ppm: 10_000,
            lease_default_flat_fee: 0,
            lease_default_duration_secs: 60 * 60 * 24 * 30,
            lease_default_target_chain_id: 0,
            lease_default_target_token: String::new(),
            lease_default_beneficiary: String::new(),
        }
    }
}

fn parse_address(label: &str, s: &str) -> Result<Address> {
    s.parse::<Address>()
        .with_context(|| format!("invalid {label}: {s}"))
}

fn parse_hex_32(label: &str, s: &str) -> Result<[u8; 32]> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(s).with_context(|| format!("invalid hex for {label}"))?;
    if bytes.len() != 32 {
        anyhow::bail!("{label} must be 32 bytes (got {})", bytes.len());
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

fn parse_csv(label: &str, s: &str) -> Result<Vec<String>> {
    let urls = s
        .split(',')
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    if urls.is_empty() {
        anyhow::bail!("{label} must be non-empty");
    }
    Ok(urls)
}

fn parse_paymasters_json(s: &str) -> Result<Vec<PaymasterServiceConfig>> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    let mut v: Vec<PaymasterServiceConfig> =
        serde_json::from_str(trimmed).context("parse HUB_PAYMASTERS_JSON")?;
    for pm in &mut v {
        pm.url = pm.url.trim().to_string();
        if pm.url.is_empty() {
            anyhow::bail!("HUB_PAYMASTERS_JSON contains an empty url");
        }
        if !pm.context.is_object() {
            anyhow::bail!("HUB_PAYMASTERS_JSON paymaster.context must be a JSON object");
        }
    }
    Ok(v)
}

pub fn load_config() -> Result<AppConfig> {
    let env: Env = envy::from_env().context("load realtor env config")?;

    if env.indexer_api_base_url.trim().is_empty() {
        anyhow::bail!("INDEXER_API_BASE_URL must be set");
    }
    if env.hub_rpc_url.trim().is_empty() {
        anyhow::bail!("HUB_RPC_URL must be set");
    }

    let bind: SocketAddr = env
        .api_bind
        .parse()
        .context("invalid API_BIND (expected host:port)")?;

    let hub_untron_v3 = parse_address("HUB_UNTRON_V3_ADDRESS", &env.hub_untron_v3_address)?;
    let hub_entrypoint = parse_address("HUB_ENTRYPOINT_ADDRESS", &env.hub_entrypoint_address)?;
    let hub_safe = parse_address("HUB_SAFE_ADDRESS", &env.hub_safe_address)?;
    let hub_module = parse_address(
        "HUB_SAFE_4337_MODULE_ADDRESS",
        &env.hub_safe_4337_module_address,
    )?;
    let hub_owner_private_key =
        parse_hex_32("HUB_OWNER_PRIVATE_KEY_HEX", &env.hub_owner_private_key_hex)?;
    let bundlers = parse_csv("HUB_BUNDLER_URLS", &env.hub_bundler_urls)?;
    let paymasters = parse_paymasters_json(&env.hub_paymasters_json)?;

    let default_lessee = parse_address("LEASE_DEFAULT_LESSEE", &env.lease_default_lessee)?;
    let default_target_token = parse_address(
        "LEASE_DEFAULT_TARGET_TOKEN",
        &env.lease_default_target_token,
    )?;
    let default_beneficiary =
        parse_address("LEASE_DEFAULT_BENEFICIARY", &env.lease_default_beneficiary)?;
    let default_target_chain_id = env.lease_default_target_chain_id;
    if default_target_chain_id == 0 {
        anyhow::bail!("LEASE_DEFAULT_TARGET_CHAIN_ID must be set (and non-zero)");
    }

    Ok(AppConfig {
        api: ApiConfig { bind },
        indexer: IndexerConfig {
            base_url: env.indexer_api_base_url,
            timeout: Duration::from_secs(env.indexer_timeout_secs.max(1)),
        },
        hub: HubConfig {
            rpc_url: env.hub_rpc_url,
            chain_id: env.hub_chain_id,
            untron_v3: hub_untron_v3,
            entrypoint: hub_entrypoint,
            safe: hub_safe,
            safe_4337_module: hub_module,
            bundler_urls: bundlers,
            owner_private_key: hub_owner_private_key,
            paymasters,
        },
        leasing: LeasingDefaults {
            lessee: default_lessee,
            lease_fee_ppm: env.lease_default_fee_ppm,
            flat_fee: env.lease_default_flat_fee,
            duration_seconds: env.lease_default_duration_secs.max(1),
            target_chain_id: default_target_chain_id,
            target_token: default_target_token,
            beneficiary: default_beneficiary,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex_32_accepts_0x_and_rejects_wrong_len() {
        let ok = format!("0x{}", "11".repeat(32));
        let out = parse_hex_32("K", &ok).unwrap();
        assert_eq!(out, [0x11u8; 32]);

        let err = parse_hex_32("K", "0x11").unwrap_err().to_string();
        assert!(err.contains("must be 32 bytes"));
    }

    #[test]
    fn parse_csv_trims_and_requires_non_empty() {
        let urls = parse_csv("U", " a, ,b ,, c ").unwrap();
        assert_eq!(
            urls,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );

        let err = parse_csv("U", " , , ").unwrap_err().to_string();
        assert!(err.contains("must be non-empty"));
    }

    #[test]
    fn parse_paymasters_json_empty_ok() {
        assert!(parse_paymasters_json("   ").unwrap().is_empty());
    }

    #[test]
    fn parse_paymasters_json_validates_url_and_context_object() {
        let ok = r#"[{"url":" https://pm.example ","context":{}}]"#;
        let v = parse_paymasters_json(ok).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].url, "https://pm.example");
        assert!(v[0].context.is_object());

        let err = parse_paymasters_json(r#"[{"url":" ","context":{}}]"#)
            .unwrap_err()
            .to_string();
        assert!(err.contains("empty url"));

        let err = parse_paymasters_json(r#"[{"url":"x","context":123}]"#)
            .unwrap_err()
            .to_string();
        assert!(err.contains("must be a JSON object"));
    }

    #[test]
    fn parse_address_accepts_valid_and_rejects_invalid() {
        let a = parse_address("A", "0x0000000000000000000000000000000000000001").unwrap();
        let expected: Address = "0x0000000000000000000000000000000000000001"
            .parse()
            .unwrap();
        assert_eq!(a, expected);

        assert!(parse_address("A", "not an address").is_err());
    }
}
