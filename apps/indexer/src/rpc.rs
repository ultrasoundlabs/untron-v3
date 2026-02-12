use crate::config::Stream;
use alloy::{
    providers::{DynProvider, ProviderBuilder},
    rpc::client::{BuiltInConnectionString, RpcClient},
    transports::layers::RetryBackoffLayer,
};
use anyhow::Result;
use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram, ObservableGauge},
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use untron_rpc_fallback::{FallbackAttemptStatus, FallbackHttpTransport, FallbackObserver};

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
    pub async fn from_config(stream: Stream, chain_id: u64, cfg: &RpcConfig) -> Result<Self> {
        build_providers(stream, chain_id, cfg).await
    }
}

pub(crate) async fn build_providers(
    stream: Stream,
    chain_id: u64,
    cfg: &RpcConfig,
) -> Result<RpcProviders> {
    if cfg.urls.is_empty() {
        anyhow::bail!("rpc urls must not be empty");
    }

    let retry_layer = RetryBackoffLayer::new(
        cfg.retry.max_rate_limit_retries,
        cfg.retry.initial_backoff_ms,
        cfg.retry.compute_units_per_second,
    );

    let mut pinned = Vec::with_capacity(cfg.urls.len());
    let mut healthy_urls = Vec::with_capacity(cfg.urls.len());
    let mut last_connect_err: Option<anyhow::Error> = None;

    for url in &cfg.urls {
        match BuiltInConnectionString::connect(url).await {
            Ok(transport) => {
                healthy_urls.push(url.clone());

                let client = RpcClient::builder()
                    .layer(retry_layer.clone())
                    .transport(transport, false);
                let provider = ProviderBuilder::default().connect_client(client);
                pinned.push(DynProvider::new(provider));
            }
            Err(err) => {
                tracing::warn!(rpc_url = %url, err = %err, "failed to connect rpc endpoint");
                last_connect_err =
                    Some(anyhow::Error::new(err).context(format!("connect transport: {url}")));
            }
        }
    }

    if healthy_urls.is_empty() {
        if let Some(err) = last_connect_err {
            return Err(err.context("all rpc endpoints failed to connect"));
        }
        anyhow::bail!("all rpc endpoints failed to connect");
    }

    let per_try_timeout_ms: u64 = std::env::var("RPC_PER_TRY_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2_500);
    let fallback_urls_csv = healthy_urls.join(",");
    let fallback_urls = FallbackHttpTransport::urls_from_csv(&fallback_urls_csv)?;
    let fallback_observer = Arc::new(RpcFallbackTelemetry::new(
        stream,
        chain_id,
        cfg.urls.len() as u64,
        healthy_urls.len() as u64,
    ));
    let fallback_transport = FallbackHttpTransport::new_with_observer(
        fallback_urls,
        Duration::from_millis(per_try_timeout_ms),
        Some(fallback_observer),
    )?;

    let client = RpcClient::builder()
        .layer(retry_layer)
        .transport(fallback_transport, false);
    let provider = ProviderBuilder::default().connect_client(client);

    Ok(RpcProviders {
        fallback: DynProvider::new(provider),
        pinned,
    })
}

#[derive(Clone)]
struct RpcFallbackTelemetry {
    inner: Arc<RpcFallbackTelemetryInner>,
}

struct RpcFallbackTelemetryInner {
    attrs: Vec<KeyValue>,
    attempts_total: Counter<u64>,
    attempt_ms: Histogram<u64>,
    switches_total: Counter<u64>,
    all_failed_total: Counter<u64>,
    _g_configured_endpoints: ObservableGauge<u64>,
    _g_healthy_endpoints: ObservableGauge<u64>,
}

impl RpcFallbackTelemetry {
    fn new(stream: Stream, chain_id: u64, configured: u64, healthy: u64) -> Self {
        let meter = global::meter("indexer");
        let attrs = vec![
            KeyValue::new("stream", stream.as_str()),
            KeyValue::new("chain_id", i64::try_from(chain_id).unwrap_or_default()),
        ];

        let attempts_total = meter
            .u64_counter("indexer.rpc_fallback_attempts_total")
            .with_description("Fallback transport attempts by endpoint/status")
            .build();
        let attempt_ms = meter
            .u64_histogram("indexer.rpc_fallback_attempt_ms")
            .with_description("Per-endpoint fallback attempt latency")
            .with_unit("ms")
            .build();
        let switches_total = meter
            .u64_counter("indexer.rpc_fallback_switches_total")
            .with_description("Fallback preferred endpoint switches")
            .build();
        let all_failed_total = meter
            .u64_counter("indexer.rpc_fallback_all_failed_total")
            .with_description("Fallback requests where all endpoints failed")
            .build();

        let configured_endpoints = Arc::new(AtomicU64::new(configured));
        let healthy_endpoints = Arc::new(AtomicU64::new(healthy));

        let attrs_clone = attrs.clone();
        let configured_clone = configured_endpoints.clone();
        let _g_configured_endpoints = meter
            .u64_observable_gauge("indexer.rpc_configured_endpoints")
            .with_description("Configured RPC endpoints for stream provider")
            .with_callback(move |observer| {
                observer.observe(configured_clone.load(Ordering::Relaxed), &attrs_clone);
            })
            .build();

        let attrs_clone = attrs.clone();
        let healthy_clone = healthy_endpoints.clone();
        let _g_healthy_endpoints = meter
            .u64_observable_gauge("indexer.rpc_healthy_endpoints")
            .with_description("Healthy RPC endpoints available at startup for stream provider")
            .with_callback(move |observer| {
                observer.observe(healthy_clone.load(Ordering::Relaxed), &attrs_clone);
            })
            .build();

        Self {
            inner: Arc::new(RpcFallbackTelemetryInner {
                attrs,
                attempts_total,
                attempt_ms,
                switches_total,
                all_failed_total,
                _g_configured_endpoints,
                _g_healthy_endpoints,
            }),
        }
    }
}

impl FallbackObserver for RpcFallbackTelemetry {
    fn on_attempt(
        &self,
        method: &str,
        endpoint_idx: usize,
        status: FallbackAttemptStatus,
        ms: u64,
    ) {
        let status = match status {
            FallbackAttemptStatus::Ok => "ok",
            FallbackAttemptStatus::Err => "err",
            FallbackAttemptStatus::Timeout => "timeout",
        };
        let attrs = [
            self.inner.attrs[0].clone(),
            self.inner.attrs[1].clone(),
            KeyValue::new("method", method.to_string()),
            KeyValue::new(
                "endpoint_idx",
                i64::try_from(endpoint_idx).unwrap_or_default(),
            ),
            KeyValue::new("status", status),
        ];
        self.inner.attempts_total.add(1, &attrs);
        self.inner.attempt_ms.record(ms, &attrs);
    }

    fn on_switch(&self, method: &str, from_idx: usize, to_idx: usize) {
        let attrs = [
            self.inner.attrs[0].clone(),
            self.inner.attrs[1].clone(),
            KeyValue::new("method", method.to_string()),
            KeyValue::new("from_idx", i64::try_from(from_idx).unwrap_or_default()),
            KeyValue::new("to_idx", i64::try_from(to_idx).unwrap_or_default()),
        ];
        self.inner.switches_total.add(1, &attrs);
    }

    fn on_all_failed(&self, method: &str) {
        let attrs = [
            self.inner.attrs[0].clone(),
            self.inner.attrs[1].clone(),
            KeyValue::new("method", method.to_string()),
        ];
        self.inner.all_failed_total.add(1, &attrs);
    }
}
