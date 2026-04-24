use super::grpc::TronGrpc;
use super::protocol::{Transaction, TriggerSmartContract};
use super::resources::{ChainFees, quote_fee_limit_sun};
use super::{TronAddress, TronWallet};
use anyhow::{Context, Result};
use prost::Message;
use sha2::{Digest, Sha256};

/// 100 TRX in sun. Acts as the hard ceiling in the default `FeeLimitPolicy::FIXED`,
/// and as a sane upper bound in dynamic policies.
pub const FIXED_FEE_LIMIT_SUN: u64 = 100_000_000;
const MAX_VARINT64_BYTES: u64 = 10;

/// Policy for computing a tx's `fee_limit` — the max TRX burn the node is allowed to charge.
///
/// Tron validates `balance >= fee_limit + call_value` before execution, so leaving `fee_limit`
/// at a fixed high value (old behavior) blocks any wallet whose balance is below that value,
/// even when the actual burn would be tiny. This struct lets the caller size `fee_limit` to the
/// live `energy_required × energy_fee`, capped at a safety ceiling.
#[derive(Debug, Clone, Copy)]
pub struct FeeLimitPolicy {
    /// Live chain fees from `GetChainParameters`. If `None`, fee_limit is always `ceiling_sun`.
    pub fees: Option<ChainFees>,
    /// Parts-per-million bonus on top of `quote_fee_limit_sun(...)`. 100_000 = +10%.
    pub headroom_ppm: u64,
    /// Hard cap on fee_limit, regardless of quote.
    pub ceiling_sun: u64,
}

impl FeeLimitPolicy {
    /// Legacy policy that matches pre-dynamic-fee-limit behavior: always stamp 100 TRX.
    pub const FIXED: FeeLimitPolicy = FeeLimitPolicy {
        fees: None,
        headroom_ppm: 0,
        ceiling_sun: FIXED_FEE_LIMIT_SUN,
    };

    pub fn compute(&self, energy_required: u64, tx_size_bytes: u64) -> u64 {
        let Some(fees) = self.fees else {
            return self.ceiling_sun;
        };
        let base = quote_fee_limit_sun(energy_required, tx_size_bytes, fees);
        let bonus = base.saturating_mul(self.headroom_ppm) / 1_000_000u64;
        base.saturating_add(bonus).min(self.ceiling_sun)
    }
}

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
    /// Builds and signs a `TriggerSmartContract` tx with a fee_limit derived from `policy`.
    ///
    /// Fails fast if `estimate_energy` reports the call would revert — the signed tx would be
    /// dead-on-arrival, and broadcasting it would still burn energy consumed before the revert.
    pub async fn build_and_sign_trigger_smart_contract(
        &self,
        grpc: &mut TronGrpc,
        contract: TronAddress,
        data: Vec<u8>,
        call_value_sun: i64,
        policy: FeeLimitPolicy,
    ) -> Result<SignedTronTx> {
        let owner = self.address.prefixed_bytes().to_vec();
        let contract_addr = contract.prefixed_bytes().to_vec();

        let estimate = grpc
            .estimate_energy(TriggerSmartContract {
                owner_address: owner.clone(),
                contract_address: contract_addr.clone(),
                call_value: call_value_sun,
                data: data.clone(),
                call_token_value: 0,
                token_id: 0,
            })
            .await
            .context("EstimateEnergy")?;

        if let Some(ret) = &estimate.result
            && !ret.result
        {
            anyhow::bail!(
                "estimate_energy reports call would revert: code={} msg_hex=0x{} msg_utf8={}",
                ret.code,
                hex::encode(&ret.message),
                String::from_utf8_lossy(&ret.message),
            );
        }

        let energy_required =
            u64::try_from(estimate.energy_required).context("energy_required out of range")?;

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

        // Estimate tx_size with fee_limit=0 and pad for the varint we'll stamp in.
        let tx_size_est = {
            let mut r = raw.clone();
            r.fee_limit = 0;
            (r.encode_to_vec().len() as u64).saturating_add(MAX_VARINT64_BYTES)
        };
        let fee_limit_sun = policy.compute(energy_required, tx_size_est);

        let (tx_final, txid_final, tx_size_final) =
            self.sign_raw_with_fee_limit(raw, tx.ret, fee_limit_sun as i64)?;

        Ok(SignedTronTx {
            tx: tx_final,
            txid: txid_final,
            fee_limit_sun,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fee_limit_policy_fixed_returns_ceiling() {
        let p = FeeLimitPolicy::FIXED;
        assert_eq!(p.compute(50_000, 500), FIXED_FEE_LIMIT_SUN);
        assert_eq!(p.compute(0, 0), FIXED_FEE_LIMIT_SUN);
    }

    #[test]
    fn fee_limit_policy_dynamic_matches_quote_plus_headroom() {
        let fees = ChainFees {
            energy_fee_sun_per_energy: 210,
            tx_fee_sun_per_byte: 1000,
        };
        let p = FeeLimitPolicy {
            fees: Some(fees),
            headroom_ppm: 100_000, // +10%
            ceiling_sun: FIXED_FEE_LIMIT_SUN,
        };
        // 2000 * 210 + 500 * 1000 = 420_000 + 500_000 = 920_000
        // +10% = 1_012_000
        assert_eq!(p.compute(2000, 500), 1_012_000);
    }

    #[test]
    fn fee_limit_policy_clamps_to_ceiling() {
        let fees = ChainFees {
            energy_fee_sun_per_energy: 1_000_000,
            tx_fee_sun_per_byte: 0,
        };
        let p = FeeLimitPolicy {
            fees: Some(fees),
            headroom_ppm: 100_000,
            ceiling_sun: 50_000_000,
        };
        // 1_000 * 1_000_000 = 1_000_000_000 >> 50_000_000 → clamped
        assert_eq!(p.compute(1_000, 0), 50_000_000);
    }

    #[test]
    fn fee_limit_policy_headroom_cannot_overflow() {
        let fees = ChainFees {
            energy_fee_sun_per_energy: 1,
            tx_fee_sun_per_byte: 0,
        };
        let p = FeeLimitPolicy {
            fees: Some(fees),
            headroom_ppm: u64::MAX,
            ceiling_sun: u64::MAX,
        };
        // Must not panic or wrap.
        let _ = p.compute(u64::MAX, 0);
    }
}
