use alloy::{
    primitives::{Address, Bytes},
    providers::{DynProvider, Provider, ProviderBuilder},
    rpc::client::{BuiltInConnectionString, RpcClient},
};
use alloy_provider::ext::Erc4337Api;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use alloy::primitives::U256;
use alloy::rpc::types::eth::erc4337::PackedUserOperation;
use alloy::rpc::types::eth::erc4337::SendUserOperationResponse;
use alloy::rpc::types::eth::erc4337::UserOperationReceipt;

use crate::packing::{pack_init_code, pack_paymaster_and_data, redact_url};

const RPC_TIMEOUT: Duration = Duration::from_secs(10);

fn looks_like_packed_userop_unsupported(err: &anyhow::Error) -> bool {
    let s = format!("{err:#}");
    // Observed from Pimlico (and similar) when we send packed v0.7 fields:
    // - `Unrecognized keys: "accountGasLimits", "gasFees", "paymasterAndData"`
    // - and/or complaints that `maxFeePerGas` is undefined (because it expects expanded fields)
    (s.contains("Unrecognized keys")
        && (s.contains("accountGasLimits") || s.contains("gasFees") || s.contains("paymasterAndData")))
        || (s.contains("maxFeePerGas") && s.contains("received undefined"))
}

fn looks_like_wrapped_params_expected(err: &anyhow::Error) -> bool {
    let s = format!("{err:#}");
    // Pimlico sometimes validates against a schema where params[0] is an object:
    //   { userOp: <op>, entryPoint: <addr> }
    // and errors come back pointing at `params[0].userOp.*`.
    s.contains("params[0].userOp")
}

/// Canonical EntryPoint v0.7 "packed" JSON shape.
///
/// Some bundlers/paymasters only accept the expanded field set (`callGasLimit`, `paymaster`, ...),
/// while others accept the canonical packed form (`accountGasLimits`, `gasFees`,
/// `paymasterAndData`).
///
/// The Safe4337 module validates signatures against the *packed* `paymasterAndData` bytes.
/// When a bundler reconstructs those bytes from expanded fields, any packing mismatch can
/// surface as `AA34 signature error`.
///
/// We *prefer* sending the canonical packed form (eliminates bundler-side packing ambiguity),
/// but we fall back to the expanded form for bundlers that don't support packed userops.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct RpcPackedUserOperationV07 {
    sender: Address,
    nonce: U256,
    init_code: Bytes,
    call_data: Bytes,
    account_gas_limits: Bytes,
    pre_verification_gas: U256,
    gas_fees: Bytes,
    paymaster_and_data: Bytes,
    signature: Bytes,
}

/// EntryPoint v0.7 expanded userop JSON shape.
///
/// Many bundlers (incl. some Pimlico endpoints) accept only this shape for
/// `eth_estimateUserOperationGas` / `eth_sendUserOperation`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct RpcUserOperationV07Expanded {
    sender: Address,
    nonce: U256,

    // v0.7 expanded shape uses factory + factoryData instead of initCode.
    #[serde(skip_serializing_if = "Option::is_none")]
    factory: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    factory_data: Option<Bytes>,

    call_data: Bytes,
    call_gas_limit: U256,
    verification_gas_limit: U256,
    pre_verification_gas: U256,
    max_fee_per_gas: U256,
    max_priority_fee_per_gas: U256,

    #[serde(skip_serializing_if = "Option::is_none")]
    paymaster: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paymaster_verification_gas_limit: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paymaster_post_op_gas_limit: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paymaster_data: Option<Bytes>,

    signature: Bytes,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct RpcUserOpAndEntryPoint<T> {
    user_op: T,
    entry_point: Address,
}

fn pack_u128_pair_be(
    a: U256,
    b: U256,
    label_a: &'static str,
    label_b: &'static str,
) -> Result<Bytes> {
    let a: u128 = u128::try_from(a).with_context(|| format!("{label_a} overflows uint128"))?;
    let b: u128 = u128::try_from(b).with_context(|| format!("{label_b} overflows uint128"))?;

    let mut out = [0u8; 32];
    out[..16].copy_from_slice(&a.to_be_bytes());
    out[16..].copy_from_slice(&b.to_be_bytes());
    Ok(Bytes::from(out.to_vec()))
}

fn to_rpc_packed_v07(op: &PackedUserOperation) -> Result<RpcPackedUserOperationV07> {
    let init_code = Bytes::from(pack_init_code(op.factory, op.factory_data.as_ref())?);

    // EntryPoint v0.7 packs two uint128s in big-endian byte order.
    // accountGasLimits := verificationGasLimit (hi 16) || callGasLimit (lo 16)
    // gasFees         := maxPriorityFeePerGas (hi 16) || maxFeePerGas (lo 16)
    let account_gas_limits = pack_u128_pair_be(
        op.verification_gas_limit,
        op.call_gas_limit,
        "verificationGasLimit",
        "callGasLimit",
    )?;
    let gas_fees = pack_u128_pair_be(
        op.max_priority_fee_per_gas,
        op.max_fee_per_gas,
        "maxPriorityFeePerGas",
        "maxFeePerGas",
    )?;

    let paymaster_and_data = Bytes::from(pack_paymaster_and_data(
        op.paymaster,
        op.paymaster_verification_gas_limit,
        op.paymaster_post_op_gas_limit,
        op.paymaster_data.as_ref(),
    )?);

    Ok(RpcPackedUserOperationV07 {
        sender: op.sender,
        nonce: op.nonce,
        init_code,
        call_data: op.call_data.clone(),
        account_gas_limits,
        pre_verification_gas: op.pre_verification_gas,
        gas_fees,
        paymaster_and_data,
        signature: op.signature.clone(),
    })
}

fn to_rpc_expanded_v07(op: &PackedUserOperation) -> Result<RpcUserOperationV07Expanded> {
    Ok(RpcUserOperationV07Expanded {
        sender: op.sender,
        nonce: op.nonce,
        factory: op.factory,
        factory_data: op.factory_data.clone(),
        call_data: op.call_data.clone(),
        call_gas_limit: op.call_gas_limit,
        verification_gas_limit: op.verification_gas_limit,
        pre_verification_gas: op.pre_verification_gas,
        max_fee_per_gas: op.max_fee_per_gas,
        max_priority_fee_per_gas: op.max_priority_fee_per_gas,
        paymaster: op.paymaster,
        paymaster_verification_gas_limit: op.paymaster_verification_gas_limit,
        paymaster_post_op_gas_limit: op.paymaster_post_op_gas_limit,
        paymaster_data: op.paymaster_data.clone(),
        signature: op.signature.clone(),
    })
}

#[derive(Clone)]
pub struct BundlerPool {
    urls: Vec<String>,
    providers: Vec<DynProvider>,
    next_idx: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UserOperationGasEstimationV07 {
    pub pre_verification_gas: alloy::primitives::U256,
    pub verification_gas_limit: alloy::primitives::U256,
    pub call_gas_limit: alloy::primitives::U256,
    pub paymaster_verification_gas_limit: alloy::primitives::U256,
    pub paymaster_post_op_gas_limit: alloy::primitives::U256,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SendUserOperationResponseObj {
    user_op_hash: alloy::primitives::Bytes,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SendUserOperationResponseAny {
    // Canonical ERC-4337 response shape:
    //   { "userOpHash": "0x..." }
    Obj(SendUserOperationResponseObj),
    // Some bundlers (incl. Pimlico) return just the hash as a string.
    Hash(alloy::primitives::Bytes),
}

impl From<SendUserOperationResponseAny> for SendUserOperationResponse {
    fn from(v: SendUserOperationResponseAny) -> Self {
        match v {
            SendUserOperationResponseAny::Obj(v) => SendUserOperationResponse {
                user_op_hash: v.user_op_hash,
            },
            SendUserOperationResponseAny::Hash(user_op_hash) => {
                SendUserOperationResponse { user_op_hash }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EstimateV07 {
    pre_verification_gas: alloy::primitives::U256,
    verification_gas_limit: alloy::primitives::U256,
    call_gas_limit: alloy::primitives::U256,
    #[serde(default)]
    paymaster_verification_gas_limit: alloy::primitives::U256,
    #[serde(default)]
    paymaster_post_op_gas_limit: alloy::primitives::U256,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EstimateV06 {
    pre_verification_gas: alloy::primitives::U256,
    verification_gas: alloy::primitives::U256,
    paymaster_verification_gas: alloy::primitives::U256,
    paymaster_post_op_gas: alloy::primitives::U256,
    call_gas_limit: alloy::primitives::U256,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EstimateAny {
    V07(EstimateV07),
    V06(EstimateV06),
}

impl From<EstimateAny> for UserOperationGasEstimationV07 {
    fn from(v: EstimateAny) -> Self {
        match v {
            EstimateAny::V07(v) => Self {
                pre_verification_gas: v.pre_verification_gas,
                verification_gas_limit: v.verification_gas_limit,
                call_gas_limit: v.call_gas_limit,
                paymaster_verification_gas_limit: v.paymaster_verification_gas_limit,
                paymaster_post_op_gas_limit: v.paymaster_post_op_gas_limit,
            },
            EstimateAny::V06(v) => Self {
                pre_verification_gas: v.pre_verification_gas,
                verification_gas_limit: v.verification_gas,
                call_gas_limit: v.call_gas_limit,
                paymaster_verification_gas_limit: v.paymaster_verification_gas,
                paymaster_post_op_gas_limit: v.paymaster_post_op_gas,
            },
        }
    }
}

impl BundlerPool {
    pub async fn new(urls: Vec<String>) -> Result<Self> {
        if urls.is_empty() {
            anyhow::bail!("bundler urls must be non-empty");
        }

        let mut healthy_urls = Vec::with_capacity(urls.len());
        let mut providers = Vec::with_capacity(urls.len());
        let mut last_connect_err: Option<anyhow::Error> = None;
        for url in &urls {
            match tokio::time::timeout(RPC_TIMEOUT, BuiltInConnectionString::connect(url)).await {
                Ok(Ok(transport)) => {
                    let client = RpcClient::builder().transport(transport, false);
                    let provider = ProviderBuilder::default().connect_client(client);
                    providers.push(DynProvider::new(provider));
                    healthy_urls.push(url.clone());
                }
                Ok(Err(err)) => {
                    let redacted = redact_url(url);
                    tracing::warn!(bundler = %redacted, err = %err, "failed to connect bundler rpc endpoint");
                    last_connect_err = Some(
                        anyhow::Error::new(err).context(format!("connect bundler rpc: {redacted}")),
                    );
                }
                Err(_) => {
                    let redacted = redact_url(url);
                    let err = anyhow::anyhow!("timed out connecting bundler rpc: {redacted}");
                    tracing::warn!(bundler = %redacted, err = %err, "failed to connect bundler rpc endpoint");
                    last_connect_err = Some(err);
                }
            }
        }

        if providers.is_empty() {
            if let Some(err) = last_connect_err {
                return Err(err.context("all bundler rpc endpoints failed to connect"));
            }
            anyhow::bail!("all bundler rpc endpoints failed to connect");
        }

        Ok(Self {
            urls: healthy_urls,
            providers,
            next_idx: 0,
        })
    }

    fn mark_success(&mut self, idx: usize) {
        if !self.providers.is_empty() {
            self.next_idx = (idx + 1) % self.providers.len();
        }
    }

    pub(crate) async fn estimate_user_operation_gas(
        &mut self,
        user_op: &PackedUserOperation,
        entry_point: Address,
    ) -> Result<UserOperationGasEstimationV07> {
        let mut last_err: Option<anyhow::Error> = None;

        let order = rotate_order(self.next_idx, self.providers.len());
        for idx in order {
            let url = &self.urls[idx];
            let url = redact_url(url);
            let provider = &self.providers[idx];

            // Use a tolerant response type: many bundlers return v0.7 field names
            // (`verificationGasLimit`, `paymasterVerificationGasLimit`, ...), but alloy's
            // `UserOperationGasEstimation` type currently models v0.6 response names.
            let rpc_op = to_rpc_packed_v07(user_op)?;
            let fut =
                provider.raw_request("eth_estimateUserOperationGas".into(), (rpc_op, entry_point));

            match tokio::time::timeout(RPC_TIMEOUT, fut).await {
                Ok(Ok(v)) => {
                    let v: EstimateAny = v;
                    self.mark_success(idx);
                    return Ok(v.into());
                }
                Ok(Err(err)) => {
                    // Some bundlers (incl. certain Pimlico endpoints) reject packed v0.7 userops.
                    // Fall back to the expanded shape to avoid returning 502s from /realtor.
                    let err = anyhow::Error::new(err).context("eth_estimateUserOperationGas");
                    if looks_like_packed_userop_unsupported(&err) {
                        tracing::warn!(
                            bundler = %url,
                            err = %format!("{err:#}"),
                            "bundler rejected packed userop; retrying with expanded v0.7 shape"
                        );

                        let rpc_op = to_rpc_expanded_v07(user_op)?;
                        let fut = provider.raw_request(
                            "eth_estimateUserOperationGas".into(),
                            (rpc_op.clone(), entry_point),
                        );
                        match tokio::time::timeout(RPC_TIMEOUT, fut).await {
                            Ok(Ok(v)) => {
                                let v: EstimateAny = v;
                                self.mark_success(idx);
                                return Ok(v.into());
                            }
                            Ok(Err(err2)) => {
                                let err2 = anyhow::Error::new(err2)
                                    .context("eth_estimateUserOperationGas (expanded fallback)");

                                // Some endpoints appear to expect wrapped params:
                                //   [ { userOp, entryPoint } ]
                                if looks_like_wrapped_params_expected(&err2) {
                                    let wrapped = RpcUserOpAndEntryPoint {
                                        user_op: rpc_op,
                                        entry_point,
                                    };
                                    let fut = provider.raw_request(
                                        "eth_estimateUserOperationGas".into(),
                                        (wrapped,),
                                    );
                                    match tokio::time::timeout(RPC_TIMEOUT, fut).await {
                                        Ok(Ok(v)) => {
                                            let v: EstimateAny = v;
                                            self.mark_success(idx);
                                            return Ok(v.into());
                                        }
                                        Ok(Err(err3)) => {
                                            let err3 = anyhow::Error::new(err3).context(
                                                "eth_estimateUserOperationGas (expanded wrapped fallback)",
                                            );
                                            tracing::warn!(
                                                bundler = %url,
                                                err = %format!("{err3:#}"),
                                                "bundler rpc failed"
                                            );
                                            last_err = Some(err3);
                                        }
                                        Err(_) => {
                                            let err = anyhow::anyhow!("timed out");
                                            tracing::warn!(
                                                bundler = %url,
                                                err = %err,
                                                "bundler rpc timed out (eth_estimateUserOperationGas expanded wrapped fallback)"
                                            );
                                            last_err = Some(err);
                                        }
                                    }
                                } else {
                                    tracing::warn!(
                                        bundler = %url,
                                        err = %format!("{err2:#}"),
                                        "bundler rpc failed"
                                    );
                                    last_err = Some(err2);
                                }
                            }
                            Err(_) => {
                                let err = anyhow::anyhow!("timed out");
                                tracing::warn!(
                                    bundler = %url,
                                    err = %err,
                                    "bundler rpc timed out (eth_estimateUserOperationGas expanded fallback)"
                                );
                                last_err = Some(err);
                            }
                        }
                    } else {
                        tracing::warn!(
                            bundler = %url,
                            err = %format!("{err:#}"),
                            "bundler rpc failed"
                        );
                        last_err = Some(err);
                    }
                }
                Err(_) => {
                    let err = anyhow::anyhow!("timed out");
                    tracing::warn!(
                        bundler = %url,
                        err = %err,
                        "bundler rpc timed out (eth_estimateUserOperationGas)"
                    );
                    last_err = Some(err);
                }
            }
        }

        Err(last_err.unwrap_or_else(|| {
            anyhow::anyhow!("all bundlers failed for eth_estimateUserOperationGas")
        }))
    }

    pub(crate) async fn send_user_operation(
        &mut self,
        user_op: &PackedUserOperation,
        entry_point: Address,
    ) -> Result<SendUserOperationResponse> {
        let mut last_err: Option<anyhow::Error> = None;

        let order = rotate_order(self.next_idx, self.providers.len());
        for idx in order {
            let url = &self.urls[idx];
            let url = redact_url(url);
            let provider = &self.providers[idx];

            // Use a tolerant response type: some bundlers return `{ userOpHash }`, others return
            // the hash string directly.
            let rpc_op = to_rpc_packed_v07(user_op)?;
            let fut = provider.raw_request("eth_sendUserOperation".into(), (rpc_op, entry_point));

            match tokio::time::timeout(RPC_TIMEOUT, fut).await {
                Ok(Ok(v)) => {
                    let v: SendUserOperationResponseAny = v;
                    let v: SendUserOperationResponse = v.into();
                    self.mark_success(idx);
                    return Ok(v);
                }
                Ok(Err(err)) => {
                    let err = anyhow::Error::new(err).context("eth_sendUserOperation");
                    if looks_like_packed_userop_unsupported(&err) {
                        tracing::warn!(
                            bundler = %url,
                            err = %format!("{err:#}"),
                            "bundler rejected packed userop; retrying eth_sendUserOperation with expanded v0.7 shape"
                        );

                        let rpc_op = to_rpc_expanded_v07(user_op)?;
                        let fut = provider
                            .raw_request("eth_sendUserOperation".into(), (rpc_op.clone(), entry_point));
                        match tokio::time::timeout(RPC_TIMEOUT, fut).await {
                            Ok(Ok(v)) => {
                                let v: SendUserOperationResponseAny = v;
                                let v: SendUserOperationResponse = v.into();
                                self.mark_success(idx);
                                return Ok(v);
                            }
                            Ok(Err(err2)) => {
                                let err2 = anyhow::Error::new(err2)
                                    .context("eth_sendUserOperation (expanded fallback)");

                                if looks_like_wrapped_params_expected(&err2) {
                                    let wrapped = RpcUserOpAndEntryPoint {
                                        user_op: rpc_op,
                                        entry_point,
                                    };
                                    let fut = provider
                                        .raw_request("eth_sendUserOperation".into(), (wrapped,));
                                    match tokio::time::timeout(RPC_TIMEOUT, fut).await {
                                        Ok(Ok(v)) => {
                                            let v: SendUserOperationResponseAny = v;
                                            let v: SendUserOperationResponse = v.into();
                                            self.mark_success(idx);
                                            return Ok(v);
                                        }
                                        Ok(Err(err3)) => {
                                            let err3 = anyhow::Error::new(err3)
                                                .context("eth_sendUserOperation (expanded wrapped fallback)");
                                            tracing::warn!(
                                                bundler = %url,
                                                err = %format!("{err3:#}"),
                                                "bundler rpc failed"
                                            );
                                            last_err = Some(err3);
                                        }
                                        Err(_) => {
                                            let err = anyhow::anyhow!("timed out");
                                            tracing::warn!(
                                                bundler = %url,
                                                err = %err,
                                                "bundler rpc timed out (eth_sendUserOperation expanded wrapped fallback)"
                                            );
                                            last_err = Some(err);
                                        }
                                    }
                                } else {
                                    tracing::warn!(
                                        bundler = %url,
                                        err = %format!("{err2:#}"),
                                        "bundler rpc failed"
                                    );
                                    last_err = Some(err2);
                                }
                            }
                            Err(_) => {
                                let err = anyhow::anyhow!("timed out");
                                tracing::warn!(
                                    bundler = %url,
                                    err = %err,
                                    "bundler rpc timed out (eth_sendUserOperation expanded fallback)"
                                );
                                last_err = Some(err);
                            }
                        }
                    } else {
                        tracing::warn!(
                            bundler = %url,
                            err = %format!("{err:#}"),
                            "bundler rpc failed"
                        );
                        last_err = Some(err);
                    }
                }
                Err(_) => {
                    let err = anyhow::anyhow!("timed out");
                    tracing::warn!(
                        bundler = %url,
                        err = %err,
                        "bundler rpc timed out (eth_sendUserOperation)"
                    );
                    last_err = Some(err);
                }
            }
        }

        Err(last_err
            .unwrap_or_else(|| anyhow::anyhow!("all bundlers failed for eth_sendUserOperation")))
    }

    pub(crate) async fn supported_entry_points(&mut self) -> Result<Vec<Address>> {
        let mut last_err: Option<anyhow::Error> = None;

        let order = rotate_order(self.next_idx, self.providers.len());
        for idx in order {
            let url = &self.urls[idx];
            let url = redact_url(url);
            let provider = &self.providers[idx];

            let fut = provider.supported_entry_points();
            match tokio::time::timeout(RPC_TIMEOUT, fut).await {
                Ok(Ok(v)) => {
                    self.mark_success(idx);
                    return Ok(v);
                }
                Ok(Err(err)) => {
                    let err = anyhow::Error::new(err).context("eth_supportedEntryPoints");
                    tracing::warn!(
                        bundler = %url,
                        err = %format!("{err:#}"),
                        "bundler rpc failed"
                    );
                    last_err = Some(err);
                }
                Err(_) => {
                    let err = anyhow::anyhow!("timed out");
                    tracing::warn!(
                        bundler = %url,
                        err = %err,
                        "bundler rpc timed out (eth_supportedEntryPoints)"
                    );
                    last_err = Some(err);
                }
            }
        }

        Err(last_err
            .unwrap_or_else(|| anyhow::anyhow!("all bundlers failed for eth_supportedEntryPoints")))
    }

    pub async fn get_user_operation_receipt(
        &mut self,
        user_op_hash: Bytes,
    ) -> Result<Option<UserOperationReceipt>> {
        let mut last_err: Option<anyhow::Error> = None;

        let order = rotate_order(self.next_idx, self.providers.len());
        for idx in order {
            let url = &self.urls[idx];
            let url = redact_url(url);
            let provider = &self.providers[idx];

            // Use raw_request to tolerate `null` responses (not found yet).
            //
            // Some bundlers (notably Pimlico) return a receipt JSON shape that can omit
            // EIP-2718 tx fields such as `type`. Alloy's `TransactionReceipt` requires that
            // field, so we patch it in before deserializing.
            let fut = provider.raw_request(
                "eth_getUserOperationReceipt".into(),
                (user_op_hash.clone(),),
            );

            match tokio::time::timeout(RPC_TIMEOUT, fut).await {
                Ok(Ok(v)) => {
                    let v: Option<serde_json::Value> = v;
                    let Some(mut v) = v else {
                        self.mark_success(idx);
                        return Ok(None);
                    };

                    // Patch missing receipt.type (EIP-2718) if bundler omitted it.
                    if let Some(receipt) = v.get_mut("receipt").and_then(|r| r.as_object_mut()) {
                        receipt
                            .entry("type")
                            .or_insert_with(|| serde_json::json!("0x0"));
                    }

                    // Some bundlers also omit ERC-4337 optional-ish fields. Alloy's
                    // `UserOperationReceipt` currently models them as required.
                    if let Some(obj) = v.as_object_mut() {
                        obj.entry("paymaster").or_insert_with(|| {
                            serde_json::json!("0x0000000000000000000000000000000000000000")
                        });
                        // On success, reason is empty.
                        obj.entry("reason")
                            .or_insert_with(|| serde_json::json!("0x"));
                    }

                    let v: UserOperationReceipt = serde_json::from_value(v)
                        .with_context(|| "deserialize eth_getUserOperationReceipt response")?;

                    self.mark_success(idx);
                    return Ok(Some(v));
                }
                Ok(Err(err)) => {
                    let err = anyhow::Error::new(err).context("eth_getUserOperationReceipt");
                    tracing::warn!(
                        bundler = %url,
                        err = %format!("{err:#}"),
                        "bundler rpc failed"
                    );
                    last_err = Some(err);
                }
                Err(_) => {
                    let err = anyhow::anyhow!("timed out");
                    tracing::warn!(
                        bundler = %url,
                        err = %err,
                        "bundler rpc timed out (eth_getUserOperationReceipt)"
                    );
                    last_err = Some(err);
                }
            }
        }

        Err(last_err.unwrap_or_else(|| {
            anyhow::anyhow!("all bundlers failed for eth_getUserOperationReceipt")
        }))
    }
}

fn rotate_order(start_idx: usize, len: usize) -> Vec<usize> {
    if len == 0 {
        return Vec::new();
    }
    let start = start_idx % len;
    (0..len).map(|o| (start + o) % len).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_rotates_from_next_idx() {
        assert_eq!(rotate_order(0, 3), vec![0, 1, 2]);
        assert_eq!(rotate_order(2, 3), vec![2, 0, 1]);
        assert_eq!(rotate_order(5, 3), vec![2, 0, 1]);
    }

    #[test]
    fn send_user_operation_response_accepts_object_or_hash_string() {
        let obj = r#"{"userOpHash":"0x1234"}"#;
        let v: SendUserOperationResponseAny = serde_json::from_str(obj).unwrap();
        let r: SendUserOperationResponse = v.into();
        assert_eq!(r.user_op_hash.as_ref(), &[0x12, 0x34]);

        let hash = r#""0x1234""#;
        let v: SendUserOperationResponseAny = serde_json::from_str(hash).unwrap();
        let r: SendUserOperationResponse = v.into();
        assert_eq!(r.user_op_hash.as_ref(), &[0x12, 0x34]);
    }
}
