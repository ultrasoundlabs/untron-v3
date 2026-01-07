use super::{
    ApiError, ErrorResponse, LeaseClaimView, LeasePayoutConfigVersionView, LeasePayoutConfigView,
    LeaseViewResponse,
};
use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::Value;
use std::sync::Arc;
use std::time::Instant;

#[utoipa::path(
    get,
    path = "/leases/{lease_id}",
    tag = "realtor",
    summary = "Fetch an aggregated lease view",
    description = "Returns a realtor-side aggregated view of a lease in the Untron V3 protocol.\n\nThis endpoint is designed to provide a stable response schema while still exposing rich information sourced from the indexer API.\n\nUint256-like values are returned as decimal strings.",
    params(
        ("lease_id" = u64, Path, description = "Global lease ID")
    ),
    responses(
        (status = 200, description = "OK", body = LeaseViewResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 502, description = "Upstream error", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn get_lease(
    State(state): State<Arc<AppState>>,
    Path(lease_id): Path<u64>,
) -> Result<Json<LeaseViewResponse>, ApiError> {
    let start = Instant::now();

    let result: Result<_, ApiError> = async {
        if lease_id == 0 {
            return Err(ApiError::BadRequest(
                "lease_id must be non-zero".to_string(),
            ));
        }

        let row = state
            .indexer
            .lease_view_row(lease_id)
            .await
            .map_err(|e| ApiError::Upstream(format!("indexer lease_view: {e}")))?;
        let Some(row) = row else {
            return Err(ApiError::BadRequest(format!(
                "unknown lease_id (not found in indexer lease_view): {lease_id}"
            )));
        };

        let lease_id_str = row
            .lease_id
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| lease_id.to_string());
        let receiver_salt = row.receiver_salt.clone().ok_or_else(|| {
            ApiError::Upstream("indexer lease_view missing receiver_salt".to_string())
        })?;
        let realtor = row
            .realtor
            .clone()
            .ok_or_else(|| ApiError::Upstream("indexer lease_view missing realtor".to_string()))?;
        let lessee = row
            .lessee
            .clone()
            .ok_or_else(|| ApiError::Upstream("indexer lease_view missing lessee".to_string()))?;

        let expected_realtor = format!("{:#x}", state.cfg.hub.safe).to_lowercase();
        let is_owned_by_this_realtor = realtor.to_lowercase() == expected_realtor;

        let start_time = row
            .start_time
            .and_then(|v| u64::try_from(v).ok())
            .ok_or_else(|| {
                ApiError::Upstream("indexer lease_view missing start_time".to_string())
            })?;
        let nukeable_after = row
            .nukeable_after
            .and_then(|v| u64::try_from(v).ok())
            .ok_or_else(|| {
                ApiError::Upstream("indexer lease_view missing nukeable_after".to_string())
            })?;

        let lease_fee_ppm = row
            .lease_fee_ppm
            .and_then(|v| u32::try_from(v).ok())
            .ok_or_else(|| {
                ApiError::Upstream("indexer lease_view missing lease_fee_ppm".to_string())
            })?;
        let flat_fee = row
            .flat_fee
            .as_ref()
            .map(ToString::to_string)
            .ok_or_else(|| ApiError::Upstream("indexer lease_view missing flat_fee".to_string()))?;

        let lease_nonce = row
            .lease_nonce
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "0".to_string());

        let payout_config_current = match (
            row.payout_target_chain_id
                .and_then(|v| u64::try_from(v).ok()),
            row.payout_target_token.as_ref(),
            row.payout_beneficiary.as_ref(),
        ) {
            (Some(target_chain_id), Some(target_token), Some(beneficiary)) => {
                Some(LeasePayoutConfigView {
                    target_chain_id,
                    target_token: target_token.clone(),
                    beneficiary: beneficiary.clone(),
                })
            }
            _ => None,
        };

        let empty_json_array = Value::Array(Vec::new());
        let payout_config_history_value = row
            .payout_config_history
            .as_ref()
            .unwrap_or(&empty_json_array);
        let payout_config_history = parse_payout_config_history(payout_config_history_value)?;

        let claims_value = row.claims.as_ref().unwrap_or(&empty_json_array);
        let claims = parse_claims(claims_value)?;
        let claims_total = row
            .claims_total
            .and_then(|v| u64::try_from(v).ok())
            .unwrap_or(claims.len() as u64);
        let claims_filled = row
            .claims_filled
            .and_then(|v| u64::try_from(v).ok())
            .unwrap_or_else(|| claims.iter().filter(|c| c.status == "filled").count() as u64);

        Ok(Json(LeaseViewResponse {
            lease_id: lease_id_str,
            receiver_salt,
            realtor,
            is_owned_by_this_realtor,
            lessee,
            start_time,
            nukeable_after,
            lease_fee_ppm,
            flat_fee,
            lease_nonce,
            payout_config_current,
            payout_config_history,
            claims,
            claims_total,
            claims_filled,
        }))
    }
    .await;

    let ms = start.elapsed().as_millis() as u64;
    match &result {
        Ok(_) => state.telemetry.http_ok("GET", "get_lease", 200, ms),
        Err(e) => {
            state
                .telemetry
                .http_err("GET", "get_lease", e.kind(), e.status_code().as_u16(), ms)
        }
    }
    result
}

fn json_decimal_string(v: &Value, _label: &'static str) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn json_u64(v: &Value, label: &'static str) -> Result<u64, ApiError> {
    let s = json_decimal_string(v, label)
        .ok_or_else(|| ApiError::Upstream(format!("indexer lease_view missing {label}")))?;
    s.parse::<u64>()
        .map_err(|e| ApiError::Upstream(format!("indexer lease_view invalid {label}: {e}")))
}

fn json_i32(v: &Value, label: &'static str) -> Result<i32, ApiError> {
    let s = json_decimal_string(v, label)
        .ok_or_else(|| ApiError::Upstream(format!("indexer lease_view missing {label}")))?;
    s.parse::<i32>()
        .map_err(|e| ApiError::Upstream(format!("indexer lease_view invalid {label}: {e}")))
}

fn json_string(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        _ => None,
    }
}

fn parse_payout_config_history(v: &Value) -> Result<Vec<LeasePayoutConfigVersionView>, ApiError> {
    let arr = v.as_array().ok_or_else(|| {
        ApiError::Upstream("indexer lease_view payout_config_history is not an array".to_string())
    })?;

    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let obj = item.as_object().ok_or_else(|| {
            ApiError::Upstream(
                "indexer lease_view payout_config_history entry is not an object".to_string(),
            )
        })?;

        let target_chain_id = obj
            .get("target_chain_id")
            .map(|v| json_u64(v, "payout_config_history.target_chain_id"))
            .transpose()?
            .ok_or_else(|| {
                ApiError::Upstream(
                    "indexer lease_view payout_config_history missing target_chain_id".to_string(),
                )
            })?;
        let target_token = obj
            .get("target_token")
            .and_then(json_string)
            .ok_or_else(|| {
                ApiError::Upstream(
                    "indexer lease_view payout_config_history missing target_token".to_string(),
                )
            })?;
        let beneficiary = obj
            .get("beneficiary")
            .and_then(json_string)
            .ok_or_else(|| {
                ApiError::Upstream(
                    "indexer lease_view payout_config_history missing beneficiary".to_string(),
                )
            })?;

        let valid_from_seq = obj
            .get("valid_from_seq")
            .map(|v| json_u64(v, "payout_config_history.valid_from_seq"))
            .transpose()?
            .unwrap_or(0);
        let valid_to_seq = match obj.get("valid_to_seq") {
            Some(Value::Null) | None => None,
            Some(other) => Some(json_u64(other, "payout_config_history.valid_to_seq")?),
        };

        out.push(LeasePayoutConfigVersionView {
            config: LeasePayoutConfigView {
                target_chain_id,
                target_token,
                beneficiary,
            },
            valid_from_seq,
            valid_to_seq,
        });
    }

    Ok(out)
}

fn parse_claims(v: &Value) -> Result<Vec<LeaseClaimView>, ApiError> {
    let arr = v.as_array().ok_or_else(|| {
        ApiError::Upstream("indexer lease_view claims is not an array".to_string())
    })?;

    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let obj = item.as_object().ok_or_else(|| {
            ApiError::Upstream("indexer lease_view claim entry is not an object".to_string())
        })?;

        let claim_id = obj
            .get("claim_id")
            .and_then(|v| json_decimal_string(v, "claims.claim_id"))
            .ok_or_else(|| {
                ApiError::Upstream("indexer lease_view claim missing claim_id".to_string())
            })?;
        let status = obj.get("status").and_then(json_string).ok_or_else(|| {
            ApiError::Upstream("indexer lease_view claim missing status".to_string())
        })?;
        let queue_index = obj
            .get("queue_index")
            .and_then(|v| json_decimal_string(v, "claims.queue_index"))
            .ok_or_else(|| {
                ApiError::Upstream("indexer lease_view claim missing queue_index".to_string())
            })?;
        let amount_usdt = obj
            .get("amount_usdt")
            .and_then(|v| json_decimal_string(v, "claims.amount_usdt"))
            .ok_or_else(|| {
                ApiError::Upstream("indexer lease_view claim missing amount_usdt".to_string())
            })?;
        let target_chain_id = obj
            .get("target_chain_id")
            .map(|v| json_u64(v, "claims.target_chain_id"))
            .transpose()?
            .ok_or_else(|| {
                ApiError::Upstream("indexer lease_view claim missing target_chain_id".to_string())
            })?;
        let target_token = obj
            .get("target_token")
            .and_then(json_string)
            .ok_or_else(|| {
                ApiError::Upstream("indexer lease_view claim missing target_token".to_string())
            })?;
        let beneficiary = obj
            .get("beneficiary")
            .and_then(json_string)
            .ok_or_else(|| {
                ApiError::Upstream("indexer lease_view claim missing beneficiary".to_string())
            })?;

        let origin = obj
            .get("origin")
            .map(|v| json_i32(v, "claims.origin"))
            .transpose()?
            .ok_or_else(|| {
                ApiError::Upstream("indexer lease_view claim missing origin".to_string())
            })?;
        let origin_id = obj
            .get("origin_id")
            .and_then(json_string)
            .unwrap_or_else(|| "0x".to_string());
        let origin_actor = obj
            .get("origin_actor")
            .and_then(json_string)
            .unwrap_or_else(|| "0x0000000000000000000000000000000000000000".to_string());
        let origin_token = obj
            .get("origin_token")
            .and_then(json_string)
            .unwrap_or_else(|| "0x0000000000000000000000000000000000000000".to_string());
        let origin_timestamp = match obj.get("origin_timestamp") {
            Some(Value::Number(n)) => n.as_i64().unwrap_or(0),
            Some(Value::String(s)) => s.parse::<i64>().unwrap_or(0),
            _ => 0,
        };
        let origin_raw_amount = obj
            .get("origin_raw_amount")
            .and_then(|v| json_decimal_string(v, "claims.origin_raw_amount"))
            .unwrap_or_else(|| "0".to_string());

        let valid_from_seq = match obj.get("valid_from_seq") {
            Some(Value::Null) | None => 0,
            Some(other) => json_u64(other, "claims.valid_from_seq")?,
        };
        let valid_to_seq = match obj.get("valid_to_seq") {
            Some(Value::Null) | None => None,
            Some(other) => Some(json_u64(other, "claims.valid_to_seq")?),
        };

        out.push(LeaseClaimView {
            claim_id,
            status,
            queue_index,
            amount_usdt,
            target_chain_id,
            target_token,
            beneficiary,
            origin,
            origin_id,
            origin_actor,
            origin_token,
            origin_timestamp,
            origin_raw_amount,
            valid_from_seq,
            valid_to_seq,
        });
    }

    Ok(out)
}
