use anyhow::{Context, Result};
use opentelemetry::{KeyValue, global, metrics::ObservableGauge};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use std::time::Duration;
use tokio::time;
use tokio_util::sync::CancellationToken;

use crate::db;

#[derive(Clone)]
pub struct PendingClaimsKpi {
    inner: Arc<Inner>,
}

struct Inner {
    pending_total: Arc<AtomicU64>,
    avg_age_seconds: Arc<AtomicU64>,
    max_age_seconds: Arc<AtomicU64>,

    _g_pending_total: ObservableGauge<u64>,
    _g_avg_age_seconds: ObservableGauge<u64>,
    _g_max_age_seconds: ObservableGauge<u64>,
}

impl PendingClaimsKpi {
    pub fn new() -> Self {
        let meter = global::meter("indexer");
        let attrs: Vec<KeyValue> = vec![];

        let pending_total = Arc::new(AtomicU64::new(0));
        let avg_age_seconds = Arc::new(AtomicU64::new(0));
        let max_age_seconds = Arc::new(AtomicU64::new(0));

        let attrs_clone = attrs.clone();
        let pending_total_clone = pending_total.clone();
        let _g_pending_total = meter
            .u64_observable_gauge("indexer.hub_claims.pending.total")
            .with_description("Count of hub claims currently pending fill (status=created)")
            .with_callback(move |observer| {
                observer.observe(pending_total_clone.load(Ordering::Relaxed), &attrs_clone);
            })
            .build();

        let attrs_clone = attrs.clone();
        let avg_age_seconds_clone = avg_age_seconds.clone();
        let _g_avg_age_seconds = meter
            .u64_observable_gauge("indexer.hub_claims.pending.avg_age_seconds")
            .with_description(
                "Average age (seconds) of hub claims currently pending fill (status=created)",
            )
            .with_unit("s")
            .with_callback(move |observer| {
                observer.observe(avg_age_seconds_clone.load(Ordering::Relaxed), &attrs_clone);
            })
            .build();

        let attrs_clone = attrs.clone();
        let max_age_seconds_clone = max_age_seconds.clone();
        let _g_max_age_seconds = meter
            .u64_observable_gauge("indexer.hub_claims.pending.max_age_seconds")
            .with_description(
                "Max age (seconds) of hub claims currently pending fill (status=created)",
            )
            .with_unit("s")
            .with_callback(move |observer| {
                observer.observe(max_age_seconds_clone.load(Ordering::Relaxed), &attrs_clone);
            })
            .build();

        Self {
            inner: Arc::new(Inner {
                pending_total,
                avg_age_seconds,
                max_age_seconds,
                _g_pending_total,
                _g_avg_age_seconds,
                _g_max_age_seconds,
            }),
        }
    }

    pub fn set(&self, pending_total: u64, avg_age_seconds: u64, max_age_seconds: u64) {
        self.inner
            .pending_total
            .store(pending_total, Ordering::Relaxed);
        self.inner
            .avg_age_seconds
            .store(avg_age_seconds, Ordering::Relaxed);
        self.inner
            .max_age_seconds
            .store(max_age_seconds, Ordering::Relaxed);
    }
}

pub struct RunPendingClaimsKpiParams {
    pub dbh: db::Db,
    pub poll_interval: Duration,
    pub shutdown: CancellationToken,
}

pub async fn run_pending_claims_kpi(params: RunPendingClaimsKpiParams) -> Result<()> {
    let RunPendingClaimsKpiParams {
        dbh,
        poll_interval,
        shutdown,
    } = params;

    let telemetry = PendingClaimsKpi::new();

    let mut ticker = time::interval(poll_interval);
    ticker.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => return Ok(()),
            _ = ticker.tick() => {
                let stats = db::pending_claims_stats(&dbh)
                    .await
                    .context("query pending claims stats")?;
                telemetry.set(stats.pending_total, stats.avg_age_seconds, stats.max_age_seconds);
            }
        }
    }
}
