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
pub struct SubjectivePreEntitleKpi {
    inner: Arc<Inner>,
}

struct Inner {
    stuck_total: Arc<AtomicU64>,
    avg_age_seconds: Arc<AtomicU64>,
    max_age_seconds: Arc<AtomicU64>,

    _g_stuck_total: ObservableGauge<u64>,
    _g_avg_age_seconds: ObservableGauge<u64>,
    _g_max_age_seconds: ObservableGauge<u64>,
}

impl SubjectivePreEntitleKpi {
    pub fn new(chain_id: u64, token: &'static str) -> Self {
        let meter = global::meter("indexer");
        let attrs = vec![
            KeyValue::new("chain_id", i64::try_from(chain_id).unwrap_or_default()),
            KeyValue::new("token", token),
        ];

        let stuck_total = Arc::new(AtomicU64::new(0));
        let avg_age_seconds = Arc::new(AtomicU64::new(0));
        let max_age_seconds = Arc::new(AtomicU64::new(0));

        let attrs_clone = attrs.clone();
        let stuck_total_clone = stuck_total.clone();
        let _g_stuck_total = meter
            .u64_observable_gauge("indexer.receiver_usdt.subjective_pre_entitle.stuck_total")
            .with_description(
                "Count of receiver-USDT deposits currently in recommended_action=subjective_pre_entitle",
            )
            .with_callback(move |observer| {
                observer.observe(stuck_total_clone.load(Ordering::Relaxed), &attrs_clone);
            })
            .build();

        let attrs_clone = attrs.clone();
        let avg_age_seconds_clone = avg_age_seconds.clone();
        let _g_avg_age_seconds = meter
            .u64_observable_gauge("indexer.receiver_usdt.subjective_pre_entitle.avg_age_seconds")
            .with_description(
                "Average age (seconds) of deposits currently in recommended_action=subjective_pre_entitle",
            )
            .with_unit("s")
            .with_callback(move |observer| {
                observer.observe(avg_age_seconds_clone.load(Ordering::Relaxed), &attrs_clone);
            })
            .build();

        let attrs_clone = attrs.clone();
        let max_age_seconds_clone = max_age_seconds.clone();
        let _g_max_age_seconds = meter
            .u64_observable_gauge("indexer.receiver_usdt.subjective_pre_entitle.max_age_seconds")
            .with_description(
                "Max age (seconds) of deposits currently in recommended_action=subjective_pre_entitle",
            )
            .with_unit("s")
            .with_callback(move |observer| {
                observer.observe(max_age_seconds_clone.load(Ordering::Relaxed), &attrs_clone);
            })
            .build();

        Self {
            inner: Arc::new(Inner {
                stuck_total,
                avg_age_seconds,
                max_age_seconds,
                _g_stuck_total,
                _g_avg_age_seconds,
                _g_max_age_seconds,
            }),
        }
    }

    pub fn set(&self, stuck_total: u64, avg_age_seconds: u64, max_age_seconds: u64) {
        self.inner.stuck_total.store(stuck_total, Ordering::Relaxed);
        self.inner
            .avg_age_seconds
            .store(avg_age_seconds, Ordering::Relaxed);
        self.inner
            .max_age_seconds
            .store(max_age_seconds, Ordering::Relaxed);
    }
}

pub struct RunSubjectivePreEntitleKpiParams {
    pub dbh: db::Db,
    pub chain_id: u64,
    pub token: &'static str,
    pub poll_interval: Duration,
    pub shutdown: CancellationToken,
}

pub async fn run_subjective_pre_entitle_kpi(
    params: RunSubjectivePreEntitleKpiParams,
) -> Result<()> {
    let RunSubjectivePreEntitleKpiParams {
        dbh,
        chain_id,
        token,
        poll_interval,
        shutdown,
    } = params;

    let telemetry = SubjectivePreEntitleKpi::new(chain_id, token);

    let mut ticker = time::interval(poll_interval);
    ticker.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => return Ok(()),
            _ = ticker.tick() => {
                let stats = crate::db::subjective_pre_entitle_stats(&dbh, chain_id, token)
                    .await
                    .with_context(|| format!("query subjective_pre_entitle stats for chain_id={chain_id} token={token}"))?;
                telemetry.set(stats.stuck_total, stats.avg_age_seconds, stats.max_age_seconds);
            }
        }
    }
}
