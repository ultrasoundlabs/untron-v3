use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct RealtorTelemetry {
    inner: Arc<Inner>,
}

struct Inner {
    http_requests_total: Counter<u64>,
    http_request_errors_total: Counter<u64>,
    http_request_ms: Histogram<u64>,
    indexer_http_ms: Histogram<u64>,

    leases_created_total: Counter<u64>,
    userops_sent_total: Counter<u64>,

    receiver_salt_zero_balance_fallback_total: Counter<u64>,
    receiver_salt_balance_picker_fallback_total: Counter<u64>,
}

impl RealtorTelemetry {
    pub fn new() -> Self {
        let meter = global::meter("realtor");

        let http_requests_total = meter
            .u64_counter("realtor.http_requests_total")
            .with_description("Total HTTP requests served")
            .build();
        let http_request_errors_total = meter
            .u64_counter("realtor.http_request_errors_total")
            .with_description("Total HTTP requests that returned an error")
            .build();
        let http_request_ms = meter
            .u64_histogram("realtor.http_request_ms")
            .with_description("HTTP handler runtime")
            .with_unit("ms")
            .build();

        let indexer_http_ms = meter
            .u64_histogram("realtor.indexer_http_ms")
            .with_description("Indexer (PostgREST) HTTP request runtime")
            .with_unit("ms")
            .build();

        let leases_created_total = meter
            .u64_counter("realtor.leases_created_total")
            .with_description("Total successful lease creations")
            .build();
        let userops_sent_total = meter
            .u64_counter("realtor.userops_sent_total")
            .with_description("Total user operations submitted to bundler")
            .build();

        let receiver_salt_zero_balance_fallback_total = meter
            .u64_counter("realtor.receiver_salt_zero_balance_fallback_total")
            .with_description("Times auto-selection had to use a zero-balance receiver salt")
            .build();
        let receiver_salt_balance_picker_fallback_total = meter
            .u64_counter("realtor.receiver_salt_balance_picker_fallback_total")
            .with_description("Times balance-based receiver selection failed and fell back")
            .build();

        Self {
            inner: Arc::new(Inner {
                http_requests_total,
                http_request_errors_total,
                http_request_ms,
                indexer_http_ms,
                leases_created_total,
                userops_sent_total,
                receiver_salt_zero_balance_fallback_total,
                receiver_salt_balance_picker_fallback_total,
            }),
        }
    }

    pub fn http_ok(&self, method: &'static str, route: &'static str, status_code: u16, ms: u64) {
        let attrs = [
            KeyValue::new("method", method),
            KeyValue::new("route", route),
            KeyValue::new("status_code", status_code.to_string()),
        ];
        self.inner.http_requests_total.add(1, &attrs);
        self.inner.http_request_ms.record(ms, &attrs);
    }

    pub fn http_err(
        &self,
        method: &'static str,
        route: &'static str,
        kind: &'static str,
        status_code: u16,
        ms: u64,
    ) {
        let attrs = [
            KeyValue::new("method", method),
            KeyValue::new("route", route),
            KeyValue::new("kind", kind),
            KeyValue::new("status_code", status_code.to_string()),
        ];
        self.inner.http_requests_total.add(1, &attrs);
        self.inner.http_request_errors_total.add(1, &attrs);
        self.inner.http_request_ms.record(ms, &attrs);
    }

    pub fn lease_created(&self) {
        self.inner.leases_created_total.add(1, &[]);
    }

    pub fn userop_sent(&self) {
        self.inner.userops_sent_total.add(1, &[]);
    }

    pub fn receiver_salt_zero_balance_fallback(&self, preferred_order: &'static str) {
        let attrs = [KeyValue::new("preferred_order", preferred_order)];
        self.inner
            .receiver_salt_zero_balance_fallback_total
            .add(1, &attrs);
    }

    pub fn receiver_salt_balance_picker_fallback(&self) {
        self.inner
            .receiver_salt_balance_picker_fallback_total
            .add(1, &[]);
    }

    pub fn indexer_http_ms(&self, op: &'static str, ok: bool, ms: u64) {
        let attrs = [
            KeyValue::new("op", op),
            KeyValue::new("status", if ok { "ok" } else { "err" }),
        ];
        self.inner.indexer_http_ms.record(ms, &attrs);
    }
}
