use super::RelayerState;
use crate::{
    hub::HubUserOpSender,
    metrics::RelayerTelemetry,
    tron::{address::TronAddress, grpc::TronGrpc, wallet::TronWallet},
};
use alloy::primitives::{Address, U256};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct HubExecutor {
    sender: Arc<Mutex<HubUserOpSender>>,
    untron_v3: Address,
    telemetry: RelayerTelemetry,
}

impl HubExecutor {
    pub fn new(
        sender: Arc<Mutex<HubUserOpSender>>,
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
        let submission = {
            let mut sender = self.sender.lock().await;
            sender.send_call(self.untron_v3, data).await
        };

        match submission {
            Ok(sub) => {
                self.telemetry.hub_userop_ok();
                state.hub_pending_nonce = Some(sub.nonce);
                tracing::info!(userop_hash = %sub.userop_hash, %name, "submitted hub userop");
                Ok(())
            }
            Err(err) => {
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
    telemetry: RelayerTelemetry,
}

impl TronExecutor {
    pub fn new(
        grpc: Arc<Mutex<TronGrpc>>,
        wallet: Arc<TronWallet>,
        telemetry: RelayerTelemetry,
    ) -> Self {
        Self {
            grpc,
            wallet,
            telemetry,
        }
    }

    pub async fn broadcast_trigger_smart_contract(
        &self,
        contract: TronAddress,
        data: Vec<u8>,
        call_value_sun: i64,
    ) -> Result<[u8; 32]> {
        let mut grpc = self.grpc.lock().await;
        let txid = self
            .wallet
            .broadcast_trigger_smart_contract(&mut grpc, contract, data, call_value_sun)
            .await;

        match txid {
            Ok(txid) => {
                self.telemetry.tron_tx_ok();
                Ok(txid)
            }
            Err(err) => {
                self.telemetry.tron_tx_err();
                Err(err)
            }
        }
    }
}
