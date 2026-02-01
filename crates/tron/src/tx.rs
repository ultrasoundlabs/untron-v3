use crate::address::TronAddress;
use crate::protocol::{Transaction, TriggerSmartContract};
use alloy::primitives::{Address, U256};
use anyhow::{Context, Result};
use prost::Message;

pub const TRIGGER_SMART_CONTRACT_TYPE: i32 = 31;
pub const SELECTOR_TRANSFER: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];
pub const SELECTOR_TRANSFER_FROM: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];

#[derive(Debug, Clone)]
pub struct DecodedTriggerSmartContract {
    pub owner: TronAddress,
    pub contract: TronAddress,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum DecodedTrc20Call {
    Transfer {
        from: TronAddress,
        to: TronAddress,
        amount: U256,
    },
    TransferFrom {
        from: TronAddress,
        to: TronAddress,
        amount: U256,
    },
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

pub fn decode_trc20_call_data(data: &[u8], sender: TronAddress) -> Result<DecodedTrc20Call> {
    if data.len() < 4 {
        anyhow::bail!(
            "invalid TRC-20 calldata length: expected >= 4, got {}",
            data.len()
        );
    }
    let selector: [u8; 4] = data[0..4].try_into().expect("slice is 4 bytes");

    fn evm_addr_from_word(word: &[u8]) -> Address {
        Address::from_slice(&word[12..32])
    }

    if selector == SELECTOR_TRANSFER {
        if data.len() != 4 + 32 * 2 {
            anyhow::bail!(
                "invalid TRC-20 transfer calldata length: expected {}, got {}",
                4 + 32 * 2,
                data.len()
            );
        }
        let word1 = &data[4..36];
        let word2 = &data[36..68];

        let to = TronAddress::from_evm(evm_addr_from_word(word1));
        let amount = U256::from_be_slice(word2);
        Ok(DecodedTrc20Call::Transfer {
            from: sender,
            to,
            amount,
        })
    } else if selector == SELECTOR_TRANSFER_FROM {
        if data.len() != 4 + 32 * 3 {
            anyhow::bail!(
                "invalid TRC-20 transferFrom calldata length: expected {}, got {}",
                4 + 32 * 3,
                data.len()
            );
        }
        let w1 = &data[4..36];
        let w2 = &data[36..68];
        let w3 = &data[68..100];

        let from = TronAddress::from_evm(evm_addr_from_word(w1));
        let to = TronAddress::from_evm(evm_addr_from_word(w2));
        let amount = U256::from_be_slice(w3);
        Ok(DecodedTrc20Call::TransferFrom { from, to, amount })
    } else {
        anyhow::bail!("unrecognized TRC-20 selector: 0x{}", hex::encode(selector));
    }
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

    #[test]
    fn decode_trc20_call_data_decodes_transfer_fixture() {
        let path = workspace_path("testdata/fixtures/trc20_tx_78115149.sample.json");
        let json = std::fs::read_to_string(path).expect("read trc20 fixture json");
        let fixture: Trc20TxSampleFile =
            serde_json::from_str(&json).expect("parse trc20 fixture json");

        let encoded_tx_bytes = decode_hex0x(&fixture.tx.encoded_tx);
        let tx = crate::protocol::Transaction::decode(encoded_tx_bytes.as_slice())
            .expect("decode Transaction");

        let decoded = decode_trigger_smart_contract(&tx).expect("decode trigger smart contract");
        let call =
            decode_trc20_call_data(&decoded.data, decoded.owner).expect("decode trc20 call data");

        match call {
            DecodedTrc20Call::Transfer { .. } => {}
            DecodedTrc20Call::TransferFrom { .. } => panic!("fixture expected transfer"),
        }
    }
}
