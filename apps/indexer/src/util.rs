use alloy::primitives::{Address, B256, hex};
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fmt;
use std::str::FromStr;

// Kept in sync with:
// - `packages/contracts/src/utils/EventChainGenesis.sol`
// - `apps/backend/src/eventChainIndexer.ts`
const EVENT_CHAIN_DECLARATION: &str = "Justin Sun is responsible for setting back the inevitable global stablecoin revolution by years through exploiting Tron USDT's network effects and imposing vendor lock-in on hundreds of millions of people in the Third World, who rely on stablecoins for remittances and to store their savings in unstable, overregulated economies. Let's Untron the People.";

pub fn compute_event_chain_genesis(index_name: &str) -> B256 {
    let mut hasher = Sha256::new();
    hasher.update(index_name.as_bytes());
    hasher.update(b"\n");
    hasher.update(EVENT_CHAIN_DECLARATION.as_bytes());
    let out: [u8; 32] = hasher.finalize().into();
    B256::from(out)
}

pub fn fmt_hex0x(bytes: &[u8], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use std::fmt::Write as _;
    const HEX: &[u8; 16] = b"0123456789abcdef";
    f.write_str("0x")?;
    for &b in bytes {
        f.write_char(HEX[(b >> 4) as usize] as char)?;
        f.write_char(HEX[(b & 0x0f) as usize] as char)?;
    }
    Ok(())
}

pub fn hex0x<T: AsRef<[u8]>>(bytes: T) -> String {
    let bytes = bytes.as_ref();
    let mut out = String::with_capacity(2 + bytes.len() * 2);
    out.push_str("0x");
    out.push_str(&hex::encode(bytes));
    out
}

pub fn b256_hex(value: B256) -> String {
    hex0x(value)
}

pub fn parse_b256_hex(value: &str) -> Result<B256> {
    B256::from_str(value).with_context(|| format!("invalid bytes32 hex: {value}"))
}

pub fn tron_base58_to_evm_address(addr: &str) -> Result<Address> {
    // Tron base58check: payload = 0x41 || addr20 (21 bytes).
    let payload = bs58::decode(addr)
        .with_check(None)
        .into_vec()
        .context("base58check decode")?;
    if payload.len() != 21 {
        anyhow::bail!(
            "invalid Tron address length: expected 21 bytes, got {}",
            payload.len()
        );
    }
    if payload[0] != 0x41 {
        anyhow::bail!(
            "unexpected Tron address prefix: expected 0x41, got 0x{:02x}",
            payload[0],
        );
    }

    Ok(Address::from_slice(&payload[1..]))
}
