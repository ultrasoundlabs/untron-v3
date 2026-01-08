mod api;
mod config;
mod indexer;
mod metrics;
mod openapi;
mod util;

use crate::config::AppConfig;
use crate::indexer::IndexerApi;
use crate::metrics::RealtorTelemetry;
use aa::paymaster::PaymasterService;
use aa::{
    PaymasterFinalizationMode, Safe4337UserOpSender, Safe4337UserOpSenderConfig,
    Safe4337UserOpSenderOptions,
};
use alloy::primitives::Address;
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::client::{BuiltInConnectionString, RpcClient};
use alloy::sol_types::SolCall;
use axum::Json;
use axum::extract::MatchedPath;
use axum::http::{Request, Response, header::HeaderName};
use axum::{Router, routing::get};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use untron_v3_bindings::untron_v3::UntronV3;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cfg = config::load_config()?;
    let otel = untron_observability::init(untron_observability::Config {
        service_name: "realtor",
        service_version: env!("CARGO_PKG_VERSION"),
    })?;

    tracing::info!("realtor starting");
    tracing::info!(
        bind = %cfg.api.bind,
        indexer = %cfg.indexer.base_url,
        hub_rpc = %cfg.hub.rpc_url,
        safe = %cfg.hub.safe.unwrap_or(Address::ZERO),
        "config loaded"
    );

    let telemetry = RealtorTelemetry::new();
    let indexer = IndexerApi::new(
        &cfg.indexer.base_url,
        cfg.indexer.timeout,
        telemetry.clone(),
    )?;
    let sender_cfg = Safe4337UserOpSenderConfig {
        rpc_url: cfg.hub.rpc_url.clone(),
        chain_id: cfg.hub.chain_id,
        entrypoint: cfg.hub.entrypoint,
        safe: cfg.hub.safe,
        safe_4337_module: cfg.hub.safe_4337_module,
        safe_deployment: cfg.hub.safe_deployment.clone(),
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
    tracing::info!(safe = %sender.safe_address(), "hub safe ready");
    let mut cfg = cfg;
    cfg.hub.safe = Some(sender.safe_address());
    if cfg.receiver_address_derivation.is_some() {
        match resolve_controller_address(&cfg.hub.rpc_url, cfg.hub.untron_v3).await {
            Ok(addr) => {
                tracing::info!(controller_address = %addr, "resolved controller address");
                cfg.hub.controller_address = Some(addr);
            }
            Err(e) => {
                tracing::warn!(err = %e, "failed to resolve controller address; receiver address fallback will be disabled");
                cfg.hub.controller_address = None;
            }
        }
    }
    let state = AppState {
        cfg,
        indexer,
        sender: Mutex::new(sender),
        telemetry,
    };
    let bind = state.cfg.api.bind;

    let request_id_header = HeaderName::from_static("x-request-id");
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers([request_id_header.clone()]);
    let app = Router::new()
        .route("/realtor", get(api::get_realtor).post(api::post_realtor))
        .route(
            "/payout_config",
            axum::routing::post(api::post_payout_config),
        )
        .route("/leases/{lease_id}", get(api::leases::get_lease))
        .route("/openapi.json", get(openapi_json))
        .route(
            "/healthz",
            get(|| async { Json(serde_json::json!({ "ok": true })) }),
        )
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
                    let span = tracing::info_span!(
                        "http.request",
                        request_id = %request_id,
                        method = %req.method(),
                        path = %matched
                    );
                    untron_observability::set_span_parent_from_headers(&span, req.headers());
                    span
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
        .layer(SetRequestIdLayer::new(request_id_header, MakeRequestUuid))
        .layer(cors);

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

async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(openapi::RealtorApiDoc::openapi())
}

async fn resolve_controller_address(
    hub_rpc_url: &str,
    untron_v3: Address,
) -> anyhow::Result<Address> {
    let transport = BuiltInConnectionString::connect(hub_rpc_url)
        .await
        .map_err(|e| anyhow::anyhow!("connect hub rpc: {e}"))?;
    let client = RpcClient::builder().transport(transport, false);
    let provider: DynProvider = DynProvider::new(ProviderBuilder::default().connect_client(client));

    let call = UntronV3::CONTROLLER_ADDRESSCall {};
    let data = call.abi_encode();
    let tx: alloy::rpc::types::eth::transaction::TransactionRequest =
        alloy::rpc::types::eth::transaction::TransactionRequest {
            to: Some(untron_v3.into()),
            input: alloy::rpc::types::eth::transaction::TransactionInput::new(data.into()),
            ..Default::default()
        };
    let out = provider
        .call(tx)
        .await
        .map_err(|e| anyhow::anyhow!("eth_call CONTROLLER_ADDRESS: {e}"))?;
    let decoded: Address =
        <UntronV3::CONTROLLER_ADDRESSCall as SolCall>::abi_decode_returns(out.as_ref())
            .map_err(|e| anyhow::anyhow!("decode CONTROLLER_ADDRESS return: {e}"))?;
    Ok(decoded)
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
