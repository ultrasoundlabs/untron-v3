mod tron;

use alloy::primitives::{B256, Bytes};
use std::fmt;

pub use tron::TronAddress;

pub fn fmt_hex0x(bytes: &[u8], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("0x")?;
    f.write_str(&hex::encode(bytes))?;
    Ok(())
}

macro_rules! string_newtype_with_as_str {
    ($(#[$meta:meta])* $vis:vis struct $name:ident;) => {
        $(#[$meta])*
        $vis struct $name(String);

        impl $name {
            pub fn new(value: String) -> Self {
                Self(value)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

macro_rules! hex0x_b256_newtype {
    ($(#[$meta:meta])* $vis:vis struct $name:ident;) => {
        $(#[$meta])*
        $vis struct $name(pub B256);

        impl From<B256> for $name {
            fn from(value: B256) -> Self {
                Self(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt_hex0x(self.0.as_ref(), f)
            }
        }
    };
}

macro_rules! hex0x_bytes_newtype {
    ($(#[$meta:meta])* $vis:vis struct $name:ident;) => {
        $(#[$meta])*
        $vis struct $name(pub Bytes);

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt_hex0x(self.0.as_ref(), f)
            }
        }
    };
}

string_newtype_with_as_str! {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct HubContractAddressDb;
}

string_newtype_with_as_str! {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct ControllerContractAddressDb;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ContractAddressDb {
    Hub(HubContractAddressDb),
    Controller(ControllerContractAddressDb),
}

impl ContractAddressDb {
    pub fn as_str(&self) -> &str {
        match self {
            ContractAddressDb::Hub(a) => a.as_str(),
            ContractAddressDb::Controller(a) => a.as_str(),
        }
    }
}

impl fmt::Display for ContractAddressDb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

hex0x_b256_newtype! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct BlockHash;
}

hex0x_b256_newtype! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct TxHash;
}

hex0x_b256_newtype! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Tip;
}

hex0x_b256_newtype! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct EventSignature;
}

hex0x_bytes_newtype! {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AbiEncodedEventData;
}
