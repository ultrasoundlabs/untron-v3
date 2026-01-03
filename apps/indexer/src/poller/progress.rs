use crate::config::Stream;
use std::time::{Duration, Instant};
use tracing::info;

use super::range::RangeMetrics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Backfill,
    Tail,
}

fn stage(from_block: u64, safe_head: u64) -> Stage {
    if from_block <= safe_head {
        Stage::Backfill
    } else {
        Stage::Tail
    }
}

#[derive(Default)]
struct IntervalTotals {
    ranges: u64,
    blocks: u64,
    event_logs: u64,
    proof_logs: u64,
    empty_ranges: u64,

    rpc_event_ms: u64,
    rpc_proof_ms: u64,
    ts_ms: u64,
    decode_ms: u64,
    db_ms: u64,
    total_ms: u64,
}

impl IntervalTotals {
    fn observe(&mut self, m: &RangeMetrics) {
        self.ranges += 1;
        self.blocks += m.to_block.saturating_sub(m.from_block).saturating_add(1);
        self.event_logs += m.event_logs as u64;
        self.proof_logs += m.proof_logs as u64;
        if m.event_logs == 0 && m.proof_logs == 0 {
            self.empty_ranges += 1;
        }

        self.rpc_event_ms += m.rpc_event_ms;
        self.rpc_proof_ms += m.rpc_proof_ms;
        self.ts_ms += m.ts_ms;
        self.decode_ms += m.decode_ms;
        self.db_ms += m.db_ms;
        self.total_ms += m.total_ms;
    }
}

pub(super) struct ProgressReporter {
    stream: Stream,
    interval: Duration,

    started_at: Instant,
    last_report_at: Instant,
    last_stage: Option<Stage>,
    caught_up_once: bool,

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
    pub(super) fn new(stream: Stream, interval: Duration) -> Self {
        let now = Instant::now();
        Self {
            stream,
            interval: interval.max(Duration::from_secs(1)),
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

    pub(super) fn on_reorg(&mut self, invalidated_blocks: u64) {
        self.reorgs_detected += 1;
        self.blocks_invalidated += invalidated_blocks;
    }

    pub(super) fn on_transient_retry(&mut self) {
        self.transient_retries += 1;
    }

    pub(super) fn on_chunk_shrink(&mut self) {
        self.chunk_shrinks += 1;
    }

    pub(super) fn on_pinned_repair_attempt(&mut self) {
        self.pinned_repairs_attempted += 1;
    }

    pub(super) fn on_pinned_repair_success(&mut self) {
        self.pinned_repairs_succeeded += 1;
    }

    pub(super) fn observe_range(&mut self, metrics: &RangeMetrics) {
        self.interval_totals.observe(metrics);
        self.lifetime_totals.observe(metrics);
    }

    pub(super) fn maybe_report(
        &mut self,
        head: u64,
        safe_head: u64,
        from_block: u64,
        chunk_blocks: u64,
    ) {
        let now = Instant::now();
        let current_stage = stage(from_block, safe_head);

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

        let backlog_blocks = if from_block > safe_head {
            0
        } else {
            safe_head - from_block + 1
        };
        let block_rate = (self.interval_totals.blocks as f64) / interval_secs;
        let log_rate = ((self.interval_totals.event_logs + self.interval_totals.proof_logs) as f64)
            / interval_secs;

        let avg = |sum_ms: u64| -> u64 {
            if self.interval_totals.ranges == 0 {
                0
            } else {
                sum_ms / self.interval_totals.ranges
            }
        };

        info!(
            stream = self.stream.as_str(),
            stage = match current_stage {
                Stage::Backfill => "backfill",
                Stage::Tail => "tail",
            },
            head,
            safe_head,
            next_block = from_block,
            backlog_blocks,
            chunk_blocks,
            interval_secs,
            since_start_secs,
            ranges = self.interval_totals.ranges,
            blocks = self.interval_totals.blocks,
            blocks_per_sec = block_rate,
            event_logs = self.interval_totals.event_logs,
            proof_logs = self.interval_totals.proof_logs,
            logs_per_sec = log_rate,
            empty_ranges = self.interval_totals.empty_ranges,
            avg_rpc_event_ms = avg(self.interval_totals.rpc_event_ms),
            avg_rpc_proof_ms = avg(self.interval_totals.rpc_proof_ms),
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
            lifetime_event_logs = self.lifetime_totals.event_logs,
            lifetime_proof_logs = self.lifetime_totals.proof_logs,
            "stream progress"
        );

        self.last_report_at = now;
        self.last_stage = Some(current_stage);
        self.interval_totals = IntervalTotals::default();
    }
}
