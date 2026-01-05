use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct RelayerTelemetry {
    inner: Arc<Inner>,
}

struct Inner {
    jobs_total: Counter<u64>,
    job_errors_total: Counter<u64>,
    hub_userops_total: Counter<u64>,
    hub_userop_errors_total: Counter<u64>,
    tron_txs_total: Counter<u64>,
    tron_tx_errors_total: Counter<u64>,

    job_ms: Histogram<u64>,
}

impl RelayerTelemetry {
    pub fn new() -> Self {
        let meter = global::meter("relayer");

        let jobs_total = meter
            .u64_counter("relayer.jobs_total")
            .with_description("Total job runs")
            .build();
        let job_errors_total = meter
            .u64_counter("relayer.job_errors_total")
            .with_description("Total job errors")
            .build();
        let hub_userops_total = meter
            .u64_counter("relayer.hub_userops_total")
            .with_description("Total hub user operations sent")
            .build();
        let hub_userop_errors_total = meter
            .u64_counter("relayer.hub_userop_errors_total")
            .with_description("Total hub user operation submission errors")
            .build();
        let tron_txs_total = meter
            .u64_counter("relayer.tron_txs_total")
            .with_description("Total Tron transactions broadcast")
            .build();
        let tron_tx_errors_total = meter
            .u64_counter("relayer.tron_tx_errors_total")
            .with_description("Total Tron transaction errors")
            .build();

        let job_ms = meter
            .u64_histogram("relayer.job_ms")
            .with_description("Per-job runtime")
            .with_unit("ms")
            .build();

        Self {
            inner: Arc::new(Inner {
                jobs_total,
                job_errors_total,
                hub_userops_total,
                hub_userop_errors_total,
                tron_txs_total,
                tron_tx_errors_total,
                job_ms,
            }),
        }
    }

    pub fn job_ok(&self, name: &'static str, ms: u64) {
        let attrs = [KeyValue::new("job", name)];
        self.inner.jobs_total.add(1, &attrs);
        self.inner.job_ms.record(ms, &attrs);
    }

    pub fn job_err(&self, name: &'static str, ms: u64) {
        let attrs = [KeyValue::new("job", name)];
        self.inner.jobs_total.add(1, &attrs);
        self.inner.job_errors_total.add(1, &attrs);
        self.inner.job_ms.record(ms, &attrs);
    }

    pub fn hub_userop_ok(&self) {
        self.inner.hub_userops_total.add(1, &[]);
    }

    pub fn hub_userop_err(&self) {
        self.inner.hub_userop_errors_total.add(1, &[]);
    }

    pub fn tron_tx_ok(&self) {
        self.inner.tron_txs_total.add(1, &[]);
    }

    pub fn tron_tx_err(&self) {
        self.inner.tron_tx_errors_total.add(1, &[]);
    }
}
