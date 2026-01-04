use crate::domain;
use alloy::primitives::B256;
use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};
use std::str::FromStr;

fn encode_hex0x_to_pg(buf: &mut PgArgumentBuffer, bytes: &[u8]) {
    buf.extend_from_slice(b"0x");
    let encoded = hex::encode(bytes);
    buf.extend_from_slice(encoded.as_bytes());
}

macro_rules! pg_hex0x_b256_impls {
    ($ty:ty) => {
        impl Type<Postgres> for $ty {
            fn type_info() -> PgTypeInfo {
                <&str as Type<Postgres>>::type_info()
            }

            fn compatible(ty: &PgTypeInfo) -> bool {
                <&str as Type<Postgres>>::compatible(ty)
            }
        }

        impl Encode<'_, Postgres> for $ty {
            fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
                encode_hex0x_to_pg(buf, self.0.as_ref());
                Ok(IsNull::No)
            }

            fn size_hint(&self) -> usize {
                2 + (32 * 2)
            }
        }

        impl Decode<'_, Postgres> for $ty {
            fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
                let s = <&str as Decode<Postgres>>::decode(value)?;
                Ok(Self(B256::from_str(s)?))
            }
        }
    };
}

pg_hex0x_b256_impls!(domain::BlockHash);
pg_hex0x_b256_impls!(domain::TxHash);
pg_hex0x_b256_impls!(domain::Tip);
pg_hex0x_b256_impls!(domain::EventSignature);

impl Type<Postgres> for domain::AbiEncodedEventData {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl Encode<'_, Postgres> for domain::AbiEncodedEventData {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        encode_hex0x_to_pg(buf, self.0.as_ref());
        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        2 + (self.0.len() * 2)
    }
}

impl Type<Postgres> for domain::TronAddress {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl Encode<'_, Postgres> for domain::TronAddress {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        buf.extend_from_slice(self.to_base58check().as_bytes());
        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        34
    }
}

impl Decode<'_, Postgres> for domain::TronAddress {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        Ok(domain::TronAddress::from_base58check(s)?)
    }
}
