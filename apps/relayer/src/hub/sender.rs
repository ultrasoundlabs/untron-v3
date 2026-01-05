use crate::config::{HubConfig, PaymasterServiceConfig};
use crate::hub::paymaster::{PaymasterPool, PaymasterService, PaymasterUserOp};
use alloy::sol_types::SolCall;
use alloy::{
    primitives::{Address, Bytes, U256},
    providers::{DynProvider, Provider, ProviderBuilder},
    rpc::client::{BuiltInConnectionString, RpcClient},
};
use anyhow::{Context, Result};
use k256::ecdsa::SigningKey;

use super::bundler_pool::BundlerPool;
use super::contracts::{HubModule, IEntryPointNonces};
use super::packing::{add_gas_buffer, hex_bytes0x, redact_url};
use super::signing::sign_userop_with_key;

use alloy::rpc::types::eth::erc4337::PackedUserOperation;

const DEFAULT_PAYMASTER_VERIFICATION_GAS_LIMIT: u64 = 500_000;

pub struct HubUserOpSender {
    cfg: HubConfig,
    provider: DynProvider,
    chain_id: u64,
    owner_key: SigningKey,
    bundlers: BundlerPool,
    paymasters: Option<PaymasterPool>,
}

#[derive(Debug, Clone)]
pub struct HubUserOpSubmission {
    pub userop_hash: String,
    pub nonce: U256,
}

impl HubUserOpSender {
    pub async fn new(cfg: HubConfig) -> Result<Self> {
        let transport = BuiltInConnectionString::connect(&cfg.rpc_url)
            .await
            .with_context(|| format!("connect hub rpc: {}", cfg.rpc_url))?;
        let client = RpcClient::builder().transport(transport, false);
        let provider = ProviderBuilder::default().connect_client(client);
        let provider = DynProvider::new(provider);

        let chain_id = match cfg.chain_id {
            Some(id) => id,
            None => provider.get_chain_id().await.context("eth_chainId")?,
        };

        let owner_key = SigningKey::from_slice(&cfg.owner_private_key)
            .context("invalid HUB_OWNER_PRIVATE_KEY_HEX")?;

        let mut bundlers = BundlerPool::new(cfg.bundler_urls.clone()).await?;
        let paymasters = make_paymaster_pool(&cfg.paymasters)?;

        match bundlers.supported_entry_points().await {
            Ok(eps) => {
                if !eps.contains(&cfg.entrypoint) {
                    tracing::warn!(
                        entrypoint = %cfg.entrypoint,
                        supported = ?eps,
                        "bundler does not advertise configured entrypoint"
                    );
                }
            }
            Err(err) => {
                tracing::warn!(err = %err, "failed to query eth_supportedEntryPoints");
            }
        }

        Ok(Self {
            cfg: cfg.clone(),
            provider,
            chain_id,
            owner_key,
            bundlers,
            paymasters,
        })
    }

    pub async fn current_nonce(&self) -> Result<U256> {
        self.entrypoint()
            .getNonce(self.cfg.safe, alloy::primitives::Uint::<192, 3>::ZERO)
            .call()
            .await
            .context("EntryPoint.getNonce")
    }

    pub async fn send_call(&mut self, to: Address, data: Vec<u8>) -> Result<HubUserOpSubmission> {
        let nonce = self.current_nonce().await?;

        let gas_price: u128 = self
            .provider
            .get_gas_price()
            .await
            .context("eth_gasPrice")?;
        let priority: u128 = self
            .provider
            .get_max_priority_fee_per_gas()
            .await
            .unwrap_or(gas_price / 10);

        let max_fee_per_gas = U256::from(gas_price);
        let max_priority_fee_per_gas = U256::from(priority);

        let call_data = HubModule::executeUserOpCall {
            to,
            value: U256::ZERO,
            data: data.into(),
            operation: 0u8,
        }
        .abi_encode();

        let base_userop = PackedUserOperation {
            sender: self.cfg.safe,
            nonce,
            factory: None,
            factory_data: None,
            call_data: call_data.into(),
            call_gas_limit: U256::from(5_000_000u64),
            verification_gas_limit: U256::from(1_500_000u64),
            pre_verification_gas: U256::from(150_000u64),
            max_fee_per_gas,
            max_priority_fee_per_gas,
            paymaster: None,
            paymaster_verification_gas_limit: None,
            paymaster_post_op_gas_limit: None,
            paymaster_data: None,
            signature: Bytes::new(),
        };

        if let Some(mut pool) = self.paymasters.take() {
            let attempts = pool
                .order()
                .filter_map(|idx| pool.service(idx).cloned().map(|svc| (idx, svc)))
                .collect::<Vec<_>>();

            for (idx, svc) in attempts {
                match self
                    .send_with_paymaster(&mut pool, idx, &svc, base_userop.clone())
                    .await
                {
                    Ok(sub) => {
                        pool.mark_success(idx);
                        self.paymasters = Some(pool);
                        return Ok(sub);
                    }
                    Err(err) => {
                        tracing::warn!(
                            paymaster = %redact_url(&svc.url),
                            err = %err,
                            "paymaster attempt failed; trying next"
                        );
                    }
                }
            }

            tracing::warn!("all configured paymasters failed; falling back to self-paid userop");
            self.paymasters = Some(pool);
        }

        self.send_self_paid(base_userop).await
    }

    fn entrypoint(&self) -> IEntryPointNonces::IEntryPointNoncesInstance<&DynProvider> {
        IEntryPointNonces::new(self.cfg.entrypoint, &self.provider)
    }

    async fn send_self_paid(
        &mut self,
        mut userop: PackedUserOperation,
    ) -> Result<HubUserOpSubmission> {
        userop.signature = self.sign_userop(&userop)?.into();

        let estimate = self
            .bundlers
            .estimate_user_operation_gas(&userop, self.cfg.entrypoint)
            .await
            .context("bundler estimate userop gas")?;

        userop.call_gas_limit = add_gas_buffer(estimate.call_gas_limit, 10)?;
        userop.verification_gas_limit = add_gas_buffer(estimate.verification_gas, 10)?;
        userop.pre_verification_gas = add_gas_buffer(estimate.pre_verification_gas, 10)?;

        userop.signature = self.sign_userop(&userop)?.into();

        let resp = self
            .bundlers
            .send_user_operation(&userop, self.cfg.entrypoint)
            .await
            .context("bundler send userop")?;

        Ok(HubUserOpSubmission {
            userop_hash: hex_bytes0x(&resp.user_op_hash),
            nonce: userop.nonce,
        })
    }

    async fn send_with_paymaster(
        &mut self,
        pool: &mut PaymasterPool,
        idx: usize,
        svc: &PaymasterService,
        mut userop: PackedUserOperation,
    ) -> Result<HubUserOpSubmission> {
        let pm_userop = to_paymaster_userop(&userop)?;
        let stub = pool
            .get_stub_data(idx, &pm_userop, self.cfg.entrypoint, self.chain_id)
            .await?;

        if let Some(s) = &stub.sponsor {
            tracing::info!(
                paymaster = %redact_url(&svc.url),
                sponsor = %s,
                "paymaster stub sponsor"
            );
        }

        let paymaster = stub
            .paymaster
            .context("pm_getPaymasterStubData missing paymaster")?;
        let paymaster_data = stub
            .paymaster_data
            .context("pm_getPaymasterStubData missing paymasterData")?;
        let stub_len = paymaster_data.len();

        userop.paymaster = Some(paymaster);
        userop.paymaster_data = Some(paymaster_data);
        userop.paymaster_verification_gas_limit = Some(
            stub.paymaster_verification_gas_limit
                .unwrap_or(U256::from(DEFAULT_PAYMASTER_VERIFICATION_GAS_LIMIT)),
        );
        userop.paymaster_post_op_gas_limit = stub.paymaster_post_op_gas_limit;
        if userop.paymaster_post_op_gas_limit.is_none() {
            anyhow::bail!(
                "pm_getPaymasterStubData missing paymasterPostOpGasLimit (required for v0.7)"
            );
        }

        userop.signature = self.sign_userop(&userop)?.into();
        let estimate = self
            .bundlers
            .estimate_user_operation_gas(&userop, self.cfg.entrypoint)
            .await
            .context("bundler estimate userop gas")?;

        userop.call_gas_limit = add_gas_buffer(estimate.call_gas_limit, 10)?;
        userop.verification_gas_limit = add_gas_buffer(estimate.verification_gas, 10)?;
        userop.pre_verification_gas = add_gas_buffer(estimate.pre_verification_gas, 10)?;

        let paymaster_provided_ver = stub.paymaster_verification_gas_limit;
        if stub.is_final == Some(true) && paymaster_provided_ver.is_some() {
        } else {
            let pm_ver = match userop.paymaster_verification_gas_limit {
                Some(v) if v > estimate.paymaster_verification_gas => v,
                _ => estimate.paymaster_verification_gas,
            };
            userop.paymaster_verification_gas_limit = Some(add_gas_buffer(pm_ver, 10)?);
        }

        if stub.is_final != Some(true) {
            let pm_userop_final = to_paymaster_userop(&userop)?;
            let final_data = pool
                .get_data(idx, &pm_userop_final, self.cfg.entrypoint, self.chain_id)
                .await?;

            if let Some(p) = final_data.paymaster
                && p != paymaster
            {
                anyhow::bail!("pm_getPaymasterData returned unexpected paymaster address");
            }
            let paymaster_data = final_data
                .paymaster_data
                .context("pm_getPaymasterData missing paymasterData")?;
            if paymaster_data.len() != stub_len {
                tracing::warn!(
                    paymaster = %redact_url(&svc.url),
                    stub_len,
                    final_len = paymaster_data.len(),
                    "paymasterData length differs between stub and final; bundler preVerificationGas may be off"
                );
            }
            userop.paymaster_data = Some(paymaster_data);
        }

        userop.signature = self.sign_userop(&userop)?.into();
        let resp = self
            .bundlers
            .send_user_operation(&userop, self.cfg.entrypoint)
            .await
            .context("bundler send userop")?;

        Ok(HubUserOpSubmission {
            userop_hash: hex_bytes0x(&resp.user_op_hash),
            nonce: userop.nonce,
        })
    }

    fn sign_userop(&self, op: &PackedUserOperation) -> Result<Vec<u8>> {
        sign_userop_with_key(
            &self.owner_key,
            self.chain_id,
            self.cfg.safe_4337_module,
            self.cfg.entrypoint,
            op,
        )
    }
}

fn make_paymaster_pool(cfgs: &[PaymasterServiceConfig]) -> Result<Option<PaymasterPool>> {
    if cfgs.is_empty() {
        return Ok(None);
    }
    let services = cfgs
        .iter()
        .map(|c| PaymasterService {
            url: c.url.clone(),
            context: c.context.clone(),
        })
        .collect::<Vec<_>>();
    Ok(Some(PaymasterPool::new(services)?))
}

fn to_paymaster_userop(op: &PackedUserOperation) -> Result<PaymasterUserOp> {
    Ok(PaymasterUserOp {
        sender: op.sender,
        nonce: op.nonce,
        call_data: op.call_data.clone(),
        call_gas_limit: op.call_gas_limit,
        verification_gas_limit: op.verification_gas_limit,
        pre_verification_gas: op.pre_verification_gas,
        max_fee_per_gas: op.max_fee_per_gas,
        max_priority_fee_per_gas: op.max_priority_fee_per_gas,
        factory: op.factory,
        factory_data: op.factory_data.clone(),
        paymaster: op.paymaster,
        paymaster_verification_gas_limit: op.paymaster_verification_gas_limit,
        paymaster_post_op_gas_limit: op.paymaster_post_op_gas_limit,
        paymaster_data: None,
    })
}
