use super::grpc::TronGrpc;
use super::protocol::{BlockExtention, BlockHeader};
use alloy::primitives::{FixedBytes, U256};
use anyhow::{Context, Result};
use prost::Message;
use sha2::{Digest, Sha256};

pub struct TronTxProofBundle {
    pub blocks: [Vec<u8>; 20],
    pub encoded_tx: Vec<u8>,
    pub proof: Vec<FixedBytes<32>>,
    pub index: U256,
}

pub struct TronTxProofBuilder {
    /// Must be 19 for the hub's `bytes[20]` proof format.
    pub finality_blocks: u64,
}

impl TronTxProofBuilder {
    pub fn new(finality_blocks: u64) -> Self {
        Self { finality_blocks }
    }

    pub async fn build(&self, grpc: &mut TronGrpc, txid: [u8; 32]) -> Result<TronTxProofBundle> {
        if self.finality_blocks != 19 {
            anyhow::bail!(
                "unsupported Tron finality_blocks: expected 19, got {}",
                self.finality_blocks
            );
        }

        let tx_info = grpc
            .get_transaction_info_by_id(txid)
            .await
            .context("get tx info")?;
        let tron_block_number =
            u64::try_from(tx_info.block_number).context("Tron tx blockNumber out of range")?;

        let head = tron_head_block(grpc).await?;
        if head < tron_block_number + self.finality_blocks {
            anyhow::bail!(
                "tx not finalized: head={}, tx_block={}, need >= {}",
                head,
                tron_block_number,
                tron_block_number + self.finality_blocks
            );
        }

        // Fetch the tx block (with tx list) first.
        let tx_block = grpc
            .get_block_by_num2(i64::try_from(tron_block_number)?)
            .await
            .context("get tx block")?;

        let (tx_index, encoded_tx, tx_trie_root) = extract_tx_and_root(&tx_block, txid)?;

        let leaves = tx_block
            .transactions
            .iter()
            .map(|txe| {
                let tx = txe
                    .transaction
                    .as_ref()
                    .context("missing Transaction in TransactionExtention")?;
                Ok(sha256_bytes32(&tx.encode_to_vec()))
            })
            .collect::<Result<Vec<_>>>()?;

        let (proof, index, computed_root) = merkle_proof_sha256(&leaves, tx_index)?;
        if computed_root.as_slice() != tx_trie_root.as_slice() {
            anyhow::bail!(
                "computed txTrieRoot mismatch (encoding/proof algo bug): computed=0x{}, header=0x{}",
                hex::encode(computed_root),
                hex::encode(tx_trie_root),
            );
        }

        // Fetch 19 blocks after, for the hub's stateful Tron reader.
        let mut blocks: [Vec<u8>; 20] = std::array::from_fn(|_| Vec::new());
        blocks[0] = encode_block_header(&tx_block.block_header)?;
        for (i, block) in blocks.iter_mut().enumerate().skip(1) {
            let num = tron_block_number + (i as u64);
            let b = grpc
                .get_block_by_num2(i64::try_from(num)?)
                .await
                .with_context(|| format!("get block {num}"))?;
            *block = encode_block_header(&b.block_header)?;
        }

        Ok(TronTxProofBundle {
            blocks,
            encoded_tx,
            proof,
            index,
        })
    }
}

async fn tron_head_block(grpc: &mut TronGrpc) -> Result<u64> {
    let b = grpc.get_now_block2().await.context("get now block")?;
    let raw = b
        .block_header
        .as_ref()
        .and_then(|h| h.raw_data.as_ref())
        .context("missing now block header.raw_data")?;
    u64::try_from(raw.number).context("now block number out of range")
}

fn extract_tx_and_root(b: &BlockExtention, txid: [u8; 32]) -> Result<(usize, Vec<u8>, Vec<u8>)> {
    let header_raw = b
        .block_header
        .as_ref()
        .and_then(|h| h.raw_data.as_ref())
        .context("missing block_header.raw_data")?;
    let tx_trie_root = header_raw.tx_trie_root.clone();

    for (idx, txe) in b.transactions.iter().enumerate() {
        if txe.txid.as_slice() == txid {
            let tx = txe
                .transaction
                .as_ref()
                .context("missing Transaction in TransactionExtention")?;
            return Ok((idx, tx.encode_to_vec(), tx_trie_root));
        }
    }

    anyhow::bail!("tx not found in block tx list");
}

fn encode_block_header(h: &Option<BlockHeader>) -> Result<Vec<u8>> {
    let h = h.as_ref().context("missing block_header")?;
    encode_block_header_stateful(h)
}

fn sha256_bytes32(bytes: &[u8]) -> FixedBytes<32> {
    let digest = Sha256::digest(bytes);
    FixedBytes::from_slice(&digest)
}

/// Encode a Tron `BlockHeader` in the exact fixed format expected by `StatefulTronTxReader`.
///
/// The onchain verifier expects:
/// - total length 174 bytes,
/// - `raw_data` length 105 bytes (0x69),
/// - `witness_signature` length 65 bytes (0x41),
/// - and specific field ordering with current-era varint lengths:
///   - timestamp (ms): 6 bytes varint
///   - block number: 4 bytes varint
///
/// If any of these assumptions are violated (future-proofing / different Tron network),
/// we fail fast so the relayer doesn't submit transactions that are guaranteed to revert.
fn encode_block_header_stateful(h: &BlockHeader) -> Result<Vec<u8>> {
    let raw = h
        .raw_data
        .as_ref()
        .context("missing block_header.raw_data")?;

    let ts_ms = u64::try_from(raw.timestamp).context("header timestamp out of range")?;
    let number = u64::try_from(raw.number).context("header number out of range")?;
    let version = u64::try_from(raw.version).context("header version out of range")?;

    let tx_trie_root = raw.tx_trie_root.as_slice();
    if tx_trie_root.len() != 32 {
        anyhow::bail!("unexpected txTrieRoot length: {}", tx_trie_root.len());
    }
    let parent_hash = raw.parent_hash.as_slice();
    if parent_hash.len() != 32 {
        anyhow::bail!("unexpected parentHash length: {}", parent_hash.len());
    }

    let witness_addr = raw.witness_address.as_slice();
    if witness_addr.len() != 21 {
        anyhow::bail!("unexpected witness_address length: {}", witness_addr.len());
    }

    let sig = h.witness_signature.as_slice();
    if sig.len() != 65 {
        anyhow::bail!("unexpected witness_signature length: {}", sig.len());
    }

    // Build the raw_data message (105 bytes).
    let ts_var = encode_varint_fixed(ts_ms, 6)?;
    let num_var = encode_varint_fixed(number, 4)?;
    let ver_var = encode_varint_fixed(version, 1)?;

    let mut raw_data = Vec::with_capacity(105);
    raw_data.push(0x08); // field 1 (timestamp), wire 0
    raw_data.extend_from_slice(&ts_var);
    raw_data.push(0x12); // field 2 (txTrieRoot), wire 2
    raw_data.push(0x20); // length 32
    raw_data.extend_from_slice(tx_trie_root);
    raw_data.push(0x1a); // field 3 (parentHash), wire 2
    raw_data.push(0x20); // length 32
    raw_data.extend_from_slice(parent_hash);
    raw_data.push(0x38); // field 7 (number), wire 0
    raw_data.extend_from_slice(&num_var);
    raw_data.push(0x4a); // field 9 (witness_address), wire 2
    raw_data.push(0x15); // length 21
    raw_data.extend_from_slice(witness_addr);
    raw_data.push(0x50); // field 10 (version), wire 0
    raw_data.extend_from_slice(&ver_var);

    if raw_data.len() != 105 {
        anyhow::bail!("unexpected raw_data length: {}", raw_data.len());
    }

    // Wrap in BlockHeader framing.
    let mut out = Vec::with_capacity(174);
    out.push(0x0a); // field 1 (raw_data), wire 2
    out.push(0x69); // length 105
    out.extend_from_slice(&raw_data);
    out.push(0x12); // field 2 (witness_signature), wire 2
    out.push(0x41); // length 65
    out.extend_from_slice(sig);

    if out.len() != 174 {
        anyhow::bail!("unexpected encoded header length: {}", out.len());
    }
    Ok(out)
}

fn encode_varint_fixed(mut v: u64, expected_len: usize) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    while v >= 0x80 {
        out.push(((v as u8) & 0x7f) | 0x80);
        v >>= 7;
    }
    out.push(v as u8);

    if out.len() != expected_len {
        anyhow::bail!(
            "unexpected varint length: got {}, expected {}",
            out.len(),
            expected_len
        );
    }
    Ok(out)
}

fn merkle_proof_sha256(
    leaves: &[FixedBytes<32>],
    leaf_index: usize,
) -> Result<(Vec<FixedBytes<32>>, U256, FixedBytes<32>)> {
    if leaves.is_empty() {
        anyhow::bail!("empty merkle tree");
    }
    if leaf_index >= leaves.len() {
        anyhow::bail!("leaf index out of range");
    }

    let mut idx = leaf_index;
    let mut level = leaves.to_vec();
    let mut proof = Vec::new();
    let mut index_bits = U256::ZERO;
    let mut bit = 0u32;

    while level.len() > 1 {
        let is_right = (idx & 1) == 1;
        if is_right {
            index_bits |= U256::from(1u64) << bit;
        }

        let sibling_idx = if is_right { idx - 1 } else { idx + 1 };
        let sibling = if sibling_idx < level.len() {
            level[sibling_idx]
        } else {
            // Duplicate last node when odd.
            level[idx]
        };
        proof.push(sibling);

        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        for j in (0..level.len()).step_by(2) {
            let left = level[j];
            let right = if j + 1 < level.len() {
                level[j + 1]
            } else {
                left
            };
            next.push(sha256_concat(left, right));
        }

        idx /= 2;
        bit += 1;
        level = next;
    }

    Ok((proof, index_bits, level[0]))
}

fn sha256_concat(a: FixedBytes<32>, b: FixedBytes<32>) -> FixedBytes<32> {
    let mut hasher = Sha256::new();
    hasher.update(a.as_slice());
    hasher.update(b.as_slice());
    let digest = hasher.finalize();
    FixedBytes::from_slice(&digest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::TronAddress;
    use crate::protocol::{BlockExtention, BlockHeader, Transaction, TransactionExtention};
    use prost::Message;

    #[test]
    fn encode_varint_fixed_rejects_unexpected_len() {
        let err = encode_varint_fixed(0, 6).unwrap_err().to_string();
        assert!(err.contains("unexpected varint length"));
    }

    #[test]
    fn merkle_proof_roundtrips_root_with_index_bits() {
        let leaves = vec![
            sha256_bytes32(b"a"),
            sha256_bytes32(b"b"),
            sha256_bytes32(b"c"),
        ];

        for leaf_index in 0..leaves.len() {
            let (proof, index_bits, root) = merkle_proof_sha256(&leaves, leaf_index).unwrap();
            assert_eq!(proof.len(), 2);

            let mut cur = leaves[leaf_index];
            for (bit, sib) in proof.into_iter().enumerate() {
                let is_right = ((index_bits >> bit) & U256::from(1u64)) == U256::from(1u64);
                cur = if is_right {
                    sha256_concat(sib, cur)
                } else {
                    sha256_concat(cur, sib)
                };
            }

            assert_eq!(cur, root);
        }
    }

    #[test]
    fn encode_block_header_stateful_has_fixed_layout() {
        let ts_ms: u64 = 1u64 << 35;
        let number: u64 = 1u64 << 21;

        let mut tx_trie_root = vec![0u8; 32];
        tx_trie_root[0] = 1;
        let mut parent_hash = vec![0u8; 32];
        parent_hash[0] = 2;

        let mut witness_address = vec![0u8; 21];
        witness_address[0] = TronAddress::MAINNET_PREFIX;
        witness_address[1..].copy_from_slice(&[0x33u8; 20]);

        let raw = crate::protocol::block_header::Raw {
            timestamp: i64::try_from(ts_ms).unwrap(),
            tx_trie_root,
            parent_hash,
            number: i64::try_from(number).unwrap(),
            witness_id: 0,
            witness_address,
            version: 1,
            account_state_root: Vec::new(),
        };

        let header = BlockHeader {
            raw_data: Some(raw),
            witness_signature: vec![0x44u8; 65],
        };

        let out = encode_block_header_stateful(&header).unwrap();
        assert_eq!(out.len(), 174);
        assert_eq!(out[0], 0x0a);
        assert_eq!(out[1], 0x69);
        assert_eq!(out[107], 0x12);
        assert_eq!(out[108], 0x41);
        assert_eq!(&out[109..], vec![0x44u8; 65].as_slice());
    }

    #[test]
    fn extract_tx_and_root_finds_tx_and_returns_header_root() {
        let txid = [0x55u8; 32];
        let header_raw = crate::protocol::block_header::Raw {
            timestamp: 0,
            tx_trie_root: vec![0x66u8; 32],
            parent_hash: vec![0u8; 32],
            number: 0,
            witness_id: 0,
            witness_address: vec![0u8; 21],
            version: 0,
            account_state_root: Vec::new(),
        };
        let header = BlockHeader {
            raw_data: Some(header_raw),
            witness_signature: vec![0u8; 65],
        };

        let tx = Transaction::default();
        let tx_bytes = tx.encode_to_vec();

        let b = BlockExtention {
            transactions: vec![TransactionExtention {
                transaction: Some(tx),
                txid: txid.to_vec(),
                constant_result: Vec::new(),
                result: None,
                energy_used: 0,
                logs: Vec::new(),
                internal_transactions: Vec::new(),
                energy_penalty: 0,
            }],
            block_header: Some(header),
            blockid: Vec::new(),
        };

        let (idx, enc, root) = extract_tx_and_root(&b, txid).unwrap();
        assert_eq!(idx, 0);
        assert_eq!(enc, tx_bytes);
        assert_eq!(root, vec![0x66u8; 32]);
    }
}
