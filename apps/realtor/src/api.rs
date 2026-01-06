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
use alloy::primitives::Address;
use alloy::primitives::U256;
use alloy::sol_types::SolCall;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use untron_v3_bindings::untron_v3::UntronV3;
use utoipa::ToSchema;

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

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateLeaseRequest {
    #[serde(default)]
    /// Optional receiver salt (bytes32 hex).
    ///
    /// - If omitted, server selects an available salt.
    /// - If provided, must exist in indexer `receiver_salt_candidates`.
    #[schema(
        example = "0x0000000000000000000000000000000000000000000000000000000000000000",
        pattern = "^0x[0-9a-fA-F]{64}$",
        nullable = true
    )]
    pub receiver_salt: Option<String>,

    #[serde(default)]
    /// Optional lessee address.
    ///
    /// - If omitted, the zero address is used.
    /// - If provided, `arbitrary_lessee_flat_fee` is added to the flat fee.
    #[schema(
        example = "0x0000000000000000000000000000000000000001",
        pattern = "^0x[0-9a-fA-F]{40}$",
        nullable = true
    )]
    pub lessee: Option<String>,

    /// Required lease duration in seconds.
    ///
    /// Must be `<= max_duration_seconds` when `max_duration_seconds != 0`.
    #[schema(example = 2592000, minimum = 1)]
    pub duration_seconds: u64,

    /// Destination EVM chainId.
    ///
    /// Must have a configured bridger route for `(target_chain_id,target_token)`.
    #[schema(example = 1, minimum = 1)]
    pub target_chain_id: u64,

    /// Target settlement token (EVM address on hub chain).
    ///
    /// Must have a configured bridger route for `(target_chain_id,target_token)`.
    #[schema(
        example = "0x0000000000000000000000000000000000000002",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub target_token: String,

    /// Beneficiary address (EVM).
    #[schema(
        example = "0x0000000000000000000000000000000000000003",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub beneficiary: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateLeaseResponse {
    /// Receiver salt selected/used for the lease (bytes32 hex).
    #[schema(
        example = "0x0000000000000000000000000000000000000000000000000000000000000000",
        pattern = "^0x[0-9a-fA-F]{64}$"
    )]
    pub receiver_salt: String,
    /// UserOperation hash.
    #[schema(example = "0x0000000000000000000000000000000000000000000000000000000000000000")]
    pub userop_hash: String,
    /// Safe4337 nonce used for the submitted UserOperation.
    #[schema(example = "0")]
    pub nonce: String,
    /// Unix timestamp after which the lease is nukeable, computed as `now + duration_seconds`.
    #[schema(example = 1700000000)]
    pub nukeable_after: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RealtorTargetPairResponse {
    /// Destination EVM chainId.
    #[schema(example = 1, minimum = 1)]
    pub target_chain_id: u64,
    /// Target settlement token (EVM address on hub chain).
    #[schema(
        example = "0x0000000000000000000000000000000000000002",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub target_token: String,
    /// Effective lease fee in PPM for this pair (currently not pair-specific).
    #[schema(example = 10000)]
    pub effective_fee_ppm: u32,
    /// Effective flat fee for this pair including any env-configured per-pair additional flat fee.
    ///
    /// Does not include `arbitrary_lessee_flat_fee` (which depends on the request).
    #[schema(example = 0)]
    pub effective_flat_fee: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RealtorInfoResponse {
    /// Hub Safe address for this realtor instance.
    #[schema(
        example = "0x0000000000000000000000000000000000000004",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub safe: String,
    /// UntronV3 contract address on hub chain.
    #[schema(
        example = "0x0000000000000000000000000000000000000005",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub untron_v3: String,

    /// Whether this realtor is allowlisted on the hub.
    ///
    /// When false, `POST /realtor` returns `403`.
    #[schema(example = true)]
    pub allowed: bool,

    /// Minimum lease fee PPM configured on hub for this realtor.
    #[schema(example = 0)]
    pub min_fee_ppm: u32,
    /// Minimum flat fee configured on hub for this realtor.
    #[schema(example = 0)]
    pub min_flat_fee: u64,
    /// Maximum allowed lease duration in seconds.
    ///
    /// If 0, no max is enforced by this service.
    #[schema(example = 2592000)]
    pub max_duration_seconds: u64,

    /// Rate limit: max leases in window.
    #[schema(example = 0)]
    pub lease_rate_max_leases: u64,
    /// Rate limit: window size in seconds.
    #[schema(example = 0)]
    pub lease_rate_window_seconds: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rate limit: remaining leases in window (if reported by indexer).
    #[schema(nullable = true)]
    pub lease_rate_remaining: Option<u64>,

    /// Default lease fee PPM from this service's env.
    #[schema(example = 10000)]
    pub default_fee_ppm: u32,
    /// Default flat fee from this service's env (before min and adders).
    #[schema(example = 0)]
    pub default_flat_fee: u64,
    /// Default duration seconds from this service's env (used only to compute effective_duration_seconds).
    #[schema(example = 2592000)]
    pub default_duration_seconds: u64,

    /// Effective duration seconds used for informational purposes (currently derived from defaults and max).
    ///
    /// `POST /realtor` requires `duration_seconds` explicitly.
    #[schema(example = 2592000)]
    pub effective_duration_seconds: u64,

    /// Supported (target_chain_id,target_token) pairs from the current bridger routing table.
    pub supported_pairs: Vec<RealtorTargetPairResponse>,

    /// Additional flat fee added when a non-null `lessee` is provided in `POST /realtor`.
    #[schema(example = 0)]
    pub arbitrary_lessee_flat_fee: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message string.
    #[schema(example = "bad request")]
    pub error: String,
}

#[utoipa::path(
    get,
    path = "/realtor",
    tag = "realtor",
    summary = "Fetch realtor terms and supported pairs",
    description = "Returns current realtor limits, effective fee defaults, fee adders, and the list of supported (target_chain_id,target_token) pairs derived from the current bridger routing table.",
    responses(
        (status = 200, description = "OK", body = RealtorInfoResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 409, description = "Conflict", body = ErrorResponse),
        (status = 429, description = "Too many requests", body = ErrorResponse),
        (status = 502, description = "Upstream error", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn get_realtor(
    State(state): State<Arc<AppState>>,
) -> Result<Json<RealtorInfoResponse>, ApiError> {
    let start = Instant::now();

    let result: Result<_, ApiError> = async {
        let now = now_unix_seconds().map_err(ApiError::Internal)?;
        let offer = compute_offer(&state, now).await?;
        let mut pairs = state
            .indexer
            .bridger_pairs_current()
            .await
            .map_err(|e| ApiError::Upstream(format!("indexer hub_bridgers current: {e}")))?;
        pairs.sort_by(|a, b| {
            (a.target_chain_id, &a.target_token).cmp(&(b.target_chain_id, &b.target_token))
        });
        pairs.dedup_by(|a, b| {
            a.target_chain_id == b.target_chain_id && a.target_token == b.target_token
        });
        let mut supported_pairs = Vec::with_capacity(pairs.len());
        for p in pairs {
            let target_token_addr: Address = p.target_token.parse().map_err(|_| {
                ApiError::Upstream(format!(
                    "indexer hub_bridgers returned invalid target_token: {}",
                    p.target_token
                ))
            })?;
            let pair_additional_flat_fee = state
                .cfg
                .leasing
                .pair_additional_flat_fees
                .get(&(p.target_chain_id, target_token_addr))
                .copied()
                .unwrap_or(0);
            supported_pairs.push(RealtorTargetPairResponse {
                target_chain_id: p.target_chain_id,
                target_token: p.target_token,
                effective_fee_ppm: offer.effective_fee_ppm,
                effective_flat_fee: offer
                    .effective_flat_fee
                    .saturating_add(pair_additional_flat_fee),
            });
        }
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
            effective_duration_seconds: offer.effective_duration_seconds,
            supported_pairs,
            arbitrary_lessee_flat_fee: state.cfg.leasing.arbitrary_lessee_flat_fee,
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

#[utoipa::path(
    post,
    path = "/realtor",
    tag = "realtor",
    summary = "Create an address lease in Untron V3 protocol.",
    description = "Creates a lease on UntronV3 using the configured hub Safe-4337 module.\n\nThe caller must provide payout destination (target_chain_id,target_token,beneficiary) and duration_seconds. Optionally, provide receiver_salt and lessee.\n\nThe (target_chain_id,target_token) pair must exist in the current bridger routing table (hub_bridgers with valid_to_seq is null). If lessee is provided, arbitrary_lessee_flat_fee is added to the flat fee.",
    request_body = CreateLeaseRequest,
    responses(
        (status = 200, description = "OK", body = CreateLeaseResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 409, description = "Conflict", body = ErrorResponse),
        (status = 429, description = "Too many requests", body = ErrorResponse),
        (status = 502, description = "Upstream error", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
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

        if req.duration_seconds == 0 {
            return Err(ApiError::BadRequest(
                "duration_seconds must be non-zero".to_string(),
            ));
        }
        if offer.max_duration_seconds != 0 && req.duration_seconds > offer.max_duration_seconds {
            return Err(ApiError::BadRequest(format!(
                "duration_seconds exceeds realtor max_duration_seconds: duration_seconds={} max_duration_seconds={}",
                req.duration_seconds, offer.max_duration_seconds
            )));
        }

        let lessee_specified = req.lessee.is_some();
        let lessee: Address = match req.lessee.as_deref() {
            None => Address::ZERO,
            Some(s) => s
                .parse()
                .map_err(|_| ApiError::BadRequest("lessee: invalid address".to_string()))?,
        };
        let target_token: Address = req
            .target_token
            .parse()
            .map_err(|_| ApiError::BadRequest("target_token: invalid address".to_string()))?;
        let beneficiary: Address = req
            .beneficiary
            .parse()
            .map_err(|_| ApiError::BadRequest("beneficiary: invalid address".to_string()))?;
        if req.target_chain_id == 0 {
            return Err(ApiError::BadRequest(
                "target_chain_id must be non-zero".to_string(),
            ));
        }
        let target_token_hex_lower = format!("{:#x}", target_token).to_lowercase();
        let pair_supported = state
            .indexer
            .bridger_pair_is_supported(&target_token_hex_lower, req.target_chain_id)
            .await
            .map_err(|e| {
                ApiError::Upstream(format!("indexer hub_bridgers by pair: {e}"))
            })?;
        if !pair_supported {
            return Err(ApiError::BadRequest(format!(
                "unsupported target_token/target_chain_id pair (no bridger configured): target_token={target_token_hex_lower} target_chain_id={}",
                req.target_chain_id
            )));
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
            None => pick_receiver_salt_for_beneficiary(&state, now, beneficiary)
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

        let nukeable_after = now.saturating_add(req.duration_seconds);
        let pair_additional_flat_fee = state
            .cfg
            .leasing
            .pair_additional_flat_fees
            .get(&(req.target_chain_id, target_token))
            .copied()
            .unwrap_or(0);
        let lessee_additional_flat_fee = if lessee_specified {
            state.cfg.leasing.arbitrary_lessee_flat_fee
        } else {
            0
        };
        let effective_flat_fee = offer
            .effective_flat_fee
            .saturating_add(pair_additional_flat_fee)
            .saturating_add(lessee_additional_flat_fee);

        tracing::info!(
            receiver_salt = %receiver_salt_hex,
            nukeable_after,
            lease_fee_ppm = offer.effective_fee_ppm,
            flat_fee = effective_flat_fee,
            "submitting createLease userop"
        );

        let call = UntronV3::createLeaseCall {
            receiverSalt: receiver_salt,
            lessee,
            nukeableAfter: nukeable_after,
            leaseFeePpm: offer.effective_fee_ppm,
            flatFee: effective_flat_fee,
            targetChainId: U256::from(req.target_chain_id),
            targetToken: target_token,
            beneficiary,
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
