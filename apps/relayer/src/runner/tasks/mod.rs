mod controller_sync;
mod hub_ops;
mod liquidity;
mod rebalance;

use crate::tron::address::TronAddress;
use alloy::primitives::{Address, FixedBytes, U256};
pub use controller_sync::{
    execute_controller_tip_proof, plan_controller_tip_proof, plan_relay_controller_chain,
};
pub use hub_ops::{execute_hub_intent, plan_pre_entitle, plan_process_controller_events};
pub use liquidity::{LiquidityIntent, execute_liquidity_intent, plan_liquidity};
pub use rebalance::{execute_controller_rebalance, plan_controller_rebalance};
use untron_v3_bindings::untron_v3::UntronV3Base::ControllerEvent;

pub const JOB_CONTROLLER_TIP_PROOF: &str = "controller_tip_proof";
pub const JOB_RELAY_CONTROLLER_CHAIN: &str = "relay_controller_chain";
pub const JOB_PROCESS_CONTROLLER_EVENTS: &str = "process_controller_events";
pub const JOB_PRE_ENTITLE: &str = "pre_entitle";
pub const JOB_FILL_CLAIMS: &str = "fill_claims";
pub const JOB_PULL_FROM_RECEIVERS: &str = "pull_from_receivers";
pub const JOB_CONTROLLER_REBALANCE: &str = "controller_rebalance";

fn tron_block_finalized(
    block_number: u64,
    tron_head: u64,
    finality_blocks: u64,
    block_lag: u64,
) -> bool {
    block_number
        .saturating_add(finality_blocks)
        .saturating_add(block_lag)
        <= tron_head
}

#[derive(Debug, Clone)]
pub enum HubIntent {
    RelayControllerEventChain {
        proof_txid: [u8; 32],
        events: Vec<ControllerEvent>,
    },
    ProcessControllerEvents,
    PreEntitle {
        receiver_salt: FixedBytes<32>,
        txid: [u8; 32],
    },
    FillClaims {
        target_token: Address,
        max_claims: u64,
    },
}

#[derive(Debug, Clone)]
pub enum TronIntent {
    ProveControllerTip {
        tip_hex: String,
        tip: FixedBytes<32>,
        next_resend_ok_at: u64,
    },
    PullFromReceivers {
        token_tron: TronAddress,
        receiver_salts: Vec<FixedBytes<32>>,
    },
    RebalanceUsdt {
        rebalancers: Vec<TronAddress>,
        in_amount: U256,
    },
}
