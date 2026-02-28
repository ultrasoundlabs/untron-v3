use alloy::primitives::{Address, Bytes, TxKind, U256};
use alloy::providers::{DynProvider, Provider};
use alloy::sol_types::SolCall;
use anyhow::Result;

use alloy::rpc::types::eth::erc4337::PackedUserOperation;

alloy::sol! {
    /// EntryPoint v0.7 packed user operation.
    ///
    /// Ref: ERC-4337 EntryPoint v0.7 (`PackedUserOperation`).
    struct PackedUserOperationV07 {
        address sender;
        uint256 nonce;
        bytes initCode;
        bytes callData;
        bytes32 accountGasLimits;
        uint256 preVerificationGas;
        bytes32 gasFees;
        bytes paymasterAndData;
        bytes signature;
    }

    interface IEntryPointSimulations {
        function simulateValidation(PackedUserOperationV07 userOp) external;
    }
}

fn pack_128(high: U256, low: U256) -> [u8; 32] {
    // pack two uint128 values into a bytes32: (high << 128) | low
    let mut out = [0u8; 32];

    let hi = high.to_be_bytes::<32>();
    let lo = low.to_be_bytes::<32>();

    // take the lowest 16 bytes of each
    out[0..16].copy_from_slice(&hi[16..32]);
    out[16..32].copy_from_slice(&lo[16..32]);
    out
}

fn pack_account_gas_limits(op: &PackedUserOperation) -> [u8; 32] {
    pack_128(op.verification_gas_limit, op.call_gas_limit)
}

fn pack_gas_fees(op: &PackedUserOperation) -> [u8; 32] {
    pack_128(op.max_priority_fee_per_gas, op.max_fee_per_gas)
}

fn init_code(op: &PackedUserOperation) -> Bytes {
    // initCode = factory (20 bytes) ++ factoryData
    match (op.factory, op.factory_data.as_ref()) {
        (Some(factory), Some(data)) => {
            let mut out = Vec::with_capacity(20 + data.len());
            out.extend_from_slice(factory.as_slice());
            out.extend_from_slice(data.as_ref());
            Bytes::from(out)
        }
        _ => Bytes::new(),
    }
}

fn paymaster_and_data(op: &PackedUserOperation) -> Bytes {
    // paymasterAndData = paymaster (20 bytes) ++ paymasterVerificationGasLimit (uint128)
    //                 ++ paymasterPostOpGasLimit (uint128) ++ paymasterData
    let Some(paymaster) = op.paymaster else {
        return Bytes::new();
    };
    let pm_ver = op.paymaster_verification_gas_limit.unwrap_or_default();
    let pm_post = op.paymaster_post_op_gas_limit.unwrap_or_default();
    let pm_data = op.paymaster_data.clone().unwrap_or_default();

    let mut out = Vec::with_capacity(20 + 16 + 16 + pm_data.len());
    out.extend_from_slice(paymaster.as_slice());

    let pm_ver_bytes = pm_ver.to_be_bytes::<32>();
    out.extend_from_slice(&pm_ver_bytes[16..32]);

    let pm_post_bytes = pm_post.to_be_bytes::<32>();
    out.extend_from_slice(&pm_post_bytes[16..32]);

    out.extend_from_slice(pm_data.as_ref());
    Bytes::from(out)
}

fn to_packed_v07(op: &PackedUserOperation) -> PackedUserOperationV07 {
    PackedUserOperationV07 {
        sender: op.sender,
        nonce: op.nonce,
        initCode: init_code(op),
        callData: op.call_data.clone(),
        accountGasLimits: pack_account_gas_limits(op).into(),
        preVerificationGas: op.pre_verification_gas,
        gasFees: pack_gas_fees(op).into(),
        paymasterAndData: paymaster_and_data(op),
        signature: op.signature.clone(),
    }
}

/// Best-effort: run EntryPoint.simulateValidation via eth_call to obtain a richer revert.
///
/// Many bundlers omit `error.data`. This uses a normal RPC provider, which tends to include
/// revert bytes.
pub async fn debug_simulate_validation(
    provider: &DynProvider,
    entrypoint: Address,
    op: &PackedUserOperation,
) -> Result<()> {
    let packed = to_packed_v07(op);
    let call = IEntryPointSimulations::simulateValidationCall { userOp: packed };
    let calldata = call.abi_encode();

    // We expect this to revert (by design). We only care about the error payload.
    let req = alloy::rpc::types::TransactionRequest {
        to: Some(TxKind::Call(entrypoint)),
        input: Some(calldata.into()).into(),
        ..Default::default()
    };

    // We expect this to revert (by design). If it succeeds, that's still useful info.
    match provider.call(req).await {
        Ok(_bytes) => {
            tracing::warn!("simulateValidation eth_call unexpectedly succeeded (no revert)");
        }
        Err(err) => {
            // This is the payload we care about; caller should log `err` with `Debug` at callsite.
            tracing::error!(raw_err = ?err, "simulateValidation eth_call reverted");
        }
    }

    Ok(())
}
