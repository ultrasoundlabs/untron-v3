use super::RelayerState;
use crate::metrics::RelayerTelemetry;
use aa::Safe4337UserOpSender;
use alloy::primitives::{Address, U256};
use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tron::{
    FeePolicy, JsonApiRentalProvider, RentalContext, RentalResourceKind, TronAddress, TronGrpc,
    TronWallet,
};

#[derive(Clone)]
pub struct HubExecutor {
    sender: Arc<Mutex<Safe4337UserOpSender>>,
    untron_v3: Address,
    telemetry: RelayerTelemetry,
}

impl HubExecutor {
    pub fn new(
        sender: Arc<Mutex<Safe4337UserOpSender>>,
        untron_v3: Address,
        telemetry: RelayerTelemetry,
    ) -> Self {
        Self {
            sender,
            untron_v3,
            telemetry,
        }
    }

    pub async fn current_nonce(&self) -> Result<U256> {
        let sender = self.sender.lock().await;
        sender.current_nonce().await
    }

    pub async fn submit(
        &self,
        state: &mut RelayerState,
        name: &'static str,
        data: Vec<u8>,
    ) -> Result<()> {
        let start = Instant::now();
        let submission = {
            let mut sender = self.sender.lock().await;
            sender.send_call(self.untron_v3, data).await
        };

        match submission {
            Ok(sub) => {
                self.telemetry
                    .hub_submit_ms(name, true, start.elapsed().as_millis() as u64);
                self.telemetry.hub_userop_ok();
                state.hub_pending_nonce = Some(sub.nonce);
                tracing::info!(userop_hash = %sub.userop_hash, %name, "submitted hub userop");
                Ok(())
            }
            Err(err) => {
                self.telemetry
                    .hub_submit_ms(name, false, start.elapsed().as_millis() as u64);
                self.telemetry.hub_userop_err();
                Err(err)
            }
        }
    }
}

#[derive(Clone)]
pub struct TronExecutor {
    grpc: Arc<Mutex<TronGrpc>>,
    wallet: Arc<TronWallet>,
    fee_policy: FeePolicy,
    energy_rental: Vec<JsonApiRentalProvider>,
    telemetry: RelayerTelemetry,
}

impl TronExecutor {
    pub fn new(
        grpc: Arc<Mutex<TronGrpc>>,
        wallet: Arc<TronWallet>,
        fee_policy: FeePolicy,
        energy_rental: Vec<JsonApiRentalProvider>,
        telemetry: RelayerTelemetry,
    ) -> Self {
        Self {
            grpc,
            wallet,
            fee_policy,
            energy_rental,
            telemetry,
        }
    }

    pub async fn broadcast_trigger_smart_contract(
        &self,
        state: &mut RelayerState,
        contract: TronAddress,
        data: Vec<u8>,
        call_value_sun: i64,
    ) -> Result<[u8; 32]> {
        let start = Instant::now();
        let mut grpc = self.grpc.lock().await;
        let txid = self
            .broadcast_trigger_smart_contract_managed(
                &mut grpc,
                state,
                contract,
                data,
                call_value_sun,
            )
            .await;

        match txid {
            Ok(txid) => {
                self.telemetry
                    .tron_broadcast_ms(true, start.elapsed().as_millis() as u64);
                self.telemetry.tron_tx_ok();
                Ok(txid)
            }
            Err(err) => {
                self.telemetry
                    .tron_broadcast_ms(false, start.elapsed().as_millis() as u64);
                self.telemetry.tron_tx_err();
                Err(err)
            }
        }
    }

    async fn broadcast_trigger_smart_contract_managed(
        &self,
        grpc: &mut TronGrpc,
        state: &mut RelayerState,
        contract: TronAddress,
        data: Vec<u8>,
        call_value_sun: i64,
    ) -> Result<[u8; 32]> {
        const MIN_ENERGY_RENTAL_AMOUNT: u64 = 32_000;

        let signed = self
            .wallet
            .build_and_sign_trigger_smart_contract(
                grpc,
                contract,
                data,
                call_value_sun,
                self.fee_policy,
            )
            .await?;

        // Preflight balance check: nodes commonly require `balance >= fee_limit` even if resources are rented.
        let account = grpc
            .get_account(self.wallet.address().prefixed_bytes().to_vec())
            .await?;
        let fee_limit_i64 = i64::try_from(signed.fee_limit_sun).unwrap_or(i64::MAX);
        if account.balance < fee_limit_i64 {
            anyhow::bail!(
                "insufficient TRX for fee_limit: balance={} sun, fee_limit={} sun",
                account.balance,
                fee_limit_i64
            );
        }

        // Attempt energy rental for the shortfall (best-effort, fall back to paying TRX).
        if !self.energy_rental.is_empty() {
            let res = grpc
                .get_account_resource(self.wallet.address().prefixed_bytes().to_vec())
                .await?;
            let parsed = tron::resources::parse_account_resources(&res)?;
            let shortfall = signed
                .energy_required
                .saturating_sub(parsed.energy_available());

            if shortfall >= MIN_ENERGY_RENTAL_AMOUNT {
                let addr = self.wallet.address();
                let addr_hex41 = format!("0x{}", hex::encode(addr.prefixed_bytes()));
                let addr_evm_hex = format!("0x{}", hex::encode(addr.evm().as_slice()));
                let txid_hex = format!("0x{}", hex::encode(signed.txid));

                let ctx = RentalContext {
                    resource: RentalResourceKind::Energy,
                    amount: shortfall,
                    address_base58check: addr.to_string(),
                    address_hex41: addr_hex41,
                    address_evm_hex: addr_evm_hex,
                    txid: Some(txid_hex),
                };

                let len = self.energy_rental.len();
                let start_cursor = state.energy_rental_cursor;
                let order = rental_try_indices(start_cursor, len);

                let mut ok = false;
                for (attempt, idx) in order.into_iter().enumerate() {
                    let p = &self.energy_rental[idx];
                    match p.rent(&ctx).await {
                        Ok(attempt_res) if attempt_res.ok => {
                            tracing::info!(
                                provider = %attempt_res.provider,
                                order_id = attempt_res.order_id.as_deref().unwrap_or(""),
                                energy = shortfall,
                                "energy rental ok"
                            );
                            state.energy_rental_cursor =
                                rental_cursor_after_attempts(start_cursor, len, attempt + 1);
                            ok = true;
                            break;
                        }
                        Ok(attempt_res) => {
                            tracing::warn!(
                                provider = %attempt_res.provider,
                                err = attempt_res.error.as_deref().unwrap_or("rental failed"),
                                "energy rental failed; trying next provider"
                            );
                            state.energy_rental_cursor =
                                rental_cursor_after_attempts(start_cursor, len, attempt + 1);
                        }
                        Err(err) => {
                            tracing::warn!(provider = %p.name(), err = %err, "energy rental errored; trying next provider");
                            state.energy_rental_cursor =
                                rental_cursor_after_attempts(start_cursor, len, attempt + 1);
                        }
                    }
                }

                if !ok {
                    tracing::warn!(
                        energy = shortfall,
                        "all energy rental providers failed; falling back to paying TRX fees"
                    );
                }
            } else if shortfall > 0 {
                tracing::debug!(
                    energy = shortfall,
                    min_energy = MIN_ENERGY_RENTAL_AMOUNT,
                    "energy shortfall below minimum rental amount; skipping rental"
                );
            }
        }

        let ret = grpc.broadcast_transaction(signed.tx).await?;
        if !ret.result {
            tracing::warn!(
                tron_return_code = ret.code,
                tron_return_message_hex = %format!("0x{}", hex::encode(&ret.message)),
                tron_return_message_utf8 = %String::from_utf8_lossy(&ret.message),
                "tron broadcast rejected"
            );
            anyhow::bail!(
                "broadcast failed: msg_hex=0x{}, msg_utf8={}",
                hex::encode(&ret.message),
                String::from_utf8_lossy(&ret.message),
            );
        }

        Ok(signed.txid)
    }
}

fn rental_try_indices(start_cursor: usize, len: usize) -> Vec<usize> {
    if len == 0 {
        return Vec::new();
    }
    let start = start_cursor % len;
    (0..len).map(|i| (start + i) % len).collect()
}

fn rental_cursor_after_attempts(start_cursor: usize, len: usize, attempts: usize) -> usize {
    if len == 0 {
        return 0;
    }
    let start = start_cursor % len;
    (start + attempts) % len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rental_try_indices_rotates_from_cursor() {
        assert_eq!(rental_try_indices(0, 3), vec![0, 1, 2]);
        assert_eq!(rental_try_indices(2, 3), vec![2, 0, 1]);
        assert_eq!(rental_try_indices(5, 3), vec![2, 0, 1]);
        assert!(rental_try_indices(0, 0).is_empty());
    }

    #[test]
    fn rental_cursor_after_attempts_advances() {
        assert_eq!(rental_cursor_after_attempts(0, 3, 1), 1);
        assert_eq!(rental_cursor_after_attempts(2, 3, 1), 0);
        assert_eq!(rental_cursor_after_attempts(2, 3, 2), 1);
        assert_eq!(rental_cursor_after_attempts(2, 3, 3), 2);
        assert_eq!(rental_cursor_after_attempts(10, 0, 1), 0);
    }
}
