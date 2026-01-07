use super::protocol::{Transaction, TriggerSmartContract};
use super::{address::TronAddress, grpc::TronGrpc};
use alloy::primitives::{Address, FixedBytes, U256, keccak256};
use anyhow::{Context, Result};
use k256::ecdsa::SigningKey;
use prost::Message;
use sha2::{Digest, Sha256};

pub struct TronWallet {
    key: SigningKey,
    address: TronAddress,
    fee_limit_sun: i64,
}

impl TronWallet {
    pub fn new(private_key: [u8; 32], fee_limit_sun: i64) -> Result<Self> {
        let key = SigningKey::from_slice(&private_key).context("invalid TRON private key")?;
        let address = tron_address_from_signing_key(&key);
        Ok(Self {
            key,
            address,
            fee_limit_sun: fee_limit_sun.max(0),
        })
    }

    pub fn address(&self) -> TronAddress {
        self.address
    }

    pub async fn broadcast_trigger_smart_contract(
        &self,
        grpc: &mut TronGrpc,
        contract: TronAddress,
        data: Vec<u8>,
        call_value_sun: i64,
    ) -> Result<[u8; 32]> {
        let account = grpc
            .get_account(self.address.prefixed_bytes().to_vec())
            .await
            .context("get Tron account")?;

        let balance = account.balance;
        if balance < self.fee_limit_sun {
            anyhow::bail!(
                "insufficient TRX for fee_limit: balance={} sun, fee_limit={} sun",
                balance,
                self.fee_limit_sun
            );
        }

        let tx_ext = grpc
            .trigger_contract(TriggerSmartContract {
                owner_address: self.address.prefixed_bytes().to_vec(),
                contract_address: contract.prefixed_bytes().to_vec(),
                call_value: call_value_sun,
                data,
                call_token_value: 0,
                token_id: 0,
            })
            .await
            .context("trigger_contract")?;

        let mut tx = tx_ext.transaction.context("node returned no transaction")?;
        let mut raw = tx.raw_data.take().context("node returned no raw_data")?;
        raw.fee_limit = self.fee_limit_sun;

        let raw_bytes = raw.encode_to_vec();
        let txid = Sha256::digest(&raw_bytes);

        let (rec_sig, recid) = self
            .key
            .clone()
            .sign_digest_recoverable(Sha256::new_with_prefix(&raw_bytes))
            .context("sign Tron tx")?;

        let mut sig65 = rec_sig.to_bytes().to_vec();
        sig65.push(recid.to_byte() + 27);

        let signed = Transaction {
            raw_data: Some(raw),
            signature: vec![sig65],
            ret: tx.ret,
        };

        let ret = grpc
            .broadcast_transaction(signed)
            .await
            .context("broadcast_transaction")?;

        if !ret.result {
            anyhow::bail!(
                "broadcast failed: {}",
                String::from_utf8_lossy(&ret.message)
            );
        }

        let mut out = [0u8; 32];
        out.copy_from_slice(&txid);
        Ok(out)
    }
}

fn tron_address_from_signing_key(key: &SigningKey) -> TronAddress {
    let public_key = key.verifying_key().to_encoded_point(false);
    let public_key_bytes = public_key.as_bytes();
    let hash = keccak256(&public_key_bytes[1..]);
    let addr20 = Address::from_slice(&hash[12..]);
    TronAddress::from_evm(addr20)
}

// ===== ABI helpers (EVM ABI, used by Tron TriggerSmartContract) =====

pub async fn trc20_balance_of(
    grpc: &mut TronGrpc,
    token_contract: TronAddress,
    owner: TronAddress,
    caller: TronAddress,
) -> Result<U256> {
    let selector = keccak256("balanceOf(address)".as_bytes());
    let mut data = Vec::with_capacity(4 + 32);
    data.extend_from_slice(&selector[..4]);
    data.extend_from_slice(&encode_address(owner.evm()));

    let tx_ext = grpc
        .trigger_constant_contract(TriggerSmartContract {
            owner_address: caller.prefixed_bytes().to_vec(),
            contract_address: token_contract.prefixed_bytes().to_vec(),
            call_value: 0,
            data,
            call_token_value: 0,
            token_id: 0,
        })
        .await
        .context("trigger_constant_contract(balanceOf)")?;

    let out = tx_ext
        .constant_result
        .first()
        .context("missing constant_result")?;
    if out.len() != 32 {
        anyhow::bail!("unexpected balanceOf output length: {}", out.len());
    }
    Ok(U256::from_be_slice(out))
}

pub fn encode_is_event_chain_tip(tip: FixedBytes<32>) -> Vec<u8> {
    let selector = keccak256("isEventChainTip(bytes32)".as_bytes());
    let mut out = Vec::with_capacity(4 + 32);
    out.extend_from_slice(&selector[..4]);
    out.extend_from_slice(tip.as_slice());
    out
}

pub fn encode_pull_from_receivers(token: Address, receiver_salts: &[FixedBytes<32>]) -> Vec<u8> {
    let selector = keccak256("pullFromReceivers(address,bytes32[])".as_bytes());

    let mut out = Vec::new();
    out.extend_from_slice(&selector[..4]);

    // head: (address, offset)
    out.extend_from_slice(&encode_address(token));
    out.extend_from_slice(&encode_u256(U256::from(64u64))); // dynamic tail starts after 2*32 bytes

    // tail: array
    out.extend_from_slice(&encode_u256(U256::from(receiver_salts.len())));
    for salt in receiver_salts {
        out.extend_from_slice(salt.as_slice());
    }

    out
}

pub fn encode_rebalance_usdt(rebalancer: Address, in_amount: U256) -> Vec<u8> {
    let selector = keccak256("rebalanceUsdt(address,uint256)".as_bytes());
    let mut out = Vec::with_capacity(4 + 32 + 32);
    out.extend_from_slice(&selector[..4]);
    out.extend_from_slice(&encode_address(rebalancer));
    out.extend_from_slice(&encode_u256(in_amount));
    out
}

fn encode_address(addr: Address) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[12..].copy_from_slice(addr.as_slice());
    out
}

fn encode_u256(v: U256) -> [u8; 32] {
    v.to_be_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_is_event_chain_tip_layout() {
        let tip = FixedBytes::from_slice(&[0x11u8; 32]);
        let out = encode_is_event_chain_tip(tip);

        let selector = keccak256("isEventChainTip(bytes32)".as_bytes());
        assert_eq!(&out[..4], &selector[..4]);
        assert_eq!(&out[4..], tip.as_slice());
    }

    #[test]
    fn encode_rebalance_usdt_layout() {
        let rebalancer = Address::from_slice(&[0x22u8; 20]);
        let amount = U256::from(123_456_789u64);
        let out = encode_rebalance_usdt(rebalancer, amount);

        let selector = keccak256("rebalanceUsdt(address,uint256)".as_bytes());
        assert_eq!(&out[..4], &selector[..4]);

        let mut addr_word = [0u8; 32];
        addr_word[12..].copy_from_slice(rebalancer.as_slice());
        assert_eq!(&out[4..36], &addr_word);
        let amt_word: [u8; 32] = amount.to_be_bytes();
        assert_eq!(&out[36..68], &amt_word);
    }

    #[test]
    fn encode_pull_from_receivers_layout() {
        let token = Address::from_slice(&[0x44u8; 20]);
        let s1 = FixedBytes::from_slice(&[0x01u8; 32]);
        let s2 = FixedBytes::from_slice(&[0x02u8; 32]);

        let out = encode_pull_from_receivers(token, &[s1, s2]);

        let selector = keccak256("pullFromReceivers(address,bytes32[])".as_bytes());
        assert_eq!(&out[..4], &selector[..4]);

        let mut token_word = [0u8; 32];
        token_word[12..].copy_from_slice(token.as_slice());
        assert_eq!(&out[4..36], &token_word);

        let mut offset_word = [0u8; 32];
        offset_word[31] = 64;
        assert_eq!(&out[36..68], &offset_word);

        let mut len_word = [0u8; 32];
        len_word[31] = 2;
        assert_eq!(&out[68..100], &len_word);
        assert_eq!(&out[100..132], s1.as_slice());
        assert_eq!(&out[132..164], s2.as_slice());
        assert_eq!(out.len(), 164);
    }
}
