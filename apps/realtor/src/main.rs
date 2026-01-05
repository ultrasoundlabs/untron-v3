mod api;
mod config;
mod indexer;
mod metrics;
mod observability;
mod util;

use crate::config::AppConfig;
use crate::indexer::IndexerApi;
use crate::metrics::RealtorTelemetry;
use aa::paymaster::PaymasterService;
use aa::{
    PaymasterFinalizationMode, Safe4337UserOpSender, Safe4337UserOpSenderConfig,
    Safe4337UserOpSenderOptions,
};
use axum::extract::MatchedPath;
use axum::http::{Request, Response, header::HeaderName};
use axum::{Router, routing::get};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cfg = config::load_config()?;
    let otel = observability::init("realtor")?;

    tracing::info!("realtor starting");
    tracing::info!(
        bind = %cfg.api.bind,
        indexer = %cfg.indexer.base_url,
        hub_rpc = %cfg.hub.rpc_url,
        safe = %cfg.hub.safe,
        "config loaded"
    );

    let indexer = IndexerApi::new(&cfg.indexer.base_url, cfg.indexer.timeout)?;
    let sender_cfg = Safe4337UserOpSenderConfig {
        rpc_url: cfg.hub.rpc_url.clone(),
        chain_id: cfg.hub.chain_id,
        entrypoint: cfg.hub.entrypoint,
        safe: cfg.hub.safe,
        safe_4337_module: cfg.hub.safe_4337_module,
        bundler_urls: cfg.hub.bundler_urls.clone(),
        owner_private_key: cfg.hub.owner_private_key,
        paymasters: cfg
            .hub
            .paymasters
            .iter()
            .map(|pm| PaymasterService {
                url: pm.url.clone(),
                context: pm.context.clone(),
            })
            .collect(),
        options: Safe4337UserOpSenderOptions {
            check_bundler_entrypoints: false,
            paymaster_finalization: PaymasterFinalizationMode::AlwaysFetchFinal,
        },
    };
    let sender = Safe4337UserOpSender::new(sender_cfg).await?;
    let telemetry = RealtorTelemetry::new();

    let state = AppState {
        cfg,
        indexer,
        sender: Mutex::new(sender),
        telemetry,
    };
    let bind = state.cfg.api.bind;

    let request_id_header = HeaderName::from_static("x-request-id");
    let app = Router::new()
        .route("/realtor", get(api::get_realtor).post(api::post_realtor))
        .with_state(Arc::new(state))
        .route_layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &Request<_>| {
                    let matched = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|m| m.as_str())
                        .unwrap_or(req.uri().path());
                    let request_id = req
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("-");
                    tracing::info_span!(
                        "http.request",
                        request_id = %request_id,
                        method = %req.method(),
                        path = %matched
                    )
                })
                .on_response(
                    |res: &Response<_>, latency: std::time::Duration, span: &tracing::Span| {
                        tracing::info!(
                            parent: span,
                            status = res.status().as_u16(),
                            latency_ms = latency.as_millis() as u64,
                            "http.response"
                        );
                    },
                ),
        )
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        .layer(SetRequestIdLayer::new(request_id_header, MakeRequestUuid));

    let shutdown = CancellationToken::new();
    let listener = tokio::net::TcpListener::bind(bind).await?;

    tracing::info!("listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(shutdown.clone()))
        .await?;

    shutdown.cancel();
    otel.shutdown().await;
    Ok(())
}

pub fn now_unix_seconds() -> Result<u64, String> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .map_err(|e| format!("clock error: {e}"))
}

async fn shutdown_signal(shutdown: CancellationToken) {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        if let Ok(mut sigterm) = signal(SignalKind::terminate()) {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {},
                _ = sigterm.recv() => {},
            }
        } else {
            let _ = tokio::signal::ctrl_c().await;
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }

    shutdown.cancel();
}

struct AppState {
    cfg: AppConfig,
    indexer: IndexerApi,
    sender: Mutex<Safe4337UserOpSender>,
    telemetry: RealtorTelemetry,
}
