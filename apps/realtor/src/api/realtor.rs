use super::lease_terms::resolve_lease_terms;
use super::offer::compute_offer;
use super::receiver_salt::{
    ensure_receiver_is_free, normalize_receiver_salt_hex, pick_receiver_salt_for_beneficiary,
    pick_receiver_salt_from_preknown,
};
use super::userop::send_userop;
use super::{
    ApiError, CreateLeaseRequest, CreateLeaseResponse, ErrorResponse, RealtorInfoResponse,
    RealtorTargetPairResponse,
};
use crate::util::parse_bytes32;
use crate::{AppState, now_unix_seconds};
use alloy::primitives::Address;
use alloy::primitives::U256;
use alloy::sol_types::SolCall;
use axum::http::HeaderMap;
use axum::{Json, extract::State};
use std::sync::Arc;
use std::time::Instant;
use untron_v3_bindings::untron_v3::UntronV3;

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
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RealtorInfoResponse>, ApiError> {
    let start = Instant::now();

    let result: Result<_, ApiError> = async {
        let now = now_unix_seconds().map_err(ApiError::Internal)?;
        let terms = resolve_lease_terms(&state, &headers)?;
        let offer = compute_offer(&state, terms.defaults, now).await?;
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
            let pair_additional_flat_fee = terms
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
            realtor_address: format!(
                "{}",
                state
                    .cfg
                    .hub
                    .safe
                    .expect("hub safe must be resolved at startup")
                    .to_checksum_buffer(None)
            ),
            untron_v3: state.cfg.hub.untron_v3.to_checksum_buffer(None).to_string(),
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
            arbitrary_lessee_flat_fee: terms.arbitrary_lessee_flat_fee,
        }))
    }
    .await;

    let ms = start.elapsed().as_millis() as u64;
    match &result {
        Ok(_) => state.telemetry.http_ok("GET", "get_realtor", 200, ms),
        Err(e) => {
            state
                .telemetry
                .http_err("GET", "get_realtor", e.kind(), e.status_code().as_u16(), ms)
        }
    }
    result
}

#[utoipa::path(
    post,
    path = "/realtor",
    tag = "realtor",
    summary = "Create an address lease in Untron V3 protocol.",
    description = "Creates a lease on Untron V3 through the API's realtor.\n\nThe caller must provide payout destination (target_chain_id,target_token,beneficiary) and duration_seconds. Optionally, provide receiver_salt and lessee.\n\nThe (target_chain_id,target_token) pair must exist in the current bridger routing table (hub_bridgers with valid_to_seq is null). If lessee is provided, arbitrary_lessee_flat_fee is added to the flat fee.",
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
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateLeaseRequest>,
) -> Result<Json<CreateLeaseResponse>, ApiError> {
    let start = Instant::now();

    let receiver_salt_provided = req.receiver_salt.is_some();
    tracing::info!(receiver_salt_provided, "create_lease request");

    let result: Result<_, ApiError> = async {
        let now = now_unix_seconds().map_err(ApiError::Internal)?;
        let terms = resolve_lease_terms(&state, &headers)?;
        let offer = compute_offer(&state, terms.defaults, now).await?;

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
        let target_token_checksum = target_token.to_checksum_buffer(None).to_string();
        let pair_supported = state
            .indexer
            .bridger_pair_is_supported(&target_token_checksum, req.target_chain_id)
            .await
            .map_err(|e| ApiError::Upstream(format!("indexer hub_bridgers by pair: {e}")))?;
        if !pair_supported {
            return Err(ApiError::BadRequest(format!(
                "unsupported target_token/target_chain_id pair (no bridger configured): target_token={target_token_checksum} target_chain_id={}",
                req.target_chain_id
            )));
        }

        let receiver_salt_hex = match req.receiver_salt.as_deref() {
            Some(s) => {
                let receiver_salt_hex = normalize_receiver_salt_hex(s)?;
                let exists_in_candidates = state
                    .indexer
                    .receiver_salt_candidate(receiver_salt_hex.as_str())
                    .await
                    .map_err(|e| {
                        ApiError::Upstream(format!("indexer receiver_salt_candidates: {e}"))
                    })?;
                let is_preknown = state
                    .cfg
                    .leasing
                    .preknown_receiver_salts
                    .iter()
                    .any(|v| v == &receiver_salt_hex);
                if exists_in_candidates.is_none() && !is_preknown {
                    return Err(ApiError::BadRequest(format!(
                        "unknown receiver_salt (not found in indexer receiver_salt_candidates): {receiver_salt_hex}"
                    )));
                }
                receiver_salt_hex
            }
            None => match pick_receiver_salt_for_beneficiary(&state, now, beneficiary).await? {
                Some(s) => s,
                None => pick_receiver_salt_from_preknown(&state, now).await?.ok_or_else(|| {
                    ApiError::Conflict(
                        "no free receiver salts available (all are leased)".to_string(),
                    )
                })?,
            },
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
        let pair_additional_flat_fee = terms
            .pair_additional_flat_fees
            .get(&(req.target_chain_id, target_token))
            .copied()
            .unwrap_or(0);
        let lessee_additional_flat_fee = if lessee_specified {
            terms.arbitrary_lessee_flat_fee
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
            nukeable_after,
        }))
    }
    .await;

    let ms = start.elapsed().as_millis() as u64;
    match &result {
        Ok(_) => state.telemetry.http_ok("POST", "post_realtor", 200, ms),
        Err(e) => state.telemetry.http_err(
            "POST",
            "post_realtor",
            e.kind(),
            e.status_code().as_u16(),
            ms,
        ),
    }
    result
}
