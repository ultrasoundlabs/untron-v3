use alloy::primitives::{Address, Bytes, U256};

alloy::sol! {
    #[sol(rpc)]
    interface IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
        function transfer(address to, uint256 amount) external returns (bool);
        function balanceOf(address owner) external view returns (uint256);
    }

    interface MultiSend {
        function multiSend(bytes transactions) external;
    }
}

#[derive(Debug, Clone)]
pub struct MultiSendTx {
    /// 0 = call, 1 = delegatecall.
    pub operation: u8,
    pub to: Address,
    pub value: U256,
    pub data: Bytes,
}

/// Encode the `transactions` bytes for the Safe `MultiSend` contract.
///
/// Each tx is encoded as:
/// - `operation` (1 byte)
/// - `to` (20 bytes)
/// - `value` (uint256, 32 bytes)
/// - `data_len` (uint256, 32 bytes)
/// - `data` (bytes)
pub fn encode_multisend_transactions(txs: &[MultiSendTx]) -> Bytes {
    let mut out = Vec::new();
    for tx in txs {
        out.push(tx.operation);
        out.extend_from_slice(tx.to.as_slice());
        out.extend_from_slice(&tx.value.to_be_bytes::<32>());
        let len = U256::from(tx.data.len());
        out.extend_from_slice(&len.to_be_bytes::<32>());
        out.extend_from_slice(tx.data.as_ref());
    }
    Bytes::from(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::sol_types::SolCall;

    #[test]
    fn encode_multisend_transactions_layout() {
        let tx1 = MultiSendTx {
            operation: 0,
            to: Address::repeat_byte(0x11),
            value: U256::from(5u64),
            data: Bytes::from(vec![0xaa, 0xbb]),
        };
        let tx2 = MultiSendTx {
            operation: 1,
            to: Address::repeat_byte(0x22),
            value: U256::ZERO,
            data: Bytes::from(vec![]),
        };

        let bytes = encode_multisend_transactions(&[tx1.clone(), tx2.clone()]);

        // tx1 header sizes: 1 + 20 + 32 + 32 + 2 = 87
        // tx2 header sizes: 1 + 20 + 32 + 32 + 0 = 85
        assert_eq!(bytes.len(), 87 + 85);

        // operation byte for tx1 and tx2.
        assert_eq!(bytes[0], 0);
        assert_eq!(bytes[87], 1);

        // ERC20 call encoding sanity (doesn't panic).
        let _ = IERC20::transferCall {
            to: Address::ZERO,
            amount: U256::from(1u64),
        }
        .abi_encode();
    }
}
