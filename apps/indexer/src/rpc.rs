use alloy::{
    providers::{DynProvider, ProviderBuilder},
    rpc::client::{BuiltInConnectionString, RpcClient},
    transports::layers::{FallbackLayer, RetryBackoffLayer},
};
use anyhow::{Context, Result};
use std::num::NonZeroUsize;
use tower::ServiceBuilder;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_rate_limit_retries: u32,
    pub initial_backoff_ms: u64,
    pub compute_units_per_second: u64,
}

#[derive(Debug, Clone)]
pub struct RpcConfig {
    pub urls: Vec<String>,
    pub retry: RetryConfig,
}

#[derive(Clone)]
pub struct RpcProviders {
    pub fallback: DynProvider,
    pub pinned: Vec<DynProvider>,
}

impl RpcProviders {
    pub async fn from_config(cfg: &RpcConfig) -> Result<Self> {
        build_providers(cfg).await
    }
}

pub(crate) async fn build_providers(cfg: &RpcConfig) -> Result<RpcProviders> {
    if cfg.urls.is_empty() {
        anyhow::bail!("rpc urls must not be empty");
    }

    let retry_layer = RetryBackoffLayer::new(
        cfg.retry.max_rate_limit_retries,
        cfg.retry.initial_backoff_ms,
        cfg.retry.compute_units_per_second,
    );

    let mut pinned = Vec::with_capacity(cfg.urls.len());
    let mut transports = Vec::with_capacity(cfg.urls.len());

    for url in &cfg.urls {
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
