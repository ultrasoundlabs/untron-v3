use alloy::primitives::{Address, B256, Bytes};
use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};
use std::fmt;
use std::str::FromStr;

fn encode_hex0x_to_pg(buf: &mut PgArgumentBuffer, bytes: &[u8]) {
    buf.extend_from_slice(b"0x");
    let start = buf.len();
    buf.resize(start + (bytes.len() * 2), 0);
    alloy::primitives::hex::encode_to_slice(bytes, &mut buf[start..])
        .expect("hex output length is fixed");
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HubContractAddressDb(String);

impl HubContractAddressDb {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HubContractAddressDb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ControllerContractAddressDb(String);

impl ControllerContractAddressDb {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ControllerContractAddressDb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockHash(pub B256);

impl From<B256> for BlockHash {
    fn from(value: B256) -> Self {
        Self(value)
    }
}

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::util::fmt_hex0x(self.0.as_ref(), f)
    }
}

impl Type<Postgres> for BlockHash {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl Encode<'_, Postgres> for BlockHash {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        encode_hex0x_to_pg(buf, self.0.as_ref());
        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        2 + (32 * 2)
    }
}

impl Decode<'_, Postgres> for BlockHash {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        Ok(Self(B256::from_str(s)?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TxHash(pub B256);

impl From<B256> for TxHash {
    fn from(value: B256) -> Self {
        Self(value)
    }
}

impl fmt::Display for TxHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::util::fmt_hex0x(self.0.as_ref(), f)
    }
}

impl Type<Postgres> for TxHash {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl Encode<'_, Postgres> for TxHash {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        encode_hex0x_to_pg(buf, self.0.as_ref());
        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        2 + (32 * 2)
    }
}

impl Decode<'_, Postgres> for TxHash {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        Ok(Self(B256::from_str(s)?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tip(pub B256);

impl From<B256> for Tip {
    fn from(value: B256) -> Self {
        Self(value)
    }
}

impl fmt::Display for Tip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::util::fmt_hex0x(self.0.as_ref(), f)
    }
}

impl Type<Postgres> for Tip {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl Encode<'_, Postgres> for Tip {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        encode_hex0x_to_pg(buf, self.0.as_ref());
        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        2 + (32 * 2)
    }
}

impl Decode<'_, Postgres> for Tip {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        Ok(Self(B256::from_str(s)?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EventSignature(pub B256);

impl From<B256> for EventSignature {
    fn from(value: B256) -> Self {
        Self(value)
    }
}

impl fmt::Display for EventSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::util::fmt_hex0x(self.0.as_ref(), f)
    }
}

impl Type<Postgres> for EventSignature {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl Encode<'_, Postgres> for EventSignature {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        encode_hex0x_to_pg(buf, self.0.as_ref());
        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        2 + (32 * 2)
    }
}

impl Decode<'_, Postgres> for EventSignature {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        Ok(Self(B256::from_str(s)?))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiEncodedEventData(pub Bytes);

impl fmt::Display for AbiEncodedEventData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::util::fmt_hex0x(self.0.as_ref(), f)
    }
}

impl Type<Postgres> for AbiEncodedEventData {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl Encode<'_, Postgres> for AbiEncodedEventData {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        encode_hex0x_to_pg(buf, self.0.as_ref());
        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        2 + (self.0.len() * 2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Caller(pub Address);

impl fmt::Display for Caller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Type<Postgres> for Caller {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl Encode<'_, Postgres> for Caller {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let checksum = self.0.to_checksum_buffer(None);
        buf.extend_from_slice(checksum.as_str().as_bytes());
        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        42
    }
}
