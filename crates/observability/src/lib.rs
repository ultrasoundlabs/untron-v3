use anyhow::Result;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry::{Context, KeyValue};
use opentelemetry_http::HeaderExtractor;
use opentelemetry_otlp::{MetricExporter, SpanExporter};
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
}

impl OtelGuard {
    pub async fn shutdown(self) {
        let _ = tokio::task::spawn_blocking(move || {
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

    let (tracer_provider, meter_provider, tracer, init_err) = if otel_disabled {
        let tracer_provider = SdkTracerProvider::builder()
            .with_resource(resource.clone())
            .with_sampler(Sampler::AlwaysOff)
            .build();
        let tracer = tracer_provider.tracer(cfg.service_name.to_string());

        let meter_provider = SdkMeterProvider::builder().with_resource(resource).build();
        (tracer_provider, meter_provider, tracer, None)
    } else {
        // Best-effort: if OTLP exporter init fails (bad env / no deps), keep the process running with
        // exporters disabled. This is especially helpful in local dev.
        let mut init_err: Option<anyhow::Error> = None;

        let tracer_provider = match (|| -> Result<_> {
            let span_exporter = SpanExporter::builder().with_tonic().build()?;
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
            let metric_exporter = MetricExporter::builder()
                .with_tonic()
                .with_temporality(Temporality::default())
                .build()?;
            let reader = PeriodicReader::builder(metric_exporter).build();
            Ok(SdkMeterProvider::builder()
                .with_resource(resource.clone())
                .with_reader(reader)
                .build())
        })() {
            Ok(mp) => mp,
            Err(e) => {
                init_err.get_or_insert(e);
                SdkMeterProvider::builder().with_resource(resource).build()
            }
        };

        (tracer_provider, meter_provider, tracer, init_err)
    };

    global::set_tracer_provider(tracer_provider.clone());
    global::set_meter_provider(meter_provider.clone());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_target(false);

    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(otel_layer)
        .init();

    if let Some(err) = init_err {
        tracing::warn!(err = %err, "OTLP exporter init failed; OpenTelemetry export disabled");
    }

    Ok(OtelGuard {
        tracer_provider,
        meter_provider,
    })
}
