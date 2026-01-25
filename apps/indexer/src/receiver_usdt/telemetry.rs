use crate::shared::rpc_telemetry::RpcTelemetry;
use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ReceiverUsdtTelemetry {
    inner: Arc<Inner>,
}

struct Inner {
    ranges_total: Counter<u64>,
    errors_total: Counter<u64>,

    logs_total: Counter<u64>,
    rows_total: Counter<u64>,

    rpc_calls_total: Counter<u64>,
    rpc_errors_total: Counter<u64>,
    rpc_call_latency_ms: Histogram<u64>,

    rpc_ms: Histogram<u64>,
    db_ms: Histogram<u64>,
    total_ms: Histogram<u64>,
}

impl ReceiverUsdtTelemetry {
    pub fn new() -> Self {
        let meter = global::meter("indexer");

        let ranges_total = meter
            .u64_counter("indexer.receiver_usdt.ranges_total")
            .with_description("Total receiver-USDT ranges processed")
            .build();
        let errors_total = meter
            .u64_counter("indexer.receiver_usdt.errors_total")
            .with_description("Total receiver-USDT range processing errors")
            .build();

        let logs_total = meter
            .u64_counter("indexer.receiver_usdt.logs_total")
            .with_description("Total receiver-USDT Transfer logs observed")
            .build();
        let rows_total = meter
            .u64_counter("indexer.receiver_usdt.rows_total")
            .with_description("Total receiver-USDT rows inserted")
            .build();

        let rpc_calls_total = meter
            .u64_counter("indexer.receiver_usdt.rpc_calls_total")
            .with_description("Total receiver-USDT RPC calls (method-level)")
            .build();
        let rpc_errors_total = meter
            .u64_counter("indexer.receiver_usdt.rpc_errors_total")
            .with_description("Total receiver-USDT RPC errors (method-level)")
            .build();
        let rpc_call_latency_ms = meter
            .u64_histogram("indexer.receiver_usdt.rpc_call_latency_ms")
            .with_description("Receiver-USDT RPC call latency (method-level)")
            .with_unit("ms")
            .build();

        let rpc_ms = meter
            .u64_histogram("indexer.receiver_usdt.rpc_ms")
            .with_description("Receiver-USDT RPC time per range")
            .with_unit("ms")
            .build();
        let db_ms = meter
            .u64_histogram("indexer.receiver_usdt.db_ms")
            .with_description("Receiver-USDT DB time per range")
            .with_unit("ms")
            .build();
        let total_ms = meter
            .u64_histogram("indexer.receiver_usdt.total_ms")
            .with_description("Receiver-USDT total range time")
            .with_unit("ms")
            .build();

        Self {
            inner: Arc::new(Inner {
                ranges_total,
                errors_total,
                logs_total,
                rows_total,
                rpc_calls_total,
                rpc_errors_total,
                rpc_call_latency_ms,
                rpc_ms,
                db_ms,
                total_ms,
            }),
        }
    }

    pub fn observe_range(
        &self,
        mode: &'static str,
        token: &str,
        receiver_count: u64,
        logs: u64,
        rows: u64,
        rpc_ms: u64,
        db_ms: u64,
        total_ms: u64,
    ) {
        let attrs = [
            KeyValue::new("mode", mode),
            KeyValue::new("token", token.to_string()),
            KeyValue::new(
                "receiver_count",
                i64::try_from(receiver_count).unwrap_or_default(),
            ),
        ];
        self.inner.ranges_total.add(1, &attrs);
        self.inner.logs_total.add(logs, &attrs);
        self.inner.rows_total.add(rows, &attrs);
        self.inner.rpc_ms.record(rpc_ms, &attrs);
        self.inner.db_ms.record(db_ms, &attrs);
        self.inner.total_ms.record(total_ms, &attrs);
    }

    pub fn error(&self, mode: &'static str, token: &str, kind: &'static str) {
        let attrs = [
            KeyValue::new("mode", mode),
            KeyValue::new("token", token.to_string()),
            KeyValue::new("kind", kind),
        ];
        self.inner.errors_total.add(1, &attrs);
    }

    pub fn rpc_error(&self, method: &'static str, purpose: &'static str) {
        let attrs = [
            KeyValue::new("method", method),
            KeyValue::new("purpose", purpose),
        ];
        self.inner.rpc_errors_total.add(1, &attrs);
    }
}

impl RpcTelemetry for ReceiverUsdtTelemetry {
    fn rpc_call(&self, method: &'static str, purpose: &'static str, ok: bool, ms: u64) {
        let attrs = [
            KeyValue::new("method", method),
            KeyValue::new("purpose", purpose),
            KeyValue::new("status", if ok { "ok" } else { "err" }),
        ];
        self.inner.rpc_calls_total.add(1, &attrs);
        self.inner.rpc_call_latency_ms.record(ms, &attrs);
    }

    fn rpc_error(&self, method: &'static str, purpose: &'static str) {
        ReceiverUsdtTelemetry::rpc_error(self, method, purpose)
    }
}
