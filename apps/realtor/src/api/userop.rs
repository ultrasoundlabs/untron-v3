use crate::api::ApiError;
use aa::Safe4337UserOpSender;
use alloy::primitives::Address;
use tokio::sync::MutexGuard;

pub(super) async fn send_userop(
    mut sender: MutexGuard<'_, Safe4337UserOpSender>,
    to: Address,
    data: Vec<u8>,
) -> Result<(String, String), ApiError> {
    let start = std::time::Instant::now();

    // NOTE: this call is a common place for hangs (bundler/RPC).
    // Keep this log at INFO while debugging; downgrade once stable.
    tracing::info!(
        to = %format!("{:#x}", to),
        data_len = data.len(),
        "send_userop: starting send_call"
    );

    let sub = sender
        .send_call(to, data)
        .await
        .map_err(|e| ApiError::Upstream(format!("send userop: {e}")))?;

    let ms = start.elapsed().as_millis() as u64;
    tracing::info!(
        ms,
        userop_hash = %sub.userop_hash,
        nonce = %sub.nonce,
        "send_userop: send_call completed"
    );

    Ok((sub.userop_hash, sub.nonce.to_string()))
}
