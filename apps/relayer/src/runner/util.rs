use crate::{metrics::RelayerTelemetry, tron::grpc::TronGrpc};
use alloy::primitives::{FixedBytes, U256};
use anyhow::{Context, Result};
use std::time::Instant;
use tracing::Instrument;

pub(super) async fn run_job<T, F, Fut>(
    telemetry: &RelayerTelemetry,
    name: &'static str,
    f: F,
) -> Result<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let span = tracing::info_span!("job", job = name);
    let start = Instant::now();
    let res = async move { f().await }.instrument(span.clone()).await;
    let ms = start.elapsed().as_millis() as u64;

    match res {
        Ok(v) => {
            telemetry.job_ok(name, ms);
            tracing::info!(parent: &span, ms, "job ok");
            Ok(v)
        }
        Err(err) => {
            telemetry.job_err(name, ms);
            tracing::error!(parent: &span, ms, err = %err, "job failed");
            Err(err)
        }
    }
}

pub(super) async fn tron_head_block(tron: &mut TronGrpc) -> Result<u64> {
    let b = tron.get_now_block2().await?;
    let raw = b
        .block_header
        .as_ref()
        .and_then(|h| h.raw_data.as_ref())
        .context("missing Tron now block header.raw_data")?;
    u64::try_from(raw.number).context("Tron head out of range")
}

pub(super) fn parse_txid32(hex32: &str) -> Result<[u8; 32]> {
    let b = parse_hex_bytes(hex32)?;
    if b.len() != 32 {
        anyhow::bail!("expected 32-byte hex, got {}", b.len());
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&b);
    Ok(out)
}

pub(super) fn parse_bytes32(hex32: &str) -> Result<FixedBytes<32>> {
    let b = parse_hex_bytes(hex32)?;
    if b.len() != 32 {
        anyhow::bail!("expected 32-byte hex, got {}", b.len());
    }
    Ok(FixedBytes::from_slice(&b))
}

pub(super) fn parse_hex_bytes(hex_bytes: &str) -> Result<Vec<u8>> {
    let s = hex_bytes.trim();
    let s = s.strip_prefix("0x").unwrap_or(s);
    hex::decode(s).context("decode hex")
}

pub(super) fn number_to_u256(n: &serde_json::Number) -> Result<U256> {
    let s = n.to_string();
    U256::from_str_radix(s.as_str(), 10).context("parse u256 decimal")
}

pub(super) fn parse_u256_decimal(s: &str) -> Result<U256> {
    let s = s.trim().replace('_', "");
    U256::from_str_radix(&s, 10).context("parse u256")
}

pub(super) fn u256_to_u64(v: U256) -> Option<u64> {
    if v > U256::from(u64::MAX) {
        return None;
    }
    Some(v.to::<u64>())
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
    fn parse_txid32_rejects_wrong_length() {
        let err = parse_txid32("0x01").unwrap_err().to_string();
        assert!(err.contains("expected 32-byte"));
    }

    #[test]
    fn parse_bytes32_accepts_32_bytes() {
        let s = format!("0x{}", "11".repeat(32));
        let b = parse_bytes32(&s).unwrap();
        assert_eq!(b.as_slice(), vec![0x11u8; 32]);
    }

    #[test]
    fn parse_u256_decimal_accepts_underscores_and_whitespace() {
        let v = parse_u256_decimal("  1_000_000 ").unwrap();
        assert_eq!(v, U256::from(1_000_000u64));
    }

    #[test]
    fn number_to_u256_parses_decimal() {
        let n = serde_json::Number::from(12345u64);
        let v = number_to_u256(&n).unwrap();
        assert_eq!(v, U256::from(12345u64));
    }

    #[test]
    fn u256_to_u64_bounds() {
        assert_eq!(u256_to_u64(U256::from(u64::MAX)), Some(u64::MAX));
        assert_eq!(u256_to_u64(U256::from(u64::MAX) + U256::from(1u64)), None);
    }
}
