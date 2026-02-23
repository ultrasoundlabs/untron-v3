pub mod address;
pub mod grpc;
pub mod proof;
pub mod rental;
pub mod resources;
pub mod sender;
pub mod tx;
pub mod wallet;

pub use address::TronAddress;
pub use grpc::TronGrpc;
pub use proof::{TronTxProofBuilder, TronTxProofBundle};
pub use rental::{
    JsonApiRentalProvider, JsonApiRentalProviderConfig, RentalAttempt, RentalContext,
    RentalResourceKind,
};
pub use resources::{AccountResources, ChainFees, TxCostQuote};
pub use sender::{FIXED_FEE_LIMIT_SUN, SignedTronTx};
pub use tx::{
    DecodedTrc20Call, DecodedTriggerSmartContract, SELECTOR_TRANSFER, SELECTOR_TRANSFER_FROM,
    TRIGGER_SMART_CONTRACT_TYPE, decode_trc20_call_data, decode_trigger_smart_contract,
};
pub use wallet::{BroadcastedTronTx, TronWallet};

pub mod protocol {
    #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
    #![allow(
        dead_code,
        unused_imports,
        unused_variables,
        non_snake_case,
        non_camel_case_types,
        non_upper_case_globals
    )]

    tonic::include_proto!("protocol");
}
