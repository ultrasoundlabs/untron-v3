use crate::api::ApiError;
use aa::Safe4337UserOpSender;
use alloy::primitives::Address;
use tokio::sync::MutexGuard;

pub(super) async fn send_userop(
    mut sender: MutexGuard<'_, Safe4337UserOpSender>,
    to: Address,
    data: Vec<u8>,
    timeout: std::time::Duration,
) -> Result<(String, String, u64), ApiError> {
    let start = std::time::Instant::now();

    tracing::info!(
        to = %format!("{:#x}", to),
        data_len = data.len(),
        timeout_ms = timeout.as_millis() as u64,
        "send_userop: starting send_call"
    );

    let sub = tokio::time::timeout(timeout, sender.send_call(to, data))
        .await
        .map_err(|_| {
            ApiError::Upstream(format!(
                "send userop: timeout after {}ms",
                timeout.as_millis()
            ))
        })?
        .map_err(|e| ApiError::Upstream(format!("send userop: {e}")))?;

    let ms = start.elapsed().as_millis() as u64;
    tracing::info!(
        ms,
        userop_hash = %sub.userop_hash,
        nonce = %sub.nonce,
        send_attempts = sub.send_attempts,
        "send_userop: send_call completed"
    );

    Ok((sub.userop_hash, sub.nonce.to_string(), sub.send_attempts))
}
