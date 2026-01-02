use anyhow::Result;
use opentelemetry::global;
// use opentelemetry::trace::TracerProvider as _;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::{MetricExporter, SpanExporter};
use opentelemetry_sdk::{
    Resource,
    metrics::{PeriodicReader, SdkMeterProvider, Temporality},
    trace::{BatchSpanProcessor, SdkTracerProvider},
};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub struct OtelGuard {
    tracer_provider: SdkTracerProvider,
    meter_provider: SdkMeterProvider,
}

impl OtelGuard {
    pub async fn shutdown(self) {
        // shutdown() is blocking; run it off the main async executor thread.
        let _ = tokio::task::spawn_blocking(move || {
            let _ = self.meter_provider.shutdown();
            let _ = self.tracer_provider.shutdown();
        })
        .await;
    }
}

pub fn init(service_name: &str) -> Result<OtelGuard> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Resource = attributes attached to every span/metric
    let resource = Resource::builder()
        .with_service_name(service_name.to_string())
        .build();

    let span_exporter = SpanExporter::builder().with_tonic().build()?;
    let span_processor = BatchSpanProcessor::builder(span_exporter).build();

    let tracer_provider = SdkTracerProvider::builder()
        .with_resource(resource.clone())
        .with_span_processor(span_processor)
        .build();

    let tracer = tracer_provider.tracer(service_name.to_string());

    // ---- Metrics (OTLP) ----
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
