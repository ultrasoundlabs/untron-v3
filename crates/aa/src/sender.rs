use crate::bundler_pool::BundlerPool;
use crate::contracts::{IEntryPointNonces, Safe4337Module};
use crate::packing::{add_gas_buffer, hex_bytes0x, redact_url};
use crate::paymaster::{PaymasterPool, PaymasterService, PaymasterUserOp};
use crate::signing::sign_userop_with_key;
use alloy::sol_types::SolCall;
use alloy::{
    primitives::{Address, Bytes, U256},
    providers::{DynProvider, Provider, ProviderBuilder},
    rpc::client::{BuiltInConnectionString, RpcClient},
};
use anyhow::{Context, Result};
use k256::ecdsa::SigningKey;

use alloy::rpc::types::eth::erc4337::PackedUserOperation;

const DEFAULT_PAYMASTER_VERIFICATION_GAS_LIMIT: u64 = 500_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymasterFinalizationMode {
    SkipIfStubFinal,
    AlwaysFetchFinal,
}

#[derive(Debug, Clone)]
pub struct Safe4337UserOpSenderOptions {
    pub check_bundler_entrypoints: bool,
    pub paymaster_finalization: PaymasterFinalizationMode,
}

impl Default for Safe4337UserOpSenderOptions {
    fn default() -> Self {
        Self {
            check_bundler_entrypoints: false,
            paymaster_finalization: PaymasterFinalizationMode::AlwaysFetchFinal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Safe4337UserOpSenderConfig {
    pub rpc_url: String,
    pub chain_id: Option<u64>,
    pub entrypoint: Address,
    pub safe: Address,
    pub safe_4337_module: Address,
    pub bundler_urls: Vec<String>,
    pub owner_private_key: [u8; 32],
    pub paymasters: Vec<PaymasterService>,
    pub options: Safe4337UserOpSenderOptions,
}

pub struct Safe4337UserOpSender {
    cfg: Safe4337UserOpSenderConfig,
    provider: DynProvider,
    chain_id: u64,
    owner_key: SigningKey,
    bundlers: BundlerPool,
    paymasters: Option<PaymasterPool>,
}

#[derive(Debug, Clone)]
pub struct Safe4337UserOpSubmission {
    pub userop_hash: String,
    pub nonce: U256,
}

impl Safe4337UserOpSender {
    pub async fn new(cfg: Safe4337UserOpSenderConfig) -> Result<Self> {
        let transport = BuiltInConnectionString::connect(&cfg.rpc_url)
            .await
            .with_context(|| format!("connect rpc: {}", cfg.rpc_url))?;
        let client = RpcClient::builder().transport(transport, false);
        let provider = ProviderBuilder::default().connect_client(client);
        let provider = DynProvider::new(provider);

        let chain_id = match cfg.chain_id {
            Some(id) => id,
            None => provider.get_chain_id().await.context("eth_chainId")?,
        };

        let owner_key =
            SigningKey::from_slice(&cfg.owner_private_key).context("invalid owner private key")?;

        let mut bundlers = BundlerPool::new(cfg.bundler_urls.clone()).await?;
        if cfg.options.check_bundler_entrypoints {
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
        }

        let paymasters = if cfg.paymasters.is_empty() {
            None
        } else {
            Some(PaymasterPool::new(cfg.paymasters.clone())?)
        };

        Ok(Self {
            cfg,
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

    pub async fn send_call(
        &mut self,
        to: Address,
        data: Vec<u8>,
    ) -> Result<Safe4337UserOpSubmission> {
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

        let call_data = Safe4337Module::executeUserOpCall {
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
    ) -> Result<Safe4337UserOpSubmission> {
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

        Ok(Safe4337UserOpSubmission {
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
    ) -> Result<Safe4337UserOpSubmission> {
        let pm_userop = to_paymaster_userop(&userop, self.cfg.options.paymaster_finalization)?;
        let stub = pool
            .get_stub_data(idx, &pm_userop, self.cfg.entrypoint, self.chain_id)
            .await?;

        if self.cfg.options.paymaster_finalization == PaymasterFinalizationMode::SkipIfStubFinal
            && let Some(s) = &stub.sponsor
        {
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

        match self.cfg.options.paymaster_finalization {
            PaymasterFinalizationMode::SkipIfStubFinal => {
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
                    let pm_userop_final =
                        to_paymaster_userop(&userop, self.cfg.options.paymaster_finalization)?;
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
            }
            PaymasterFinalizationMode::AlwaysFetchFinal => {
                let pm_userop =
                    to_paymaster_userop(&userop, self.cfg.options.paymaster_finalization)?;
                let final_data = pool
                    .get_data(idx, &pm_userop, self.cfg.entrypoint, self.chain_id)
                    .await?;

                let paymaster = final_data
                    .paymaster
                    .context("pm_getPaymasterData missing paymaster")?;
                let paymaster_data = final_data
                    .paymaster_data
                    .context("pm_getPaymasterData missing paymasterData")?;
                userop.paymaster = Some(paymaster);
                userop.paymaster_data = Some(paymaster_data);
            }
        }

        userop.signature = self.sign_userop(&userop)?.into();
        let resp = self
            .bundlers
            .send_user_operation(&userop, self.cfg.entrypoint)
            .await
            .context("bundler send userop")?;

        if self.cfg.options.paymaster_finalization == PaymasterFinalizationMode::AlwaysFetchFinal {
            tracing::info!(paymaster = %redact_url(&svc.url), "userop sponsored");
        }

        Ok(Safe4337UserOpSubmission {
            userop_hash: hex_bytes0x(&resp.user_op_hash),
            nonce: userop.nonce,
        })
    }

    fn sign_userop(&self, userop: &PackedUserOperation) -> Result<Vec<u8>> {
        sign_userop_with_key(
            &self.owner_key,
            self.chain_id,
            self.cfg.safe_4337_module,
            self.cfg.entrypoint,
            userop,
        )
    }
}

fn to_paymaster_userop(
    op: &PackedUserOperation,
    mode: PaymasterFinalizationMode,
) -> Result<PaymasterUserOp> {
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
        paymaster_data: match mode {
            PaymasterFinalizationMode::SkipIfStubFinal => None,
            PaymasterFinalizationMode::AlwaysFetchFinal => op.paymaster_data.clone(),
        },
    })
}
