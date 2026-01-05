use anyhow::Result;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::{MetricExporter, SpanExporter};
use opentelemetry_sdk::{
    Resource,
    metrics::{PeriodicReader, SdkMeterProvider, Temporality},
    trace::{BatchSpanProcessor, Sampler, SdkTracerProvider},
};
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

pub fn init(service_name: &str) -> Result<OtelGuard> {
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
        .with_service_name(service_name.to_string())
        .build();

    if otel_disabled {
        let tracer_provider = SdkTracerProvider::builder()
            .with_resource(resource.clone())
            .with_sampler(Sampler::AlwaysOff)
            .build();
        let tracer = tracer_provider.tracer(service_name.to_string());

        let meter_provider = SdkMeterProvider::builder().with_resource(resource).build();

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

        return Ok(OtelGuard {
            tracer_provider,
            meter_provider,
        });
    }

    let span_exporter = SpanExporter::builder().with_tonic().build()?;
    let span_processor = BatchSpanProcessor::builder(span_exporter).build();
    let tracer_provider = SdkTracerProvider::builder()
        .with_resource(resource.clone())
        .with_sampler(sampler)
        .with_span_processor(span_processor)
        .build();
    let tracer = tracer_provider.tracer(service_name.to_string());

    let metric_exporter = MetricExporter::builder()
        .with_tonic()
        .with_temporality(Temporality::default())
        .build()?;
    let reader = PeriodicReader::builder(metric_exporter).build();
    let meter_provider = SdkMeterProvider::builder()
        .with_resource(resource)
        .with_reader(reader)
        .build();

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

    Ok(OtelGuard {
        tracer_provider,
        meter_provider,
    })
}
