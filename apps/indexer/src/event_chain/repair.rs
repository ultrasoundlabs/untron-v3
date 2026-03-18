use crate::{config::GapRepairConfig, db};
use anyhow::Result;
use std::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;
use tracing::warn;

use super::{errors, range, state::PollState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RepairWindow {
    pub from_block: u64,
    pub to_block: u64,
}

#[derive(Debug)]
pub(super) struct GapRepairState {
    missing_seq: Option<u64>,
    attempt: u32,
    pad_blocks: u64,
    next_retry_at: Instant,
}

impl GapRepairState {
    pub(super) fn new() -> Self {
        Self {
            missing_seq: None,
            attempt: 0,
            pad_blocks: 0,
            next_retry_at: Instant::now(),
        }
    }

    pub(super) fn observe_gap(&mut self, missing_seq: u64, cfg: &GapRepairConfig) {
        if self.missing_seq == Some(missing_seq) {
            return;
        }

        self.missing_seq = Some(missing_seq);
        self.attempt = 0;
        self.pad_blocks = cfg.initial_pad_blocks;
        self.next_retry_at = Instant::now();
    }

    pub(super) fn clear(&mut self) {
        self.missing_seq = None;
        self.attempt = 0;
        self.pad_blocks = 0;
        self.next_retry_at = Instant::now();
    }

    pub(super) fn should_retry_now(&self) -> bool {
        Instant::now() >= self.next_retry_at
    }

    pub(super) fn retry_after(&self) -> Duration {
        self.next_retry_at.saturating_duration_since(Instant::now())
    }

    pub(super) fn attempt_number(&self) -> u32 {
        self.attempt.saturating_add(1)
    }

    pub(super) fn repair_window(
        &self,
        gap: &db::event_chain::BlockingGap,
        safe_head: u64,
        cfg: &GapRepairConfig,
    ) -> RepairWindow {
        let mut from_block = gap.left_block.saturating_sub(self.pad_blocks);
        let mut to_block = gap
            .right_block
            .saturating_add(self.pad_blocks)
            .min(safe_head);

        if to_block < gap.right_block {
            to_block = gap.right_block;
        }

        let span = to_block.saturating_sub(from_block).saturating_add(1);
        let min_span = gap
            .right_block
            .saturating_sub(gap.left_block)
            .saturating_add(1);
        let max_span = cfg.max_window_blocks.max(min_span);

        if span > max_span {
            to_block = gap.right_block.min(safe_head);
            from_block = to_block.saturating_add(1).saturating_sub(max_span);
            if from_block > gap.left_block {
                from_block = gap.left_block;
                to_block = from_block
                    .saturating_add(max_span.saturating_sub(1))
                    .min(safe_head);
            }
        }

        RepairWindow {
            from_block,
            to_block,
        }
    }

    pub(super) fn record_failure(&mut self, cfg: &GapRepairConfig) {
        self.attempt = self.attempt.saturating_add(1);
        self.pad_blocks = if self.pad_blocks == 0 {
            1
        } else {
            self.pad_blocks.saturating_mul(2)
        };

        let shift = self.attempt.saturating_sub(1).min(16);
        let factor = 1u32 << shift;
        let backoff = cfg
            .initial_backoff
            .saturating_mul(factor)
            .min(cfg.max_backoff);
        self.next_retry_at = Instant::now() + backoff;
    }
}

pub(super) async fn scan_window(
    dbh: &db::Db,
    shutdown: &CancellationToken,
    state: &mut PollState,
    window: RepairWindow,
) -> Result<()> {
    let mut current = window.from_block;

    while current <= window.to_block {
        if shutdown.is_cancelled() {
            return Ok(());
        }

        let chunk = state.chunk_current.max(1);
        let to_block = window
            .to_block
            .min(current.saturating_add(chunk.saturating_sub(1)));

        match range::process_range(dbh, shutdown, state, current, to_block).await {
            Ok(Some(_)) => {
                current = to_block.saturating_add(1);
            }
            Ok(None) => return Ok(()),
            Err(e) => {
                if state.chunk_current > 1 {
                    state.chunk_current = (state.chunk_current / 2).max(1);
                    warn!(
                        stream = state.stream.as_str(),
                        from_block = current,
                        to_block,
                        chunk_blocks = state.chunk_current,
                        err = %e,
                        "gap repair range failed; shrinking chunk"
                    );
                    continue;
                }

                let mut repaired = false;
                for idx in 0..state.pinned_providers.len() {
                    if shutdown.is_cancelled() {
                        return Ok(());
                    }

                    let pinned = state.pinned_providers[idx].clone();

                    match range::process_range_with_provider(
                        dbh, shutdown, state, &pinned, current, to_block,
                    )
                    .await
                    {
                        Ok(Some(_)) => {
                            current = to_block.saturating_add(1);
                            repaired = true;
                            break;
                        }
                        Ok(None) => return Ok(()),
                        Err(e2) => {
                            warn!(
                                stream = state.stream.as_str(),
                                from_block = current,
                                to_block,
                                pinned_index = idx,
                                err = %e2,
                                "gap repair pinned provider failed"
                            );
                        }
                    }
                }

                if repaired {
                    continue;
                }

                if errors::looks_like_transient(&e) {
                    warn!(
                        stream = state.stream.as_str(),
                        from_block = current,
                        to_block,
                        err = %e,
                        "gap repair scan failed transiently"
                    );
                }

                return Err(e);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> GapRepairConfig {
        GapRepairConfig {
            enabled: true,
            initial_pad_blocks: 16,
            max_window_blocks: 128,
            initial_backoff: Duration::from_secs(5),
            max_backoff: Duration::from_secs(300),
        }
    }

    fn gap(left_block: u64, right_block: u64) -> db::event_chain::BlockingGap {
        db::event_chain::BlockingGap {
            missing_seq: 11,
            later_seq: 12,
            left_block,
            right_block,
        }
    }

    #[test]
    fn repair_window_expands_with_padding() {
        let mut state = GapRepairState::new();
        let cfg = cfg();
        state.observe_gap(11, &cfg);

        let window = state.repair_window(&gap(100, 120), 1_000, &cfg);
        assert_eq!(window.from_block, 84);
        assert_eq!(window.to_block, 136);

        state.record_failure(&cfg);
        let widened = state.repair_window(&gap(100, 120), 1_000, &cfg);
        assert_eq!(widened.from_block, 68);
        assert_eq!(widened.to_block, 152);
    }

    #[test]
    fn repair_window_honors_max_window_without_excluding_base_range() {
        let mut state = GapRepairState::new();
        let cfg = GapRepairConfig {
            max_window_blocks: 32,
            ..cfg()
        };
        state.observe_gap(11, &cfg);

        let window = state.repair_window(&gap(100, 180), 1_000, &cfg);
        assert_eq!(window.from_block, 100);
        assert_eq!(window.to_block, 180);
    }

    #[test]
    fn observe_gap_resets_attempts_for_new_missing_seq() {
        let cfg = cfg();
        let mut state = GapRepairState::new();

        state.observe_gap(11, &cfg);
        state.record_failure(&cfg);
        assert_eq!(state.attempt_number(), 2);

        state.observe_gap(12, &cfg);
        assert_eq!(state.attempt_number(), 1);
        assert!(state.should_retry_now());
    }
}
