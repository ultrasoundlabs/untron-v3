use aa::SafeDeterministicDeploymentConfig;
use alloy::primitives::{Address, U256};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::time::Duration;
use tron::{JsonApiRentalProviderConfig, TronAddress};

#[derive(Debug, Clone, Deserialize)]
pub struct PaymasterServiceConfig {
    pub url: String,
    #[serde(default)]
    pub context: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub indexer: IndexerConfig,
    pub hub: HubConfig,
    pub tron: TronConfig,
    pub jobs: JobConfig,
}

#[derive(Debug, Clone)]
pub struct IndexerConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub max_head_lag_blocks: u64,
}

#[derive(Debug, Clone)]
pub struct HubConfig {
    pub rpc_url: String,
    pub chain_id: Option<u64>,
    pub untron_v3: Address,

    pub entrypoint: Address,
    pub safe: Option<Address>,
    pub safe_4337_module: Address,
    pub safe_deployment: Option<SafeDeterministicDeploymentConfig>,
    pub multisend: Option<Address>,

    pub bundler_urls: Vec<String>,

    pub owner_private_key: [u8; 32],

    pub paymasters: Vec<PaymasterServiceConfig>,

    pub uniswap_v4: Option<UniswapV4Config>,
}

#[derive(Debug, Clone)]
pub struct UniswapV4Config {
    pub pool_manager: Option<Address>,
    pub position_manager: Option<Address>,
    pub allowed_pools: Vec<UniswapV4AllowedPool>,
    /// Slippage as a decimal fraction (e.g. 0.003 = 0.3%).
    pub slippage: f64,
    /// If true, allow topping up swap output with Safe-held target tokens.
    pub allow_topup: bool,
}

#[derive(Debug, Clone)]
pub struct UniswapV4AllowedPool {
    pub currency0: Address,
    pub currency1: Address,
    pub fee: u32,
    pub tick_spacing: i32,
    pub hooks: Address,
}

#[derive(Debug, Clone)]
pub struct TronConfig {
    pub grpc_url: String,
    pub api_key: Option<String>,
    pub private_key: [u8; 32],
    pub controller_address: String,

    pub block_lag: u64,
    /// Extra headroom on computed fee_limit (ppm, i.e. 100_000 = +10%).
    pub fee_limit_headroom_ppm: u64,
    /// Optional list of external energy rental providers.
    pub energy_rental_providers: Vec<JsonApiRentalProviderConfig>,
    /// Max time to poll Tron until rented energy is reflected in AccountResource.
    pub energy_rental_confirm_max_wait: Duration,
}

#[derive(Debug, Clone)]
pub struct JobConfig {
    pub tick_interval: Duration,
    pub tron_finality_blocks: u64,
    pub tip_proof_resend_blocks: u64,

    pub process_controller_max_events: u64,
    pub fill_max_claims: u64,

    pub controller_rebalance_threshold_usdt: String,
    pub controller_rebalance_keep_usdt: String,
    pub controller_rebalance_prioritized_rebalancers: Vec<TronAddress>,
    /// Per-prioritized-rebalancer max rebalance amount (USDT min-units) under which the address is
    /// considered "preferred". If the limit is 0 (or missing), it is always preferred.
    ///
    /// This list is positionally aligned with `controller_rebalance_prioritized_rebalancers`.
    pub controller_rebalance_prioritized_rebalancers_limits_usdt: Vec<U256>,

    pub pull_liquidity_ppm: u64,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Env {
    indexer_api_base_url: String,

    indexer_timeout_secs: u64,

    indexer_max_head_lag_blocks: u64,

    hub_rpc_url: String,

    hub_chain_id: Option<u64>,

    hub_untron_v3_address: String,

    hub_entrypoint_address: String,

    hub_safe_address: String,

    hub_safe_4337_module_address: String,

    #[serde(default)]
    hub_multisend_address: String,

    #[serde(default)]
    hub_safe_proxy_factory_address: String,

    #[serde(default)]
    hub_safe_singleton_address: String,

    #[serde(default)]
    hub_safe_module_setup_address: String,

    hub_owner_private_key_hex: String,

    #[serde(default)]
    hub_bundler_urls: String,

    #[serde(default)]
    hub_paymasters_json: String,

    #[serde(default)]
    uniswap_v4_pool_manager_address: String,

    #[serde(default)]
    uniswap_v4_position_manager_address: String,

    #[serde(default)]
    uniswap_v4_allowed_pools_json: String,

    #[serde(default)]
    uniswap_v4_slippage: f64,

    #[serde(default)]
    uniswap_v4_allow_topup: bool,

    tron_grpc_url: String,

    tron_api_key: Option<String>,

    tron_private_key_hex: String,

    tron_controller_address: String,

    tron_block_lag: u64,

    #[serde(default)]
    tron_fee_limit_headroom_ppm: u64,

    #[serde(default)]
    tron_energy_rental_apis_json: String,

    #[serde(default)]
    tron_energy_rental_confirm_max_wait_secs: u64,

    relayer_tick_interval_secs: u64,

    tron_finality_blocks: u64,

    tron_tip_proof_resend_blocks: u64,

    process_controller_max_events: u64,

    fill_max_claims: u64,

    controller_rebalance_threshold_usdt: String,

    controller_rebalance_keep_usdt: String,

    #[serde(default)]
    controller_rebalance_prioritized_rebalancers: String,

    // Optional comma-separated list of USDT min-unit limits aligned with
    // CONTROLLER_REBALANCE_PRIORITIZED_REBALANCERS. 0 means "always preferred".
    #[serde(default)]
    controller_rebalance_prioritized_rebalancers_limits_usdt: String,

    pull_liquidity_ppm: u64,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            indexer_api_base_url: String::new(),
            indexer_timeout_secs: 10,
            indexer_max_head_lag_blocks: 50,
            hub_rpc_url: String::new(),
            hub_chain_id: None,
            hub_untron_v3_address: String::new(),
            hub_entrypoint_address: String::new(),
            hub_safe_address: String::new(),
            hub_safe_4337_module_address: String::new(),
            hub_multisend_address: String::new(),
            hub_safe_proxy_factory_address: String::new(),
            hub_safe_singleton_address: String::new(),
            hub_safe_module_setup_address: String::new(),
            hub_owner_private_key_hex: String::new(),
            hub_bundler_urls: String::new(),
            hub_paymasters_json: String::new(),
            uniswap_v4_pool_manager_address: String::new(),
            uniswap_v4_position_manager_address: String::new(),
            uniswap_v4_allowed_pools_json: String::new(),
            uniswap_v4_slippage: 0.003,
            uniswap_v4_allow_topup: false,
            tron_grpc_url: String::new(),
            tron_api_key: None,
            tron_private_key_hex: String::new(),
            tron_controller_address: String::new(),
            tron_block_lag: 0,
            tron_fee_limit_headroom_ppm: 100_000,
            tron_energy_rental_apis_json: String::new(),
            tron_energy_rental_confirm_max_wait_secs: 6,
            relayer_tick_interval_secs: 5,
            tron_finality_blocks: 19,
            tron_tip_proof_resend_blocks: 20,
            process_controller_max_events: 100,
            fill_max_claims: 50,
            controller_rebalance_threshold_usdt: "0".to_string(),
            controller_rebalance_keep_usdt: "1".to_string(),
            controller_rebalance_prioritized_rebalancers: String::new(),
            controller_rebalance_prioritized_rebalancers_limits_usdt: String::new(),
            pull_liquidity_ppm: 500_000,
        }
    }
}

fn parse_address(label: &str, s: &str) -> Result<Address> {
    s.parse::<Address>()
        .with_context(|| format!("invalid {label}: {s}"))
}

fn parse_optional_address(label: &str, s: &str) -> Result<Option<Address>> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let addr = parse_address(label, trimmed)?;
    if addr == Address::ZERO {
        Ok(None)
    } else {
        Ok(Some(addr))
    }
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

fn parse_tron_address_csv_optional(label: &str, s: &str) -> Result<Vec<TronAddress>> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for raw in trimmed.split(',') {
        let v = raw.trim();
        if v.is_empty() {
            continue;
        }
        let addr =
            TronAddress::parse_text(v).with_context(|| format!("invalid {label} entry: {v}"))?;
        if seen.insert(addr) {
            out.push(addr);
        }
    }
    Ok(out)
}

fn parse_u256_csv_optional(label: &str, s: &str) -> Result<Vec<U256>> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    for raw in trimmed.split(',') {
        let v = raw.trim();
        if v.is_empty() {
            // Preserve positional alignment with the corresponding prioritized rebalancer.
            // Empty entries mean "no cap" (0 = always preferred).
            out.push(U256::ZERO);
            continue;
        }
        let v = v.replace('_', "");
        if v.is_empty() {
            out.push(U256::ZERO);
            continue;
        }
        let n = U256::from_str_radix(&v, 10)
            .with_context(|| format!("invalid {label} entry (expected base-10 u256): {raw}"))?;
        out.push(n);
    }
    Ok(out)
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

#[derive(Debug, Deserialize)]
struct UniswapV4AllowedPoolRaw {
    currency0: String,
    currency1: String,
    fee: u32,
    tick_spacing: i32,
    #[serde(default)]
    hooks: String,
}

fn parse_uniswap_v4_allowed_pools_json(s: &str) -> Result<Vec<UniswapV4AllowedPool>> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let raws: Vec<UniswapV4AllowedPoolRaw> =
        serde_json::from_str(trimmed).context("parse UNISWAP_V4_ALLOWED_POOLS_JSON")?;
    let mut out = Vec::with_capacity(raws.len());

    for (idx, raw) in raws.into_iter().enumerate() {
        let currency0 = parse_address(
            &format!("UNISWAP_V4_ALLOWED_POOLS_JSON[{idx}].currency0"),
            &raw.currency0,
        )?;
        let currency1 = parse_address(
            &format!("UNISWAP_V4_ALLOWED_POOLS_JSON[{idx}].currency1"),
            &raw.currency1,
        )?;
        if currency0 == currency1 {
            anyhow::bail!("UNISWAP_V4_ALLOWED_POOLS_JSON[{idx}] has identical currency0/currency1");
        }
        if raw.fee == 0 {
            anyhow::bail!("UNISWAP_V4_ALLOWED_POOLS_JSON[{idx}].fee must be > 0");
        }

        let hooks = if raw.hooks.trim().is_empty() {
            Address::ZERO
        } else {
            parse_address(
                &format!("UNISWAP_V4_ALLOWED_POOLS_JSON[{idx}].hooks"),
                &raw.hooks,
            )?
        };

        out.push(UniswapV4AllowedPool {
            currency0,
            currency1,
            fee: raw.fee,
            tick_spacing: raw.tick_spacing,
            hooks,
        });
    }

    Ok(out)
}

pub fn load_config() -> Result<AppConfig> {
    let env: Env = envy::from_env().context("load relayer env config")?;

    if env.indexer_api_base_url.trim().is_empty() {
        anyhow::bail!("INDEXER_API_BASE_URL must be set");
    }
    if env.hub_rpc_url.trim().is_empty() {
        anyhow::bail!("HUB_RPC_URL must be set");
    }
    if env.tron_grpc_url.trim().is_empty() {
        anyhow::bail!("TRON_GRPC_URL must be set");
    }
    if env.tron_private_key_hex.trim().is_empty() {
        anyhow::bail!("TRON_PRIVATE_KEY_HEX must be set");
    }
    if env.tron_controller_address.trim().is_empty() {
        anyhow::bail!("TRON_CONTROLLER_ADDRESS must be set");
    }

    let hub_untron_v3 = parse_address("HUB_UNTRON_V3_ADDRESS", &env.hub_untron_v3_address)?;
    let hub_entrypoint = parse_address("HUB_ENTRYPOINT_ADDRESS", &env.hub_entrypoint_address)?;
    let hub_safe = parse_optional_address("HUB_SAFE_ADDRESS", &env.hub_safe_address)?;
    let hub_module = parse_address(
        "HUB_SAFE_4337_MODULE_ADDRESS",
        &env.hub_safe_4337_module_address,
    )?;
    let hub_multisend =
        parse_optional_address("HUB_MULTISEND_ADDRESS", &env.hub_multisend_address)?;
    let hub_safe_deployment = if hub_safe.is_some() {
        None
    } else {
        Some(SafeDeterministicDeploymentConfig {
            proxy_factory: parse_address(
                "HUB_SAFE_PROXY_FACTORY_ADDRESS",
                &env.hub_safe_proxy_factory_address,
            )?,
            singleton: parse_address(
                "HUB_SAFE_SINGLETON_ADDRESS",
                &env.hub_safe_singleton_address,
            )?,
            module_setup: parse_address(
                "HUB_SAFE_MODULE_SETUP_ADDRESS",
                &env.hub_safe_module_setup_address,
            )?,
            salt_nonce: alloy::primitives::U256::ZERO,
        })
    };
    let hub_owner_private_key =
        parse_hex_32("HUB_OWNER_PRIVATE_KEY_HEX", &env.hub_owner_private_key_hex)?;

    let bundlers = parse_csv("HUB_BUNDLER_URLS", &env.hub_bundler_urls)?;
    let paymasters = parse_paymasters_json(&env.hub_paymasters_json)?;

    let allowed_v4_pools = parse_uniswap_v4_allowed_pools_json(&env.uniswap_v4_allowed_pools_json)?;
    let uniswap_v4 = if allowed_v4_pools.is_empty() {
        None
    } else {
        Some(UniswapV4Config {
            pool_manager: parse_optional_address(
                "UNISWAP_V4_POOL_MANAGER_ADDRESS",
                &env.uniswap_v4_pool_manager_address,
            )?,
            position_manager: parse_optional_address(
                "UNISWAP_V4_POSITION_MANAGER_ADDRESS",
                &env.uniswap_v4_position_manager_address,
            )?,
            allowed_pools: allowed_v4_pools,
            slippage: env.uniswap_v4_slippage.clamp(0.0, 1.0),
            allow_topup: env.uniswap_v4_allow_topup,
        })
    };

    Ok(AppConfig {
        indexer: IndexerConfig {
            base_url: env.indexer_api_base_url,
            timeout: Duration::from_secs(env.indexer_timeout_secs.max(1)),
            max_head_lag_blocks: env.indexer_max_head_lag_blocks.max(1),
        },
        hub: HubConfig {
            rpc_url: env.hub_rpc_url,
            chain_id: env.hub_chain_id,
            untron_v3: hub_untron_v3,
            entrypoint: hub_entrypoint,
            safe: hub_safe,
            safe_4337_module: hub_module,
            safe_deployment: hub_safe_deployment,
            multisend: hub_multisend,
            bundler_urls: bundlers,
            owner_private_key: hub_owner_private_key,
            paymasters,
            uniswap_v4,
        },
        tron: TronConfig {
            grpc_url: env.tron_grpc_url,
            api_key: env.tron_api_key.filter(|s| !s.trim().is_empty()),
            private_key: parse_hex_32("TRON_PRIVATE_KEY_HEX", &env.tron_private_key_hex)?,
            controller_address: env.tron_controller_address,
            block_lag: env.tron_block_lag,
            fee_limit_headroom_ppm: env.tron_fee_limit_headroom_ppm.min(1_000_000),
            energy_rental_providers: parse_tron_energy_rental_apis_json(
                &env.tron_energy_rental_apis_json,
            )?,
            energy_rental_confirm_max_wait: Duration::from_secs(
                env.tron_energy_rental_confirm_max_wait_secs,
            ),
        },
        jobs: JobConfig {
            tick_interval: Duration::from_secs(env.relayer_tick_interval_secs.max(1)),
            tron_finality_blocks: env.tron_finality_blocks,
            tip_proof_resend_blocks: env.tron_tip_proof_resend_blocks.max(1),
            process_controller_max_events: env.process_controller_max_events,
            fill_max_claims: env.fill_max_claims,
            controller_rebalance_threshold_usdt: env.controller_rebalance_threshold_usdt,
            controller_rebalance_keep_usdt: env.controller_rebalance_keep_usdt,
            controller_rebalance_prioritized_rebalancers: parse_tron_address_csv_optional(
                "CONTROLLER_REBALANCE_PRIORITIZED_REBALANCERS",
                &env.controller_rebalance_prioritized_rebalancers,
            )?,
            controller_rebalance_prioritized_rebalancers_limits_usdt: parse_u256_csv_optional(
                "CONTROLLER_REBALANCE_PRIORITIZED_REBALANCERS_LIMITS_USDT",
                &env.controller_rebalance_prioritized_rebalancers_limits_usdt,
            )?,
            pull_liquidity_ppm: env.pull_liquidity_ppm.min(1_000_000),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;

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
    fn parse_tron_address_csv_optional_empty_ok_and_dedups() {
        assert!(
            parse_tron_address_csv_optional("T", "   ")
                .unwrap()
                .is_empty()
        );

        let a = TronAddress::parse_text("T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb").unwrap();
        let b = TronAddress::parse_text("0x0000000000000000000000000000000000000001").unwrap();
        let out = parse_tron_address_csv_optional(
            "T",
            " T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb, 0x0000000000000000000000000000000000000001, T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb ",
        )
        .unwrap();
        assert_eq!(out, vec![a, b]);
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
    fn parse_tron_energy_rental_apis_json_empty_ok() {
        assert!(
            parse_tron_energy_rental_apis_json("   ")
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn parse_tron_energy_rental_apis_json_validates_name_and_url() {
        let ok = r#"[{"name":" p1 ","url":" https://r.example ","method":"POST","headers":{},"body":{},"response":{"success_pointer":"/ok"}}]"#;
        let v = parse_tron_energy_rental_apis_json(ok).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].name, "p1");
        assert_eq!(v[0].url, "https://r.example");

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
    fn parse_address_accepts_valid_and_rejects_invalid() {
        let a = parse_address("A", "0x0000000000000000000000000000000000000001").unwrap();
        let expected: Address = "0x0000000000000000000000000000000000000001"
            .parse()
            .unwrap();
        assert_eq!(a, expected);

        assert!(parse_address("A", "not an address").is_err());
    }

    #[test]
    fn parse_uniswap_v4_allowed_pools_json_empty_ok() {
        assert!(
            parse_uniswap_v4_allowed_pools_json("   ")
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn parse_uniswap_v4_allowed_pools_json_parses_and_defaults_hooks() {
        let raw = r#"[{
          "currency0":"0x0000000000000000000000000000000000000001",
          "currency1":"0x0000000000000000000000000000000000000002",
          "fee":500,
          "tick_spacing":10
        }]"#;
        let out = parse_uniswap_v4_allowed_pools_json(raw).unwrap();
        assert_eq!(out.len(), 1);
        assert_eq!(
            out[0].currency0,
            "0x0000000000000000000000000000000000000001"
                .parse::<Address>()
                .unwrap()
        );
        assert_eq!(
            out[0].currency1,
            "0x0000000000000000000000000000000000000002"
                .parse::<Address>()
                .unwrap()
        );
        assert_eq!(out[0].fee, 500);
        assert_eq!(out[0].tick_spacing, 10);
        assert_eq!(out[0].hooks, Address::ZERO);
    }
}
