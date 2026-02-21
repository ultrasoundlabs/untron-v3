use crate::bundler_pool::BundlerPool;
use crate::contracts::{IEntryPointDeposits, IEntryPointNonces, Safe4337Module};
use crate::packing::{add_gas_buffer, hex_bytes0x, redact_url};
use crate::paymaster::{PaymasterPool, PaymasterService, PaymasterUserOp};
use crate::safe::{Safe4337Config, SafeDeterministicDeploymentConfig, ensure_safe_deployed};
use crate::signing::sign_userop_with_key;
use alloy::sol_types::SolCall;
use alloy::{
    primitives::{Address, Bytes, U256},
    providers::{DynProvider, Provider, ProviderBuilder},
    rpc::types::{TransactionInput, TransactionRequest},
};
use anyhow::{Context, Result};
use k256::ecdsa::SigningKey;
use std::time::Duration;

use alloy::rpc::types::eth::erc4337::{PackedUserOperation, UserOperationReceipt};

const GAS_BUFFER_PCT: u64 = 10;
const PAYMASTER_POST_OP_GAS_BUFFER_PCT: u64 = 10;

// ERC-4337 bundler sanity checks expect paymasterVerificationGasLimit < MAX_VERIFICATION_GAS (500_000).
const MAX_PAYMASTER_VERIFICATION_GAS_LIMIT_EXCLUSIVE: u64 = 500_000;
const DEFAULT_PAYMASTER_VERIFICATION_GAS_LIMIT: u64 = 450_000;
const MIN_PRIORITY_FEE_WEI: u128 = 200_000;

fn cap_paymaster_verification_gas_limit(v: U256) -> (U256, bool) {
    let max = U256::from(MAX_PAYMASTER_VERIFICATION_GAS_LIMIT_EXCLUSIVE - 1);
    if v > max { (max, true) } else { (v, false) }
}

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
    pub safe: Option<Address>,
    pub safe_4337_module: Address,
    pub safe_deployment: Option<SafeDeterministicDeploymentConfig>,
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
    safe: Address,
    bundlers: BundlerPool,
    paymasters: Option<PaymasterPool>,
}

#[derive(Debug, Clone)]
pub struct Safe4337UserOpSubmission {
    pub userop_hash: String,
    pub nonce: U256,
    /// Number of eth_sendUserOperation attempts performed (including the successful one).
    pub send_attempts: u64,
}

impl Safe4337UserOpSender {
    async fn preflight_safe_execute_userop_call(&self, call_data: Bytes) -> Result<()> {
        let tx = TransactionRequest {
            from: Some(self.cfg.entrypoint),
            to: Some(self.safe.into()),
            input: TransactionInput::new(call_data.clone()),
            ..Default::default()
        };

        match self.provider.call(tx).await {
            Ok(_) => Ok(()),
            Err(err) => {
                let err_fmt = format!("{err:#}");
                tracing::error!(
                    safe = %self.safe,
                    entrypoint = %self.cfg.entrypoint,
                    err = %err_fmt,
                    "preflight eth_call for Safe4337 executeUserOp failed"
                );
                Err(anyhow::Error::new(err).context("preflight eth_call safe executeUserOp"))
            }
        }
    }

    pub async fn new(cfg: Safe4337UserOpSenderConfig) -> Result<Self> {
        // NOTE: cfg.rpc_url is treated as a CSV list of HTTP(S) JSON-RPC endpoints.
        // We use a best-effort per-request failover transport (sticky on success).
        let client = untron_rpc_fallback::rpc_client_from_urls_csv(
            &cfg.rpc_url,
            std::time::Duration::from_secs(4),
        )
        .with_context(|| format!("connect rpc (fallback csv): {}", cfg.rpc_url))?;
        let provider = ProviderBuilder::default().connect_client(client);
        let provider = DynProvider::new(provider);

        let chain_id = match cfg.chain_id {
            Some(id) => id,
            None => provider.get_chain_id().await.context("eth_chainId")?,
        };

        let owner_key =
            SigningKey::from_slice(&cfg.owner_private_key).context("invalid owner private key")?;

        let safe = match cfg.safe {
            Some(addr) if addr != Address::ZERO => addr,
            _ => {
                let deploy = cfg
                    .safe_deployment
                    .clone()
                    .context("HUB_SAFE_ADDRESS is not set; HUB_SAFE_PROXY_FACTORY_ADDRESS/HUB_SAFE_SINGLETON_ADDRESS/HUB_SAFE_MODULE_SETUP_ADDRESS must be set")?;
                let safe_4337 = Safe4337Config {
                    entrypoint: cfg.entrypoint,
                    safe_4337_module: cfg.safe_4337_module,
                };
                ensure_safe_deployed(
                    &cfg.rpc_url,
                    chain_id,
                    cfg.owner_private_key,
                    &safe_4337,
                    &deploy,
                )
                .await
                .context("ensure safe deployed")?
            }
        };

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

        let cfg = Safe4337UserOpSenderConfig {
            safe: Some(safe),
            ..cfg
        };

        Ok(Self {
            cfg,
            provider,
            chain_id,
            owner_key,
            safe,
            bundlers,
            paymasters,
        })
    }

    pub async fn current_nonce(&self) -> Result<U256> {
        self.entrypoint()
            .getNonce(self.safe, alloy::primitives::Uint::<192, 3>::ZERO)
            .call()
            .await
            .context("EntryPoint.getNonce")
    }

    pub fn safe_address(&self) -> Address {
        self.safe
    }

    pub async fn wait_user_operation_receipt(
        &mut self,
        userop_hash0x: &str,
        timeout: Duration,
    ) -> Result<UserOperationReceipt> {
        let start = std::time::Instant::now();

        let userop_hash0x = userop_hash0x.trim();
        let hex_str = userop_hash0x.strip_prefix("0x").unwrap_or(userop_hash0x);
        let bytes = hex::decode(hex_str).context("invalid userop hash hex")?;
        let userop_hash = Bytes::from(bytes);

        let deadline = std::time::Instant::now() + timeout;
        let mut backoff = Duration::from_millis(250);

        loop {
            match self
                .bundlers
                .get_user_operation_receipt(userop_hash.clone())
                .await
            {
                Ok(Some(r)) => {
                    tracing::info!(
                        ms = start.elapsed().as_millis() as u64,
                        "eth_getUserOperationReceipt resolved"
                    );
                    return Ok(r);
                }
                Ok(None) => {
                    // Not found yet.
                }
                Err(e) => {
                    tracing::warn!(err = %format!("{e:#}"), "eth_getUserOperationReceipt failed; will fall back / retry");
                }
            }

            if std::time::Instant::now() >= deadline {
                anyhow::bail!("timed out waiting for eth_getUserOperationReceipt");
            }

            tokio::time::sleep(backoff).await;
            backoff = (backoff * 2).min(Duration::from_secs(2));
        }
    }

    pub async fn send_call(
        &mut self,
        to: Address,
        data: Vec<u8>,
    ) -> Result<Safe4337UserOpSubmission> {
        let nonce = self.current_nonce().await?;
        self.send_call_with_nonce(nonce, to, data).await
    }

    pub async fn send_call_with_nonce(
        &mut self,
        nonce: U256,
        to: Address,
        data: Vec<u8>,
    ) -> Result<Safe4337UserOpSubmission> {
        self.send_call_with_nonce_operation(nonce, to, data, 0)
            .await
    }

    pub async fn send_call_with_nonce_operation(
        &mut self,
        nonce: U256,
        to: Address,
        data: Vec<u8>,
        operation: u8,
    ) -> Result<Safe4337UserOpSubmission> {
        // Prefer standard EIP-1559 fee estimation (eth_feeHistory). This avoids bundler-specific gas APIs.
        let (max_fee_per_gas, max_priority_fee_per_gas) = match self
            .provider
            .estimate_eip1559_fees()
            .await
        {
            Ok(est) => {
                let mut max_fee = est.max_fee_per_gas;
                let max_priority = est.max_priority_fee_per_gas.max(MIN_PRIORITY_FEE_WEI);
                if max_fee < max_priority {
                    max_fee = max_priority;
                }
                (U256::from(max_fee), U256::from(max_priority))
            }
            Err(err) => {
                // Fallback for non-EIP-1559 chains / RPCs: eth_gasPrice with a 2x buffer.
                tracing::warn!(err = %err, "estimate_eip1559_fees failed; falling back to eth_gasPrice");
                let gas_price: u128 = self
                    .provider
                    .get_gas_price()
                    .await
                    .context("eth_gasPrice")?;
                let max_fee = gas_price.saturating_mul(2);
                (
                    U256::from(max_fee),
                    U256::from(MIN_PRIORITY_FEE_WEI.min(max_fee)),
                )
            }
        };

        let call_data = Safe4337Module::executeUserOpCall {
            to,
            value: U256::ZERO,
            data: data.into(),
            operation,
        }
        .abi_encode();
        self.preflight_safe_execute_userop_call(call_data.clone().into())
            .await?;

        let base_userop = PackedUserOperation {
            sender: self.safe,
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

        if self.cfg.paymasters.is_empty() {
            tracing::warn!(
                safe = %self.safe,
                entrypoint = %self.cfg.entrypoint,
                "no paymasters configured; attempting self-paid userop"
            );
        }

        self.send_self_paid(base_userop).await
    }

    fn entrypoint(&self) -> IEntryPointNonces::IEntryPointNoncesInstance<&DynProvider> {
        IEntryPointNonces::new(self.cfg.entrypoint, &self.provider)
    }

    fn entrypoint_deposits(
        &self,
    ) -> IEntryPointDeposits::IEntryPointDepositsInstance<&DynProvider> {
        IEntryPointDeposits::new(self.cfg.entrypoint, &self.provider)
    }

    async fn preflight_self_paid(&self) -> Result<()> {
        let deposit = self
            .entrypoint_deposits()
            .balanceOf(self.safe)
            .call()
            .await
            .context("EntryPoint.balanceOf")?;
        let eth_balance = self
            .provider
            .get_balance(self.safe)
            .await
            .context("eth_getBalance(safe)")?;

        if deposit.is_zero() && eth_balance.is_zero() {
            anyhow::bail!(
                "safe has no EntryPoint deposit and no ETH balance (cannot self-pay userops): safe={:#x} entrypoint={:#x}; configure a paymaster (HUB_PAYMASTERS_JSON) or fund+deposit for self-paid userops",
                self.safe,
                self.cfg.entrypoint
            );
        }

        if deposit.is_zero() {
            tracing::warn!(
                safe = %self.safe,
                entrypoint = %self.cfg.entrypoint,
                eth_balance = %eth_balance,
                "safe has zero EntryPoint deposit; self-paid userops may fail unless deposit is funded"
            );
        }

        Ok(())
    }

    async fn send_self_paid(
        &mut self,
        mut userop: PackedUserOperation,
    ) -> Result<Safe4337UserOpSubmission> {
        // If we're here, either no paymasters are configured or they all failed. Surface a clearer
        // error than "eth_estimateUserOperationGas" if the Safe is unfunded.
        if self.paymasters.is_none() {
            self.preflight_self_paid().await?;
        }

        userop.signature = self.sign_userop(&userop)?.into();

        let estimate = self
            .bundlers
            .estimate_user_operation_gas(&userop, self.cfg.entrypoint)
            .await
            .context("bundler estimate userop gas")?;

        userop.call_gas_limit = add_gas_buffer(estimate.call_gas_limit, GAS_BUFFER_PCT)?;
        userop.verification_gas_limit =
            add_gas_buffer(estimate.verification_gas_limit, GAS_BUFFER_PCT)?;
        userop.pre_verification_gas =
            add_gas_buffer(estimate.pre_verification_gas, GAS_BUFFER_PCT)?;

        userop.signature = self.sign_userop(&userop)?.into();

        let nonce = userop.nonce;
        let (resp, send_attempts) = self
            .send_user_operation_with_retries(userop)
            .await
            .context("bundler send userop")?;

        Ok(Safe4337UserOpSubmission {
            userop_hash: hex_bytes0x(&resp.user_op_hash),
            nonce,
            send_attempts,
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
                sponsor = %s.name,
                icon = ?s.icon,
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
        let stub_pm_ver = stub.paymaster_verification_gas_limit;
        userop.paymaster_verification_gas_limit =
            Some(stub_pm_ver.unwrap_or(U256::from(DEFAULT_PAYMASTER_VERIFICATION_GAS_LIMIT)));

        let stub_pm_post = stub.paymaster_post_op_gas_limit;
        let pm_post = stub_pm_post.context(
            "pm_getPaymasterStubData missing paymasterPostOpGasLimit (required for v0.7)",
        )?;
        userop.paymaster_post_op_gas_limit =
            Some(add_gas_buffer(pm_post, PAYMASTER_POST_OP_GAS_BUFFER_PCT)?);

        userop.signature = self.sign_userop(&userop)?.into();
        let estimate = self
            .bundlers
            .estimate_user_operation_gas(&userop, self.cfg.entrypoint)
            .await
            .context("bundler estimate userop gas")?;

        userop.call_gas_limit = add_gas_buffer(estimate.call_gas_limit, GAS_BUFFER_PCT)?;
        userop.verification_gas_limit =
            add_gas_buffer(estimate.verification_gas_limit, GAS_BUFFER_PCT)?;
        userop.pre_verification_gas =
            add_gas_buffer(estimate.pre_verification_gas, GAS_BUFFER_PCT)?;

        let pm_ver_base = match userop.paymaster_verification_gas_limit {
            Some(v) if v > estimate.paymaster_verification_gas_limit => v,
            _ => estimate.paymaster_verification_gas_limit,
        };
        let (pm_ver, capped) =
            cap_paymaster_verification_gas_limit(add_gas_buffer(pm_ver_base, GAS_BUFFER_PCT)?);
        if capped {
            tracing::warn!(
                paymaster = %redact_url(&svc.url),
                pm_ver = %pm_ver,
                "capped paymasterVerificationGasLimit to bundler MAX_VERIFICATION_GAS-1"
            );
        }
        userop.paymaster_verification_gas_limit = Some(pm_ver);

        match self.cfg.options.paymaster_finalization {
            PaymasterFinalizationMode::SkipIfStubFinal => {
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

                if let Some(p) = final_data.paymaster
                    && p != paymaster
                {
                    anyhow::bail!("pm_getPaymasterData returned unexpected paymaster address");
                }
                let paymaster_data = final_data
                    .paymaster_data
                    .context("pm_getPaymasterData missing paymasterData")?;
                let final_len = paymaster_data.len();
                if final_len != stub_len {
                    tracing::warn!(
                        paymaster = %redact_url(&svc.url),
                        stub_len,
                        final_len,
                        "paymasterData length differs between stub and final; bundler preVerificationGas may be off"
                    );
                }
                userop.paymaster = Some(paymaster);
                userop.paymaster_data = Some(paymaster_data);
            }
        }

        // Do not mutate any signed UserOperation fields after paymaster finalization.
        userop.signature = self.sign_userop(&userop)?.into();
        let nonce = userop.nonce;
        let (resp, send_attempts) = self
            .send_user_operation_with_retries(userop)
            .await
            .context("bundler send userop")?;

        if self.cfg.options.paymaster_finalization == PaymasterFinalizationMode::AlwaysFetchFinal {
            tracing::info!(paymaster = %redact_url(&svc.url), "userop sponsored");
        }

        Ok(Safe4337UserOpSubmission {
            userop_hash: hex_bytes0x(&resp.user_op_hash),
            nonce,
            send_attempts,
        })
    }

    async fn send_user_operation_with_retries(
        &mut self,
        mut userop: PackedUserOperation,
    ) -> Result<(
        alloy::rpc::types::eth::erc4337::SendUserOperationResponse,
        u64,
    )> {
        let sponsored = userop.paymaster.is_some();

        // Goal: handle transient bundler rejects during gas spikes by retrying with bumped
        // EIP-1559 fees, without relying on bundler-specific gas price RPCs.
        const MAX_RETRIES: usize = 3;

        fn parse_min_priority_fee_wei(msg: &str) -> Option<U256> {
            let needle = "maxPriorityFeePerGas must be at least ";
            let i = msg.find(needle)?;
            let s = &msg[i + needle.len()..];
            let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
            if digits.is_empty() {
                return None;
            }
            digits.parse::<u128>().ok().map(U256::from)
        }

        for attempt in 0..=MAX_RETRIES {
            if attempt > 0 {
                if sponsored {
                    tracing::warn!(
                        attempt,
                        "retrying sponsored eth_sendUserOperation without fee bump"
                    );
                } else {
                    // Always bump by +25% (rounded up) as a generic strategy.
                    let bump_num = U256::from(125u64);
                    let bump_den = U256::from(100u64);
                    userop.max_priority_fee_per_gas =
                        (userop.max_priority_fee_per_gas.saturating_mul(bump_num)
                            + (bump_den - U256::from(1u64)))
                            / bump_den;
                    if userop.max_priority_fee_per_gas < U256::from(MIN_PRIORITY_FEE_WEI) {
                        userop.max_priority_fee_per_gas = U256::from(MIN_PRIORITY_FEE_WEI);
                    }

                    // Keep maxFee >= priority fee.
                    if userop.max_fee_per_gas < userop.max_priority_fee_per_gas {
                        userop.max_fee_per_gas = userop.max_priority_fee_per_gas;
                    }

                    // Any fee change requires re-signing.
                    userop.signature = self.sign_userop(&userop)?.into();

                    tracing::warn!(
                        attempt,
                        max_fee_per_gas = %userop.max_fee_per_gas,
                        max_priority_fee_per_gas = %userop.max_priority_fee_per_gas,
                        "retrying eth_sendUserOperation with bumped fees"
                    );
                }
            }

            match self
                .bundlers
                .send_user_operation(&userop, self.cfg.entrypoint)
                .await
            {
                Ok(resp) => return Ok((resp, (attempt + 1) as u64)),
                Err(err) => {
                    let msg = err.to_string();

                    if sponsored && msg.contains("AA34") {
                        return Err(err);
                    }

                    if !sponsored && let Some(min) = parse_min_priority_fee_wei(&msg) {
                        // If the bundler tells us the minimum, snap up to it (plus a small cushion)
                        // rather than doing blind multiplicative bumps.
                        let cushion = U256::from(110u64);
                        let denom = U256::from(100u64);
                        let target =
                            (min.saturating_mul(cushion) + (denom - U256::from(1u64))) / denom;
                        if userop.max_priority_fee_per_gas < target {
                            userop.max_priority_fee_per_gas = target;
                            if userop.max_fee_per_gas < userop.max_priority_fee_per_gas {
                                userop.max_fee_per_gas = userop.max_priority_fee_per_gas;
                            }
                            userop.signature = self.sign_userop(&userop)?.into();
                            tracing::warn!(
                                attempt,
                                min_required = %min,
                                bumped_to = %userop.max_priority_fee_per_gas,
                                "bundler rejected priority fee; bumping and retrying"
                            );

                            if attempt < MAX_RETRIES {
                                continue;
                            }
                        }
                    }

                    // For other errors, only retry if we have attempts left.
                    if attempt < MAX_RETRIES {
                        tracing::warn!(attempt, err = %format!("{err:#}"), "eth_sendUserOperation failed; will retry");
                        continue;
                    }
                    return Err(err);
                }
            }
        }

        unreachable!("loop returns on success or last error")
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
