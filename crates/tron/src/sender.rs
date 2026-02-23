use super::grpc::TronGrpc;
use super::protocol::{Transaction, TriggerSmartContract};
use super::{TronAddress, TronWallet};
use anyhow::{Context, Result};
use prost::Message;
use sha2::{Digest, Sha256};

/// 100 TRX in sun.
pub const FIXED_FEE_LIMIT_SUN: u64 = 100_000_000;
const FIXED_FEE_LIMIT_SUN_I64: i64 = FIXED_FEE_LIMIT_SUN as i64;

#[derive(Debug, Clone)]
pub struct SignedTronTx {
    pub tx: Transaction,
    /// `sha256(raw_data_bytes)`.
    pub txid: [u8; 32],
    pub fee_limit_sun: u64,
    pub energy_required: u64,
    pub tx_size_bytes: u64,
}

impl TronWallet {
    /// Builds and signs a TriggerSmartContract tx with a fixed 100 TRX fee limit.
    pub async fn build_and_sign_trigger_smart_contract(
        &self,
        grpc: &mut TronGrpc,
        contract: TronAddress,
        data: Vec<u8>,
        call_value_sun: i64,
    ) -> Result<SignedTronTx> {
        let owner = self.address.prefixed_bytes().to_vec();
        let contract_addr = contract.prefixed_bytes().to_vec();

        let energy_required_i64 = grpc
            .estimate_energy(TriggerSmartContract {
                owner_address: owner.clone(),
                contract_address: contract_addr.clone(),
                call_value: call_value_sun,
                data: data.clone(),
                call_token_value: 0,
                token_id: 0,
            })
            .await?
            .energy_required;
        let energy_required =
            u64::try_from(energy_required_i64).context("energy_required out of range")?;

        // Ask node to build the tx skeleton (ref block bytes/hash/etc).
        let tx_ext = grpc
            .trigger_contract(TriggerSmartContract {
                owner_address: owner,
                contract_address: contract_addr,
                call_value: call_value_sun,
                data,
                call_token_value: 0,
                token_id: 0,
            })
            .await
            .context("trigger_contract")?;

        let mut tx = tx_ext.transaction.context("node returned no transaction")?;
        let raw = tx.raw_data.take().context("node returned no raw_data")?;

        let (tx_final, txid_final, tx_size_final) =
            self.sign_raw_with_fee_limit(raw, tx.ret, FIXED_FEE_LIMIT_SUN_I64)?;

        Ok(SignedTronTx {
            tx: tx_final,
            txid: txid_final,
            fee_limit_sun: FIXED_FEE_LIMIT_SUN,
            energy_required,
            tx_size_bytes: tx_size_final,
        })
    }

    fn sign_raw_with_fee_limit(
        &self,
        mut raw: super::protocol::transaction::Raw,
        ret: Vec<super::protocol::transaction::Result>,
        fee_limit_sun: i64,
    ) -> Result<(Transaction, [u8; 32], u64)> {
        raw.fee_limit = fee_limit_sun.max(0);

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
            ret,
        };

        let size = u64::try_from(signed.encode_to_vec().len()).unwrap_or(u64::MAX);

        let mut out = [0u8; 32];
        out.copy_from_slice(&txid);
        Ok((signed, out, size))
    }
}
