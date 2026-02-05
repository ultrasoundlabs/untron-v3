#[allow(unused_imports)]
use super::ErrorResponse;
use super::userop::send_userop;
use super::{ApiError, SetPayoutConfigRequest, SetPayoutConfigResponse};
use crate::AppState;
use crate::util::{number_to_u64, parse_hex_bytes};
use alloy::primitives::{Address, B256, Signature, U256, keccak256};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::client::{BuiltInConnectionString, RpcClient};
use alloy::sol_types::{SolCall, SolStruct};
use axum::http::HeaderMap;
use axum::{Json, extract::State};
use serde_json::Value;
use std::sync::Arc;
use std::time::Instant;
use untron_v3_bindings::untron_v3::{UntronV3, UntronV3Base};

alloy::sol! {
    interface IERC1271 {
        function isValidSignature(bytes32 hash, bytes signature) external view returns (bytes4);
    }

    /// EIP-712 struct hashed by UntronV3 in `setPayoutConfigWithSig`.
    struct PayoutConfigUpdate {
        uint256 leaseId;
        uint256 targetChainId;
        address targetToken;
        address beneficiary;
        uint256 nonce;
        uint256 deadline;
    }
}

#[utoipa::path(
    post,
    path = "/payout_config",
    tag = "realtor",
    request_body = SetPayoutConfigRequest,
    responses(
        (status = 200, description = "OK", body = SetPayoutConfigResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 409, description = "Conflict", body = ErrorResponse),
        (status = 429, description = "Too many requests", body = ErrorResponse),
        (status = 502, description = "Upstream error", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
/// Relay a gasless payout config update.
pub async fn post_payout_config(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(req): Json<SetPayoutConfigRequest>,
) -> Result<Json<SetPayoutConfigResponse>, ApiError> {
    let start = Instant::now();

    let audit_ctx = crate::audit::AuditContext::from_headers(&headers);
    let audit_req_body: Option<Value> = serde_json::to_value(&req).ok().map(|mut v| {
        if let Value::Object(m) = &mut v {
            m.insert(
                "signature".to_string(),
                Value::String("<redacted>".to_string()),
            );
        }
        v
    });

    let result: Result<_, ApiError> = async {
        if req.lease_id == 0 {
            return Err(ApiError::BadRequest("lease_id must be non-zero".to_string()));
        }
        if req.target_chain_id == 0 {
            return Err(ApiError::BadRequest(
                "target_chain_id must be non-zero".to_string(),
            ));
        }
        if req.deadline == 0 {
            return Err(ApiError::BadRequest("deadline must be non-zero".to_string()));
        }

        let target_token: Address = req
            .target_token
            .parse()
            .map_err(|_| ApiError::BadRequest("target_token: invalid address".to_string()))?;
        if target_token == Address::ZERO {
            return Err(ApiError::BadRequest(
                "target_token must be non-zero".to_string(),
            ));
        }
        let beneficiary: Address = req
            .beneficiary
            .parse()
            .map_err(|_| ApiError::BadRequest("beneficiary: invalid address".to_string()))?;

        let signature_bytes = parse_hex_bytes(&req.signature)
            .map_err(|e| ApiError::BadRequest(format!("signature: {e}")))?;
        if signature_bytes.is_empty() {
            return Err(ApiError::BadRequest("signature must be non-empty".to_string()));
        }

        let lease = state
            .indexer
            .hub_lease(req.lease_id)
            .await
            .map_err(|e| ApiError::Upstream(format!("indexer hub_leases: {e}")))?;
        let Some(lease) = lease else {
            return Err(ApiError::BadRequest(format!(
                "unknown lease_id (not found in indexer hub_leases): {}",
                req.lease_id
            )));
        };

        let expected_realtor = format!(
            "{:#x}",
            state
                .cfg
                .hub
                .safe
                .expect("hub safe must be resolved at startup")
        )
        .to_lowercase();
        let lease_realtor = lease
            .realtor
            .ok_or_else(|| ApiError::Upstream("indexer hub_leases missing realtor".to_string()))?
            .to_lowercase();
        if lease_realtor != expected_realtor {
            return Err(ApiError::Forbidden(format!(
                "lease not owned by this realtor: lease_realtor={lease_realtor}"
            )));
        }

        let lessee: Address = lease
            .lessee
            .ok_or_else(|| ApiError::Upstream("indexer hub_leases missing lessee".to_string()))?
            .parse()
            .map_err(|_| ApiError::Upstream("indexer hub_leases invalid lessee address".to_string()))?;

        let lease_nonce_row = state
            .indexer
            .hub_lease_nonce(req.lease_id)
            .await
            .map_err(|e| ApiError::Upstream(format!("indexer hub_lease_nonces: {e}")))?;
        let lease_nonce: u64 = match lease_nonce_row.and_then(|r| r.nonce) {
            None => 0,
            Some(n) => number_to_u64(&n, "lease nonce")
                .map_err(|e| ApiError::Upstream(format!("indexer hub_lease_nonces nonce: {e}")))?,
        };

        if let Some(chain) = state
            .indexer
            .hub_chain(req.target_chain_id)
            .await
            .map_err(|e| ApiError::Upstream(format!("indexer hub_chains: {e}")))? && chain.deprecated == Some(true) {
                return Err(ApiError::BadRequest("target chain is deprecated".to_string()));
        }

        let protocol_cfg = state
            .indexer
            .hub_protocol_config()
            .await
            .map_err(|e| ApiError::Upstream(format!("indexer hub_protocol_config: {e}")))?;
        let protocol_cfg = protocol_cfg
            .ok_or_else(|| ApiError::Upstream("indexer hub_protocol_config missing".to_string()))?;
        let usdt: Address = protocol_cfg
            .usdt
            .ok_or_else(|| ApiError::Upstream("indexer hub_protocol_config missing usdt".to_string()))?
            .parse()
            .map_err(|_| ApiError::Upstream("indexer hub_protocol_config invalid usdt address".to_string()))?;

        let target_token_checksum = target_token.to_checksum_buffer(None).to_string();

        if target_token != usdt {
            let swap_rate = state
                .indexer
                .hub_swap_rate(&target_token_checksum)
                .await
                .map_err(|e| ApiError::Upstream(format!("indexer hub_swap_rates: {e}")))?;
            let rate_ppm = swap_rate.and_then(|r| r.rate_ppm).unwrap_or(0);
            if rate_ppm == 0 {
                return Err(ApiError::BadRequest(format!(
                    "no swap rate configured for target_token: {target_token_checksum}"
                )));
            }
        }

        if state.cfg.hub.chain_id.map(|id| id != req.target_chain_id).unwrap_or(true) {
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
        }

        let chain_id = state.cfg.hub.chain_id.ok_or_else(|| {
            ApiError::Internal(
                "HUB_CHAIN_ID must be set to validate payout config signatures".to_string(),
            )
        })?;

        let domain = alloy::sol_types::eip712_domain! {
            name: "Untron",
            version: "1",
            chain_id: chain_id,
            verifying_contract: state.cfg.hub.untron_v3,
        };

        let update = PayoutConfigUpdate {
            leaseId: U256::from(req.lease_id),
            targetChainId: U256::from(req.target_chain_id),
            targetToken: target_token,
            beneficiary,
            nonce: U256::from(lease_nonce),
            deadline: U256::from(req.deadline),
        };
        let struct_hash: B256 = update.eip712_hash_struct();
        let domain_separator: B256 = domain.separator();

        let mut preimage = [0u8; 66];
        preimage[0] = 0x19;
        preimage[1] = 0x01;
        preimage[2..34].copy_from_slice(domain_separator.as_slice());
        preimage[34..66].copy_from_slice(struct_hash.as_slice());
        let digest = keccak256(preimage);

        let recovered = if signature_bytes.len() == 65 {
            Signature::try_from(signature_bytes.as_slice())
                .ok()
                .and_then(|sig| sig.recover_address_from_prehash(&digest).ok())
        } else {
            None
        };

        let signature_ok = match recovered {
            Some(addr) if addr == lessee => true,
            _ => {
                // If the lessee is a contract, try ERC-1271.
                let transport = BuiltInConnectionString::connect(&state.cfg.hub.rpc_url)
                    .await
                    .map_err(|e| ApiError::Upstream(format!("connect rpc: {e}")))?;
                let client = RpcClient::builder().transport(transport, false);
                let provider: DynProvider =
                    DynProvider::new(ProviderBuilder::default().connect_client(client));

                let code = provider
                    .get_code_at(lessee)
                    .await
                    .map_err(|e| ApiError::Upstream(format!("eth_getCode lessee: {e}")))?;
                if code.is_empty() {
                    false
                } else {
                    const MAGIC: [u8; 4] = [0x16, 0x26, 0xba, 0x7e];
                    let call = IERC1271::isValidSignatureCall {
                        hash: digest,
                        signature: signature_bytes.clone().into(),
                    };
                    let data = call.abi_encode();
                    let tx = alloy::rpc::types::eth::transaction::TransactionRequest {
                        to: Some(lessee.into()),
                        input: alloy::rpc::types::eth::transaction::TransactionInput::new(
                            data.into(),
                        ),
                        ..Default::default()
                    };
                    let out = provider.call(tx).await.map_err(|e| {
                        ApiError::Upstream(format!("eth_call isValidSignature: {e}"))
                    })?;
                    out.get(0..4) == Some(&MAGIC)
                }
            }
        };

        if !signature_ok {
            return Err(ApiError::BadRequest(
                "invalid signature for current lessee".to_string(),
            ));
        }

        let call = UntronV3::setPayoutConfigWithSigCall {
            leaseId: U256::from(req.lease_id),
            config: UntronV3Base::PayoutConfig {
                targetChainId: U256::from(req.target_chain_id),
                targetToken: target_token,
                beneficiary,
            },
            deadline: U256::from(req.deadline),
            signature: signature_bytes.into(),
        };
        let data = call.abi_encode();

        let mut sender = state.sender.lock().await;
        let (userop_hash, _nonce, send_attempts) = send_userop(
            &mut sender,
            state.cfg.hub.untron_v3,
            data,
            state.cfg.hub.bundler_timeout,
        )
        .await?;

        state.telemetry.userop_sent();
        state
            .telemetry
            .userop_send_retries(send_attempts.saturating_sub(1));

        Ok(Json(SetPayoutConfigResponse { userop_hash }))
    }
    .await;

    let ms = start.elapsed().as_millis() as u64;
    match &result {
        Ok(_) => state
            .telemetry
            .http_ok("POST", "post_payout_config", 200, ms),
        Err(e) => state.telemetry.http_err(
            "POST",
            "post_payout_config",
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
            action: "set_payout_config",
            method: "POST",
            path: "/payout_config",
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
