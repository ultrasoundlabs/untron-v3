mod bundler_pool;
mod contracts;
mod packing;
pub mod paymaster;
mod safe;
mod sender;
mod signing;

pub use sender::{
    PaymasterFinalizationMode, Safe4337UserOpSender, Safe4337UserOpSenderConfig,
    Safe4337UserOpSenderOptions, Safe4337UserOpSubmission,
};

pub use safe::{Safe4337Config, SafeDeterministicDeploymentConfig};

/// Wait for an ERC-4337 UserOperation receipt via bundler RPC (eth_getUserOperationReceipt).
///
/// This is useful when callers do not want to hold a `Safe4337UserOpSender` mutex guard while waiting.
pub async fn wait_user_operation_receipt(
    bundler_urls: Vec<String>,
    userop_hash0x: &str,
    timeout: std::time::Duration,
) -> anyhow::Result<alloy::rpc::types::eth::erc4337::UserOperationReceipt> {
    use anyhow::Context;
    use std::time::Duration;

    let userop_hash0x = userop_hash0x.trim();
    let hex_str = userop_hash0x.strip_prefix("0x").unwrap_or(userop_hash0x);
    let bytes = hex::decode(hex_str).context("invalid userop hash hex")?;
    let userop_hash = alloy::primitives::Bytes::from(bytes);

    let mut pool = bundler_pool::BundlerPool::new(bundler_urls).await?;

    let deadline = std::time::Instant::now() + timeout;
    let mut backoff = Duration::from_millis(250);

    loop {
        match pool.get_user_operation_receipt(userop_hash.clone()).await {
            Ok(Some(r)) => return Ok(r),
            Ok(None) => {}
            Err(e) => {
                tracing::warn!(err = %format!("{e:#}"), "eth_getUserOperationReceipt failed; retrying");
            }
        }

        if std::time::Instant::now() >= deadline {
            anyhow::bail!("timed out waiting for eth_getUserOperationReceipt");
        }

        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(Duration::from_secs(2));
    }
}

