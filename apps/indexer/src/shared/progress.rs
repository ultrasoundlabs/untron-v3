use std::time::{Duration, Instant};
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Backfill,
    Tail,
}

fn stage(next_block: u64, safe_head: u64) -> Stage {
    if next_block <= safe_head {
        Stage::Backfill
    } else {
        Stage::Tail
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RangeMetrics {
    pub from_block: u64,
    pub to_block: u64,
    pub logs: u64,
    pub rows: u64,

    pub rpc_ms: u64,
    pub ts_ms: u64,
    pub decode_ms: u64,
    pub db_ms: u64,
    pub total_ms: u64,
}

#[derive(Default)]
struct IntervalTotals {
    ranges: u64,
    blocks: u64,
    logs: u64,
    rows: u64,
    empty_ranges: u64,

    rpc_ms: u64,
    ts_ms: u64,
    decode_ms: u64,
    db_ms: u64,
    total_ms: u64,
}

impl IntervalTotals {
    fn observe(&mut self, m: &RangeMetrics) {
        self.ranges += 1;
        self.blocks += m.to_block.saturating_sub(m.from_block).saturating_add(1);
        self.logs += m.logs;
        self.rows += m.rows;
        if m.logs == 0 && m.rows == 0 {
            self.empty_ranges += 1;
        }

        self.rpc_ms += m.rpc_ms;
        self.ts_ms += m.ts_ms;
        self.decode_ms += m.decode_ms;
        self.db_ms += m.db_ms;
        self.total_ms += m.total_ms;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Extra {
    EventChain {
        chunk_blocks: u64,
    },
    ReceiverUsdt {
        receiver_count: usize,
        batch_size: usize,
    },
}

pub struct ProgressReporter {
    label: &'static str,
    interval: Duration,
    kind: Extra,

    started_at: Instant,
    last_report_at: Instant,
    last_stage: Option<Stage>,
    caught_up_once: bool,

    // Event-chain-only counters (kept here to avoid two reporter impls).
    reorgs_detected: u64,
    blocks_invalidated: u64,
    transient_retries: u64,
    chunk_shrinks: u64,
    pinned_repairs_attempted: u64,
    pinned_repairs_succeeded: u64,

    interval_totals: IntervalTotals,
    lifetime_totals: IntervalTotals,
}

impl ProgressReporter {
    pub fn new_event_chain(label: &'static str, interval: Duration, chunk_blocks: u64) -> Self {
        Self::new(label, interval, Extra::EventChain { chunk_blocks })
    }

    pub fn new_receiver_usdt(
        label: &'static str,
        interval: Duration,
        receiver_count: usize,
        batch_size: usize,
    ) -> Self {
        Self::new(
            label,
            interval,
            Extra::ReceiverUsdt {
                receiver_count,
                batch_size,
            },
        )
    }

    fn new(label: &'static str, interval: Duration, kind: Extra) -> Self {
        let now = Instant::now();
        Self {
            label,
            interval: interval.max(Duration::from_secs(1)),
            kind,
            started_at: now,
            last_report_at: now,
            last_stage: None,
            caught_up_once: false,
            reorgs_detected: 0,
            blocks_invalidated: 0,
            transient_retries: 0,
            chunk_shrinks: 0,
            pinned_repairs_attempted: 0,
            pinned_repairs_succeeded: 0,
            interval_totals: IntervalTotals::default(),
            lifetime_totals: IntervalTotals::default(),
        }
    }

    pub fn update_receiver_usdt_params(&mut self, receiver_count: usize, batch_size: usize) {
        if let Extra::ReceiverUsdt {
            receiver_count: rc,
            batch_size: bs,
        } = &mut self.kind
        {
            *rc = receiver_count;
            *bs = batch_size;
        }
    }

    pub fn update_event_chain_chunk_blocks(&mut self, chunk_blocks: u64) {
        if let Extra::EventChain { chunk_blocks: cb } = &mut self.kind {
            *cb = chunk_blocks;
        }
    }

    pub fn observe_range(&mut self, metrics: RangeMetrics) {
        self.interval_totals.observe(&metrics);
        self.lifetime_totals.observe(&metrics);
    }

    pub fn on_reorg(&mut self, invalidated_blocks: u64) {
        self.reorgs_detected += 1;
        self.blocks_invalidated += invalidated_blocks;
    }

    pub fn on_transient_retry(&mut self) {
        self.transient_retries += 1;
    }

    pub fn on_chunk_shrink(&mut self) {
        self.chunk_shrinks += 1;
    }

    pub fn on_pinned_repair_attempt(&mut self) {
        self.pinned_repairs_attempted += 1;
    }

    pub fn on_pinned_repair_success(&mut self) {
        self.pinned_repairs_succeeded += 1;
    }

    pub fn maybe_report(&mut self, head: u64, safe_head: u64, next_block: u64) {
        let now = Instant::now();
        let current_stage = stage(next_block, safe_head);

        let should_report = now.duration_since(self.last_report_at) >= self.interval
            || self.last_stage.map(|s| s != current_stage).unwrap_or(true)
            || (!self.caught_up_once && current_stage == Stage::Tail);

        if !should_report {
            return;
        }

        if current_stage == Stage::Tail {
            self.caught_up_once = true;
        }

        let interval_secs = now
            .duration_since(self.last_report_at)
            .as_secs_f64()
            .max(0.001);
        let since_start_secs = now.duration_since(self.started_at).as_secs_f64().max(0.001);

        let backlog_blocks = if next_block > safe_head {
            0
        } else {
            safe_head - next_block + 1
        };
        let block_rate = (self.interval_totals.blocks as f64) / interval_secs;
        let log_rate = (self.interval_totals.logs as f64) / interval_secs;
        let row_rate = (self.interval_totals.rows as f64) / interval_secs;

        let avg = |sum_ms: u64| -> u64 {
            if self.interval_totals.ranges == 0 {
                0
            } else {
                sum_ms / self.interval_totals.ranges
            }
        };

        match self.kind {
            Extra::EventChain { chunk_blocks } => {
                info!(
                    indexer = self.label,
                    stage = match current_stage {
                        Stage::Backfill => "backfill",
                        Stage::Tail => "tail",
                    },
                    head,
                    safe_head,
                    next_block,
                    backlog_blocks,
                    chunk_blocks,
                    interval_secs,
                    since_start_secs,
                    ranges = self.interval_totals.ranges,
                    blocks = self.interval_totals.blocks,
                    blocks_per_sec = block_rate,
                    logs = self.interval_totals.logs,
                    logs_per_sec = log_rate,
                    rows = self.interval_totals.rows,
                    rows_per_sec = row_rate,
                    empty_ranges = self.interval_totals.empty_ranges,
                    avg_rpc_ms = avg(self.interval_totals.rpc_ms),
                    avg_timestamp_ms = avg(self.interval_totals.ts_ms),
                    avg_decode_ms = avg(self.interval_totals.decode_ms),
                    avg_db_ms = avg(self.interval_totals.db_ms),
                    avg_total_ms = avg(self.interval_totals.total_ms),
                    reorgs_detected = self.reorgs_detected,
                    blocks_invalidated = self.blocks_invalidated,
                    transient_retries = self.transient_retries,
                    chunk_shrinks = self.chunk_shrinks,
                    pinned_repairs_attempted = self.pinned_repairs_attempted,
                    pinned_repairs_succeeded = self.pinned_repairs_succeeded,
                    lifetime_ranges = self.lifetime_totals.ranges,
                    lifetime_blocks = self.lifetime_totals.blocks,
                    lifetime_logs = self.lifetime_totals.logs,
                    lifetime_rows = self.lifetime_totals.rows,
                    "stream progress"
                );
            }
            Extra::ReceiverUsdt {
                receiver_count,
                batch_size,
            } => {
                info!(
                    indexer = self.label,
                    stage = match current_stage {
                        Stage::Backfill => "backfill",
                        Stage::Tail => "tail",
                    },
                    head,
                    safe_head,
                    next_block,
                    backlog_blocks,
                    receiver_count,
                    batch_size,
                    interval_secs,
                    since_start_secs,
                    ranges = self.interval_totals.ranges,
                    blocks = self.interval_totals.blocks,
                    blocks_per_sec = block_rate,
                    logs = self.interval_totals.logs,
                    logs_per_sec = log_rate,
                    rows = self.interval_totals.rows,
                    rows_per_sec = row_rate,
                    empty_ranges = self.interval_totals.empty_ranges,
                    avg_rpc_ms = avg(self.interval_totals.rpc_ms),
                    avg_timestamp_ms = avg(self.interval_totals.ts_ms),
                    avg_decode_ms = avg(self.interval_totals.decode_ms),
                    avg_db_ms = avg(self.interval_totals.db_ms),
                    avg_total_ms = avg(self.interval_totals.total_ms),
                    lifetime_ranges = self.lifetime_totals.ranges,
                    lifetime_blocks = self.lifetime_totals.blocks,
                    lifetime_logs = self.lifetime_totals.logs,
                    lifetime_rows = self.lifetime_totals.rows,
                    "stream progress"
                );
            }
        }

        self.last_report_at = now;
        self.last_stage = Some(current_stage);
        self.interval_totals = IntervalTotals::default();
    }
}
