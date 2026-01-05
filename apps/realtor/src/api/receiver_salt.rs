use crate::AppState;
use crate::api::ApiError;
use crate::util::parse_bytes32;
use alloy::primitives::Address;

pub(super) fn normalize_receiver_salt_hex(receiver_salt: &str) -> Result<String, ApiError> {
    let b = parse_bytes32(receiver_salt)
        .map_err(|e| ApiError::BadRequest(format!("receiver_salt: {e}")))?;
    Ok(format!("0x{}", hex::encode(b.as_slice())))
}

pub(super) async fn ensure_receiver_is_free(
    state: &AppState,
    receiver_salt_hex: &str,
    now: u64,
) -> Result<(), ApiError> {
    let row = state
        .indexer
        .receiver_salt_candidate(receiver_salt_hex)
        .await
        .map_err(|e| ApiError::Upstream(format!("indexer receiver_salt_candidates: {e}")))?;
    let Some(row) = row else {
        return Err(ApiError::BadRequest(format!(
            "unknown receiver_salt (not found in indexer receiver_salt_candidates): {receiver_salt_hex}"
        )));
    };

    if row.is_free == Some(true) {
        return Ok(());
    }

    let nukeable_after = row
        .nukeable_after
        .and_then(|v| u64::try_from(v).ok())
        .unwrap_or(0);
    if nukeable_after > now {
        return Err(ApiError::Conflict(format!(
            "receiver lease not nukeable yet (nukeable_after={nukeable_after})"
        )));
    }
    Err(ApiError::Conflict(
        "receiver lease not nukeable yet".to_string(),
    ))
}

pub(super) async fn pick_receiver_salt_for_beneficiary(
    state: &AppState,
    _now: u64,
    beneficiary: Address,
) -> Result<Option<String>, ApiError> {
    const LIMIT: u64 = 50;

    let beneficiary_lower = address_lower_hex(beneficiary);
    let has_filled_claims = state
        .indexer
        .beneficiary_has_filled_claims(beneficiary_lower.as_str())
        .await
        .map_err(|e| ApiError::Upstream(format!("indexer hub_claims filled: {e}")))?;

    tracing::debug!(
        beneficiary = %beneficiary_lower,
        has_filled_claims,
        "selecting receiver salt"
    );

    let order: &'static str = if has_filled_claims {
        "balance_amount.desc"
    } else {
        "balance_amount.asc"
    };

    let preferred = state
        .indexer
        .receiver_salt_candidates(order, LIMIT, true, true)
        .await
        .map_err(|e| ApiError::Upstream(format!("indexer receiver_salt_candidates: {e}")))?;
    if let Some(s) = preferred.into_iter().find_map(|r| r.receiver_salt) {
        tracing::info!(receiver_salt = %s, "selected receiver salt (non-zero balance)");
        return Ok(Some(s));
    }

    state.telemetry.receiver_salt_balance_picker_fallback();
    tracing::warn!("no non-zero-balance free receiver salts; falling back to any free salt");

    let fallback = state
        .indexer
        .receiver_salt_candidates(order, LIMIT, true, false)
        .await
        .map_err(|e| ApiError::Upstream(format!("indexer receiver_salt_candidates: {e}")))?;

    let mut picked = None;
    for r in fallback {
        let Some(s) = r.receiver_salt else {
            continue;
        };
        picked = Some((s, r.has_balance.unwrap_or(false)));
        break;
    }

    let Some((salt, has_balance)) = picked else {
        return Ok(None);
    };

    if !has_balance {
        state.telemetry.receiver_salt_zero_balance_fallback(order);
    }

    tracing::info!(receiver_salt = %salt, "selected receiver salt (fallback)");
    Ok(Some(salt))
}

fn address_lower_hex(addr: Address) -> String {
    format!("{:#x}", addr).to_lowercase()
}
