use crate::{config::Stream, util};
use alloy::primitives::Address;
use anyhow::{Context, Result};
use std::{future::Future, time::Duration};
use tokio::time;
use tokio_util::sync::CancellationToken;

pub(super) async fn await_or_cancel<T>(
    shutdown: &CancellationToken,
    fut: impl Future<Output = Result<T>>,
) -> Result<Option<T>> {
    tokio::select! {
        _ = shutdown.cancelled() => Ok(None),
        res = fut => Ok(Some(res?)),
    }
}

pub(super) async fn sleep_or_cancel(
    shutdown: &CancellationToken,
    duration: Duration,
) -> Result<()> {
    tokio::select! {
        _ = shutdown.cancelled() => Ok(()),
        _ = time::sleep(duration) => Ok(()),
    }
}

pub(super) fn resolve_rpc_contract_address(
    stream: Stream,
    contract_address_db: &str,
) -> Result<Address> {
    match stream {
        Stream::Hub => contract_address_db
            .parse::<Address>()
            .with_context(|| format!("invalid hub contract address: {contract_address_db}")),
        Stream::Controller => util::tron_base58_to_evm_address(contract_address_db),
    }
}
