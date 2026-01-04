use alloy::primitives::Address;
use sha2::{Digest, Sha256};
use std::fmt;
use std::str::FromStr;

/// Tron mainnet base58check address (0x41 || addr20).
///
/// - In-memory representation is always the 20-byte EVM-form `Address`.
/// - String/DB representation is always canonical Tron base58check (T...).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TronAddress(Address);

impl TronAddress {
    pub const MAINNET_PREFIX: u8 = 0x41;

    pub fn from_evm(addr: Address) -> Self {
        Self(addr)
    }

    pub fn evm(self) -> Address {
        self.0
    }

    pub fn from_base58check(value: &str) -> anyhow::Result<Self> {
        let payload = bs58::decode(value)
            .with_check(None)
            .into_vec()
            .map_err(|e| anyhow::Error::new(e).context("base58check decode"))?;

        if payload.len() != 21 {
            anyhow::bail!(
                "invalid Tron address length: expected 21 bytes, got {}",
                payload.len()
            );
        }
        if payload[0] != Self::MAINNET_PREFIX {
            anyhow::bail!(
                "unexpected Tron address prefix: expected 0x{:02x}, got 0x{:02x}",
                Self::MAINNET_PREFIX,
                payload[0],
            );
        }

        Ok(Self(Address::from_slice(&payload[1..])))
    }

    /// Parses either Tron base58check (T...) or EVM hex (0x...).
    ///
    /// Mirrors the DB helper `chain.tron_address_from_text(text)`.
    pub fn parse_text(value: &str) -> anyhow::Result<Self> {
        let trimmed = value.trim();
        if trimmed.starts_with('T') {
            return Self::from_base58check(trimmed);
        }
        if let Ok(evm) = trimmed.parse::<Address>() {
            return Ok(Self::from_evm(evm));
        }
        anyhow::bail!("invalid Tron address text: {value}");
    }

    pub fn to_base58check(self) -> String {
        let mut payload = [0u8; 21];
        payload[0] = Self::MAINNET_PREFIX;
        payload[1..].copy_from_slice(self.0.as_slice());

        // checksum = first 4 bytes of SHA256(SHA256(payload))
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let first = hasher.finalize_reset();
        hasher.update(first);
        let second = hasher.finalize();
        let checksum = &second[..4];

        let mut with_checksum = [0u8; 25];
        with_checksum[..21].copy_from_slice(&payload);
        with_checksum[21..].copy_from_slice(checksum);

        bs58::encode(with_checksum).into_string()
    }
}

impl From<Address> for TronAddress {
    fn from(value: Address) -> Self {
        Self::from_evm(value)
    }
}

impl From<TronAddress> for Address {
    fn from(value: TronAddress) -> Self {
        value.0
    }
}

impl fmt::Display for TronAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_base58check())
    }
}

impl FromStr for TronAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_text(s)
    }
}
