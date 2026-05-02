use alloy_json_rpc::RequestPacket;
use alloy_rpc_client::RpcClient;
use alloy_transport::{BoxTransport, TransportError, TransportErrorKind, TransportFut};
use alloy_transport_http::ReqwestTransport;
use parking_lot::Mutex;
use std::{sync::Arc, task, time::Instant};
use tokio::time::{Duration, timeout};
use tower::Service;
use tracing::{debug, warn};
use url::Url;

/// Shared, cloneable JSON-RPC request pacer.
///
/// This uses fixed spacing rather than a bursty token bucket. If configured for
/// 50 requests/sec, calls are reserved at least 20ms apart across every clone
/// sharing the same limiter, which prevents short sliding-window bursts above
/// the upstream RPC's advertised throughput.
#[derive(Clone, Debug)]
pub struct RpcRateLimiter {
    inner: Arc<RpcRateLimiterInner>,
}

#[derive(Debug)]
struct RpcRateLimiterInner {
    spacing: Duration,
    next_at: Mutex<tokio::time::Instant>,
}

impl RpcRateLimiter {
    pub fn new(max_requests_per_second: u32) -> anyhow::Result<Self> {
        anyhow::ensure!(
            max_requests_per_second > 0,
            "max_requests_per_second must be greater than zero"
        );

        let spacing = spacing_for_rate(max_requests_per_second);
        Ok(Self {
            inner: Arc::new(RpcRateLimiterInner {
                spacing,
                next_at: Mutex::new(tokio::time::Instant::now()),
            }),
        })
    }

    pub async fn until_ready(&self) {
        let scheduled = {
            let mut next_at = self.inner.next_at.lock();
            let now = tokio::time::Instant::now();
            let scheduled = (*next_at).max(now);
            *next_at = scheduled + self.inner.spacing;
            scheduled
        };

        tokio::time::sleep_until(scheduled).await;
    }
}

fn spacing_for_rate(max_requests_per_second: u32) -> Duration {
    const NANOS_PER_SECOND: u64 = 1_000_000_000;
    let rate = u64::from(max_requests_per_second);
    let nanos = NANOS_PER_SECOND.div_ceil(rate).max(1);
    Duration::from_nanos(nanos)
}

#[derive(Clone, Debug)]
pub struct RateLimitedTransport {
    inner: BoxTransport,
    limiter: RpcRateLimiter,
}

impl RateLimitedTransport {
    pub fn new(inner: BoxTransport, limiter: RpcRateLimiter) -> Self {
        Self { inner, limiter }
    }
}

impl Service<RequestPacket> for RateLimitedTransport {
    type Response = alloy_json_rpc::ResponsePacket;
    type Error = TransportError;
    type Future = TransportFut<'static>;

    fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> task::Poll<Result<(), Self::Error>> {
        task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: RequestPacket) -> Self::Future {
        let limiter = self.limiter.clone();
        let mut inner = self.inner.clone();
        Box::pin(async move {
            limiter.until_ready().await;
            tower::ServiceExt::ready(&mut inner).await?;
            inner.call(req).await
        })
    }
}

/// A best-effort JSON-RPC transport that failovers across multiple HTTP endpoints.
///
/// Design goals (v1):
/// - minimal surface area: drop-in for `RpcClient::builder().transport(...)`
/// - fast failover on network/http errors and timeouts
/// - sticky success: remember the last good endpoint
///
/// Non-goals (yet):
/// - sophisticated health scoring
/// - per-method policies / circuit breaker metrics
#[derive(Clone)]
pub struct FallbackHttpTransport {
    inner: Arc<Inner>,
}

struct Inner {
    transports: Vec<BoxTransport>,
    /// Index of the preferred transport (sticky on success).
    preferred: Mutex<usize>,
    per_try_timeout: Duration,
    observer: Option<Arc<dyn FallbackObserver>>,
    limiter: Option<RpcRateLimiter>,
}

#[derive(Clone, Copy, Debug)]
pub enum FallbackAttemptStatus {
    Ok,
    Err,
    Timeout,
}

pub trait FallbackObserver: Send + Sync {
    fn on_attempt(&self, method: &str, endpoint_idx: usize, status: FallbackAttemptStatus, ms: u64);
    fn on_switch(&self, method: &str, from_idx: usize, to_idx: usize);
    fn on_all_failed(&self, method: &str);
}

impl FallbackHttpTransport {
    pub fn new(urls: Vec<Url>, per_try_timeout: Duration) -> anyhow::Result<Self> {
        Self::new_with_observer(urls, per_try_timeout, None)
    }

    pub fn new_with_observer(
        urls: Vec<Url>,
        per_try_timeout: Duration,
        observer: Option<Arc<dyn FallbackObserver>>,
    ) -> anyhow::Result<Self> {
        Self::new_with_observer_and_limiter(urls, per_try_timeout, observer, None)
    }

    pub fn new_with_observer_and_limiter(
        urls: Vec<Url>,
        per_try_timeout: Duration,
        observer: Option<Arc<dyn FallbackObserver>>,
        limiter: Option<RpcRateLimiter>,
    ) -> anyhow::Result<Self> {
        anyhow::ensure!(!urls.is_empty(), "at least one RPC URL is required");

        let transports = urls
            .into_iter()
            .map(|u| BoxTransport::new(ReqwestTransport::new(u)))
            .collect::<Vec<_>>();

        Ok(Self {
            inner: Arc::new(Inner {
                transports,
                preferred: Mutex::new(0),
                per_try_timeout,
                observer,
                limiter,
            }),
        })
    }

    pub fn urls_from_csv(csv: &str) -> anyhow::Result<Vec<Url>> {
        let urls = csv
            .split(|c: char| c == ',' || c.is_whitespace())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| Url::parse(s).map_err(|e| anyhow::anyhow!("invalid rpc url '{s}': {e}")))
            .collect::<Result<Vec<_>, _>>()?;
        anyhow::ensure!(!urls.is_empty(), "no RPC URLs found in list");
        Ok(urls)
    }

    /// Return the first URL from a CSV list.
    ///
    /// Useful for components that still require a single RPC endpoint string.
    pub fn first_url_from_csv(csv: &str) -> anyhow::Result<Url> {
        Ok(Self::urls_from_csv(csv)?
            .into_iter()
            .next()
            .expect("non-empty"))
    }

    pub fn rpc_client(self) -> RpcClient {
        // `false` means "not local" (used by alloy for some defaults). We keep it false.
        RpcClient::builder().transport(self, false)
    }

    fn preferred_index(&self) -> usize {
        *self.inner.preferred.lock()
    }

    fn set_preferred(&self, idx: usize) {
        *self.inner.preferred.lock() = idx;
    }
}

impl Service<RequestPacket> for FallbackHttpTransport {
    type Response = alloy_json_rpc::ResponsePacket;
    type Error = TransportError;
    type Future = TransportFut<'static>;

    fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> task::Poll<Result<(), Self::Error>> {
        // We'll lazily attempt requests; treat as always-ready.
        task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: RequestPacket) -> Self::Future {
        let this = self.clone();
        Box::pin(async move {
            let n = this.inner.transports.len();
            let start = this.preferred_index();
            let method = request_method_label(&req);

            // Try preferred first, then round-robin.
            for offset in 0..n {
                let idx = (start + offset) % n;

                let mut t = this.inner.transports[idx].clone();
                let per_try_timeout = this.inner.per_try_timeout;
                if let Some(limiter) = &this.inner.limiter {
                    limiter.until_ready().await;
                }

                let attempt_start = Instant::now();
                let attempt = async {
                    // `BoxTransport` is a `tower::Service`, so we must poll_ready + call.
                    // poll_ready is usually cheap for HTTP.
                    tower::ServiceExt::ready(&mut t).await?;
                    t.call(req.clone()).await
                };

                match timeout(per_try_timeout, attempt).await {
                    Ok(Ok(resp)) => {
                        if let Some(observer) = &this.inner.observer {
                            observer.on_attempt(
                                method,
                                idx,
                                FallbackAttemptStatus::Ok,
                                attempt_start.elapsed().as_millis() as u64,
                            );
                        }
                        if idx != start {
                            if let Some(observer) = &this.inner.observer {
                                observer.on_switch(method, start, idx);
                            }
                            debug!(
                                from = start,
                                to = idx,
                                "rpc transport failover succeeded; updating preferred"
                            );
                        }
                        this.set_preferred(idx);
                        return Ok(resp);
                    }
                    Ok(Err(e)) => {
                        if let Some(observer) = &this.inner.observer {
                            observer.on_attempt(
                                method,
                                idx,
                                FallbackAttemptStatus::Err,
                                attempt_start.elapsed().as_millis() as u64,
                            );
                        }
                        warn!(idx, err = %e, "rpc transport attempt failed");
                        continue;
                    }
                    Err(_) => {
                        if let Some(observer) = &this.inner.observer {
                            observer.on_attempt(
                                method,
                                idx,
                                FallbackAttemptStatus::Timeout,
                                attempt_start.elapsed().as_millis() as u64,
                            );
                        }
                        warn!(
                            idx,
                            timeout_ms = per_try_timeout.as_millis(),
                            "rpc transport attempt timed out"
                        );
                        continue;
                    }
                }
            }

            if let Some(observer) = &this.inner.observer {
                observer.on_all_failed(method);
            }

            // If all failed, return a synthetic error.
            Err(TransportErrorKind::custom(std::io::Error::other(
                "all rpc endpoints failed",
            )))
        })
    }
}

/// Convenience: build an `RpcClient` from a CSV list of HTTP(S) URLs.
pub fn rpc_client_from_urls_csv(
    urls_csv: &str,
    per_try_timeout: Duration,
) -> anyhow::Result<RpcClient> {
    let urls = FallbackHttpTransport::urls_from_csv(urls_csv)?;
    Ok(FallbackHttpTransport::new(urls, per_try_timeout)?.rpc_client())
}

fn request_method_label(req: &RequestPacket) -> &str {
    let mut methods = req.method_names();
    let Some(first) = methods.next() else {
        return "<empty>";
    };
    if methods.next().is_some() {
        "<batch>"
    } else {
        first
    }
}

#[cfg(test)]
mod tests {
    use super::spacing_for_rate;
    use tokio::time::Duration;

    #[test]
    fn rate_spacing_rounds_up_to_avoid_microbursts() {
        assert_eq!(spacing_for_rate(50), Duration::from_millis(20));
        assert_eq!(spacing_for_rate(3), Duration::from_nanos(333_333_334));
    }
}
