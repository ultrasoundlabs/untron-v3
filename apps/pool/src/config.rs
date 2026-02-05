use anyhow::{Context, Result};
use serde::Deserialize;
use std::time::Duration;
use tron::JsonApiRentalProviderConfig;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub tron: TronConfig,
    pub oneclick: OneClickConfig,
    pub jobs: JobConfig,
}

#[derive(Debug, Clone)]
pub struct TronConfig {
    pub grpc_urls: Vec<String>,
    pub api_key: Option<String>,
    pub private_key: [u8; 32],
    pub usdt_contract_address: String,

    pub fee_limit_cap_sun: u64,
    pub fee_limit_headroom_ppm: u64,
    pub energy_rental_providers: Vec<JsonApiRentalProviderConfig>,
    pub energy_rental_settle_delay: Duration,
}

#[derive(Debug, Clone)]
pub struct OneClickConfig {
    pub base_url: String,
    pub bearer_token: Option<String>,
    pub origin_asset: String,
    pub destination_asset: String,
    pub beneficiary: String,
    pub slippage_bps: f64,
    pub deadline_secs: u64,
    pub referral: Option<String>,
    pub status_poll_interval: Duration,
    pub status_max_wait: Duration,
    pub backoff_base: Duration,
    pub backoff_max: Duration,
}

#[derive(Debug, Clone)]
pub struct JobConfig {
    pub poll_interval: Duration,
    pub usdt_balance_threshold: String,
    pub usdt_balance_keep_usdt: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Env {
    #[serde(default)]
    tron_grpc_url: String,
    #[serde(default)]
    tron_grpc_urls: String,
    tron_api_key: Option<String>,
    tron_private_key_hex: String,
    tron_usdt_contract_address: String,

    #[serde(default)]
    tron_fee_limit_cap_sun: u64,
    #[serde(default)]
    tron_fee_limit_headroom_ppm: u64,
    #[serde(default)]
    tron_energy_rental_apis_json: String,
    #[serde(default)]
    tron_energy_rental_settle_delay_secs: u64,

    oneclick_base_url: String,
    #[serde(default)]
    oneclick_bearer_token: String,
    oneclick_origin_asset: String,
    oneclick_destination_asset: String,
    oneclick_beneficiary: String,
    #[serde(default)]
    oneclick_slippage_bps: f64,
    #[serde(default)]
    oneclick_deadline_secs: u64,
    #[serde(default)]
    oneclick_referral: String,
    #[serde(default)]
    oneclick_status_poll_interval_secs: u64,
    #[serde(default)]
    oneclick_status_max_wait_secs: u64,
    #[serde(default)]
    oneclick_backoff_base_secs: u64,
    #[serde(default)]
    oneclick_backoff_max_secs: u64,

    pool_poll_interval_secs: u64,
    pool_usdt_balance_threshold: String,
    pool_usdt_balance_keep_usdt: String,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            tron_grpc_url: String::new(),
            tron_grpc_urls: String::new(),
            tron_api_key: None,
            tron_private_key_hex: String::new(),
            tron_usdt_contract_address: "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string(),
            tron_fee_limit_cap_sun: 30_000_000,
            tron_fee_limit_headroom_ppm: 100_000,
            tron_energy_rental_apis_json: String::new(),
            tron_energy_rental_settle_delay_secs: 6,
            oneclick_base_url: "https://1click.chaindefuser.com".to_string(),
            oneclick_bearer_token: String::new(),
            oneclick_origin_asset: String::new(),
            oneclick_destination_asset: String::new(),
            oneclick_beneficiary: String::new(),
            oneclick_slippage_bps: 100.0,
            oneclick_deadline_secs: 900,
            oneclick_referral: String::new(),
            oneclick_status_poll_interval_secs: 10,
            oneclick_status_max_wait_secs: 1800,
            oneclick_backoff_base_secs: 60,
            oneclick_backoff_max_secs: 3600,
            pool_poll_interval_secs: 15,
            pool_usdt_balance_threshold: "0".to_string(),
            pool_usdt_balance_keep_usdt: "1".to_string(),
        }
    }
}

fn parse_tron_energy_rental_apis_json(s: &str) -> Result<Vec<JsonApiRentalProviderConfig>> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    let mut v: Vec<JsonApiRentalProviderConfig> =
        serde_json::from_str(trimmed).context("parse TRON_ENERGY_RENTAL_APIS_JSON")?;
    for p in &mut v {
        p.name = p.name.trim().to_string();
        p.url = p.url.trim().to_string();
        if p.name.is_empty() {
            anyhow::bail!("TRON_ENERGY_RENTAL_APIS_JSON contains an empty provider name");
        }
        if p.url.is_empty() {
            anyhow::bail!("TRON_ENERGY_RENTAL_APIS_JSON contains an empty provider url");
        }
        if p.method.trim().is_empty() {
            p.method = "POST".to_string();
        }
    }
    Ok(v)
}

fn parse_csv_optional(s: &str) -> Vec<String> {
    s.split(',')
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_string)
        .collect()
}

pub fn load_config() -> Result<AppConfig> {
    let env: Env = envy::from_env().context("load pool env config")?;

    let tron_grpc_urls = if !env.tron_grpc_urls.trim().is_empty() {
        parse_csv_optional(&env.tron_grpc_urls)
    } else if !env.tron_grpc_url.trim().is_empty() {
        vec![env.tron_grpc_url.trim().to_string()]
    } else {
        Vec::new()
    };
    if tron_grpc_urls.is_empty() {
        anyhow::bail!("TRON_GRPC_URLS (or TRON_GRPC_URL) must be set");
    }
    if env.tron_private_key_hex.trim().is_empty() {
        anyhow::bail!("TRON_PRIVATE_KEY_HEX must be set");
    }
    if env.oneclick_base_url.trim().is_empty() {
        anyhow::bail!("ONECLICK_BASE_URL must be set");
    }
    if env.oneclick_origin_asset.trim().is_empty() {
        anyhow::bail!("ONECLICK_ORIGIN_ASSET must be set");
    }
    if env.oneclick_destination_asset.trim().is_empty() {
        anyhow::bail!("ONECLICK_DESTINATION_ASSET must be set");
    }
    if env.oneclick_beneficiary.trim().is_empty() {
        anyhow::bail!("ONECLICK_BENEFICIARY must be set");
    }

    Ok(AppConfig {
        tron: TronConfig {
            grpc_urls: tron_grpc_urls,
            api_key: env.tron_api_key.filter(|s| !s.trim().is_empty()),
            private_key: crate::util::parse_hex_32(
                "TRON_PRIVATE_KEY_HEX",
                &env.tron_private_key_hex,
            )?,
            usdt_contract_address: env.tron_usdt_contract_address,
            fee_limit_cap_sun: env.tron_fee_limit_cap_sun.max(1),
            fee_limit_headroom_ppm: env.tron_fee_limit_headroom_ppm.min(1_000_000),
            energy_rental_providers: parse_tron_energy_rental_apis_json(
                &env.tron_energy_rental_apis_json,
            )?,
            energy_rental_settle_delay: Duration::from_secs(
                env.tron_energy_rental_settle_delay_secs,
            ),
        },
        oneclick: OneClickConfig {
            base_url: env.oneclick_base_url,
            bearer_token: {
                let t = env.oneclick_bearer_token.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(t.to_string())
                }
            },
            origin_asset: env.oneclick_origin_asset,
            destination_asset: env.oneclick_destination_asset,
            beneficiary: env.oneclick_beneficiary,
            slippage_bps: env.oneclick_slippage_bps.max(0.0),
            deadline_secs: env.oneclick_deadline_secs.max(30),
            referral: {
                let t = env.oneclick_referral.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(t.to_string())
                }
            },
            status_poll_interval: Duration::from_secs(
                env.oneclick_status_poll_interval_secs.max(1),
            ),
            status_max_wait: Duration::from_secs(env.oneclick_status_max_wait_secs),
            backoff_base: Duration::from_secs(env.oneclick_backoff_base_secs.max(1)),
            backoff_max: Duration::from_secs(env.oneclick_backoff_max_secs.max(1)),
        },
        jobs: JobConfig {
            poll_interval: Duration::from_secs(env.pool_poll_interval_secs.max(1)),
            usdt_balance_threshold: env.pool_usdt_balance_threshold,
            usdt_balance_keep_usdt: env.pool_usdt_balance_keep_usdt,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env_lock() -> &'static std::sync::Mutex<()> {
        static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
        LOCK.get_or_init(|| std::sync::Mutex::new(()))
    }

    fn set_required_env_for_success() {
        // Safety: tests take a global env lock; no concurrent env access within this test process.
        unsafe {
            std::env::set_var("TRON_GRPC_URLS", "https://example.invalid:50051");
            std::env::set_var("TRON_PRIVATE_KEY_HEX", format!("0x{}", "11".repeat(32)));

            std::env::set_var("ONECLICK_BASE_URL", "https://1click.example.invalid");
            std::env::set_var("ONECLICK_BEARER_TOKEN", "");
            std::env::set_var("ONECLICK_ORIGIN_ASSET", "tron:usdt");
            std::env::set_var("ONECLICK_DESTINATION_ASSET", "arbitrum:usdt");
            std::env::set_var(
                "ONECLICK_BENEFICIARY",
                "0x0000000000000000000000000000000000000001",
            );

            std::env::set_var("POOL_USDT_BALANCE_THRESHOLD", "0");
            std::env::set_var("POOL_USDT_BALANCE_KEEP_USDT", "1");
        }
    }

    fn clear_env(keys: &[&str]) {
        for k in keys {
            // Safety: tests take a global env lock; no concurrent env access within this test process.
            unsafe {
                std::env::remove_var(k);
            }
        }
    }

    #[test]
    fn parse_tron_energy_rental_apis_json_empty_ok() {
        assert!(
            parse_tron_energy_rental_apis_json("   ")
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn parse_tron_energy_rental_apis_json_validates_name_url_and_defaults_method() {
        let ok = r#"[{"name":" p1 ","url":" https://r.example ","headers":{},"body":{},"response":{"success_pointer":"/ok"}}]"#;
        let v = parse_tron_energy_rental_apis_json(ok).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].name, "p1");
        assert_eq!(v[0].url, "https://r.example");
        assert_eq!(v[0].method, "POST");

        let err = parse_tron_energy_rental_apis_json(
            r#"[{"name":" ","url":"x","method":"POST","headers":{},"body":{},"response":{"success_pointer":"/ok"}}]"#,
        )
        .unwrap_err()
        .to_string();
        assert!(err.contains("empty provider name"));

        let err = parse_tron_energy_rental_apis_json(
            r#"[{"name":"x","url":" ","method":"POST","headers":{},"body":{},"response":{"success_pointer":"/ok"}}]"#,
        )
        .unwrap_err()
        .to_string();
        assert!(err.contains("empty provider url"));
    }

    #[test]
    fn load_config_requires_expected_vars() {
        let _guard = env_lock().lock().unwrap();
        clear_env(&[
            "TRON_GRPC_URL",
            "TRON_GRPC_URLS",
            "TRON_PRIVATE_KEY_HEX",
            "ONECLICK_BASE_URL",
            "ONECLICK_BEARER_TOKEN",
            "ONECLICK_ORIGIN_ASSET",
            "ONECLICK_DESTINATION_ASSET",
            "ONECLICK_BENEFICIARY",
            "POOL_USDT_BALANCE_THRESHOLD",
            "POOL_USDT_BALANCE_KEEP_USDT",
        ]);

        let err = load_config().unwrap_err().to_string();
        assert!(err.contains("TRON_GRPC_URLS"));

        set_required_env_for_success();
        clear_env(&["ONECLICK_ORIGIN_ASSET"]);
        let err = load_config().unwrap_err().to_string();
        assert!(err.contains("ONECLICK_ORIGIN_ASSET must be set"));
    }

    #[test]
    fn load_config_uses_default_usdt_contract_when_unset() {
        let _guard = env_lock().lock().unwrap();
        clear_env(&[
            "TRON_GRPC_URL",
            "TRON_GRPC_URLS",
            "TRON_PRIVATE_KEY_HEX",
            "TRON_USDT_CONTRACT_ADDRESS",
            "ONECLICK_BASE_URL",
            "ONECLICK_BEARER_TOKEN",
            "ONECLICK_ORIGIN_ASSET",
            "ONECLICK_DESTINATION_ASSET",
            "ONECLICK_BENEFICIARY",
            "POOL_USDT_BALANCE_THRESHOLD",
            "POOL_USDT_BALANCE_KEEP_USDT",
        ]);

        set_required_env_for_success();
        let cfg = load_config().unwrap();
        assert_eq!(
            cfg.tron.usdt_contract_address,
            "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t"
        );
    }
}
