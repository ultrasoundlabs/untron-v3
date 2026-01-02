use anyhow::{Context, Result};

use crate::domain;

#[derive(Clone)]
pub(super) struct ValidatedLog {
    pub(super) log: alloy::rpc::types::Log,
    pub(super) block_number: u64,
    pub(super) block_hash: domain::BlockHash,
    pub(super) tx_hash: domain::TxHash,
    pub(super) log_index: u32,
    pub(super) block_timestamp: Option<u64>,
}

pub(super) fn validate_logs(logs: Vec<alloy::rpc::types::Log>) -> Result<Vec<ValidatedLog>> {
    logs.into_iter()
        .map(|l| {
            let block_timestamp = l.block_timestamp;
            let block_number = l
                .block_number
                .with_context(|| format!("log missing block_number: {:?}", l))?;
            let block_hash = l
                .block_hash
                .with_context(|| format!("log missing block_hash: {:?}", l))?;
            let tx_hash = l
                .transaction_hash
                .with_context(|| format!("log missing transaction_hash: {:?}", l))?;
            let log_index = l
                .log_index
                .with_context(|| format!("log missing log_index: {:?}", l))?;
            let log_index = u32::try_from(log_index).context("log_index out of range for u32")?;
            Ok(ValidatedLog {
                log: l,
                block_number,
                block_hash: domain::BlockHash(block_hash),
                tx_hash: domain::TxHash(tx_hash),
                log_index,
                block_timestamp,
            })
        })
        .collect()
}
