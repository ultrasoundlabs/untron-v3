mod offer;
mod receiver_salt;
mod userop;

use crate::api::offer::compute_offer;
use crate::api::receiver_salt::{
    ensure_receiver_is_free, normalize_receiver_salt_hex, pick_receiver_salt_for_beneficiary,
};
use crate::api::userop::send_userop;
use crate::util::parse_bytes32;
use crate::{AppState, now_unix_seconds};
use alloy::primitives::U256;
use alloy::sol_types::SolCall;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use untron_v3_bindings::untron_v3::UntronV3;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Forbidden(String),
    Conflict(String),
    TooManyRequests(String),
    Upstream(String),
    Internal(String),
}

impl ApiError {
    fn kind(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "bad_request",
            Self::Forbidden(_) => "forbidden",
            Self::Conflict(_) => "conflict",
            Self::TooManyRequests(_) => "too_many_requests",
            Self::Upstream(_) => "upstream",
            Self::Internal(_) => "internal",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::Upstream(_) => StatusCode::BAD_GATEWAY,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let kind = self.kind();
        let status = self.status_code();
        let msg = match self {
            Self::BadRequest(m)
            | Self::Forbidden(m)
            | Self::Conflict(m)
            | Self::TooManyRequests(m)
            | Self::Upstream(m)
            | Self::Internal(m) => m,
        };

        match status {
            s if s.is_server_error() => {
                tracing::warn!(%kind, status = s.as_u16(), error = %msg, "api error");
            }
            _ => {
                tracing::info!(%kind, status = status.as_u16(), error = %msg, "api error");
            }
        }

        (status, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateLeaseRequest {
    #[serde(default)]
    pub receiver_salt: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateLeaseResponse {
    pub receiver_salt: String,
    pub userop_hash: String,
    pub nonce: String,
    pub nukeable_after: u64,
}

#[derive(Debug, Serialize)]
pub struct RealtorInfoResponse {
    pub safe: String,
    pub untron_v3: String,

    pub allowed: bool,

    pub min_fee_ppm: u32,
    pub min_flat_fee: u64,
    pub max_duration_seconds: u64,

    pub lease_rate_max_leases: u64,
    pub lease_rate_window_seconds: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_rate_remaining: Option<u64>,

    pub default_fee_ppm: u32,
    pub default_flat_fee: u64,
    pub default_duration_seconds: u64,

    pub effective_fee_ppm: u32,
    pub effective_flat_fee: u64,
    pub effective_duration_seconds: u64,

    pub lessee: String,
    pub target_chain_id: u64,
    pub target_token: String,
    pub beneficiary: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_receiver_salt: Option<String>,
}

pub async fn get_realtor(
    State(state): State<Arc<AppState>>,
) -> Result<Json<RealtorInfoResponse>, ApiError> {
    let start = Instant::now();

    let result: Result<_, ApiError> = async {
        let now = now_unix_seconds().map_err(ApiError::Internal)?;
        let offer = compute_offer(&state, now).await?;
        let suggested_receiver_salt =
            pick_receiver_salt_for_beneficiary(&state, now, offer.beneficiary)
                .await
                .ok()
                .flatten();
        Ok(Json(RealtorInfoResponse {
            safe: format!("{:#x}", state.cfg.hub.safe),
            untron_v3: format!("{:#x}", state.cfg.hub.untron_v3),
            allowed: offer.allowed,
            min_fee_ppm: offer.min_fee_ppm,
            min_flat_fee: offer.min_flat_fee,
            max_duration_seconds: offer.max_duration_seconds,
            lease_rate_max_leases: offer.lease_rate_max_leases,
            lease_rate_window_seconds: offer.lease_rate_window_seconds,
            lease_rate_remaining: offer.lease_rate_remaining,
            default_fee_ppm: offer.default_fee_ppm,
            default_flat_fee: offer.default_flat_fee,
            default_duration_seconds: offer.default_duration_seconds,
            effective_fee_ppm: offer.effective_fee_ppm,
            effective_flat_fee: offer.effective_flat_fee,
            effective_duration_seconds: offer.effective_duration_seconds,
            lessee: format!("{:#x}", offer.lessee),
            target_chain_id: offer.target_chain_id,
            target_token: format!("{:#x}", offer.target_token),
            beneficiary: format!("{:#x}", offer.beneficiary),
            suggested_receiver_salt,
        }))
    }
    .await;

    let ms = start.elapsed().as_millis() as u64;
    match &result {
        Ok(_) => state.telemetry.http_ok("get_realtor", ms),
        Err(e) => state.telemetry.http_err("get_realtor", e.kind(), ms),
    }
    result
}

pub async fn post_realtor(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateLeaseRequest>,
) -> Result<Json<CreateLeaseResponse>, ApiError> {
    let start = Instant::now();

    let receiver_salt_provided = req.receiver_salt.is_some();
    tracing::info!(receiver_salt_provided, "create_lease request");

    let result: Result<_, ApiError> = async {
        let now = now_unix_seconds().map_err(ApiError::Internal)?;
        let offer = compute_offer(&state, now).await?;

        if !offer.allowed {
            return Err(ApiError::Forbidden(
                "this realtor is not allowlisted on the hub".to_string(),
            ));
        }

        let receiver_salt_hex = match req.receiver_salt.as_deref() {
            Some(s) => {
                let receiver_salt_hex = normalize_receiver_salt_hex(s)?;
                let exists = state
                    .indexer
                    .receiver_salt_candidate(receiver_salt_hex.as_str())
                    .await
                    .map_err(|e| {
                        ApiError::Upstream(format!("indexer receiver_salt_candidates: {e}"))
                    })?;
                if exists.is_none() {
                    return Err(ApiError::BadRequest(format!(
                        "unknown receiver_salt (not found in indexer receiver_salt_candidates): {receiver_salt_hex}"
                    )));
                }
                receiver_salt_hex
            }
            None => pick_receiver_salt_for_beneficiary(&state, now, offer.beneficiary)
                .await?
                .ok_or_else(|| {
                    ApiError::Conflict("no free receiver salts available (all are leased)".to_string())
                })?,
        };

        ensure_receiver_is_free(&state, &receiver_salt_hex, now).await?;

        if offer.lease_rate_remaining == Some(0) {
            return Err(ApiError::TooManyRequests(format!(
                "rate limit: {} leases per {}s",
                offer.lease_rate_max_leases, offer.lease_rate_window_seconds
            )));
        }

        let receiver_salt = parse_bytes32(&receiver_salt_hex)
            .map_err(|e| ApiError::BadRequest(format!("receiver_salt: {e}")))?;

        let nukeable_after = now.saturating_add(offer.effective_duration_seconds);

        tracing::info!(
            receiver_salt = %receiver_salt_hex,
            nukeable_after,
            lease_fee_ppm = offer.effective_fee_ppm,
            flat_fee = offer.effective_flat_fee,
            "submitting createLease userop"
        );

        let call = UntronV3::createLeaseCall {
            receiverSalt: receiver_salt,
            lessee: offer.lessee,
            nukeableAfter: nukeable_after,
            leaseFeePpm: offer.effective_fee_ppm,
            flatFee: offer.effective_flat_fee,
            targetChainId: U256::from(offer.target_chain_id),
            targetToken: offer.target_token,
            beneficiary: offer.beneficiary,
        };
        let data = call.abi_encode();

        let (userop_hash, nonce) =
            send_userop(state.sender.lock().await, state.cfg.hub.untron_v3, data).await?;

        state.telemetry.userop_sent();
        state.telemetry.lease_created();

        tracing::info!(%userop_hash, %nonce, "lease userop submitted");

        Ok(Json(CreateLeaseResponse {
            receiver_salt: receiver_salt_hex,
            userop_hash,
            nonce,
            nukeable_after,
        }))
    }
    .await;

    let ms = start.elapsed().as_millis() as u64;
    match &result {
        Ok(_) => state.telemetry.http_ok("post_realtor", ms),
        Err(e) => state.telemetry.http_err("post_realtor", e.kind(), ms),
    }
    result
}
