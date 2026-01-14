use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct BackoffConfig {
    pub base: Duration,
    pub max: Duration,
}

#[derive(Debug, Default)]
pub struct BackoffState {
    streak: u32,
    cooldown_until: Option<Instant>,
}

impl BackoffState {
    pub fn in_cooldown(&self) -> Option<Duration> {
        let until = self.cooldown_until?;
        let now = Instant::now();
        if until <= now {
            return None;
        }
        Some(until.duration_since(now))
    }

    pub fn on_success(&mut self) {
        self.streak = 0;
        self.cooldown_until = None;
    }

    pub fn on_failure(&mut self, cfg: BackoffConfig) -> Duration {
        self.streak = self.streak.saturating_add(1);

        // base * 2^(streak-1), capped.
        let pow = self.streak.saturating_sub(1).min(31);
        let mult = 1u64 << pow;
        let base_secs = cfg.base.as_secs().max(1);
        let mut secs = base_secs.saturating_mul(mult);
        let max_secs = cfg.max.as_secs().max(1);
        if secs > max_secs {
            secs = max_secs;
        }
        let dur = Duration::from_secs(secs);
        self.cooldown_until = Some(Instant::now() + dur);
        dur
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_exponential_with_cap() {
        let cfg = BackoffConfig {
            base: Duration::from_secs(10),
            max: Duration::from_secs(100),
        };
        let mut st = BackoffState::default();

        assert_eq!(st.on_failure(cfg), Duration::from_secs(10));
        assert_eq!(st.on_failure(cfg), Duration::from_secs(20));
        assert_eq!(st.on_failure(cfg), Duration::from_secs(40));
        assert_eq!(st.on_failure(cfg), Duration::from_secs(80));
        assert_eq!(st.on_failure(cfg), Duration::from_secs(100));
        assert_eq!(st.on_failure(cfg), Duration::from_secs(100));

        st.on_success();
        assert!(st.in_cooldown().is_none());
        assert_eq!(st.on_failure(cfg), Duration::from_secs(10));
    }
}
