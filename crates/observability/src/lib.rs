use anyhow::Result;
use opentelemetry::global;
use opentelemetry::trace::{TraceContextExt, TracerProvider};
use opentelemetry::{Context, KeyValue};
use opentelemetry_http::HeaderExtractor;
use opentelemetry_otlp::{MetricExporter, SpanExporter, WithHttpConfig};
use opentelemetry_sdk::{
    Resource,
    metrics::{PeriodicReader, SdkMeterProvider, Temporality},
    propagation::TraceContextPropagator,
    trace::{BatchSpanProcessor, Sampler, SdkTracerProvider},
};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub struct OtelGuard {
    tracer_provider: SdkTracerProvider,
    meter_provider: SdkMeterProvider,
    logger_provider: Option<opentelemetry_sdk::logs::SdkLoggerProvider>,

    /// When enabled via env, a background thread emits a small span periodically to prove the OTLP pipeline.
    ///
    /// This is intentionally best-effort and safe to leave enabled only temporarily.
    debug_heartbeat_stop: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
}

impl OtelGuard {
    pub async fn shutdown(self) {
        if let Some(stop) = &self.debug_heartbeat_stop {
            stop.store(true, std::sync::atomic::Ordering::Relaxed);
        }

        let _ = tokio::task::spawn_blocking(move || {
            if let Some(lp) = self.logger_provider {
                let _ = lp.shutdown();
            }
            let _ = self.meter_provider.shutdown();
            let _ = self.tracer_provider.shutdown();
        })
        .await;
    }
}

#[derive(Clone, Copy)]
pub struct Config<'a> {
    pub service_name: &'a str,
    pub service_version: &'a str,
}

pub fn extract_trace_context(headers: &http::HeaderMap) -> Context {
    global::get_text_map_propagator(|p| p.extract(&HeaderExtractor(headers)))
}

pub fn set_span_parent_from_headers(span: &tracing::Span, headers: &http::HeaderMap) {
    let cx = extract_trace_context(headers);
    let _ = span.set_parent(cx);
}

pub fn init(cfg: Config<'_>) -> Result<OtelGuard> {
    // Optional: emit a small heartbeat span periodically to validate end-to-end tracing.
    // Enable temporarily with OTEL_DEBUG_HEARTBEAT=1.
    let debug_heartbeat_enabled = std::env::var("OTEL_DEBUG_HEARTBEAT")
        .ok()
        .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"));

    let debug_heartbeat_interval_secs = std::env::var("OTEL_DEBUG_HEARTBEAT_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(10)
        .clamp(1, 3600);

    // Needed for W3C `traceparent` header propagation.
    global::set_text_map_propagator(TraceContextPropagator::new());

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let otel_disabled = std::env::var("OTEL_DISABLED")
        .ok()
        .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"));

    let sample_ratio = std::env::var("OTEL_TRACES_SAMPLE_RATIO")
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.01)
        .clamp(0.0, 1.0);

    let sampler = if sample_ratio <= 0.0 {
        Sampler::AlwaysOff
    } else if sample_ratio >= 1.0 {
        Sampler::AlwaysOn
    } else {
        Sampler::TraceIdRatioBased(sample_ratio)
    };

    let resource = Resource::builder()
        .with_service_name(cfg.service_name.to_string())
        .with_attributes(vec![
            KeyValue::new("service.version", cfg.service_version.to_string()),
            KeyValue::new(
                "service.instance.id",
                std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string()),
            ),
            KeyValue::new(
                "deployment.environment",
                std::env::var("DEPLOYMENT_ENV").unwrap_or_else(|_| "unknown".to_string()),
            ),
        ])
        .build();

    let logs_enabled = std::env::var("OTEL_LOGS_ENABLED")
        .ok()
        .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"));

    let (tracer_provider, meter_provider, logger_provider, tracer, init_err) = if otel_disabled {
        let tracer_provider = SdkTracerProvider::builder()
            .with_resource(resource.clone())
            .with_sampler(Sampler::AlwaysOff)
            .build();
        let tracer = tracer_provider.tracer(cfg.service_name.to_string());

        let meter_provider = SdkMeterProvider::builder().with_resource(resource.clone()).build();
        (tracer_provider, meter_provider, None, tracer, None)
    } else {
        // Best-effort: if OTLP exporter init fails (bad env / no deps), keep the process running with
        // exporters disabled. This is especially helpful in local dev.
        let mut init_err: Option<anyhow::Error> = None;

        let tracer_provider = match (|| -> Result<_> {
            let protocol = std::env::var("OTEL_EXPORTER_OTLP_PROTOCOL").unwrap_or_else(|_| "grpc".to_string());

            let span_exporter = if protocol.starts_with("http") {
                // The OTLP/HTTP exporter requires an explicit HTTP client.
                // Use a blocking client so exports don't depend on a Tokio reactor.
                let client = reqwest::blocking::Client::new();
                SpanExporter::builder()
                    .with_http()
                    .with_http_client(client)
                    .build()?
            } else {
                SpanExporter::builder().with_tonic().build()?
            };

            let span_processor = BatchSpanProcessor::builder(span_exporter).build();
            Ok(SdkTracerProvider::builder()
                .with_resource(resource.clone())
                .with_sampler(sampler)
                .with_span_processor(span_processor)
                .build())
        })() {
            Ok(tp) => tp,
            Err(e) => {
                init_err = Some(e);
                SdkTracerProvider::builder()
                    .with_resource(resource.clone())
                    .with_sampler(Sampler::AlwaysOff)
                    .build()
            }
        };
        let tracer = tracer_provider.tracer(cfg.service_name.to_string());

        let meter_provider = match (|| -> Result<_> {
            let protocol = std::env::var("OTEL_EXPORTER_OTLP_PROTOCOL").unwrap_or_else(|_| "grpc".to_string());

            let metric_exporter = if protocol.starts_with("http") {
                // The OTLP/HTTP exporter requires an explicit HTTP client.
                // Use a blocking client so exports don't depend on a Tokio reactor.
                let client = reqwest::blocking::Client::new();
                MetricExporter::builder()
                    .with_http()
                    .with_http_client(client)
                    .with_temporality(Temporality::default())
                    .build()?
            } else {
                MetricExporter::builder()
                    .with_tonic()
                    .with_temporality(Temporality::default())
                    .build()?
            };

            let reader = PeriodicReader::builder(metric_exporter).build();
            Ok(SdkMeterProvider::builder()
                .with_resource(resource.clone())
                .with_reader(reader)
                .build())
        })() {
            Ok(mp) => mp,
            Err(e) => {
                init_err.get_or_insert(e);
                SdkMeterProvider::builder().with_resource(resource.clone()).build()
            }
        };

        // Optional: OTLP logs export (tracing events -> OTel logs).
        // Enable with OTEL_LOGS_ENABLED=1.
        let logger_provider = if logs_enabled {
            match (|| -> Result<_> {
                let protocol = std::env::var("OTEL_EXPORTER_OTLP_PROTOCOL")
                    .unwrap_or_else(|_| "grpc".to_string());

                let log_exporter = if protocol.starts_with("http") {
                    let client = reqwest::blocking::Client::new();
                    opentelemetry_otlp::LogExporter::builder()
                        .with_http()
                        .with_http_client(client)
                        .build()?
                } else {
                    opentelemetry_otlp::LogExporter::builder().with_tonic().build()?
                };

                let processor = opentelemetry_sdk::logs::BatchLogProcessor::builder(log_exporter).build();

                Ok(opentelemetry_sdk::logs::SdkLoggerProvider::builder()
                    .with_resource(resource.clone())
                    .with_log_processor(processor)
                    .build())
            })() {
                Ok(lp) => Some(lp),
                Err(e) => {
                    init_err.get_or_insert(e);
                    None
                }
            }
        } else {
            None
        };

        (tracer_provider, meter_provider, logger_provider, tracer, init_err)
    };

    global::set_tracer_provider(tracer_provider.clone());
    global::set_meter_provider(meter_provider.clone());

    // logger_provider is used by the tracing->OpenTelemetry logs bridge.
    // We intentionally do not register a global logger provider here.

    // Log formatting: include trace/span ids when we're inside an active tracing span.
    // This makes Tempo -> Loki trace-to-logs correlation possible once logs are ingested.

    let fmt_layer = tracing_subscriber::fmt::layer().compact().with_target(false);

    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let otel_logs_layer = logger_provider
        .as_ref()
        .map(|lp| opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(lp));

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(otel_layer);

    if let Some(layer) = otel_logs_layer {
        registry.with(layer).init();
    } else {
        registry.init();
    }

    if let Some(err) = init_err {
        tracing::warn!(err = %err, "OTLP exporter init failed; OpenTelemetry export disabled");
    }

    let debug_heartbeat_stop = if debug_heartbeat_enabled {
        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let stop2 = stop.clone();

        // Background thread (no tokio dependency) so it works even in binaries without an async runtime.
        std::thread::Builder::new()
            .name("otel-debug-heartbeat".to_string())
            .spawn(move || {
                let interval = std::time::Duration::from_secs(debug_heartbeat_interval_secs);
                loop {
                    if stop2.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }

                    // Emit a minimal span; exporters will ship it if the pipeline works.
                    let span = tracing::info_span!(
                        "otel.debug.heartbeat",
                        otel.kind = "internal",
                        interval_secs = debug_heartbeat_interval_secs
                    );
                    let _g = span.enter();
                    tracing::info!("otel debug heartbeat");

                    std::thread::sleep(interval);
                }
            })?;

        Some(stop)
    } else {
        None
    };

    Ok(OtelGuard {
        tracer_provider,
        meter_provider,
        logger_provider,
        debug_heartbeat_stop,
    })
}
