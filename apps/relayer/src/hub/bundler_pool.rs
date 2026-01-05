use alloy::{
    primitives::Address,
    providers::{DynProvider, ProviderBuilder},
    rpc::client::{BuiltInConnectionString, RpcClient},
};
use alloy_provider::ext::Erc4337Api;
use anyhow::{Context, Result};
use std::time::Duration;

use alloy::rpc::types::eth::erc4337::{PackedUserOperation, SendUserOperation};
use alloy::rpc::types::eth::erc4337::{SendUserOperationResponse, UserOperationGasEstimation};

const RPC_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone)]
pub(super) struct BundlerPool {
    urls: Vec<String>,
    providers: Vec<DynProvider>,
    next_idx: usize,
}

impl BundlerPool {
    pub(super) async fn new(urls: Vec<String>) -> Result<Self> {
        if urls.is_empty() {
            anyhow::bail!("HUB_BUNDLER_URLS must be non-empty");
        }

        let mut providers = Vec::with_capacity(urls.len());
        for url in &urls {
            let transport = BuiltInConnectionString::connect(url)
                .await
                .with_context(|| format!("connect bundler rpc: {url}"))?;
            let client = RpcClient::builder().transport(transport, false);
            let provider = ProviderBuilder::default().connect_client(client);
            providers.push(DynProvider::new(provider));
        }

        Ok(Self {
            urls,
            providers,
            next_idx: 0,
        })
    }

    fn mark_success(&mut self, idx: usize) {
        if !self.providers.is_empty() {
            self.next_idx = (idx + 1) % self.providers.len();
        }
    }

    pub(super) async fn estimate_user_operation_gas(
        &mut self,
        user_op: &PackedUserOperation,
        entry_point: Address,
    ) -> Result<UserOperationGasEstimation> {
        let mut last_err: Option<anyhow::Error> = None;

        let order = rotate_order(self.next_idx, self.providers.len());
        for idx in order {
            let url = &self.urls[idx];
            let provider = &self.providers[idx];

            let fut = provider.estimate_user_operation_gas(
                SendUserOperation::EntryPointV07(user_op.clone()),
                entry_point,
            );

            match tokio::time::timeout(RPC_TIMEOUT, fut).await {
                Ok(Ok(v)) => {
                    self.mark_success(idx);
                    return Ok(v);
                }
                Ok(Err(err)) => {
                    let err = anyhow::Error::new(err).context("eth_estimateUserOperationGas");
                    tracing::warn!(bundler = %url, err = %err, "bundler rpc failed");
                    last_err = Some(err);
                }
                Err(_) => {
                    let err = anyhow::anyhow!("timed out");
                    tracing::warn!(
                        bundler = %url,
                        err = %err,
                        "bundler rpc timed out (eth_estimateUserOperationGas)"
                    );
                    last_err = Some(err);
                }
            }
        }

        Err(last_err.unwrap_or_else(|| {
            anyhow::anyhow!("all bundlers failed for eth_estimateUserOperationGas")
        }))
    }

    pub(super) async fn send_user_operation(
        &mut self,
        user_op: &PackedUserOperation,
        entry_point: Address,
    ) -> Result<SendUserOperationResponse> {
        let mut last_err: Option<anyhow::Error> = None;

        let order = rotate_order(self.next_idx, self.providers.len());
        for idx in order {
            let url = &self.urls[idx];
            let provider = &self.providers[idx];

            let fut = provider.send_user_operation(
                SendUserOperation::EntryPointV07(user_op.clone()),
                entry_point,
            );

            match tokio::time::timeout(RPC_TIMEOUT, fut).await {
                Ok(Ok(v)) => {
                    self.mark_success(idx);
                    return Ok(v);
                }
                Ok(Err(err)) => {
                    let err = anyhow::Error::new(err).context("eth_sendUserOperation");
                    tracing::warn!(bundler = %url, err = %err, "bundler rpc failed");
                    last_err = Some(err);
                }
                Err(_) => {
                    let err = anyhow::anyhow!("timed out");
                    tracing::warn!(
                        bundler = %url,
                        err = %err,
                        "bundler rpc timed out (eth_sendUserOperation)"
                    );
                    last_err = Some(err);
                }
            }
        }

        Err(last_err
            .unwrap_or_else(|| anyhow::anyhow!("all bundlers failed for eth_sendUserOperation")))
    }

    pub(super) async fn supported_entry_points(&mut self) -> Result<Vec<Address>> {
        let mut last_err: Option<anyhow::Error> = None;

        let order = rotate_order(self.next_idx, self.providers.len());
        for idx in order {
            let url = &self.urls[idx];
            let provider = &self.providers[idx];

            let fut = provider.supported_entry_points();
            match tokio::time::timeout(RPC_TIMEOUT, fut).await {
                Ok(Ok(v)) => {
                    self.mark_success(idx);
                    return Ok(v);
                }
                Ok(Err(err)) => {
                    let err = anyhow::Error::new(err).context("eth_supportedEntryPoints");
                    tracing::warn!(bundler = %url, err = %err, "bundler rpc failed");
                    last_err = Some(err);
                }
                Err(_) => {
                    let err = anyhow::anyhow!("timed out");
                    tracing::warn!(
                        bundler = %url,
                        err = %err,
                        "bundler rpc timed out (eth_supportedEntryPoints)"
                    );
                    last_err = Some(err);
                }
            }
        }

        Err(last_err
            .unwrap_or_else(|| anyhow::anyhow!("all bundlers failed for eth_supportedEntryPoints")))
    }
}

fn rotate_order(start_idx: usize, len: usize) -> Vec<usize> {
    if len == 0 {
        return Vec::new();
    }
    let start = start_idx % len;
    (0..len).map(|o| (start + o) % len).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_rotates_from_next_idx() {
        assert_eq!(rotate_order(0, 3), vec![0, 1, 2]);
        assert_eq!(rotate_order(2, 3), vec![2, 0, 1]);
        assert_eq!(rotate_order(5, 3), vec![2, 0, 1]);
    }
}
