use dep::std::ecdsa_secp256k1;
use keccak256::keccak256;
use sha256::sha256_var;

type Bytes20 = [u8; 20];

struct Block {
    prev_block_id: [u8; 32],
    new_block_id: [u8; 32],
    tx_root: [u8; 32],

    raw_data: [u8; 128],
    raw_data_length: u64,
    public_key: [u8; 64],
    signature: [u8; 64],
}

// Derive a 20-byte identifier from the 64-byte (x||y) pubkey.
// Since your public args are bytes20[27], we need *some* deterministic mapping.
// Here: proposer_id = last 20 bytes of sha256(pubkey_bytes).
fn pubkey_to_id(pubkey: [u8; 64]) -> Bytes20 {
    let digest: [u8; 32] = sha256_var(pubkey, 64);

    let mut out: Bytes20 = [0; 20];
    for i in 0..20 {
        out[i] = digest[12 + i];
    }
    out
}

fn read_varint_u64(bytes: [u8; 128], mut offset: u32) -> (u64, u32) {
    let mut value: u64 = 0;
    let mut done = false;

    // protobuf varints are up to 10 bytes for u64
    for i in 0..10 {
        if !done {
            let b: u8 = bytes[offset];
            let chunk: u64 = (b & 0x7f) as u64;
            value = value + (chunk << ((i as u64) * 7u64));
            done = (b & 0x80) == 0;
            offset = offset + 1;
        }
    }

    assert(done); // must terminate within 10 bytes
    (value, offset)
}

fn verify_proposer(
    block: Block,
    srs: [[u8; 20]; 27],
    witness_delegatees: [[u8; 20]; 27],
    used: &mut [bool; 27],
) {
    let proposer_id = pubkey_to_id(block.public_key);

    let mut allowed = false;
    for i in 0..27 {
        // allow if in either list at index i (and only “use” that index once)
        let matches = (proposer_id == srs[i]) | (proposer_id == witness_delegatees[i]);
        if (!allowed) & matches {
            assert(!used[i]);
            used[i] = true;
            allowed = true;
        }
    }
    assert(allowed);
}

// Parses & verifies header layout.
// Returns the parsed timestamp (field 1) so main() can bind endingBlockTimestamp.
fn verify_header_and_get_timestamp(block: Block, hash: [u8; 32]) -> u64 {
    let mut offset: u32 = 0;

    // field 1: timestamp (VARINT)
    assert((block.raw_data[offset] & 7) == 0);
    assert((block.raw_data[offset] >> 3) == 1);
    offset = offset + 1;

    let (timestamp, new_offset) = read_varint_u64(block.raw_data, offset);
    offset = new_offset;

    // field 2: txroot (LEN) length should be 32
    assert((block.raw_data[offset] & 7) == 2);
    assert((block.raw_data[offset] >> 3) == 2);
    offset = offset + 1;

    let tx_len: u8 = block.raw_data[offset];
    assert(tx_len == 32);
    offset = offset + 1;

    for i in 0..32 {
        assert(block.tx_root[i] == block.raw_data[offset]);
        offset = offset + 1;
    }

    // field 3: prevblockhash (LEN) length should be 32
    assert((block.raw_data[offset] & 7) == 2);
    assert((block.raw_data[offset] >> 3) == 3);
    offset = offset + 1;

    let prev_len: u8 = block.raw_data[offset];
    assert(prev_len == 32);
    offset = offset + 1;

    for i in 0..32 {
        assert(block.prev_block_id[i] == block.raw_data[offset]);
        offset = offset + 1;
    }

    // field 7: block number (VARINT)
    assert((block.raw_data[offset] & 7) == 0);
    assert((block.raw_data[offset] >> 3) == 7);
    offset = offset + 1;

    let (block_number, _) = read_varint_u64(block.raw_data, offset);

    // new_block_id encoding: first 8 bytes are big-endian block number,
    // remaining 24 bytes must match hash[8..32]
    let mut asserted_number: u64 = 0;
    for i in 0..8 {
        asserted_number = asserted_number + ((block.new_block_id[i] as u64) << (56 - (8 * i) as u64));
    }
    assert(asserted_number == block_number);

    for i in 8..32 {
        assert(block.new_block_id[i] == hash[i]);
    }

    timestamp
}

fn verify_signature(block: Block, hash: [u8; 32]) {
    let mut pub_key_x: [u8; 32] = [0; 32];
    let mut pub_key_y: [u8; 32] = [0; 32];

    for i in 0..32 {
        pub_key_x[i] = block.public_key[i];
        pub_key_y[i] = block.public_key[i + 32];
    }

    assert(ecdsa_secp256k1::verify_signature(pub_key_x, pub_key_y, block.signature, hash));
}

// for debug/structure parity with your original
unconstrained fn create_hash(block: Block) -> [u8; 32] {
    sha256_var(block.raw_data, block.raw_data_length)
}

fn verify_hash(block: Block, hash: [u8; 32]) {
    assert(sha256_var(block.raw_data, block.raw_data_length) == hash);
}

fn main(
    srs: pub [Bytes20; 27],
    witnessDelegatees: pub [Bytes20; 27],
    startingBlock: pub [u8; 32],
    endingBlock: pub [u8; 32],
    endingBlockTxTrieRoot: pub [u8; 32],
    endingBlockTimestamp: pub u32,

    // --- private witness ---
    blocks: [Block; 18],
) {
    let mut used_proposers: [bool; 27] = [false; 27];

    // Range binding
    assert(startingBlock == blocks[0].prev_block_id);
    assert(endingBlock == blocks[17].new_block_id);

    for i in 1..18 {
        assert(blocks[i].prev_block_id == blocks[i - 1].new_block_id);
    }

    // Bind “ending” values
    assert(blocks[17].tx_root == endingBlockTxTrieRoot);

    for i in 0..18 {
        let block = blocks[i];

        verify_proposer(block, srs, witnessDelegatees, &mut used_proposers);

        let hash = create_hash(block);
        let ts = verify_header_and_get_timestamp(block, hash);

        verify_hash(block, hash);
        verify_signature(block, hash);

        if i == 17 {
            assert(ts == (endingBlockTimestamp as u64));
        }
    }
}
