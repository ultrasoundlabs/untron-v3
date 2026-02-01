use crate::address::TronAddress;
use crate::protocol::{Transaction, TriggerSmartContract};
use alloy::primitives::Address;
use anyhow::{Context, Result};
use prost::Message;

pub const TRIGGER_SMART_CONTRACT_TYPE: i32 = 31;

#[derive(Debug, Clone)]
pub struct DecodedTriggerSmartContract {
    pub owner: TronAddress,
    pub contract: TronAddress,
    pub data: Vec<u8>,
}

fn tron_address_from_prefixed_bytes(bytes: &[u8]) -> Result<TronAddress> {
    if bytes.len() != 21 {
        anyhow::bail!(
            "invalid Tron address length: expected 21, got {}",
            bytes.len()
        );
    }
    if bytes[0] != TronAddress::MAINNET_PREFIX {
        anyhow::bail!(
            "invalid Tron address prefix: expected 0x{:02x}, got 0x{:02x}",
            TronAddress::MAINNET_PREFIX,
            bytes[0]
        );
    }
    Ok(TronAddress::from_evm(Address::from_slice(&bytes[1..])))
}

pub fn decode_trigger_smart_contract(tx: &Transaction) -> Result<DecodedTriggerSmartContract> {
    let raw = tx
        .raw_data
        .as_ref()
        .context("missing Transaction.raw_data")?;
    let contract0 = raw
        .contract
        .first()
        .context("missing Transaction.raw_data.contract[0]")?;

    if contract0.r#type != TRIGGER_SMART_CONTRACT_TYPE {
        anyhow::bail!(
            "unexpected tron contract type: expected {}, got {}",
            TRIGGER_SMART_CONTRACT_TYPE,
            contract0.r#type
        );
    }

    let any = contract0
        .parameter
        .as_ref()
        .context("missing Transaction.raw_data.contract[0].parameter")?;
    let trigger = TriggerSmartContract::decode(any.value.as_slice())
        .context("decode TriggerSmartContract")?;

    Ok(DecodedTriggerSmartContract {
        owner: tron_address_from_prefixed_bytes(&trigger.owner_address)
            .context("decode TriggerSmartContract.owner_address")?,
        contract: tron_address_from_prefixed_bytes(&trigger.contract_address)
            .context("decode TriggerSmartContract.contract_address")?,
        data: trigger.data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    fn workspace_path(rel: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
    }

    fn decode_hex0x(s: &str) -> Vec<u8> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        hex::decode(s).expect("valid hex")
    }

    #[derive(Deserialize)]
    struct Trc20TxSampleFixture {
        #[serde(rename = "encodedTx")]
        encoded_tx: String,
        #[serde(rename = "tronTokenEvm")]
        tron_token_evm: String,
    }

    #[derive(Deserialize)]
    struct Trc20TxSampleFile {
        #[serde(rename = "tx")]
        tx: Trc20TxSampleFixture,
    }

    #[test]
    fn decode_trigger_contract_matches_fixture_token() {
        let path = workspace_path("testdata/fixtures/trc20_tx_78115149.sample.json");
        let json = std::fs::read_to_string(path).expect("read trc20 fixture json");
        let fixture: Trc20TxSampleFile =
            serde_json::from_str(&json).expect("parse trc20 fixture json");

        let encoded_tx_bytes = decode_hex0x(&fixture.tx.encoded_tx);
        let tx = crate::protocol::Transaction::decode(encoded_tx_bytes.as_slice())
            .expect("decode Transaction");

        let decoded = decode_trigger_smart_contract(&tx).expect("decode trigger smart contract");

        let expected_contract = Address::from_slice(&decode_hex0x(&fixture.tx.tron_token_evm));
        assert_eq!(decoded.contract.evm(), expected_contract);
    }
}
