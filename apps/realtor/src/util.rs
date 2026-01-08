use alloy::primitives::{Address, B256, FixedBytes, keccak256};
use anyhow::{Context, Result};

pub fn parse_hex_bytes(hex_bytes: &str) -> Result<Vec<u8>> {
    let s = hex_bytes.trim();
    let s = s.strip_prefix("0x").unwrap_or(s);
    hex::decode(s).context("decode hex")
}

pub fn parse_bytes32(hex32: &str) -> Result<FixedBytes<32>> {
    let b = parse_hex_bytes(hex32)?;
    if b.len() != 32 {
        anyhow::bail!("expected 32-byte hex, got {}", b.len());
    }
    Ok(FixedBytes::from_slice(&b))
}

pub fn compute_create2_address(
    create2_prefix: u8,
    deployer: Address,
    salt: FixedBytes<32>,
    init_code_hash: B256,
) -> Address {
    let mut data = [0u8; 1 + 20 + 32 + 32];
    data[0] = create2_prefix;
    data[1..21].copy_from_slice(deployer.as_slice());
    data[21..53].copy_from_slice(salt.as_slice());
    data[53..85].copy_from_slice(init_code_hash.as_slice());
    let hash = keccak256(data);
    Address::from_slice(&hash.as_slice()[12..])
}

pub fn number_to_u64(n: &serde_json::Number, label: &'static str) -> Result<u64> {
    let s = n.to_string();
    s.parse::<u64>().with_context(|| format!("parse {label}"))
}

pub fn i64_to_u64(v: i64, label: &'static str) -> Result<u64> {
    u64::try_from(v).with_context(|| format!("parse {label}"))
}

pub fn i64_to_u32(v: i64, label: &'static str) -> Result<u32> {
    u32::try_from(v).with_context(|| format!("parse {label}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex_bytes_accepts_0x_and_trims() {
        assert_eq!(parse_hex_bytes("  0x0a0b ").unwrap(), vec![0x0a, 0x0b]);
        assert_eq!(parse_hex_bytes("0a0b").unwrap(), vec![0x0a, 0x0b]);
    }

    #[test]
    fn parse_bytes32_accepts_32_bytes() {
        let s = format!("0x{}", "11".repeat(32));
        let b = parse_bytes32(&s).unwrap();
        assert_eq!(b.as_slice(), vec![0x11u8; 32]);
    }
}
