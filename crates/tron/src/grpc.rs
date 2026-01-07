use super::protocol::{
    Account, AccountResourceMessage, BlockExtention, BytesMessage, ChainParameters, EmptyMessage,
    EstimateEnergyMessage, NumberMessage, Return, Transaction, TransactionExtention,
    TransactionInfo, TriggerSmartContract, wallet_client::WalletClient,
};
use anyhow::{Context, Result};
use std::str::FromStr;
use tonic::{Request, metadata::MetadataValue, transport::Channel};

#[derive(Clone)]
pub struct TronGrpc {
    api_key: Option<MetadataValue<tonic::metadata::Ascii>>,
    wallet: WalletClient<Channel>,
}

impl TronGrpc {
    pub async fn connect(grpc_url: &str, api_key: Option<&str>) -> Result<Self> {
        let channel = Channel::from_shared(grpc_url.to_string())
            .context("invalid TRON_GRPC_URL")?
            .connect()
            .await
            .context("connect TRON gRPC")?;

        let api_key = match api_key {
            Some(k) if !k.trim().is_empty() => {
                Some(MetadataValue::from_str(k).context("invalid TRON_API_KEY (metadata value)")?)
            }
            _ => None,
        };

        Ok(Self {
            api_key,
            wallet: WalletClient::new(channel),
        })
    }

    fn req<T>(&self, msg: T) -> Request<T> {
        let mut req = Request::new(msg);
        if let Some(key) = &self.api_key {
            req.metadata_mut().insert("tron-pro-api-key", key.clone());
        }
        req
    }

    pub async fn get_now_block2(&mut self) -> Result<BlockExtention> {
        let resp = self
            .wallet
            .get_now_block2(self.req(EmptyMessage {}))
            .await
            .context("GetNowBlock2")?;
        Ok(resp.into_inner())
    }

    pub async fn get_block_by_num2(&mut self, num: i64) -> Result<BlockExtention> {
        let resp = self
            .wallet
            .get_block_by_num2(self.req(NumberMessage { num }))
            .await
            .context("GetBlockByNum2")?;
        Ok(resp.into_inner())
    }

    pub async fn get_transaction_info_by_id(&mut self, txid: [u8; 32]) -> Result<TransactionInfo> {
        let resp = self
            .wallet
            .get_transaction_info_by_id(self.req(BytesMessage {
                value: txid.to_vec(),
            }))
            .await
            .context("GetTransactionInfoById")?;
        Ok(resp.into_inner())
    }

    pub async fn trigger_contract(
        &mut self,
        msg: TriggerSmartContract,
    ) -> Result<TransactionExtention> {
        let resp = self
            .wallet
            .trigger_contract(self.req(msg))
            .await
            .context("TriggerContract")?;
        Ok(resp.into_inner())
    }

    pub async fn trigger_constant_contract(
        &mut self,
        msg: TriggerSmartContract,
    ) -> Result<TransactionExtention> {
        let resp = self
            .wallet
            .trigger_constant_contract(self.req(msg))
            .await
            .context("TriggerConstantContract")?;
        Ok(resp.into_inner())
    }

    pub async fn broadcast_transaction(&mut self, tx: Transaction) -> Result<Return> {
        let resp = self
            .wallet
            .broadcast_transaction(self.req(tx))
            .await
            .context("BroadcastTransaction")?;
        Ok(resp.into_inner())
    }

    pub async fn get_account(&mut self, address_prefixed: Vec<u8>) -> Result<Account> {
        let resp = self
            .wallet
            .get_account(self.req(Account {
                address: address_prefixed,
                ..Default::default()
            }))
            .await
            .context("GetAccount")?;
        Ok(resp.into_inner())
    }

    pub async fn get_account_resource(
        &mut self,
        address_prefixed: Vec<u8>,
    ) -> Result<AccountResourceMessage> {
        let resp = self
            .wallet
            .get_account_resource(self.req(Account {
                address: address_prefixed,
                ..Default::default()
            }))
            .await
            .context("GetAccountResource")?;
        Ok(resp.into_inner())
    }

    pub async fn get_chain_parameters(&mut self) -> Result<ChainParameters> {
        let resp = self
            .wallet
            .get_chain_parameters(self.req(EmptyMessage {}))
            .await
            .context("GetChainParameters")?;
        Ok(resp.into_inner())
    }

    pub async fn estimate_energy(
        &mut self,
        msg: TriggerSmartContract,
    ) -> Result<EstimateEnergyMessage> {
        let resp = self
            .wallet
            .estimate_energy(self.req(msg))
            .await
            .context("EstimateEnergy")?;
        Ok(resp.into_inner())
    }
}
