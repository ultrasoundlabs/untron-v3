use crate::config::RetryConfig;
use alloy::{
    providers::{DynProvider, ProviderBuilder},
    rpc::client::{BuiltInConnectionString, RpcClient},
    transports::layers::{FallbackLayer, RetryBackoffLayer},
};
use anyhow::{Context, Result};
use std::num::NonZeroUsize;
use tower::ServiceBuilder;

#[derive(Clone)]
pub struct RpcProviders {
    pub fallback: DynProvider,
    pub pinned: Vec<DynProvider>,
}

pub async fn build_providers(rpc_urls: &[String], retry: &RetryConfig) -> Result<RpcProviders> {
    if rpc_urls.is_empty() {
        anyhow::bail!("rpc_urls must not be empty");
    }

    let retry_layer = RetryBackoffLayer::new(
        retry.max_rate_limit_retries,
        retry.initial_backoff_ms,
        retry.compute_units_per_second,
    );

    let mut pinned = Vec::with_capacity(rpc_urls.len());
    let mut transports = Vec::with_capacity(rpc_urls.len());

    for url in rpc_urls {
        let transport = BuiltInConnectionString::connect(url)
            .await
            .with_context(|| format!("connect transport: {url}"))?;
        transports.push(transport.clone());

        let client = RpcClient::builder()
            .layer(retry_layer.clone())
            .transport(transport, false);
        let provider = ProviderBuilder::default().connect_client(client);
        pinned.push(DynProvider::new(provider));
    }

    let fallback_transport = ServiceBuilder::new()
        .layer(
            FallbackLayer::default()
                .with_active_transport_count(NonZeroUsize::new(1).expect("nonzero")),
        )
        .service(transports);

    let client = RpcClient::builder()
        .layer(retry_layer)
        .transport(fallback_transport, false);
    let provider = ProviderBuilder::default().connect_client(client);

    Ok(RpcProviders {
        fallback: DynProvider::new(provider),
        pinned,
    })
}
