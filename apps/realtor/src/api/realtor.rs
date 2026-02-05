#[allow(unused_imports)]
use super::ErrorResponse;
use super::lease_terms::resolve_lease_terms;
use super::offer::compute_offer;
use super::receiver_salt::{
    ensure_receiver_is_free, normalize_receiver_salt_hex, pick_receiver_salt_for_beneficiary,
    pick_receiver_salt_random_free, should_skip_known_receiver_salts,
};
use super::userop::send_userop;
use super::{
    ApiError, CreateLeaseRequest, CreateLeaseResponse, RealtorInfoResponse,
    RealtorTargetPairResponse,
};
use crate::util::parse_bytes32;
use crate::{AppState, now_unix_seconds};
use alloy::primitives::Address;
use alloy::primitives::U256;
use alloy::sol_types::SolCall;
use axum::http::HeaderMap;
use axum::{Json, extract::State};
use serde_json::Value;
use std::sync::Arc;
use std::time::Instant;
use untron_v3_bindings::untron_v3::UntronV3;

#[utoipa::path(
    get,
    path = "/realtor",
    tag = "realtor",
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
/// Fetch realtor terms and supported pairs.
pub async fn get_realtor(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RealtorInfoResponse>, ApiError> {
    let start = Instant::now();

    let result: Result<_, ApiError> = async {
        let now = now_unix_seconds().map_err(ApiError::Internal)?;
        let user = headers
            .get("x-untron-principal-id")
            .and_then(|v| v.to_str().ok())
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(str::to_string);
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
            user,
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
/// Create an address lease in Untron V3 protocol.
pub async fn post_realtor(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateLeaseRequest>,
) -> Result<Json<CreateLeaseResponse>, ApiError> {
    let start = Instant::now();

    let audit_ctx = crate::audit::AuditContext::from_headers(&headers);
    let audit_req_body: Option<Value> = serde_json::to_value(&req).ok();

    let receiver_salt_provided = req.receiver_salt.is_some();
    tracing::info!(receiver_salt_provided, "create_lease request");
    let req_start = Instant::now();

    let result: Result<_, ApiError> = async {
        let now = now_unix_seconds().map_err(ApiError::Internal)?;

        let t_terms = Instant::now();
        let terms = resolve_lease_terms(&state, &headers)?;
        tracing::info!(ms = t_terms.elapsed().as_millis() as u64, "post_realtor: resolved lease terms");

        let t_offer = Instant::now();
        let offer = compute_offer(&state, terms.defaults, now).await?;
        tracing::info!(ms = t_offer.elapsed().as_millis() as u64, allowed = offer.allowed, "post_realtor: computed offer");

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
        let t_pair = Instant::now();
        let pair_supported = state
            .indexer
            .bridger_pair_is_supported(&target_token_checksum, req.target_chain_id)
            .await
            .map_err(|e| ApiError::Upstream(format!("indexer hub_bridgers by pair: {e}")))?;
        tracing::info!(
            ms = t_pair.elapsed().as_millis() as u64,
            target_chain_id = req.target_chain_id,
            target_token = %target_token_checksum,
            pair_supported,
            "post_realtor: checked bridger pair support"
        );
        if !pair_supported {
            return Err(ApiError::BadRequest(format!(
                "unsupported target_token/target_chain_id pair (no bridger configured): target_token={target_token_checksum} target_chain_id={}",
                req.target_chain_id
            )));
        }

        let t_salt = Instant::now();
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
            None => {
                if should_skip_known_receiver_salts(req.duration_seconds) {
                    tracing::info!(
                        duration_seconds = req.duration_seconds,
                        "duration > 1 day; skipping known receiver salts and selecting random salt"
                    );
                    pick_receiver_salt_random_free(&state, now).await?
                } else {
                    match pick_receiver_salt_for_beneficiary(&state, now, beneficiary).await? {
                        Some(s) => s,
                        None => pick_receiver_salt_random_free(&state, now).await?,
                    }
                }
            }
        };

        tracing::info!(ms = t_salt.elapsed().as_millis() as u64, receiver_salt = %receiver_salt_hex, "post_realtor: selected receiver salt");

        let t_free = Instant::now();
        ensure_receiver_is_free(&state, &receiver_salt_hex, now).await?;
        tracing::info!(ms = t_free.elapsed().as_millis() as u64, receiver_salt = %receiver_salt_hex, "post_realtor: ensured receiver is free");

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

        let t_lock = Instant::now();
        let mut sender = state.sender.lock().await;
        tracing::info!(ms = t_lock.elapsed().as_millis() as u64, "post_realtor: acquired sender lock");

        let t_userop = Instant::now();
        let (userop_hash, nonce, send_attempts) = send_userop(
            &mut sender,
            state.cfg.hub.untron_v3,
            data,
            state.cfg.hub.bundler_timeout,
        )
        .await?;
        tracing::info!(
            ms = t_userop.elapsed().as_millis() as u64,
            %userop_hash,
            %nonce,
            send_attempts,
            "post_realtor: send_userop finished"
        );

        state.telemetry.userop_sent();
        state
            .telemetry
            .userop_send_retries(send_attempts.saturating_sub(1));
        state.telemetry.lease_created();

        tracing::info!(%userop_hash, %nonce, "lease userop submitted");

        tracing::info!(ms = req_start.elapsed().as_millis() as u64, "post_realtor: completed request successfully (inner)");

        // Resolve global lease id before returning.
        //
        // Strategy:
        //  1) Prefer bundler receipt (eth_getUserOperationReceipt) and parse the LeaseCreated event.
        //  2) Fallback to indexer latest lease by receiver_salt, waiting until it matches this request's nukeable_after.
        let lease_id: u64 = {
            use alloy::sol_types::SolEventInterface;
            use untron_v3_bindings::r#untron_v3::UntronV3::UntronV3Events;

            const RECEIPT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(45);

            fn lease_id_from_receipt(
                state: &AppState,
                receiver_salt_hex: &str,
                expected_nukeable_after: u64,
                receipt: &alloy::rpc::types::eth::erc4337::UserOperationReceipt,
            ) -> Option<u64> {
                let contract = state.cfg.hub.untron_v3;

                for log in &receipt.logs {
                    if log.address() != contract {
                        continue;
                    }
                    if log.topics().is_empty() {
                        continue;
                    }

                    let ev = match UntronV3Events::decode_raw_log(log.topics(), log.data().data.as_ref()) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                    match ev {
                        UntronV3Events::LeaseCreated(inner) => {
                            let salt_hex = format!("0x{}", hex::encode(inner.receiverSalt.0));
                            if salt_hex.to_lowercase() != receiver_salt_hex.to_lowercase() {
                                continue;
                            }
                            if inner.nukeableAfter != expected_nukeable_after {
                                continue;
                            }
                            let lease_id_u64 = u64::try_from(inner.leaseId).ok()?;
                            if lease_id_u64 == 0 {
                                continue;
                            }
                            return Some(lease_id_u64);
                        }
                        _ => continue,
                    }
                }
                None
            }

            // (1) Receipt path (do not hold sender lock)
            match aa::wait_user_operation_receipt(
                state.cfg.hub.bundler_urls.clone(),
                &userop_hash,
                RECEIPT_TIMEOUT,
            )
            .await
            {
                Ok(receipt) => {
                    if let Some(id) = lease_id_from_receipt(&state, &receiver_salt_hex, nukeable_after, &receipt) {
                        id
                    } else {
                        tracing::warn!(%userop_hash, receiver_salt = %receiver_salt_hex, "userop receipt did not contain matching LeaseCreated event; falling back to indexer");
                        0
                    }
                }
                Err(e) => {
                    tracing::warn!(%userop_hash, receiver_salt = %receiver_salt_hex, err = %format!("{e:#}"), "failed to fetch userop receipt; falling back to indexer");
                    0
                }
            }
        };

        let lease_id: u64 = if lease_id != 0 {
            lease_id
        } else {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(45);
            let mut backoff = std::time::Duration::from_millis(250);
            loop {
                let latest = state
                    .indexer
                    .latest_lease_by_receiver_salt(receiver_salt_hex.as_str())
                    .await
                    .map_err(|e| {
                        ApiError::Upstream(format!(
                            "indexer hub_leases latest by receiver_salt (fallback): {e}"
                        ))
                    })?;

                if let Some(row) = latest {
                    if let (Some(id), Some(nukeable)) = (row.lease_id, row.nukeable_after) {
                        let id_u64 = id
                            .as_u64()
                            .or_else(|| id.as_i64().and_then(|v| u64::try_from(v).ok()))
                            .or_else(|| id.to_string().parse::<u64>().ok());
                        if id_u64.is_some() && u64::try_from(nukeable).ok() == Some(nukeable_after) {
                            break id_u64.unwrap_or(0);
                        }
                    }
                }

                if std::time::Instant::now() >= deadline {
                    return Err(ApiError::Upstream(
                        "timed out waiting for indexer to surface newly-created lease".to_string(),
                    ));
                }

                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(std::time::Duration::from_secs(2));
            }
        };

        // Derive receiver addresses without depending on indexer state.
        // This avoids races when clients immediately need the deposit address after lease creation.
        let (receiver_address_tron, receiver_address_evm) = {
            use crate::util::compute_create2_address;
            use crate::util::parse_bytes32;
            use alloy::eips::BlockId;
            use alloy::primitives::{B256, keccak256};
            use alloy::providers::{DynProvider, Provider, ProviderBuilder};
            use alloy::rpc::client::{BuiltInConnectionString, RpcClient};
            use alloy::sol_types::SolCall;
            use tron::TronAddress;
            use untron_v3_bindings::untron_controller::UntronController;

            async fn fetch_receiver_init_code_hash(
                tron_rpc_url: &str,
                controller: alloy::primitives::Address,
            ) -> Result<B256, ApiError> {
                let transport = BuiltInConnectionString::connect(tron_rpc_url)
                    .await
                    .map_err(|e| ApiError::Upstream(format!("connect tron rpc: {e}")))?;
                let client = RpcClient::builder().transport(transport, false);
                let provider: DynProvider =
                    DynProvider::new(ProviderBuilder::default().connect_client(client));

                let contract = UntronController::new(controller, provider.clone());
                let call = contract.receiverBytecode();

                // Tron JSON-RPC accepts `data` but may reject `input` (and may even error if both are present).
                // Alloy defaults to `input`, so normalize into `data`-only.
                let request = call.clone().into_transaction_request().normalized_data();
                let return_data = provider
                    .call(request)
                    .block(BlockId::latest())
                    .await
                    .map_err(|e| ApiError::Upstream(format!("eth_call(receiverBytecode): {e}")))?;

                if return_data.is_empty() {
                    return Ok(B256::ZERO);
                }
                let decoded =
                    <UntronController::receiverBytecodeCall as SolCall>::abi_decode_returns(
                        return_data.as_ref(),
                    )
                    .map_err(|e| {
                        ApiError::Upstream(format!("decode receiverBytecode() return: {e}"))
                    })?;
                if decoded.is_empty() {
                    return Ok(B256::ZERO);
                }
                Ok(keccak256(decoded))
            }

            let maybe = (|| {
                let tron_rpc_url = state.cfg.tron_rpc_url.as_deref()?;
                let controller = state.cfg.hub.controller_address?;
                let salt = parse_bytes32(&receiver_salt_hex).ok()?;
                Some((tron_rpc_url, controller, salt))
            })();

            if let Some((tron_rpc_url, controller, salt)) = maybe {
                let init_code_hash = state
                    .tron_receiver_init_code_hash
                    .get_or_try_init(|| fetch_receiver_init_code_hash(tron_rpc_url, controller))
                    .await
                    .ok()
                    .copied()
                    .unwrap_or(B256::ZERO);
                if init_code_hash == B256::ZERO {
                    (None, None)
                } else {
                    let receiver_evm = compute_create2_address(
                        TronAddress::MAINNET_PREFIX,
                        controller,
                        salt,
                        init_code_hash,
                    );
                    let receiver_evm_str = receiver_evm.to_checksum_buffer(None).to_string();
                    let receiver_tron = TronAddress::from_evm(receiver_evm).to_string();
                    (Some(receiver_tron), Some(receiver_evm_str))
                }
            } else {
                (None, None)
            }
        };

        Ok(Json(CreateLeaseResponse {
            receiver_salt: receiver_salt_hex,
            receiver_address_tron,
            receiver_address_evm,
            userop_hash,
            lease_id,
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

    if let Some(audit_db) = state.audit_db.clone() {
        let response_body = match &result {
            Ok(Json(resp)) => serde_json::to_value(resp).ok(),
            Err(_) => None,
        };
        let (status_code, error_kind, error_message) = match &result {
            Ok(_) => (200u16, None, None),
            Err(e) => (
                e.status_code().as_u16(),
                Some(e.kind()),
                Some(e.message().to_string()),
            ),
        };
        let entry = crate::audit::WriteAction {
            request_id: audit_ctx.request_id,
            principal_id: audit_ctx.principal_id,
            remote_ip: audit_ctx.remote_ip,
            user_agent: audit_ctx.user_agent,
            action: "create_lease",
            method: "POST",
            path: "/realtor",
            status_code,
            duration_ms: ms,
            error_kind,
            error_message,
            request_body: audit_req_body,
            response_body,
        };
        tokio::spawn(async move {
            if let Err(e) = audit_db.insert_write_action(entry).await {
                tracing::warn!(err = %e, "audit insert failed");
            }
        });
    }
    result
}
