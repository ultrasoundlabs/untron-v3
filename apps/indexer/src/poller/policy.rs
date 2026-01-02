pub(super) const MAX_TRANSIENT_RETRIES: u32 = 3;

pub(super) fn grow_chunk(current: u64, target: u64) -> u64 {
    if current >= target {
        return current;
    }
    current.saturating_mul(2).min(target)
}

pub(super) fn shrink_chunk(current: u64) -> u64 {
    (current / 2).max(1)
}
