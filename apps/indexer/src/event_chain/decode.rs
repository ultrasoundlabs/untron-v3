use crate::config::Stream;
use alloy::{
    primitives::{Address, B256, Bytes, I256, U256},
    sol,
    sol_types::{Error as SolError, SolEventInterface},
};
use anyhow::Result;
use serde::Serialize;
use std::{borrow::Cow, fmt::Display};

// WARNING: you're about to see some of the most blasphemous macro wizardry
// you've likely ever seen. Don't even think about touching this code unless
// you're absolutely sure you know what you're doing.

#[derive(Debug, Clone)]
pub struct AsString<T>(pub T);

impl<T> From<T> for AsString<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Display> Serialize for AsString<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.0)
    }
}

type S<T> = AsString<T>;

macro_rules! sol_ty_to_rust {
    (address) => {
        S<Address>
    };
    (uint256) => {
        S<U256>
    };
    (uint64) => {
        S<u64>
    };
    (uint32) => {
        S<u32>
    };
    (uint8) => {
        S<u8>
    };
    (int256) => {
        S<I256>
    };
    (bytes32) => {
        B256
    };
    (bytes) => {
        Bytes
    };
    (bool) => {
        bool
    };
}

macro_rules! stream_events {
    (
        $Iface:ident,
        decoded = $Decoded:path,
        $Out:ident,
        {
            $(
                $Variant:ident {
                    $( $field:ident : $solty:tt ),* $(,)?
                }
            ),* $(,)?
        }
    ) => {
        sol! {
            interface $Iface {
                $(
                    event $Variant( $( $solty $field ),* );
                )*
            }
        }

        #[derive(Debug, Clone, Serialize)]
        #[serde(tag = "event_type", content = "args")]
        pub enum $Out {
            $(
                $Variant {
                    $( $field: sol_ty_to_rust!($solty), )*
                },
            )*
            Unknown {
                #[serde(skip_serializing)]
                _signature: B256,
                #[serde(skip_serializing)]
                _data: Bytes,
            },
        }

        impl From<$Decoded> for $Out {
            fn from(decoded: $Decoded) -> Self {
                use $Decoded as Decoded;
                match decoded {
                    $(
                        Decoded::$Variant(ev) => $Out::$Variant {
                            $( $field: ev.$field.into(), )*
                        },
                    )*
                }
            }
        }

        impl StreamEvent for $Out {
            type Decoded = $Decoded;

            fn unknown(sig: B256, data: &Bytes) -> Self {
                Self::Unknown {
                    _signature: sig,
                    _data: data.clone(),
                }
            }
        }
    };
}

#[derive(Debug, Clone)]
pub enum SemanticEvent {
    Hub(HubEvent),
    Controller(ControllerEvent),
}

impl SemanticEvent {
    pub fn into_db_parts(self) -> (Cow<'static, str>, serde_json::Value) {
        match self {
            SemanticEvent::Hub(ev) => split_tagged(&ev),
            SemanticEvent::Controller(ev) => split_tagged(&ev),
        }
    }
}

fn split_tagged<T: Serialize>(value: &T) -> (Cow<'static, str>, serde_json::Value) {
    let mut v = serde_json::to_value(value).expect("serializable");
    let obj = v.as_object_mut().expect("tagged enum serializes to object");

    let event_type = obj
        .remove("event_type")
        .and_then(|v| v.as_str().map(|s| Cow::Owned(s.to_string())))
        .unwrap_or(Cow::Borrowed("Unknown"));

    let args = obj
        .remove("args")
        .unwrap_or_else(|| serde_json::Value::Object(Default::default()));

    (event_type, args)
}

// Stream-style interfaces: our stored `abiEncodedEventData` is `abi.encode(...)` of the full
// parameter list (including parameters that are `indexed` in the emitted Solidity events).
// Therefore, we declare these events without `indexed` so all params are decoded from `data`.
stream_events! {
    HubStreamEvents,
    decoded = HubStreamEvents::HubStreamEventsEvents,
    HubEvent,
    {
        OwnershipTransferred { old_owner: address, new_owner: address },
        UsdtSet { usdt: address },
        TronUsdtSet { tron_usdt: address },
        TronReaderSet { reader: address },

        ProtocolFloorSet { floor_ppm: uint256 },
        ProtocolFlatFeeFloorSet { floor_flat_fee: uint64 },
        ProtocolMaxLeaseDurationSet { max_lease_duration_seconds: uint32 },
        LesseePayoutConfigRateLimitSet { max_updates: uint256, window_seconds: uint256 },

        RealtorSet { realtor: address, allowed: bool },
        RealtorMinFeeSet { realtor: address, min_fee_ppm: uint256 },
        RealtorMinFlatFeeSet { realtor: address, min_flat_fee: uint64 },
        RealtorMaxLeaseDurationSet { realtor: address, max_lease_duration_seconds: uint32 },
        RealtorLeaseRateLimitSet { realtor: address, max_leases: uint256, window_seconds: uint256 },

        LpSet { lp: address, allowed: bool },
        LpDeposited { lp: address, amount: uint256 },
        LpWithdrawn { lp: address, amount: uint256 },

        ChainDeprecatedSet { target_chain_id: uint256, deprecated: bool },
        SwapRateSet { target_token: address, rate_ppm: uint256 },
        BridgerSet { target_token: address, target_chain_id: uint256, bridger: address },

        LeaseCreated {
            lease_id: uint256,
            receiver_salt: bytes32,
            lease_number: uint256,
            realtor: address,
            lessee: address,
            start_time: uint64,
            nukeable_after: uint64,
            lease_fee_ppm: uint32,
            flat_fee: uint64,
        },
        LeaseNonceUpdated { lease_id: uint256, nonce: uint256 },
        PayoutConfigUpdated {
            lease_id: uint256,
            target_chain_id: uint256,
            target_token: address,
            beneficiary: address,
        },
        ClaimCreated {
            lease_id: uint256,
            claim_id: uint256,
            target_token: address,
            queue_index: uint256,
            amount_usdt: uint256,
            target_chain_id: uint256,
            beneficiary: address,
            origin: uint8,
            origin_id: bytes32,
            origin_actor: address,
            origin_token: address,
            origin_timestamp: uint64,
            origin_raw_amount: uint256,
        },
        ClaimFilled {
            lease_id: uint256,
            claim_id: uint256,
            target_token: address,
            queue_index: uint256,
            amount_usdt: uint256,
            target_chain_id: uint256,
            beneficiary: address,
        },

        TokensRescued { token: address, amount: uint256 },
        ProtocolPnlUpdated { pnl: int256, delta: int256, reason: uint8 },
        ControllerEventChainTipUpdated {
            previous_tip: bytes32,
            block_number: uint256,
            block_timestamp: uint256,
            event_signature: bytes32,
            abi_encoded_event_data: bytes,
        },
        ControllerEventProcessed {
            event_index: uint256,
            block_number: uint256,
            block_timestamp: uint256,
            event_signature: bytes32,
            abi_encoded_event_data: bytes,
        },
    }
}

stream_events! {
    ControllerStreamEvents,
    decoded = ControllerStreamEvents::ControllerStreamEventsEvents,
    ControllerEvent,
    {
        OwnerChanged { new_owner: address },
        ExecutorChanged { new_executor: address },
        UsdtSet { new_usdt: address },
        LpSet { new_lp: address },
        PayloadSet { rebalancer: address, payload: bytes },
        ReceiverDeployed { receiver: address, salt: bytes32 },
        PulledFromReceiver {
            receiver_salt: bytes32,
            token: address,
            token_amount: uint256,
            exchange_rate: uint256,
            usdt_amount: uint256,
        },
        UsdtRebalanced { in_amount: uint256, out_amount: uint256, rebalancer: address },
        ControllerUsdtTransfer { recipient: address, amount: uint256 },
        LpExchangeRateSet { token: address, exchange_rate: uint256 },
        LpTokensWithdrawn { token: address, amount: uint256 },
    }
}

pub fn decode_semantic_event(
    stream: Stream,
    event_signature: B256,
    abi_encoded_event_data: &Bytes,
) -> Result<SemanticEvent> {
    Ok(match stream {
        Stream::Hub => SemanticEvent::Hub(decode_event::<HubEvent>(
            event_signature,
            abi_encoded_event_data,
        )?),
        Stream::Controller => SemanticEvent::Controller(decode_event::<ControllerEvent>(
            event_signature,
            abi_encoded_event_data,
        )?),
    })
}

trait StreamEvent: Sized + From<Self::Decoded> {
    type Decoded: SolEventInterface;
    fn unknown(sig: B256, data: &Bytes) -> Self;
}

fn decode_event<E: StreamEvent>(event_signature: B256, data: &Bytes) -> Result<E> {
    match <E::Decoded as SolEventInterface>::decode_raw_log(&[event_signature], data.as_ref()) {
        Ok(ev) => Ok(E::from(ev)),
        Err(SolError::InvalidLog { .. }) => Ok(E::unknown(event_signature, data)),
        Err(e) => Err(anyhow::Error::new(e)),
    }
}
