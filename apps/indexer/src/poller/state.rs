use crate::{config::Stream, domain};
use alloy::primitives::Address;

#[derive(Clone)]
pub(super) struct PollState {
    pub(super) stream: Stream,
    pub(super) chain_id: i64,
    pub(super) contract_address_db: domain::ContractAddressDb,
    pub(super) contract_address_rpc: Address,

    pub(super) confirmations: u64,
    pub(super) reorg_scan_depth: u64,

    pub(super) chunk_target: u64,
    pub(super) chunk_current: u64,

    pub(super) provider: alloy::providers::DynProvider,
    pub(super) pinned_providers: Vec<alloy::providers::DynProvider>,

    pub(super) timestamps: super::timestamps::TimestampState,
}
