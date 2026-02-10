use alloy_json_rpc::RequestPacket;
use alloy_rpc_client::RpcClient;
use alloy_transport::{BoxTransport, TransportError, TransportErrorKind, TransportFut};
use alloy_transport_http::ReqwestTransport;
use parking_lot::Mutex;
use std::{sync::Arc, task};
use tokio::time::{timeout, Duration};
use tower::Service;
use tracing::{debug, warn};
use url::Url;

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
#[derive(Clone, Debug)]
pub struct FallbackHttpTransport {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    transports: Vec<BoxTransport>,
    /// Index of the preferred transport (sticky on success).
    preferred: Mutex<usize>,
    per_try_timeout: Duration,
}

impl FallbackHttpTransport {
    pub fn new(urls: Vec<Url>, per_try_timeout: Duration) -> anyhow::Result<Self> {
        anyhow::ensure!(!urls.is_empty(), "at least one RPC URL is required");

        let transports = urls
            .into_iter()
            .map(|u| BoxTransport::new(ReqwestTransport::new(u)))
            .collect::<Vec<_>>();

        Ok(Self {
            inner: Arc::new(Inner { transports, preferred: Mutex::new(0), per_try_timeout }),
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
        Ok(Self::urls_from_csv(csv)?.into_iter().next().expect("non-empty"))
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

            // Try preferred first, then round-robin.
            for offset in 0..n {
                let idx = (start + offset) % n;

                let mut t = this.inner.transports[idx].clone();
                let per_try_timeout = this.inner.per_try_timeout;

                let attempt = async {
                    // `BoxTransport` is a `tower::Service`, so we must poll_ready + call.
                    // poll_ready is usually cheap for HTTP.
                    tower::ServiceExt::ready(&mut t).await?;
                    t.call(req.clone()).await
                };

                match timeout(per_try_timeout, attempt).await {
                    Ok(Ok(resp)) => {
                        if idx != start {
                            debug!(from = start, to = idx, "rpc transport failover succeeded; updating preferred");
                        }
                        this.set_preferred(idx);
                        return Ok(resp);
                    }
                    Ok(Err(e)) => {
                        warn!(idx, err = %e, "rpc transport attempt failed");
                        continue;
                    }
                    Err(_) => {
                        warn!(idx, timeout_ms = per_try_timeout.as_millis(), "rpc transport attempt timed out");
                        continue;
                    }
                }
            }

            // If all failed, return a synthetic error.
            Err(
                TransportErrorKind::custom(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "all rpc endpoints failed",
                ))
                .into(),
            )
        })
    }
}

/// Convenience: build an `RpcClient` from a CSV list of HTTP(S) URLs.
pub fn rpc_client_from_urls_csv(urls_csv: &str, per_try_timeout: Duration) -> anyhow::Result<RpcClient> {
    let urls = FallbackHttpTransport::urls_from_csv(urls_csv)?;
    Ok(FallbackHttpTransport::new(urls, per_try_timeout)?.rpc_client())
}
