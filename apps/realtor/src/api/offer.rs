use crate::AppState;
use crate::api::ApiError;
use crate::util::{i64_to_u32, i64_to_u64, number_to_u64};
use alloy::primitives::Address;

pub(super) struct Offer {
    pub(super) allowed: bool,

    pub(super) min_fee_ppm: u32,
    pub(super) min_flat_fee: u64,
    pub(super) max_duration_seconds: u64,

    pub(super) lease_rate_max_leases: u64,
    pub(super) lease_rate_window_seconds: u64,
    pub(super) lease_rate_remaining: Option<u64>,

    pub(super) default_fee_ppm: u32,
    pub(super) default_flat_fee: u64,
    pub(super) default_duration_seconds: u64,

    pub(super) effective_fee_ppm: u32,
    pub(super) effective_flat_fee: u64,
    pub(super) effective_duration_seconds: u64,
}

pub(super) async fn compute_offer(state: &AppState, _now: u64) -> Result<Offer, ApiError> {
    let safe_addr_checksum = address_checksum(
        state
            .cfg
            .hub
            .safe
            .expect("hub safe must be resolved at startup"),
    );

    let cfg = state
        .indexer
        .realtor_effective_config(&safe_addr_checksum)
        .await
        .map_err(|e| ApiError::Upstream(format!("indexer realtor_effective_config: {e}")))?;

    let allowed = cfg.as_ref().and_then(|r| r.allowed).unwrap_or(false);

    let min_fee_ppm = cfg
        .as_ref()
        .and_then(|r| r.min_fee_ppm)
        .map(|v| i64_to_u32(v, "min_fee_ppm"))
        .transpose()
        .map_err(|e| ApiError::Upstream(format!("indexer realtor min_fee_ppm: {e}")))?
        .unwrap_or(0);

    let min_flat_fee = cfg
        .as_ref()
        .and_then(|r| r.min_flat_fee.as_ref())
        .map(|n| number_to_u64(n, "min_flat_fee"))
        .transpose()
        .map_err(|e| ApiError::Upstream(format!("indexer realtor min_flat_fee: {e}")))?
        .unwrap_or(0);

    let max_duration_secs = cfg
        .as_ref()
        .and_then(|r| r.max_duration_seconds)
        .map(|v| i64_to_u64(v, "max_duration_seconds"))
        .transpose()
        .map_err(|e| ApiError::Upstream(format!("indexer realtor max_duration_seconds: {e}")))?
        .unwrap_or(0);

    let lease_rate_max_leases = cfg
        .as_ref()
        .and_then(|r| r.lease_rate_max_leases.as_ref())
        .and_then(|n| number_to_u64(n, "lease_rate_max_leases").ok())
        .unwrap_or(0);

    let lease_rate_window_seconds = cfg
        .as_ref()
        .and_then(|r| r.lease_rate_window_seconds.as_ref())
        .and_then(|n| number_to_u64(n, "lease_rate_window_seconds").ok())
        .unwrap_or(0);

    let default_fee_ppm = state.cfg.leasing.lease_fee_ppm;
    let default_flat_fee = state.cfg.leasing.flat_fee;
    let default_duration_seconds = state.cfg.leasing.duration_seconds.max(1);

    let effective_fee_ppm = default_fee_ppm.max(min_fee_ppm);
    let effective_flat_fee = default_flat_fee.max(min_flat_fee);

    let effective_duration_seconds = if max_duration_secs == 0 {
        default_duration_seconds
    } else {
        default_duration_seconds.min(max_duration_secs).max(1)
    };

    let lease_rate_remaining = match cfg.as_ref().and_then(|r| r.lease_rate_remaining.as_ref()) {
        None => None,
        Some(n) => Some(number_to_u64(n, "lease_rate_remaining").map_err(|e| {
            ApiError::Upstream(format!("indexer realtor lease_rate_remaining: {e}"))
        })?),
    };

    Ok(Offer {
        allowed,
        min_fee_ppm,
        min_flat_fee,
        max_duration_seconds: max_duration_secs,
        lease_rate_max_leases,
        lease_rate_window_seconds,
        lease_rate_remaining,
        default_fee_ppm,
        default_flat_fee,
        default_duration_seconds,
        effective_fee_ppm,
        effective_flat_fee,
        effective_duration_seconds,
    })
}

fn address_checksum(addr: Address) -> String {
    addr.to_checksum_buffer(None).to_string()
}
