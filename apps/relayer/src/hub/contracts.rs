alloy::sol! {
    #[sol(rpc)]
    interface IEntryPointNonces {
        function getNonce(address sender, uint192 key) external view returns (uint256);
    }

    interface HubModule {
        function executeUserOp(address to, uint256 value, bytes data, uint8 operation) external;
    }

    /// EIP-712 struct hashed by the module (see `HubModule._getSafeOp`).
    struct SafeOp {
        address safe;
        uint256 nonce;
        bytes initCode;
        bytes callData;
        uint128 verificationGasLimit;
        uint128 callGasLimit;
        uint256 preVerificationGas;
        uint128 maxPriorityFeePerGas;
        uint128 maxFeePerGas;
        bytes paymasterAndData;
        uint48 validAfter;
        uint48 validUntil;
        address entryPoint;
    }
}
