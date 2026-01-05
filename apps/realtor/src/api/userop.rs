use crate::api::ApiError;
use aa::Safe4337UserOpSender;
use alloy::primitives::Address;
use tokio::sync::MutexGuard;

pub(super) async fn send_userop(
    mut sender: MutexGuard<'_, Safe4337UserOpSender>,
    to: Address,
    data: Vec<u8>,
) -> Result<(String, String), ApiError> {
    tracing::debug!(to = %format!("{:#x}", to), data_len = data.len(), "sending userop");
    let sub = sender
        .send_call(to, data)
        .await
        .map_err(|e| ApiError::Upstream(format!("send userop: {e}")))?;
    Ok((sub.userop_hash, sub.nonce.to_string()))
}
