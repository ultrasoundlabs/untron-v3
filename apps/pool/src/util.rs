use alloy::primitives::U256;
use anyhow::{Context, Result};

pub fn parse_hex_32(label: &str, s: &str) -> Result<[u8; 32]> {
    let s = s.trim();
    let s = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(s).with_context(|| format!("invalid hex for {label}"))?;
    if bytes.len() != 32 {
        anyhow::bail!("{label} must be 32 bytes (got {})", bytes.len());
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

pub fn parse_u256_decimal(label: &str, s: &str) -> Result<U256> {
    let s = s.trim().replace('_', "");
    U256::from_str_radix(&s, 10).with_context(|| format!("parse {label} as decimal U256"))
}
