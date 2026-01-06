#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

use opentelemetry::global;
use opentelemetry_http::HeaderInjector;
use tracing_opentelemetry::OpenTelemetrySpanExt;

impl Client {
    /// Hook invoked by generated builders before the request is executed.
    ///
    /// We use it to inject W3C trace context (`traceparent`) into the outgoing request,
    /// so calls from `realtor`/`relayer` to the indexer can be stitched into a single trace.
    pub async fn pre(
        &self,
        request: &mut reqwest::Request,
        _info: &progenitor_client::OperationInfo,
    ) -> Result<(), Error<()>> {
        let cx = tracing::Span::current().context();
        global::get_text_map_propagator(|p| {
            p.inject_context(&cx, &mut HeaderInjector(request.headers_mut()))
        });
        Ok(())
    }
}
