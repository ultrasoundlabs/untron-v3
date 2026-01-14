use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct PoolTelemetry {
    inner: Arc<Inner>,
}

struct Inner {
    ticks_total: Counter<u64>,
    tick_errors_total: Counter<u64>,

    oneclick_quotes_total: Counter<u64>,
    oneclick_quote_errors_total: Counter<u64>,

    tron_transfers_total: Counter<u64>,
    tron_transfer_errors_total: Counter<u64>,

    oneclick_deposit_submits_total: Counter<u64>,
    oneclick_deposit_submit_errors_total: Counter<u64>,

    energy_rental_attempts_total: Counter<u64>,
    energy_rental_errors_total: Counter<u64>,

    tick_ms: Histogram<u64>,
    tron_balance_ms: Histogram<u64>,
    oneclick_quote_ms: Histogram<u64>,
    tron_transfer_ms: Histogram<u64>,
    oneclick_submit_ms: Histogram<u64>,
    energy_rental_ms: Histogram<u64>,

    oneclick_status_polls_total: Counter<u64>,
    oneclick_status_poll_errors_total: Counter<u64>,
    oneclick_status_terminals_total: Counter<u64>,
    oneclick_status_poll_ms: Histogram<u64>,

    oneclick_backoffs_total: Counter<u64>,
    oneclick_backoff_secs: Histogram<u64>,
}

impl PoolTelemetry {
    pub fn new() -> Self {
        let meter = global::meter("pool");

        let ticks_total = meter
            .u64_counter("pool.ticks_total")
            .with_description("Total pool ticks")
            .build();
        let tick_errors_total = meter
            .u64_counter("pool.tick_errors_total")
            .with_description("Total pool tick errors")
            .build();

        let oneclick_quotes_total = meter
            .u64_counter("pool.oneclick_quotes_total")
            .with_description("Total 1Click quote requests")
            .build();
        let oneclick_quote_errors_total = meter
            .u64_counter("pool.oneclick_quote_errors_total")
            .with_description("Total 1Click quote request errors")
            .build();

        let tron_transfers_total = meter
            .u64_counter("pool.tron_transfers_total")
            .with_description("Total Tron TRC-20 transfers broadcast")
            .build();
        let tron_transfer_errors_total = meter
            .u64_counter("pool.tron_transfer_errors_total")
            .with_description("Total Tron TRC-20 transfer errors")
            .build();

        let oneclick_deposit_submits_total = meter
            .u64_counter("pool.oneclick_deposit_submits_total")
            .with_description("Total 1Click deposit submissions")
            .build();
        let oneclick_deposit_submit_errors_total = meter
            .u64_counter("pool.oneclick_deposit_submit_errors_total")
            .with_description("Total 1Click deposit submission errors")
            .build();

        let energy_rental_attempts_total = meter
            .u64_counter("pool.energy_rental_attempts_total")
            .with_description("Total energy rental attempts")
            .build();
        let energy_rental_errors_total = meter
            .u64_counter("pool.energy_rental_errors_total")
            .with_description("Total energy rental errors")
            .build();

        let tick_ms = meter
            .u64_histogram("pool.tick_ms")
            .with_description("Tick runtime")
            .with_unit("ms")
            .build();
        let tron_balance_ms = meter
            .u64_histogram("pool.tron_balance_ms")
            .with_description("Tron TRC-20 balance read runtime")
            .with_unit("ms")
            .build();
        let oneclick_quote_ms = meter
            .u64_histogram("pool.oneclick_quote_ms")
            .with_description("1Click quote request runtime")
            .with_unit("ms")
            .build();
        let tron_transfer_ms = meter
            .u64_histogram("pool.tron_transfer_ms")
            .with_description("Tron TRC-20 transfer broadcast runtime")
            .with_unit("ms")
            .build();
        let oneclick_submit_ms = meter
            .u64_histogram("pool.oneclick_submit_ms")
            .with_description("1Click deposit submit runtime")
            .with_unit("ms")
            .build();
        let energy_rental_ms = meter
            .u64_histogram("pool.energy_rental_ms")
            .with_description("Energy rental provider call runtime")
            .with_unit("ms")
            .build();

        let oneclick_status_polls_total = meter
            .u64_counter("pool.oneclick_status_polls_total")
            .with_description("Total 1Click status polls")
            .build();
        let oneclick_status_poll_errors_total = meter
            .u64_counter("pool.oneclick_status_poll_errors_total")
            .with_description("Total 1Click status poll errors")
            .build();
        let oneclick_status_terminals_total = meter
            .u64_counter("pool.oneclick_status_terminals_total")
            .with_description("Total 1Click terminal statuses observed")
            .build();
        let oneclick_status_poll_ms = meter
            .u64_histogram("pool.oneclick_status_poll_ms")
            .with_description("1Click status poll runtime")
            .with_unit("ms")
            .build();

        let oneclick_backoffs_total = meter
            .u64_counter("pool.oneclick_backoffs_total")
            .with_description("Total 1Click backoff activations")
            .build();
        let oneclick_backoff_secs = meter
            .u64_histogram("pool.oneclick_backoff_secs")
            .with_description("Backoff duration applied after 1Click failure")
            .with_unit("s")
            .build();

        Self {
            inner: Arc::new(Inner {
                ticks_total,
                tick_errors_total,
                oneclick_quotes_total,
                oneclick_quote_errors_total,
                tron_transfers_total,
                tron_transfer_errors_total,
                oneclick_deposit_submits_total,
                oneclick_deposit_submit_errors_total,
                energy_rental_attempts_total,
                energy_rental_errors_total,
                tick_ms,
                tron_balance_ms,
                oneclick_quote_ms,
                tron_transfer_ms,
                oneclick_submit_ms,
                energy_rental_ms,
                oneclick_status_polls_total,
                oneclick_status_poll_errors_total,
                oneclick_status_terminals_total,
                oneclick_status_poll_ms,
                oneclick_backoffs_total,
                oneclick_backoff_secs,
            }),
        }
    }

    pub fn tick_ok(&self, ms: u64) {
        self.inner.ticks_total.add(1, &[]);
        self.inner
            .tick_ms
            .record(ms, &[KeyValue::new("status", "ok")]);
    }

    pub fn tick_err(&self, ms: u64) {
        self.inner.ticks_total.add(1, &[]);
        self.inner.tick_errors_total.add(1, &[]);
        self.inner
            .tick_ms
            .record(ms, &[KeyValue::new("status", "err")]);
    }

    pub fn tron_balance_ms(&self, ok: bool, ms: u64) {
        self.inner.tron_balance_ms.record(
            ms,
            &[KeyValue::new("status", if ok { "ok" } else { "err" })],
        );
    }

    pub fn oneclick_quote_ms(&self, ok: bool, ms: u64) {
        self.inner.oneclick_quotes_total.add(1, &[]);
        if !ok {
            self.inner.oneclick_quote_errors_total.add(1, &[]);
        }
        self.inner.oneclick_quote_ms.record(
            ms,
            &[KeyValue::new("status", if ok { "ok" } else { "err" })],
        );
    }

    pub fn tron_transfer_ms(&self, ok: bool, ms: u64) {
        self.inner.tron_transfers_total.add(1, &[]);
        if !ok {
            self.inner.tron_transfer_errors_total.add(1, &[]);
        }
        self.inner.tron_transfer_ms.record(
            ms,
            &[KeyValue::new("status", if ok { "ok" } else { "err" })],
        );
    }

    pub fn oneclick_submit_ms(&self, ok: bool, ms: u64) {
        self.inner.oneclick_deposit_submits_total.add(1, &[]);
        if !ok {
            self.inner.oneclick_deposit_submit_errors_total.add(1, &[]);
        }
        self.inner.oneclick_submit_ms.record(
            ms,
            &[KeyValue::new("status", if ok { "ok" } else { "err" })],
        );
    }

    pub fn energy_rental_ms(&self, provider: &str, ok: bool, ms: u64) {
        self.inner
            .energy_rental_attempts_total
            .add(1, &[KeyValue::new("provider", provider.to_string())]);
        if !ok {
            self.inner
                .energy_rental_errors_total
                .add(1, &[KeyValue::new("provider", provider.to_string())]);
        }
        self.inner.energy_rental_ms.record(
            ms,
            &[
                KeyValue::new("provider", provider.to_string()),
                KeyValue::new("status", if ok { "ok" } else { "err" }),
            ],
        );
    }

    pub fn oneclick_status_poll_ms(&self, ok: bool, status: &'static str, ms: u64) {
        self.inner.oneclick_status_polls_total.add(
            1,
            &[
                KeyValue::new("status", status),
                KeyValue::new("ok", ok.to_string()),
            ],
        );
        if !ok {
            self.inner.oneclick_status_poll_errors_total.add(1, &[]);
        }
        self.inner.oneclick_status_poll_ms.record(
            ms,
            &[
                KeyValue::new("status", status),
                KeyValue::new("ok", ok.to_string()),
            ],
        );
    }

    pub fn oneclick_status_terminal(&self, status: &'static str) {
        self.inner
            .oneclick_status_terminals_total
            .add(1, &[KeyValue::new("status", status)]);
    }

    pub fn oneclick_backoff(&self, reason: &'static str, secs: u64) {
        self.inner
            .oneclick_backoffs_total
            .add(1, &[KeyValue::new("reason", reason)]);
        self.inner
            .oneclick_backoff_secs
            .record(secs, &[KeyValue::new("reason", reason)]);
    }
}
